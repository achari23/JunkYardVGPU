# Copyright © Microsoft Corporation
# SPDX-License-Identifier: MIT

libd3d12_compiler_args = []

libd3d12_compiler_args += cc.get_supported_arguments([
    '/we4267',
    '/we4996',
    '/we4146',
    '/we4244',
    '/we4389',
    '/we4838',
    '/we4302',
    '/we4018',
    '/we4056',
    '/we4305',
    '/we4351',
    '/we4756',
    '/we4800',
    '/we4291',
    '/we4624',
    '/we4309',
    '/we5105',
    '/we4020',
    '/we4024',
    '/we4189',
  ])

_with_gallium_d3d12_graphics = get_option('gallium-d3d12-graphics')
with_gallium_d3d12_graphics = false
if not _with_gallium_d3d12_graphics.disabled()
  with_gallium_d3d12_graphics = true
  libd3d12_compiler_args += '-DHAVE_GALLIUM_D3D12_GRAPHICS'
endif

if not with_gallium_d3d12_video and not with_gallium_d3d12_graphics
  error('d3d12 gallium driver must have at least one of gallium-d3d12-video or gallium-d3d12-graphics enabled.')
endif

files_libd3d12 = files(
  'd3d12_batch.cpp',
  'd3d12_copy.cpp',
  'd3d12_bufmgr.cpp',
  'd3d12_context_common.cpp',
  'd3d12_fence.cpp',
  'd3d12_format.c',
  'd3d12_residency.cpp',
  'd3d12_resource.cpp',
  'd3d12_resource_state.cpp',
  'd3d12_screen.cpp',
)

if with_gallium_d3d12_graphics
  files_libd3d12 += [
    'd3d12_blit.cpp',
    'd3d12_draw.cpp',
    'd3d12_surface.cpp',
    'd3d12_context_graphics.cpp',
    'd3d12_cmd_signature.cpp',
    'd3d12_compiler.cpp',
    'd3d12_compute_transforms.cpp',
    'd3d12_descriptor_pool.cpp',
    'd3d12_nir_passes.c',
    'd3d12_pipeline_state.cpp',
    'd3d12_query.cpp',
    'd3d12_root_signature.cpp',
    'd3d12_gs_variant.cpp',
    'd3d12_lower_image_casts.c',
    'd3d12_lower_point_sprite.c',
    'd3d12_tcs_variant.cpp',
  ]
endif

if with_gallium_d3d12_video
  files_libd3d12 += [
    'd3d12_video_dec.cpp',
    'd3d12_video_dec_references_mgr.cpp',
    'd3d12_video_buffer.cpp',
    'd3d12_video_enc.cpp',
    'd3d12_video_encoder_bitstream.cpp',
    'd3d12_video_texture_array_dpb_manager.cpp',
    'd3d12_video_array_of_textures_dpb_manager.cpp',
    'd3d12_video_screen.cpp',
    'd3d12_video_proc.cpp',
  ]

if _codecs.contains('h264dec')
  files_libd3d12 += [
    'd3d12_video_dec_h264.cpp',
  ]
endif

if _codecs.contains('h265dec')
  files_libd3d12 += [
    'd3d12_video_dec_hevc.cpp',
  ]
endif

if _codecs.contains('av1dec')
  files_libd3d12 += [
    'd3d12_video_dec_av1.cpp',
  ]
endif

if _codecs.contains('vp9dec')
  files_libd3d12 += [
    'd3d12_video_dec_vp9.cpp',
  ]
endif

if _codecs.contains('h264enc')
  files_libd3d12 += [
    'd3d12_video_enc_h264.cpp',
    'd3d12_video_encoder_references_manager_h264.cpp',
    'd3d12_video_encoder_nalu_writer_h264.cpp',
    'd3d12_video_encoder_bitstream_builder_h264.cpp',
  ]
endif

if _codecs.contains('h265enc')
  files_libd3d12 += [
    'd3d12_video_enc_hevc.cpp',
    'd3d12_video_encoder_bitstream_builder_hevc.cpp',
    'd3d12_video_encoder_nalu_writer_hevc.cpp',
    'd3d12_video_encoder_references_manager_hevc.cpp',
  ]
endif

if _codecs.contains('av1enc')
  files_libd3d12 += [
    'd3d12_video_enc_av1.cpp',
    'd3d12_video_encoder_references_manager_av1.cpp',
    'd3d12_video_encoder_bitstream_builder_av1.cpp',
  ]
endif

endif # with_gallium_d3d12_video

is_xbox = target_machine.system().startswith('Gaming.Xbox')
if host_machine.system() == 'windows'
  if is_xbox
    files_libd3d12 += files('d3d12_dxgi_xbox_screen.cpp')
  else
    files_libd3d12 += files('d3d12_dxgi_screen.cpp')
  endif
endif
if is_xbox == false
  files_libd3d12 += files('d3d12_dxcore_screen.cpp')
endif

libd3d12_deps = [dep_dxheaders, idep_mesautil]
if with_gallium_d3d12_graphics
  libd3d12_deps += [idep_nir_headers, idep_libdxil_compiler]
endif

libd3d12 = static_library(
  'd3d12',
  [files_libd3d12, sha1_h],
  gnu_symbol_visibility : 'hidden',
  include_directories : [inc_include, inc_src, inc_mesa, inc_gallium, inc_gallium_aux],
  dependencies: libd3d12_deps,
  c_args: libd3d12_compiler_args,
  cpp_args: libd3d12_compiler_args,
)

driver_d3d12 = declare_dependency(
  compile_args : '-DGALLIUM_D3D12',
  link_with : [libd3d12],
)
