// Copyright (c) 2022 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![no_main]
#![no_std]
#![feature(exclusive_range_pattern)]

mod peripheral;
mod util;

use core::arch::asm;
use core::panic::PanicInfo;

use peripheral::drivers::gpio;
use peripheral::drivers::gpio::constants::PinFunction;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
  let pin = 47;
  unsafe {
    // Set pin 47 to output
    // FSEL_4 handles pins 40-49, pin 47 uses bits 21-23.
    // Enabling the first bit (21st bit in the actual value) sets the pin function to output.
    gpio::pin_function_set(pin, PinFunction::OUTPUT);

    loop {
      // Set pin to HIGH
      gpio::pin_output_set(pin);
      // Wait arbitrary amount of time
      for _ in 0..5000000 {
        asm!("nop")
      }
      // Set pin to LOW
      gpio::pin_output_clear(pin);
      // Wait arbitrary amount of time
      for _ in 0..5000000 {
        asm!("nop")
      }
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
