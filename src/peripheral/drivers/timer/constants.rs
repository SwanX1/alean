#![allow(dead_code)]

pub const TIMER_LOAD: *mut u32 = apply_mask!(0x7E003000);
pub const TIMER_VALUE: *mut u32 = apply_mask!(0x7E003004);
pub const TIMER_CONTROL: *mut u32 = apply_mask!(0x7E003008);
pub const TIMER_IRQ_CLR: *mut u32 = apply_mask!(0x7E00300C);
pub const TIMER_RAW_IRQ: *mut u32 = apply_mask!(0x7E003010);
pub const TIMER_MASKED_IRQ: *mut u32 = apply_mask!(0x7E003014);
pub const TIMER_RELOAD: *mut u32 = apply_mask!(0x7E003018);
pub const TIMER_PREDIVIDER: *mut u32 = apply_mask!(0x7E00301C);
pub const TIMER_FREE_RUNNING_COUNTER: *mut u32 = apply_mask!(0x7E003020);
