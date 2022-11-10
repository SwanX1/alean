// Copyright (c) 2022 Kārlis Čerņavskis, licensed under GNU AGPL v3.0

// Peripherals at bus address 0x7Ennnnnn are available at physical address 0x20nnnnnn
pub const FSEL_4: *mut u32 = 0x20200010 as *mut u32;
pub const PSET_1: *mut u32 = 0x20200020 as *mut u32;
pub const PCLR_1: *mut u32 = 0x2020002C as *mut u32;
