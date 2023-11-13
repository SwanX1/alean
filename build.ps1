$sd = 0
$clean = 0
foreach ($arg in $args) {
  if ($arg -eq "-sd") {
    $sd = 1
  }
  if ($arg -eq "-clean") {
    $clean = 1
  }
}

if ($clean) {
  echo "Cleaning build directory"
  xargo clean
}

echo "Building"
echo "$ xargo"
xargo build --release --target armv6k-none-eabihf
echo "$ arm-none-eabi-gcc boot.s"
arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c src\boot.s -o target\boot.o
echo "$ arm-none-eabi-gcc libalean.rlib"
arm-none-eabi-gcc -T linker.ld -o target\kernel.elf -ffreestanding -O2 -nostdlib target\boot.o target\arm-none-eabihf\release\libalean.rlib
echo "Copying compiled files to binary file"
echo "$ arm-none-eabi-objcopy kernel.elf"
arm-none-eabi-objcopy target\kernel.elf -O binary target\kernel.img

$qemu_img = Get-Command qemu-img -ErrorAction SilentlyContinue
if ($qemu_img) {
  echo "Creating QEMU image"
  cp target\kernel.img target\qemu_kernel.img
  qemu-img resize -f raw target\qemu_kernel.img 64k
}

if ($sd) {
  if (Test-Path D:\ -PathType Container) {
    echo "Copying to D:\ drive"
    if (Test-Path D:\kernel.img -PathType Leaf) {
      echo "Deleting existing kernel.img file"
      rm D:\kernel.img
    }
    echo "Copying kernel.img"
    cp target\kernel.img D:\kernel.img

    $firmware_files = @("bootcode.bin", "fixup.dat", "start.elf", "LICENCE.broadcom")
    foreach ($file in $firmware_files) {
      if (!(Test-Path D:\$file -PathType Leaf)) {
        echo "Copying firmware: $file"
        cp firmware\$file D:\$file
      }
    }

    if (!(Test-Path D:\README -PathType Leaf)) {
      echo "There are firmware files (bootcode.bin, fixup.dat, and start.elf) in this directory that are not ALEAN code. These are under broadcom's licence (LICENCE.broadcom)" > D:\README
    }
  } else {
    echo "SD card not found, but -sd flag was passed. Is the SD card on D:\?"
  }
}