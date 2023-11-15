use core::hint;

use super::timer_value;

pub fn wait_nanos(nanos: u32) {
  unsafe {
    let start: u32 = timer_value();
    while timer_value() < (start + nanos) {
      hint::spin_loop()
    }
  }
}
