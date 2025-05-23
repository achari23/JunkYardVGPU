// Copyright 2019 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::mem::size_of;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Context;
use base::error;
use base::AsRawDescriptor;
use base::Error as SysError;
use base::Event;
use base::RawDescriptor;
use base::Result as SysResult;
use base::Timer;
use base::Tube;
use base::TubeError;
use base::WorkerThread;
use cros_async::select2;
use cros_async::select3;
use cros_async::AsyncError;
use cros_async::EventAsync;
use cros_async::Executor;
use cros_async::TimerAsync;
use data_model::Le32;
use data_model::Le64;
use futures::pin_mut;
use remain::sorted;
use snapshot::AnySnapshot;
use thiserror::Error;
use vm_control::MemSlot;
use vm_control::VmMemoryMappingRequest;
use vm_control::VmMemoryMappingResponse;
use vm_memory::GuestAddress;
use vm_memory::GuestMemory;
use zerocopy::FromBytes;
use zerocopy::Immutable;
use zerocopy::IntoBytes;
use zerocopy::KnownLayout;

use super::async_utils;
use super::copy_config;
use super::DescriptorChain;
use super::DeviceType;
use super::Interrupt;
use super::Queue;
use super::VirtioDevice;

const QUEUE_SIZE: u16 = 256;
const QUEUE_SIZES: &[u16] = &[QUEUE_SIZE];

/* Feature bits */
const VIRTIO_PMEM_F_DISCARD: u32 = 63;

const VIRTIO_PMEM_REQ_TYPE_FLUSH: u32 = 0;
const VIRTIO_PMEM_REQ_TYPE_DISCARD: u32 = u32::MAX;
const VIRTIO_PMEM_RESP_TYPE_OK: u32 = 0;
const VIRTIO_PMEM_RESP_TYPE_EIO: u32 = 1;

#[derive(Copy, Clone, Debug, Default, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
struct virtio_pmem_config {
    start_address: Le64,
    size: Le64,
}

#[derive(Copy, Clone, Debug, Default, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
struct virtio_pmem_resp {
    status_code: Le32,
}

#[derive(Copy, Clone, Debug, Default, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
struct virtio_pmem_req {
    type_: Le32,
}

#[derive(Copy, Clone, Debug, Default, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
struct virtio_pmem_range_req {
    type_: Le32,
    padding_: Le32,
    start_address: Le64,
    size: Le64,
}

#[sorted]
#[derive(Error, Debug)]
enum Error {
    /// Failed to get value from pageout timer.
    #[error("failed to get value from pageout timer: {0}")]
    PageoutTimer(AsyncError),
    /// Failed to read from virtqueue.
    #[error("failed to read from virtqueue: {0}")]
    ReadQueue(io::Error),
    /// Failed to receive tube response.
    #[error("failed to receive tube response: {0}")]
    ReceiveResponse(TubeError),
    /// Failed to send tube request.
    #[error("failed to send tube request: {0}")]
    SendingRequest(TubeError),
    /// Failed to write to virtqueue.
    #[error("failed to write to virtqueue: {0}")]
    WriteQueue(io::Error),
}

type Result<T> = ::std::result::Result<T, Error>;

async fn pageout(
    ex: &Executor,
    swap_interval: Duration,
    pmem_device_tube: &Tube,
    mapping_arena_slot: u32,
    mapping_size: usize,
) -> Result<()> {
    let timer = Timer::new().expect("Failed to create a timer");
    let mut pageout_timer =
        TimerAsync::new(timer, ex).expect("Failed to create an async pageout timer");
    pageout_timer
        .reset_repeating(swap_interval)
        .expect("Failed to reset pageout timer");

    loop {
        pageout_timer.wait().await.map_err(Error::PageoutTimer)?;
        let request = VmMemoryMappingRequest::MadvisePageout {
            slot: mapping_arena_slot,
            offset: 0,
            size: mapping_size,
        };

        pmem_device_tube
            .send(&request)
            .map_err(Error::SendingRequest)?;
        match pmem_device_tube
            .recv::<VmMemoryMappingResponse>()
            .map_err(Error::ReceiveResponse)?
        {
            VmMemoryMappingResponse::Ok => {}
            VmMemoryMappingResponse::Err(e) => {
                error!("failed to page out the memory mapping: {}", e);
            }
        };
    }
}

fn execute_request(
    request_type: u32,
    start_address: u64,
    size: u64,
    pmem_device_tube: &Tube,
    mapping_arena_slot: u32,
    mapping_size: usize,
) -> u32 {
    match request_type {
        VIRTIO_PMEM_REQ_TYPE_FLUSH => {
            let request = VmMemoryMappingRequest::MsyncArena {
                slot: mapping_arena_slot,
                offset: 0, // The pmem backing file is always at offset 0 in the arena.
                size: mapping_size,
            };

            if let Err(e) = pmem_device_tube.send(&request) {
                error!("failed to send request: {}", e);
                return VIRTIO_PMEM_RESP_TYPE_EIO;
            }

            match pmem_device_tube.recv() {
                Ok(response) => match response {
                    VmMemoryMappingResponse::Ok => VIRTIO_PMEM_RESP_TYPE_OK,
                    VmMemoryMappingResponse::Err(e) => {
                        error!("failed flushing disk image: {}", e);
                        VIRTIO_PMEM_RESP_TYPE_EIO
                    }
                },
                Err(e) => {
                    error!("failed to receive data: {}", e);
                    VIRTIO_PMEM_RESP_TYPE_EIO
                }
            }
        }

        VIRTIO_PMEM_REQ_TYPE_DISCARD => {
            let request = VmMemoryMappingRequest::MadviseRemove {
                slot: mapping_arena_slot,
                offset: usize::try_from(start_address).unwrap(),
                size: usize::try_from(size).unwrap(),
            };

            if let Err(e) = pmem_device_tube.send(&request) {
                error!("failed to send request: {}", e);
                return VIRTIO_PMEM_RESP_TYPE_EIO;
            }

            match pmem_device_tube.recv() {
                Ok(response) => match response {
                    VmMemoryMappingResponse::Ok => VIRTIO_PMEM_RESP_TYPE_OK,
                    VmMemoryMappingResponse::Err(e) => {
                        error!("failed to discard memory range: {}", e);
                        VIRTIO_PMEM_RESP_TYPE_EIO
                    }
                },
                Err(e) => {
                    error!("failed to receive data: {}", e);
                    VIRTIO_PMEM_RESP_TYPE_EIO
                }
            }
        }

        _ => {
            error!("unknown request type: {}", request_type);
            VIRTIO_PMEM_RESP_TYPE_EIO
        }
    }
}

fn handle_request(
    avail_desc: &mut DescriptorChain,
    pmem_device_tube: &Tube,
    mapping_arena_slot: u32,
    mapping_size: usize,
) -> Result<usize> {
    let (request_type, start_address, size) =
        if avail_desc.reader.available_bytes() == size_of::<virtio_pmem_req>() {
            let request = avail_desc
                .reader
                .read_obj::<virtio_pmem_req>()
                .map_err(Error::ReadQueue)?;
            (request.type_.to_native(), 0, 0)
        } else {
            let request = avail_desc
                .reader
                .read_obj::<virtio_pmem_range_req>()
                .map_err(Error::ReadQueue)?;
            (
                request.type_.to_native(),
                request.start_address.to_native(),
                request.size.to_native(),
            )
        };
    let status_code = execute_request(
        request_type,
        start_address,
        size,
        pmem_device_tube,
        mapping_arena_slot,
        mapping_size,
    );

    let response = virtio_pmem_resp {
        status_code: status_code.into(),
    };

    avail_desc
        .writer
        .write_obj(response)
        .map_err(Error::WriteQueue)?;

    Ok(avail_desc.writer.bytes_written())
}

async fn handle_queue(
    queue: &mut Queue,
    mut queue_event: EventAsync,
    pmem_device_tube: &Tube,
    mapping_arena_slot: u32,
    mapping_size: usize,
) {
    loop {
        let mut avail_desc = match queue.next_async(&mut queue_event).await {
            Err(e) => {
                error!("Failed to read descriptor {}", e);
                return;
            }
            Ok(d) => d,
        };

        let written = match handle_request(
            &mut avail_desc,
            pmem_device_tube,
            mapping_arena_slot,
            mapping_size,
        ) {
            Ok(n) => n,
            Err(e) => {
                error!("pmem: failed to handle request: {}", e);
                0
            }
        };
        queue.add_used_with_bytes_written(avail_desc, written as u32);
        queue.trigger_interrupt();
    }
}

fn run_worker(
    queue: &mut Queue,
    pmem_device_tube: &Tube,
    kill_evt: Event,
    mapping_arena_slot: u32,
    mapping_size: usize,
    swap_interval: Option<Duration>,
) {
    let ex = Executor::new().unwrap();

    let queue_evt = queue
        .event()
        .try_clone()
        .expect("failed to clone queue event");
    let queue_evt = EventAsync::new(queue_evt, &ex).expect("failed to set up the queue event");

    // Process requests from the virtio queue.
    let queue_fut = handle_queue(
        queue,
        queue_evt,
        pmem_device_tube,
        mapping_arena_slot,
        mapping_size,
    );
    pin_mut!(queue_fut);

    // Exit if the kill event is triggered.
    let kill = async_utils::await_and_exit(&ex, kill_evt);
    pin_mut!(kill);

    let interval = swap_interval.unwrap_or(Duration::ZERO);
    if interval.is_zero() {
        if let Err(e) = ex.run_until(select2(queue_fut, kill)) {
            error!("error happened in executor: {}", e);
        }
    } else {
        let pageout_fut = pageout(
            &ex,
            interval,
            pmem_device_tube,
            mapping_arena_slot,
            mapping_size,
        );
        pin_mut!(pageout_fut);
        if let Err(e) = ex.run_until(select3(queue_fut, kill, pageout_fut)) {
            error!("error happened in executor: {}", e);
        }
    }
}

/// Specifies how memory slot is initialized.
pub enum MemSlotConfig {
    /// The memory region has already been mapped to the guest.
    MemSlot {
        /// index of the guest-mapped memory regions.
        idx: MemSlot,
    },
    /// The memory region that is not initialized yet and whose slot index will be provided via
    /// `Tube` later. e.g. pmem-ext2 device, where fs construction will be done in the main
    /// process.
    LazyInit { tube: Tube },
}

pub struct Pmem {
    worker_thread: Option<WorkerThread<(Queue, Tube)>>,
    features: u64,
    disk_image: Option<File>,
    mapping_address: GuestAddress,
    mem_slot: MemSlotConfig,
    mapping_size: u64,
    pmem_device_tube: Option<Tube>,
    swap_interval: Option<Duration>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PmemSnapshot {
    mapping_address: GuestAddress,
    mapping_size: u64,
}

/// Configuration of a virtio-pmem device.
pub struct PmemConfig {
    /// Disk image exposed to the guest.
    /// If the memory region is not backed by a file, this should be `None`.
    pub disk_image: Option<File>,
    /// Guest physical address where the memory will be mapped.
    pub mapping_address: GuestAddress,
    pub mem_slot: MemSlotConfig,
    /// The size of the mapped region.
    pub mapping_size: u64,
    /// A communication channel to the main process to send memory requests.
    pub pmem_device_tube: Tube,
    /// Interval for periodic swap out of memory mapping
    pub swap_interval: Option<Duration>,
    /// Whether the region is writeble or not.
    pub mapping_writable: bool,
}

impl Pmem {
    pub fn new(base_features: u64, cfg: PmemConfig) -> SysResult<Pmem> {
        if cfg.mapping_size > usize::MAX as u64 {
            return Err(SysError::new(libc::EOVERFLOW));
        }

        let mut avail_features = base_features;
        if cfg.mapping_writable {
            if let MemSlotConfig::LazyInit { .. } = cfg.mem_slot {
                error!("pmem-ext2 must be a read-only device");
                return Err(SysError::new(libc::EINVAL));
            }

            avail_features |= 1 << VIRTIO_PMEM_F_DISCARD;
        }

        Ok(Pmem {
            worker_thread: None,
            features: avail_features,
            disk_image: cfg.disk_image,
            mapping_address: cfg.mapping_address,
            mem_slot: cfg.mem_slot,
            mapping_size: cfg.mapping_size,
            pmem_device_tube: Some(cfg.pmem_device_tube),
            swap_interval: cfg.swap_interval,
        })
    }
}

impl VirtioDevice for Pmem {
    fn keep_rds(&self) -> Vec<RawDescriptor> {
        let mut keep_rds = Vec::new();
        if let Some(disk_image) = &self.disk_image {
            keep_rds.push(disk_image.as_raw_descriptor());
        }

        if let Some(ref pmem_device_tube) = self.pmem_device_tube {
            keep_rds.push(pmem_device_tube.as_raw_descriptor());
        }

        if let MemSlotConfig::LazyInit { tube } = &self.mem_slot {
            keep_rds.push(tube.as_raw_descriptor());
        }

        keep_rds
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Pmem
    }

    fn queue_max_sizes(&self) -> &[u16] {
        QUEUE_SIZES
    }

    fn features(&self) -> u64 {
        self.features
    }

    fn read_config(&self, offset: u64, data: &mut [u8]) {
        let config = virtio_pmem_config {
            start_address: Le64::from(self.mapping_address.offset()),
            size: Le64::from(self.mapping_size),
        };
        copy_config(data, 0, config.as_bytes(), offset);
    }

    fn activate(
        &mut self,
        _memory: GuestMemory,
        _interrupt: Interrupt,
        mut queues: BTreeMap<usize, Queue>,
    ) -> anyhow::Result<()> {
        if queues.len() != 1 {
            return Err(anyhow!("expected 1 queue, got {}", queues.len()));
        }

        let mut queue = queues.remove(&0).unwrap();

        // We checked that this fits in a usize in `Pmem::new`.
        let mapping_size = self.mapping_size as usize;

        let pmem_device_tube = self
            .pmem_device_tube
            .take()
            .context("missing pmem device tube")?;

        let swap_interval = self.swap_interval;

        let mapping_arena_slot = match &self.mem_slot {
            MemSlotConfig::MemSlot { idx } => *idx,
            MemSlotConfig::LazyInit { tube } => tube
                .recv::<u32>()
                .context("failed to receive memory slot for ext2 pmem device")?,
        };

        self.worker_thread = Some(WorkerThread::start("v_pmem", move |kill_event| {
            run_worker(
                &mut queue,
                &pmem_device_tube,
                kill_event,
                mapping_arena_slot,
                mapping_size,
                swap_interval,
            );
            (queue, pmem_device_tube)
        }));

        Ok(())
    }

    fn reset(&mut self) -> anyhow::Result<()> {
        if let Some(worker_thread) = self.worker_thread.take() {
            let (_queue, pmem_device_tube) = worker_thread.stop();
            self.pmem_device_tube = Some(pmem_device_tube);
        }
        Ok(())
    }

    fn virtio_sleep(&mut self) -> anyhow::Result<Option<BTreeMap<usize, Queue>>> {
        if let Some(worker_thread) = self.worker_thread.take() {
            let (queue, pmem_device_tube) = worker_thread.stop();
            self.pmem_device_tube = Some(pmem_device_tube);
            return Ok(Some(BTreeMap::from([(0, queue)])));
        }
        Ok(None)
    }

    fn virtio_wake(
        &mut self,
        queues_state: Option<(GuestMemory, Interrupt, BTreeMap<usize, Queue>)>,
    ) -> anyhow::Result<()> {
        if let Some((mem, interrupt, queues)) = queues_state {
            self.activate(mem, interrupt, queues)?;
        }
        Ok(())
    }

    fn virtio_snapshot(&mut self) -> anyhow::Result<AnySnapshot> {
        AnySnapshot::to_any(PmemSnapshot {
            mapping_address: self.mapping_address,
            mapping_size: self.mapping_size,
        })
        .context("failed to serialize pmem snapshot")
    }

    fn virtio_restore(&mut self, data: AnySnapshot) -> anyhow::Result<()> {
        let snapshot: PmemSnapshot =
            AnySnapshot::from_any(data).context("failed to deserialize pmem snapshot")?;
        anyhow::ensure!(
            snapshot.mapping_address == self.mapping_address
                && snapshot.mapping_size == self.mapping_size,
            "pmem snapshot doesn't match config: expected {:?}, got {:?}",
            (self.mapping_address, self.mapping_size),
            (snapshot.mapping_address, snapshot.mapping_size),
        );
        Ok(())
    }
}
