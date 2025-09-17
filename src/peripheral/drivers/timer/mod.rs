// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(unused, reason = "This module may be unused, as it is providing peripheral functionality that may not be used anywhere")]

use crate::util::mem::Register;

pub mod constants;
pub mod util;

// u8 - corresponds to which timer (0-3)
// *mut u32 - corresponds to the compare register for that timer
pub struct Timer(u8, Register);

pub const TIMER0: Timer = Timer(0, constants::TIMER_C0);
pub const TIMER1: Timer = Timer(1, constants::TIMER_C1);
pub const TIMER2: Timer = Timer(2, constants::TIMER_C2);
pub const TIMER3: Timer = Timer(3, constants::TIMER_C3);

impl Timer {
  #[inline]
  pub fn set_compare(&self, value: u32) {
    self.1.write(value);
  }

  #[inline]
  pub fn get_compare(&self) -> u32 {
    self.1.read()
  }

  #[inline]
  pub fn set_compare_from_now(&self, offset: u32) {
    let now = timer_counter_lower();
    self.set_compare(now.wrapping_add(offset));
  }

  #[inline]
  pub fn has_triggered(&self) -> bool {
    constants::TIMER_CS.read_bit(self.0 as u32)
  }

  #[inline]
  pub fn clear_interrupt(&self) {
    constants::TIMER_CS.write_bit(self.0 as u32, 1);
  }
}

#[inline]
pub fn timer_counter_lower() -> u32 {
  constants::TIMER_CLO.read()
}

#[inline]
pub fn timer_counter_higher() -> u32 {
  constants::TIMER_CHI.read()
}

#[inline]
pub fn timer_counter() -> u64 {
  ((timer_counter_higher() as u64) << 32) | timer_counter_lower() as u64
}
