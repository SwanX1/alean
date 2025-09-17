// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![macro_use]

macro_rules! apply_mask {
  ($address:expr) => {
    (0x20000000 | (0x00FFFFFF & $address)) as *mut u32
  };
}

pub mod drivers {
  pub mod gpio;
  pub mod timer;
  pub mod spi;
}
