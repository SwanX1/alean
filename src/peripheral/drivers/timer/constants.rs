#![allow(dead_code)]

const BASE: u32 = 0x7E003000;

/// System Timer Control/Status
pub const TIMER_CS: *mut u32 = apply_mask!(BASE + 0x00);
/// System Timer Counter Lower 32 bits
pub const TIMER_CLO: *mut u32 = apply_mask!(BASE + 0x04);
/// System Timer Counter Higher 32 bits
pub const TIMER_CHI: *mut u32 = apply_mask!(BASE + 0x08);
/// System Timer Compare 0
pub const TIMER_C0: *mut u32 = apply_mask!(BASE + 0x0C);
/// System Timer Compare 1
pub const TIMER_C1: *mut u32 = apply_mask!(BASE + 0x10);
/// System Timer Compare 2
pub const TIMER_C2: *mut u32 = apply_mask!(BASE + 0x14);
/// System Timer Compare 3
pub const TIMER_C3: *mut u32 = apply_mask!(BASE + 0x18);

// Where did I get these from? These are not in the BCM2835 ARM Peripherals doc.
// pub const TIMER_LOAD: *mut u32 = apply_mask!(0x7E003000);
// pub const TIMER_VALUE: *mut u32 = apply_mask!(0x7E003004);
// pub const TIMER_CONTROL: *mut u32 = apply_mask!(0x7E003008);
// pub const TIMER_IRQ_CLR: *mut u32 = apply_mask!(0x7E00300C);
// pub const TIMER_RAW_IRQ: *mut u32 = apply_mask!(0x7E003010);
// pub const TIMER_MASKED_IRQ: *mut u32 = apply_mask!(0x7E003014);
// pub const TIMER_RELOAD: *mut u32 = apply_mask!(0x7E003018);
// pub const TIMER_PREDIVIDER: *mut u32 = apply_mask!(0x7E00301C);
// pub const TIMER_FREE_RUNNING_COUNTER: *mut u32 = apply_mask!(0x7E003020);
