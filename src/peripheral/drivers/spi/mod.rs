#[allow(dead_code)]

use core::{hint::{self, unreachable_unchecked}, ptr::{read_volatile, write_volatile}};

pub mod constants;

pub const CS: SpiControlStatus = SpiControlStatus {
  unfinished_operation: None,
};

pub struct SpiControlStatus {
  // None = No operation
  // Some(_) = Operation in progress, unfinished.
  // The array represents the state of each of the 32 bits in the register:
  //   None = don't care, do not modify when writing
  //   Some(true) = set bit to 1 when writing
  //   Some(false) = set bit to 0 when writing
  unfinished_operation: Option<[Option<bool>; 32]>,
}

impl SpiControlStatus {
  fn set_bit(&mut self, bit: usize, value: bool) {
    if bit > 31 {

      unsafe { unreachable_unchecked() };
    }
    if self.unfinished_operation.is_none() {
      self.unfinished_operation = Some([None; 32]);
    }
    if let Some(ref mut ops) = self.unfinished_operation {
      ops[bit as usize] = Some(value);
    }
  }

  pub fn apply(&mut self) {
    if let Some(ops) = self.unfinished_operation {
      // SAFETY: This is a virtual register, so we can read and write it safely.
      let mut reg_value = unsafe { read_volatile(constants::SPI_CS) };
      for (bit, bit_op) in ops.iter().enumerate() {
        if let Some(bit_value) = bit_op {
          if bit >= 26 || (16..=20).contains(&bit) {
            // SAFETY: We can never reach this point because none of the bits above 25 are writable.
            // This is ensured by:
            // 1. The set_bit method, which already asserts that bit is <= 31.
            // 2. The public methods that set bits only set bits <= 25 (see [constants::pins]).
            // 3. Read-only bits are never set by public methods (bits 20 - 16, inclusive)
            unsafe { unreachable_unchecked() };
          }
          // We use (bit_value & 1) here to guard against a non 0/1 bit_value
          reg_value = (reg_value & !(1 << bit)) | ((if *bit_value { 1 } else { 0 }) << bit);
        }
      }
      // SAFETY: This is a virtual register, so we can read and write it safely.
      unsafe { write_volatile(constants::SPI_CS, reg_value) };
      self.unfinished_operation = None;
    }
  }

  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_len_long(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_LEN_LONG, enable);
    self
  }

  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_dma_len(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_DMA_LEN, enable);
    self
  }

  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_cspol2(mut self, active_high: bool) -> Self {
    self.set_bit(constants::pins::CS_CSPOL2, active_high);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_cspol1(mut self, active_high: bool) -> Self {
    self.set_bit(constants::pins::CS_CSPOL1, active_high);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_cspol0(mut self, active_high: bool) -> Self {
    self.set_bit(constants::pins::CS_CSPOL0, active_high);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_lossi(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_LOSSI, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_ren(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_REN, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_adcs(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_ADCS, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_intr(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_INTR, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_intd(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_INTD, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_dmaen(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_DMAEN, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_ta(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_TA, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_cspol(mut self, active_high: bool) -> Self {
    self.set_bit(constants::pins::CS_CSPOL, active_high);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_clear_rx(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_CLEAR_RX, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_clear_tx(mut self, enable: bool) -> Self {
    self.set_bit(constants::pins::CS_CLEAR_TX, enable);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_cpol(mut self, high: bool) -> Self {
    self.set_bit(constants::pins::CS_CPOL, high);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_cpha(mut self, leading: bool) -> Self {
    self.set_bit(constants::pins::CS_CPHA, leading);
    self
  }
  #[must_use = "This function does not modify the register until .apply() is called"]
  pub fn set_cs(mut self, cs: u8) -> Self {
    if cs >= 3 {
      panic!("Chip select must be 0, 1, or 2");
    }
    self.set_bit(constants::pins::CS_CS_START, (cs & 0b01) != 0);
    self.set_bit(constants::pins::CS_CS_END, (cs & 0b10) != 0);
    self
  }

  pub fn rx_fifo_full(&self) -> bool {
    // SAFETY: This is a virtual register, so we can read and write it safely.
    let reg_value = unsafe { read_volatile(constants::SPI_CS) };
    (reg_value & (1 << constants::pins::CS_RXF)) != 0
  }

  pub fn rx_fifo_needs_reading(&self) -> bool {
    // SAFETY: This is a virtual register, so we can read and write it safely.
    let reg_value = unsafe { read_volatile(constants::SPI_CS) };
    (reg_value & (1 << constants::pins::CS_RXR)) != 0
  }

  pub fn tx_fifo_can_accept_data(&self) -> bool {
    // SAFETY: This is a virtual register, so we can read and write it safely.
    let reg_value = unsafe { read_volatile(constants::SPI_CS) };
    (reg_value & (1 << constants::pins::CS_TXD)) != 0
  }
  pub fn rx_fifo_contains_data(&self) -> bool {
    // SAFETY: This is a virtual register, so we can read and write it safely.
    let reg_value = unsafe { read_volatile(constants::SPI_CS) };
    (reg_value & (1 << constants::pins::CS_RXD)) != 0
  }
  pub fn transfer_done(&self) -> bool {
    // SAFETY: This is a virtual register, so we can read and write it safely.
    let reg_value = unsafe { read_volatile(constants::SPI_CS) };
    (reg_value & (1 << constants::pins::CS_DONE)) != 0
  }
}

pub fn write_tx(data: u8) {
  // SAFETY: This is a virtual register, so we can read and write it safely.
  unsafe { write_volatile(hint::black_box(constants::SPI_FIFO), data as u32) };
}

pub fn write_tx_long(data: u32) {
  // SAFETY: This is a virtual register, so we can read and write it safely.
  unsafe { write_volatile(hint::black_box(constants::SPI_FIFO), data) };
}

pub fn read_rx() -> u8 {
  // Black box used here, since reading from the FIFO has side effects (removes data from FIFO).
  // SAFETY: This is a virtual register, so we can read and write it safely.
  (hint::black_box(unsafe { read_volatile(constants::SPI_FIFO) }) & 0xFF) as u8
}

pub fn read_rx_long() -> u32 {
  // Black box used here, since reading from the FIFO has side effects (removes data from FIFO).
  // SAFETY: This is a virtual register, so we can read and write it safely.
  unsafe { read_volatile(constants::SPI_FIFO) }
}

pub fn set_cdiv(div: u16) {
  if (div < 2 && div != 0) || div % 2 != 0 {
    panic!("Clock divider must be an even number >= 2");
  }
  // SAFETY: This is a virtual register, so we can read and write it safely.
  unsafe { write_volatile(constants::SPI_CLK, (div & 0xFFFF) as u32) };
}

pub fn set_dlen(len: u16) {
  // SAFETY: This is a virtual register, so we can read and write it safely.
  unsafe { write_volatile(constants::SPI_DLEN, (len & 0xFFFF) as u32) };
}
