// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(dead_code)]
#![allow(non_snake_case)]

use core::ptr::{read_volatile, write_volatile};

use self::constants::PinFunction;
use crate::util::mem::write_bit;

pub mod constants;

pub fn pin_function_set(pin: u32, function: PinFunction) -> () {
  let FSEL: *mut u32 = match pin {
    0..=9 => constants::GPIO_FSEL0,
    10..=19 => constants::GPIO_FSEL1,
    20..=29 => constants::GPIO_FSEL2,
    30..=39 => constants::GPIO_FSEL3,
    40..=49 => constants::GPIO_FSEL4,
    50..=53 => constants::GPIO_FSEL5,
    _ => panic!("Invalid GPIO pin {}", pin),
  };

  let base_bit: u32 = pin % 10;

  // SAFETY: Reading from a valid virtual address mapped to GPIO peripheral
  let current_value: u32 = unsafe { read_volatile(FSEL) };

  let offset = base_bit * 3;
  let write_value: u32 = function.value() << offset;
  let mask_value: u32 = 0b111 << offset;

  // SAFETY: Writing to a valid virtual address mapped to GPIO peripheral
  unsafe { write_volatile(FSEL, (current_value & (!mask_value)) | write_value) };
}

pub fn pin_output_set(pin: u32) -> () {
  let SET: *mut u32 = match pin {
    0..=31 => constants::GPIO_SET0,
    32..=53 => constants::GPIO_SET1,
    _ => panic!("Invalid GPIO pin {}", pin),
  };

  // SAFETY: Writing to a valid virtual address mapped to GPIO peripheral
  unsafe { write_bit(SET, pin % 32, 1) };
}

pub fn pin_output_clear(pin: u32) -> () {
  let CLR: *mut u32 = match pin {
    0..=31 => constants::GPIO_CLR0,
    32..=53 => constants::GPIO_CLR1,
    _ => panic!("Invalid GPIO pin {}", pin),
  };

  // SAFETY: Writing to a valid virtual address mapped to GPIO peripheral
  unsafe { write_bit(CLR, pin % 32, 1) };
}
