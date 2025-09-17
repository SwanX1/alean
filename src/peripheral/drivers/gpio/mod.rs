// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(dead_code)]
#![allow(non_snake_case)]

use self::constants::PinFunction;
use crate::util::mem::Register;

pub mod constants;

pub fn pin_function_set(pin: u32, function: PinFunction) -> () {
  let FSEL: Register = match pin {
    0..=9 => constants::GPIO_FSEL0,
    10..=19 => constants::GPIO_FSEL1,
    20..=29 => constants::GPIO_FSEL2,
    30..=39 => constants::GPIO_FSEL3,
    40..=49 => constants::GPIO_FSEL4,
    50..=53 => constants::GPIO_FSEL5,
    _ => panic!("Invalid GPIO pin {}", pin),
  };

  let base_bit: u32 = pin % 10;

  let current_value: u32 = FSEL.read();

  let offset = base_bit * 3;
  let write_value: u32 = function.value() << offset;
  let mask_value: u32 = 0b111 << offset;

  FSEL.write((current_value & (!mask_value)) | write_value);
}

pub fn pin_output_set(pin: u32) -> () {
  let SET: Register = match pin {
    0..=31 => constants::GPIO_SET0,
    32..=53 => constants::GPIO_SET1,
    _ => panic!("Invalid GPIO pin {}", pin),
  };

  SET.write_bit(pin % 32, 1);
}

pub fn pin_output_clear(pin: u32) -> () {
  let CLR: Register = match pin {
    0..=31 => constants::GPIO_CLR0,
    32..=53 => constants::GPIO_CLR1,
    _ => panic!("Invalid GPIO pin {}", pin),
  };

  CLR.write_bit(pin % 32, 1);
}
