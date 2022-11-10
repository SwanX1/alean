// Copyright (c) 2022 Kārlis Čerņavskis, licensed under GNU AGPL v3.0

use core::arch::asm;

/**
* Execute NOP instruction 500k times
*/
pub fn noop500k() -> () {
  for _ in 1..500000 {
    unsafe {
      asm!("nop");
    }
  }
}
