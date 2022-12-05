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

2. Install xargo
   ```
   cargo install xargo
   ```

3. Install [Arm GNU Toolchain](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain)
   Add the installation path to your PATH environment variable. The installer adds it to the user's path, but put it in the system's path, because some programs might not find it.

4. Clone the repository using Git *(`--recurse-submodules` is important!)*
   ```
   git clone --recurse-submodules --depth 1 https://github.com/SwanX1/alean.git
   ```

### Compiling

Compile using [`./build.ps1`](./build.ps1) (raw commands are in the [`build`](./build) file)<br>
The compiled files are in the `target` directory.

To use this in a Raspberry PI, just format an SD card with a FAT32 partition (see [Raspberry Pi's documentation](https://www.raspberrypi.com/documentation/computers/getting-started.html#sd-cards)), place everything from [`firmware`](./firmware/) into that partition as well as the `kernel.img` file you've compiled.

It's as easy as pie! *(hehe get it?)*

### License
This project is licensed under [GNU Affero General Public License v3.0](./LICENSE) for now.

Files in the `firmware` directory are under [Broadcom's license](./LICENSE.broadcom), as it is pre-compiled firmware to boot the Raspberry PI.<br>
Firmware files are taken from the [raspberrypi/firmware](https://github.com/raspberrypi/firmware) repository. View [SwanX1/alean-firmware](https://github.com/SwanX1/alean-firmware) for more information.

If distributing ISO or any other archives containing this software, please make sure to include the license files in the archive as well.
