// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0

use core::hint::spin_loop;

use super::timer_counter_lower;

#[allow(unused, reason = "This function may be unused as it is a utility function")]
pub fn wait_nanos(nanos: u32) {
  let target = timer_counter_lower().wrapping_add(nanos);
  while timer_counter_lower() < target {
    spin_loop();
  }
}
 