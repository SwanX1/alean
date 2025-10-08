// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![no_main]
#![no_std]
#![feature(likely_unlikely)]

mod peripheral;
mod util;
mod exceptions;
mod syscall;

use peripheral::drivers::gpio;
use peripheral::drivers::gpio::constants::PinFunction;
use peripheral::drivers::timer::util::wait_nanos;

use crate::peripheral::drivers::{uart::{uart_set_fifo, uart_write_str}, watchdog};

core::arch::global_asm!(include_str!("boot.s"), options(raw));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
  uart_set_fifo(true);
  uart_write_str("No kernel implementation yet\n");
  uart_write_str("Testing syscall. You should see a \"HANDLING SYSCALL\" message below.\n");

  unsafe { syscall::syscall0(42); }

  uart_write_str("Shutting down.\n");

  watchdog::power_off();
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
