// Copyright (c) 2022 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
pub const PERIPHERAL_ADDRESS_BASE: u32 = 0x20000000;
pub const PERIPHERAL_ADDRESS_MASK: u32 = 0x00FFFFFF;

pub const fn apply_mask(address: u32) -> u32 {
  return PERIPHERAL_ADDRESS_BASE | (PERIPHERAL_ADDRESS_MASK & address);
}
