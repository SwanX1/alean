#![allow(dead_code)]

use crate::peripheral::drivers::constants::apply_mask;

pub const TIMER_LOAD: *mut u32 = apply_mask(0x7E003000) as *mut u32;
pub const TIMER_VALUE: *mut u32 = apply_mask(0x7E003004) as *mut u32;
pub const TIMER_CONTROL: *mut u32 = apply_mask(0x7E003008) as *mut u32;
pub const TIMER_IRQ_CLR: *mut u32 = apply_mask(0x7E00300C) as *mut u32;
pub const TIMER_RAW_IRQ: *mut u32 = apply_mask(0x7E003010) as *mut u32;
pub const TIMER_MASKED_IRQ: *mut u32 = apply_mask(0x7E003014) as *mut u32;
pub const TIMER_RELOAD: *mut u32 = apply_mask(0x7E003018) as *mut u32;
pub const TIMER_PREDIVIDER: *mut u32 = apply_mask(0x7E00301C) as *mut u32;
pub const TIMER_FREE_RUNNING_COUNTER: *mut u32 = apply_mask(0x7E003020) as *mut u32;
