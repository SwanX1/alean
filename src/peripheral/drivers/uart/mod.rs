// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(unused, reason = "This module may be unused, as it is providing peripheral functionality that may not be used anywhere")]

pub mod constants;

#[inline(always)]
pub fn uart_transmit_fifo_empty() -> bool {
  constants::UART_FR.read_bit(7)
}

#[inline(always)]
pub fn uart_receive_fifo_full() -> bool {
  constants::UART_FR.read_bit(6)
}

#[inline(always)]
pub fn uart_transmit_fifo_full() -> bool {
  constants::UART_FR.read_bit(5)
}

#[inline(always)]
pub fn uart_receive_fifo_empty() -> bool {
  constants::UART_FR.read_bit(4)
}

#[inline(always)]
pub fn uart_busy() -> bool {
  constants::UART_FR.read_bit(3)
}

#[inline(always)]
pub fn uart_set_fifo(enabled: bool) {
  constants::UART_LCRH.write_bit(4, if enabled { 1 } else { 0 });
}

#[repr(transparent)]
pub struct UartData(u32);
impl UartData {
  #[inline(always)]
  fn new(data: u32) -> Self {
    Self(data)
  }

  #[inline(always)]
  pub fn has_error(&self) -> bool {
    self.0 & (
      (1 << 11) | // overrun
      (1 << 10) | // break
      (1 << 9)  | // parity
      (1 << 8)    // framing
    ) != 0
  }

  #[inline(always)]
  pub fn overrun_error(&self) -> bool {
    self.0 & (1 << 11) != 0
  }

  #[inline(always)]
  pub fn break_error(&self) -> bool {
    self.0 & (1 << 10) != 0
  }

  #[inline(always)]
  pub fn parity_error(&self) -> bool {
    self.0 & (1 << 9) != 0
  }

  #[inline(always)]
  pub fn framing_error(&self) -> bool {
    self.0 & (1 << 8) != 0
  }

  #[inline(always)]
  pub fn data(&self) -> u8 {
    (self.0 & 0xff) as u8
  }
}

#[inline(always)]
pub fn uart_read() -> UartData {
  UartData::new(constants::UART_DR.read())
}

#[inline(always)]
pub fn uart_write(data: u8) {
  constants::UART_DR.write(data as u32);
}

#[inline(always)]
pub fn uart_write_str(s: &str) {
  for &b in s.as_bytes() {
    // Send the byte
    uart_write_byte(b);
  }
}

#[inline(always)]
pub fn uart_write_byte(b: u8) {
  // Wait until we can send
  while uart_transmit_fifo_full() {}
  // Send the byte
  uart_write(b);
}

// TODO: Make this interrupt-driven
#[inline(always)]
pub fn uart_read_blocking() -> UartData {
  // Wait until something is in the buffer
  while uart_receive_fifo_empty() {}
  // Read the byte
  uart_read()
}

// TODO: implement other UART functions (baud rate setting, interrupts, etc.)