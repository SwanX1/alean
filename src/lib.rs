// Copyright (c) 2022 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![feature(core_intrinsics)]
#![no_main]
#![no_std]

use core::intrinsics::{volatile_load, volatile_store};
use core::panic::PanicInfo;

mod constants;
mod util;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
  unsafe {
    // Set pin 47 to output
    // FSEL_4 handles pins 40-49, pin 47 uses bits 21-23.
    // Enabling the first bit (21st bit in the actual value) sets the pin function to output.
    let mut ra: u32;
    ra = volatile_load(constants::FSEL_4);
    ra = (ra & !(7 << 21)) | (1 << 21);
    volatile_store(constants::FSEL_4, ra);

    loop {
      // Set pin 15 to HIGH
      // PSET_1 handles pins 32-53, we set bit 15 to 1 to set this pin.
      volatile_store(constants::PSET_1 as *mut u32, 1 << 15);
      // Wait arbitrary amount of time
      util::noop500k();
      // Set pin 15 to LOW
      // PCLR_1 handles pins 32-53, we set bit 15 to 1 to clear this pin.
      volatile_store(constants::PCLR_1 as *mut u32, 1 << 15);
      // Wait arbitrary amount of time
      util::noop500k();
    }
  }
}

/**
* Panic handler doesn't attempt to log any information, due to no actual means to do so
* yet. Currently, it just enters an infinite loop and waits for a manual reboot.
*/
// TODO: When display output is coded, make sure panic outputs panic information.
#[panic_handler]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
