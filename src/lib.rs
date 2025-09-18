// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![no_main]
#![no_std]
#![feature(likely_unlikely)]

mod peripheral;
mod util;

use peripheral::drivers::gpio;
use peripheral::drivers::gpio::constants::PinFunction;
use peripheral::drivers::timer::util::wait_nanos;

use crate::peripheral::drivers::uart::{uart_read_blocking, uart_set_fifo, uart_write, uart_write_str};

core::arch::global_asm!(include_str!("boot.s"), options(raw));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
  uart_set_fifo(true);
  uart_write_str("No kernel implementation yet\n");
  uart_write_str("Entering loopback mode. Type something and it will be echoed back.\n");
  loop {
    // Loopback
    let c = uart_read_blocking().data();
    uart_write(c);
    if c == b'\r' {
      uart_write(b'\n');
    }
  }
}

const ACT_LED: u32 = 47;

// TODO: Log panic info, disable core feature `panic_immediate_abort`
#[panic_handler]
pub fn kernel_panic(_info: &core::panic::PanicInfo) -> ! {
  // Spin loop

  // Set pin 47 to output
  gpio::pin_function_set(ACT_LED, PinFunction::OUTPUT);

  loop {
    // Set pin to HIGH
    gpio::pin_output_set(ACT_LED);
    // Wait 1 second
    wait_nanos(1_000_000_000);
    // Set pin to LOW
    gpio::pin_output_clear(ACT_LED);
    // Wait 1 second
    wait_nanos(1_000_000_000);
  }
}
