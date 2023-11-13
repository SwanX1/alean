// Copyright (c) 2022 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
use core::ptr::{read_volatile, write_volatile};

#[allow(dead_code)] // These are utility functions, it's may or may not be used

pub unsafe fn read_bit(addr: *mut u32, bit: u32) -> bool {
  let value = read_volatile(addr);
  let bit = (value >> bit) & 1;
  return bit == 1;
}

pub unsafe fn write_bit(addr: *mut u32, bit: u32, bit_value: u32) -> () {
  let mut value = read_volatile(addr);
  // We use (bit_value & 1) here to guard against a non 0/1 bit_value
  value = (value & !(1 << bit)) | ((bit_value & 1) << bit);
  write_volatile(addr, value);
}
