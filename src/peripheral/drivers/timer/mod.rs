#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

pub mod constants;
pub mod util;

// u8 - corresponds to which timer (0-3)
// *mut u32 - corresponds to the compare register for that timer
pub struct Timer(u8, *mut u32);

pub const TIMER0: Timer = Timer(0, constants::TIMER_C0);
pub const TIMER1: Timer = Timer(1, constants::TIMER_C1);
pub const TIMER2: Timer = Timer(2, constants::TIMER_C2);
pub const TIMER3: Timer = Timer(3, constants::TIMER_C3);

impl Timer {
  #[inline]
  pub fn set_compare(&self, value: u32) {
    unsafe { write_volatile(self.1, value) }
  }

  #[inline]
  pub fn get_compare(&self) -> u32 {
    unsafe { read_volatile(self.1) }
  }

  #[inline]
  pub fn set_compare_from_now(&self, offset: u32) {
    let now = timer_counter_lower();
    self.set_compare(now.wrapping_add(offset));
  }

  #[inline]
  pub fn has_triggered(&self) -> bool {
    let cs = unsafe { read_volatile(constants::TIMER_CS) };
    (cs & (1 << self.0)) != 0
  }

  #[inline]
  pub fn clear_interrupt(&self) {
    unsafe { write_volatile(constants::TIMER_CS, 1 << self.0) }
  }
}

#[inline]
pub fn timer_counter_lower() -> u32 {
  unsafe { read_volatile(constants::TIMER_CLO) }
}

#[inline]
pub fn timer_counter_higher() -> u32 {
  unsafe { read_volatile(constants::TIMER_CHI) }
}

#[inline]
pub fn timer_counter() -> u64 {
  ((timer_counter_higher() as u64) << 32) | timer_counter_lower() as u64
}
