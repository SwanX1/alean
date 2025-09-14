// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
use core::ptr::{read_volatile, write_volatile};

#[allow(dead_code)] // These are utility functions, it's may or may not be used

/// Read a specific bit from a memory-mapped register
/// Caller must ensure that the address is valid and aligned.
pub unsafe fn read_bit(addr: *mut u32, bit: u32) -> bool {
  let value = unsafe { read_volatile(addr) };
  let bit = (value >> bit) & 1;
  return bit == 1;
}

/// Write a specific bit from a memory-mapped register
/// Caller must ensure that the address is valid and aligned.
pub unsafe fn write_bit(addr: *mut u32, bit: u32, bit_value: u32) -> () {
  let mut value = unsafe { read_volatile(addr) };
  // We use (bit_value & 1) here to guard against a non 0/1 bit_value
  value = (value & !(1 << bit)) | ((bit_value & 1) << bit);
  unsafe { write_volatile(addr, value) };
}
