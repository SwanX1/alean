#[allow(dead_code)] // These are utility functions, they may or may not be used

// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
use core::ptr::{read_volatile, write_volatile};


/// Read a specific bit from a memory-mapped register
/// Caller must ensure that the address is valid and aligned.
#[inline]
pub unsafe fn read_bit<T: Into<*const u32>>(addr: T, bit: u32) -> bool {
  /// SAFETY: Caller has already ensured that the address is valid and aligned.
  let value = unsafe { read_volatile(addr.into()) };
  let bit = value & (1 << bit);
  return bit != 0;
}

/// Write a specific bit from a memory-mapped register
/// Caller must ensure that the address is valid and aligned.
#[inline]
pub unsafe fn write_bit<T: Into<*mut u32> + Copy>(addr: T, bit: u32, bit_value: u32) -> () {
  /// SAFETY: Caller has already ensured that the address is valid and aligned.
  let mut value = unsafe { read_volatile(addr.into()) };
  // We use (bit_value & 1) here to guard against a non 0/1 bit_value
  value = (value & !(1 << bit)) | ((bit_value & 1) << bit);
  /// SAFETY: See above.
  unsafe { write_volatile(addr.into(), value) };
}

/// A memory-mapped register. Abstracts away the unsafe read/write operations.
/// When creating a Register, the caller must ensure that the address is valid and aligned.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Register(*mut u32);

impl Register {
  /// SAFETY: Caller must ensure that the address is valid and aligned.
  /// The address is assumed to be in the peripheral address space, and it is masked accordingly.
  #[inline(always)]
  pub const unsafe fn new(addr: u32) -> Self {
    Register((0x20000000 | (0x00FFFFFF & addr)) as *mut u32)
  }
  
  /// Utility function mirroring [Register::new], but not unsafe.<br>
  /// This is just a convenience function to avoid repeating the unsafe block.<p>
  /// Safety considerations are the same as for [Register::new].
  /// See [Register::new] for details.
  #[inline(always)]
  pub const fn from_addr(addr: u32) -> Self {
    unsafe { Register::new(addr) }
  }

  #[inline(always)]
  pub fn read(&self) -> u32 {
    // SAFETY: Caller has already ensured that the address is valid and aligned when creating the Register.
    unsafe { read_volatile(self.0) }
  }

  #[inline(always)]
  pub fn write(&self, value: u32) {
    // SAFETY: Caller has already ensured that the address is valid and aligned when creating the Register.
    unsafe { write_volatile(self.0, value) }
  }

  #[inline(always)]
  pub fn write_bit(&self, bit: u32, bit_value: u32) -> () {
    // SAFETY: Caller has already ensured that the address is valid and aligned when creating the Register.
    unsafe { write_bit(self, bit, bit_value) };
  }

  #[inline(always)]
  pub fn read_bit(&self, bit: u32) -> bool {
    // SAFETY: Caller has already ensured that the address is valid and aligned when creating the Register.
    unsafe { read_bit(self, bit) }
  }
}

impl Into<*mut u32> for &Register {
  #[inline(always)]
  fn into(self) -> *mut u32 {
    self.0
  }
}

impl Into<*const u32> for &Register {
  #[inline(always)]
  fn into(self) -> *const u32 {
    self.0 as *const u32
  }
}