# Project Overview
## Purpose 
The purpose of this project is to be able to run the Vulkan test suite on a Google Pixel Fold running Alpine Linux, using a VGPU. 

## Background 
A VGPU system is necessary as the Pixel Fold does runs Bionic libc rather than GNU libc. A VGPU system allows a user to queue GPU commands from one device to another when the devices have different libc versions.  The tools that make up the VGPU interface are: GFXstream, Mesa, and Rutabaga. 

GFXstream is a the engine which helps to forward graphics API calls between the host and guest device.

Mesa is the 

Rutabaga is the full abstraction layer which handles the dispatching of the GPU API calls and the display visualization. Below is the diagram of Rutabaga.

![Rutabaga Diagram](docs/rutabaga_document.png)


## Project MVP 
The MVP of our project is to display the interaction between the host and guest gfxstream applications. In our case Kumquat Media server is the host application and vkcube is the guest application. 


# Building the VGPU

## Termux, Android, Alpine Linux and more 
While our current Pixel Build runs Alpine Linux currently the executables for Rutabaga, libaemu, and gfxstream need to be built in Android OS to link properly against Bionic. The reason for this is that the some of the libraries required are not available in Alpine Linux so the executables fail to build. The workaround is to compile on Android OS, pull the executables to the Linux build, and copy the necessary shared object files into /lib/system64.

## Setting up Termux on Android OS  
1. Disable built-in Linux on Android
    1. Go to Settings -> System -> Developer Options -> Linux development environment
    2. Move the switch to the off position
2. Navigate to this [website](https://f-droid.org/en/) 
3. Click on `Download F-Droid` button
4. Go to the Download and enable installation of apk from Chrome
5. open F-Droid and search for Termux
6. Install Termux
    1. When warned that Termux is meant for an older version of Android, press `more information` then `Install anyway`
7. Start Termux and in notifications press `Acquire wakelock`
8. Now run the following commands in termux `pkg update; pkg upgrade; pkg install tur-repo; pkg install code-server`
9. To launch code server run: `code-server --auth none --bind-addr 0.0.0.0:8080`
10. You can now navigate to `https://<device IPv4 addr>:8080` to view the VScode instance and terminal of the phone

## Setting up your environment on the Alpine Linux Build
1. If this is the first time booting the phone you will need to flash the phone. You can find instructions on this in the Vulkan support google drive. 
2. To connect to the phone you can either communicate via Serial or by plugging the phone into the router and ssh'ing in. For MacOS users the router is the only option. Windows users may change their Internet Sharing IP subnet to 172.16.42.1-255 to enable internet sharing with the Linux build 
3. After ssh'ing into the Linux phone, you will need to partition a large enough drive to hold the repo (~300 MB). To create a new partition: 
    1. `sudo mkdir /<name of dir where you want to make your mount>`
    2. `sudo mkfs.ext4 /dev/sdaX` where X can be any integer between 1 and 31 as long as nothing is mounted on that sda, you can check this by running `mount`
    3. `sudo mount -t auto /dev/sdaX /<same dir you just made>`
    4. Now make a directory in `~` and run `sudo mount -t auto /dev/sdaX ~/<new dir>`
    5. This new directory will be where you work going forward, run `chmod 777 <new dir>` to make it rw+ permissions 
4. You now need to remove iptables and add back iptables using `sudo apk del iptables` followed by `sudo apk add iptables` and then `sudo apk add git`
5. You can now configure a ssh key and link it to your github account
6. Now clone our repo. 

## Building the Application on Android OS
1. Clone this repo
2. (optional) Cut your own branch 
3. Build AEMU 
    ```
    cd aemu
    cmake -DAEMU_COMMON_GEN_PKGCONFIG=ON \
            -DAEMU_COMMON_BUILD_CONFIG=gfxstream \
            -DENABLE_VKCEREAL_TESTS=OFF -B build
    cmake --build build -j
    sudo cmake --install build
    ```
4. Build gfxstream host 
    ```
    cd gfxstream/
    meson setup host-build/
    meson install -C host-build/
    ```
5. Install FFI bindings for Rutabaga 
    ```
    cd $(crosvm_dir)/rutabaga_gfx/ffi/
    meson setup rutabaga-ffi-build/
    meson install -C rutabaga-ffi-build/
    ```
6. (optional) push your binaries to your repo branch


## Necessary Android Dependencies

## Necessary Linux Dependencies

## Running the executables 
1. To run both executables on the same device, consider using a tool like tmux
2. You can execute kumquat media server by navigating to crosvm/rutabaga_gfx/kumquat/server/ and executing `./target/debug/kumquat`
3. You can execute vkcube by: 
```
    export MESA_LOADER_DRIVER_OVERRIDE=zink
    export VIRTGPU_KUMQUAT=1
    export VK_ICD_FILENAMES=$(mesa_dir)/guest-build/src/gfxstream/guest/vulkan/gfxstream_vk_devenv_icd.x86_64.json
    vkcube
```
# Resources, Repositories, and External Tools used in this Repo

AEMU:           https://android.googlesource.com/platform/hardware/google/aemu
GFXStream:      https://android.googlesource.com/platform/hardware/google/gfxstream
Mesa:           https://gitlab.freedesktop.org/mesa/mesa.git 


