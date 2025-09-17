// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![no_main]
#![no_std]

mod peripheral;
mod util;

use core::panic::PanicInfo;

use peripheral::drivers::gpio;
use peripheral::drivers::gpio::constants::PinFunction;
use peripheral::drivers::timer::util::wait_nanos;

const ACT_LED: u32 = 47;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
  panic!("No kernel implementation yet");
}

/**
* Panic handler doesn't attempt to log any information, due to no actual means to do so
* yet. Currently, it just enters an infinite loop and waits for a manual reboot.
*/
// TODO: When display output is coded, make sure panic outputs panic information.
// ! You can ignore any errors that appear in IDE, we do not use std.
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
  // Spin loop

  // Set pin 47 to output
  gpio::pin_function_set(ACT_LED, PinFunction::OUTPUT);

  loop {
    // Set pin to HIGH
    gpio::pin_output_set(ACT_LED);
    // Wait 1 second
    wait_nanos(1e9);
    // Set pin to LOW
    gpio::pin_output_clear(ACT_LED);
    // Wait 1 second
    wait_nanos(1e9);
  }
}
