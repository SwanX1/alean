use core::hint::spin_loop;

use super::timer_counter_lower;

pub fn wait_nanos(nanos: u32) {
  let target = timer_counter_lower().wrapping_add(nanos);
  while timer_counter_lower() < target {
    spin_loop();
  }
}
 