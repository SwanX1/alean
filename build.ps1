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
xargo build --release --target arm-none-eabihf
arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c src\boot.s -o target\boot.o
arm-none-eabi-gcc -T linker.ld -o target\kernel.elf -ffreestanding -O2 -nostdlib target\boot.o target\arm-none-eabihf\release\libalean.rlib
echo "Copying compiled files to binary file"
arm-none-eabi-objcopy target\kernel.elf -O binary target\kernel.img

if ($sd) {
  if (Test-Path D:\ -PathType Container) {
    echo "Copying to D:\ drive"
    if (Test-Path D:\kernel.img -PathType Leaf) {
      echo "Deleting existing kernel.img file"
      rm D:\kernel.img
    }
    cp target\kernel.img D:\kernel.img
  }
}