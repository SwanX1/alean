### ALEAN
ALEAN is an operating system for the Raspberry Pi Zero W (*BCM2835* chip)

Since this is written for the *BCM2835* chip, it should work on Raspberry Pi 1 Models A,
A+, B, B+, the Raspberry Pi Zero, the Raspberry Pi Zero W, and the Raspberry Pi Compute
Module 1.

### Workspace Setup
1. Install and configure Rustup
   ```
   curl https://sh.rustup.rs -sSf | sh
   rustup override set nightly
   rustup component add rust-src
   ```

2. Install [Arm GNU Toolchain](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain)
   Add the installation path to your PATH environment variable. The installer adds it to the user's path, but put it in the system's path, because some programs might not find it.

3. Clone this repository using Git *(`--recurse-submodules` is important!)*
   ```
   git clone --recurse-submodules --depth 1 https://github.com/SwanX1/alean.git
   ```
   
    - Note: If you've cloned this repository without `--recurse-submodules`, you can run the following git commands to clone the `firmware` directory as well.
     ```
     git submodule init
     git submodule update --depth=1
     ```

### Compiling
Compile using [`./build.sh`](./build.sh)
The files needed on the SD card are in the `build` directory.

Note: If you're doing active development, run the script with a `--no-release` flag, which doesn't optimize the binary (along with Rust's STD).

To use this in a Raspberry PI, just format an SD card with a FAT32 partition (see [Raspberry Pi's documentation](https://www.raspberrypi.com/documentation/computers/getting-started.html#sd-cards)), place everything from `build` into that partition. All files necessary for booting are also automatically copied into the `build` directory.

It's as easy as pie! *(hehe get it?)*

### License
View [`attribution`](./attribution/) for more information.

### Contact
You may contact me about the project via e-mail: karliscern [at] gmail [dot] com.
