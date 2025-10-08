// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(unused, reason = "This module may be unused, as it is providing peripheral functionality that may not be used anywhere")]
pub mod constants;

#[repr(transparent)]
pub struct WatchdogTimeout(u32);

impl WatchdogTimeout {
  pub fn from_secs(secs: u32) -> Self {
    Self(secs << 16)
  }

  pub fn as_secs(&self) -> u32 {
    self.0 >> 16
  }

  pub fn as_msecs(&self) -> u32 {
    (self.0 * 1000) >> 16
  }

  pub fn as_ticks(&self) -> u32 {
    self.0
  }
}

impl Into<WatchdogTimeout> for u32 {
  fn into(self) -> WatchdogTimeout {
    WatchdogTimeout::from_secs(self)
  }
}

pub fn is_watchdog_running() -> bool {
  constants::PM_RSTC.read() & constants::PM_RSTC_WRCFG_FULL_RESET != 0
}

pub fn start_watchdog<T: Into<WatchdogTimeout>>(timeout: T) {
  constants::PM_WDOG.write(
    constants::PM_PASSWORD | 
    (timeout.into().as_ticks() & constants::PM_WDOG_TIME_SET)
  );

  let cur = constants::PM_RSTC.read();
  constants::PM_RSTC.write(
    constants::PM_PASSWORD | 
    (cur & constants::PM_RSTC_WRCFG_CLR) | // clear WRCFG bits
    constants::PM_RSTC_WRCFG_FULL_RESET
  );
}

pub fn stop_watchdog() {
  constants::PM_RSTC.write(constants::PM_PASSWORD | constants::PM_RSTC_RESET);
}

pub fn watchdog_timeout() -> WatchdogTimeout {
  WatchdogTimeout(constants::PM_WDOG.read() & constants::PM_WDOG_TIME_SET)
}

// Note: partition is 0-63, where 63 means "halt"
pub fn restart(partition: u8) {
  let p = partition as u32;
  let partition_bits =
    ((p & 0x01) << 0) |
    ((p & 0x02) << 1) |
    ((p & 0x04) << 2) |
    ((p & 0x08) << 3) |
    ((p & 0x10) << 4) |
    ((p & 0x20) << 5);

  let cur = constants::PM_RSTS.read();
  constants::PM_RSTS.write(
    constants::PM_PASSWORD |
    (cur & constants::PM_RSTS_PARTITION_CLR) | // Clear partition bits
    partition_bits 
  );

  constants::PM_WDOG.write(constants::PM_PASSWORD | 10); // timeout: 10 ticks

  let cur = constants::PM_RSTC.read();
  constants::PM_RSTC.write(
    constants::PM_PASSWORD | 
    (cur & constants::PM_RSTC_WRCFG_CLR) | // clear WRCFG bits
    constants::PM_RSTC_WRCFG_FULL_RESET
  );
}

/// NB! This will not return, the board will power off.
/// This works by requesting partition 63 (halt) before firing the watchdog.
/// The board will reboot into the bootloader, which will then not do anything,
/// effectively halting the board.
pub fn power_off() -> ! {
  restart(63);
  loop {
    // Wait for watchdog to fire
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
  }
}
