// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(unused, reason = "Constants may be unused, they should be declared regardless of usage.")]

use crate::util::mem::Register;

// These registers are not documented in the BCM2835 ARM Peripherals manual, or anywhere else.
// They are however used in the Linux kernel, and their addresses can be found there.
// These are reverse engineered and used as "it works", not guaranteed to be correct.

// Power Management, Reset controller and Watchdog registers
pub const BASE: u32 = 0x7F100000;

pub const PM_RSTC: Register = Register::from_addr(BASE + 0x1C);
pub const PM_RSTS: Register = Register::from_addr(BASE + 0x20);
pub const PM_WDOG: Register = Register::from_addr(BASE + 0x24);

// I haven't the faintest idea what these values are, they are taken from Raspberry Pi Linux kernel source.
// See: linux/drivers/watchdog/bcm2835_wdt.c

/// Password to use when writing to PM_RSTC, PM_RSTS, or PM_WDOG
pub const PM_PASSWORD: u32 = 0x5A000000;
pub const PM_WDOG_TIME_SET: u32 = 0x000FFFFF;
pub const PM_RSTC_WRCFG_CLR: u32 = 0xFFFFFFCF;
pub const PM_RSTS_HADWRH_SET: u32 = 0x00000040;
pub const PM_RSTC_WRCFG_SET: u32 = 0x00000030;
pub const PM_RSTC_WRCFG_FULL_RESET: u32 = 0x00000020;
pub const PM_RSTC_RESET: u32 = 0x00000102;

/// Mask to clear partition bits in PM_RSTS
pub const PM_RSTS_PARTITION_CLR: u32 = 0xFFFFFAAA;
