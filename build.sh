#!/bin/bash

# For my fellow Windows users:
# Unfortunately, bash on Windows may not work as expected.
# Use WSL2 (Windows Subsystem for Linux) instead, or a Linux VM to run this script.
# If you know what you're doing, you can try to run the commands manually in PowerShell or CMD.
# The main commands here are:
#   RUST_TARGET_PATH="C:\path\to\this\repo" cargo +nightly build -Zbuild-std=core,compiler_builtins --target armv6k-none-eabihf $RELEASE_FLAG
#   arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c src/boot.s -o target/boot.o
#   arm-none-eabi-gcc -T linker.ld -o target/kernel.elf -z noexecstack -ffreestanding -O2 -nostdlib target/boot.o target/armv6k-none-eabihf/release/libalean.rlib
#   arm-none-eabi-objcopy target/kernel.elf -O binary target/kernel.img
# Replace "C:\path\to\this\repo" with the actual path to this repository on your system.
# This is a workaround, because cargo doesn't pass this environment variable to rustc. See: https://github.com/japaric/xargo/issues/44#issuecomment-324730919

# Shortcircuit if any command fails
set -e

# Check for required tools
UNINSTALLED_TOOLS=()
for tool in cargo arm-none-eabi-gcc arm-none-eabi-objcopy
do
    if ! command -v $tool &> /dev/null
    then
        UNINSTALLED_TOOLS+=($tool)
    fi
done
if [ ${#UNINSTALLED_TOOLS[@]} -ne 0 ]; then
    echo "The following required tools are not installed:"
    for tool in "${UNINSTALLED_TOOLS[@]}"
    do
        echo "- $tool"
    done
    
    # Check if arm-none-eabi-gcc or arm-none-eabi-objcopy is missing specifically
    if [[ " ${UNINSTALLED_TOOLS[@]} " =~ " arm-none-eabi-gcc " ]] || [[ " ${UNINSTALLED_TOOLS[@]} " =~ " arm-none-eabi-objcopy " ]]; then
        echo ""
        echo "You can install the Arm GNU Toolchain from https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain"
        echo "Make sure to add the installation path to your PATH environment variable."
    fi

    exit 1
fi

# Check for --no-release flag
RELEASE_FLAG="--release"
for arg in "$@"
do
    if [ "$arg" == "--no-release" ]; then
        RELEASE_FLAG=""
    fi
done

# Build the kernel
RUST_TARGET_PATH=$(pwd) cargo +nightly build -Zbuild-std=core,compiler_builtins --target armv6k-none-eabihf $RELEASE_FLAG

# Compile the boot assembly
arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c src/boot.s -o target/boot.o

# Link the kernel and boot files into a single ELF
arm-none-eabi-gcc -T linker.ld -o target/kernel.elf -z noexecstack -ffreestanding -O2 -nostdlib target/boot.o target/armv6k-none-eabihf/release/libalean.rlib

# Convert the ELF to a binary image
arm-none-eabi-objcopy target/kernel.elf -O binary target/kernel.img

# Clear out the build directory
rm -rf build
mkdir -p build

# Copy the kernel image and firmware files to the build directory
cp target/kernel.img build/
cp -r firmware/{bootcode.bin,fixup.dat,start.elf,LICENCE.broadcom} build/