#![allow(dead_code)] // We are defining constants that might not be used

const BASE: u32 = 0x7E204000;

/// SPI Master Control and Status
/// This register contains the main control and status bits for the SPI
pub const SPI_CS: *mut u32 = apply_mask!(BASE + 0x00);
/// ## SPI Master TX and RX FIFOs
/// This register allows TX data to be written to the TX FIFO and RX data to be read from the RX FIFO
/// 
/// ### DMA Mode (DMAEN set)
/// If TA is clear, the first 32-bit write to this register
/// will control SPI_DLEN and SPI_CS. Subsequent
/// reads and writes will be taken as four-byte data
/// words to be read/written to the FIFOs
/// 
/// ### Poll/Interrupt Mode (DMAEN clear, TA set)
/// Writes to the register write bytes to TX FIFO.
/// Reads from register read bytes from the RX FIFO.
pub const SPI_FIFO: *mut u32 = apply_mask!(BASE + 0x04);
/// SPI Master Clock Divider
/// This register allows the SPI clock rate to be set
///   SCLK = Core Clock / DIV
/// If DIV is set to 0, the divisor is 65536. The divisor must be a power of 2. Odd numbers rounded down.
/// The maximum SPI clock rate is of the APB clock.
/// Note: Register is 32 bits, but only the lower 16 bits should be used, others are reserved (write as 0, read as don't care)
pub const SPI_CLK: *mut u32 = apply_mask!(BASE + 0x08);
/// SPI Master Data Length
/// This register allows the SPI data length rate to be set.
/// The number of bytes to transfer. This field is only valid for DMA mode (DMAEN set) and controls how many bytes to transmit (and therefore receive).
/// Note: Register is 32 bits, but only the lower 16 bits should be used, others are reserved (write as 0, read as don't care)
pub const SPI_DLEN: *mut u32 = apply_mask!(BASE + 0x0C);
/// SPI LOSSI mode TOH
/// This register allows the LoSSI output hold delay to be set.
/// A value of 0 causes a 1 clock delay.
/// Note: Register is 32 bits, but only the lower 4 bits should be used, others are reserved (write as 0, read as don't care)
pub const SPI_LTOH: *mut u32 = apply_mask!(BASE + 0x10);
/// SPI DMA DREQ Controls
/// This register controls the generation of the DREQ and Panic signals to an external DMA engine.
/// The DREQ signals are generated when the FIFOs reach their defined levels and need servicing.
/// The Panic signals instruct the external DMA engine to raise the priority of its AXI requests.
pub const SPI_DC: *mut u32 = apply_mask!(BASE + 0x14);

// These are for reference only, to avoid magic numbers in the code.
// They should not be used anywhere else, so we use pub(in super) to limit their visibility.
pub(in super) mod pins {
  /// Enable Long data word in Lossi mode if DMA_LEN is set.
  /// 0 = writing to the FIFO will write a single byte
  /// 1 = writing to the FIFO will write a 32 bit word
  /// (RW) RESET: 0
  pub const CS_LEN_LONG: usize = 25; 
  /// Enable DMA mode in Lossi mode
  /// (RW) RESET: 0
  pub const CS_DMA_LEN: usize = 24;
  /// Chip Select 2 Polarity
  /// 0 = Chip select is active low.
  /// 1 = Chip select is active high.
  /// (RW) RESET: 0
  pub const CS_CSPOL2: usize = 23;
  /// Chip Select 1 Polarity
  /// 0 = Chip select is active low.
  /// 1 = Chip select is active high.
  /// (RW) RESET: 0
  pub const CS_CSPOL1: usize = 22;
  /// Chip Select 0 Polarity
  /// 0 = Chip select is active low.
  /// 1 = Chip select is active high.
  /// (RW) RESET: 0
  pub const CS_CSPOL0: usize = 21;
  /// RX FIFO Full
  /// 0 = RXFIFO is not full.
  /// 1 = RX FIFO is full. No further serial data will be sent/received until data is read from FIFO.
  /// (RO)
  pub const CS_RXF: usize = 20;
  /// RX FIFO needs Reading ( full)
  /// 0 = RX FIFO is less than full (or not active TA = 0).
  /// 1 = RX FIFO is or more full. Cleared by reading sufficient data from the RX FIFO or setting TA to 0.
  /// (RO)
  pub const CS_RXR: usize = 19;
  /// TX FIFO can accept Data
  /// 0 = TX FIFO is full and so cannot accept more data.
  /// 1 = TX FIFO has space for at least 1 byte.
  /// (RO)
  pub const CS_TXD: usize = 18;
  /// RX FIFO contains Data
  /// 0 = RX FIFO is empty.
  /// 1 = RX FIFO contains at least 1 byte.
  /// (RO)
  pub const CS_RXD: usize = 17;
  /// Transfer Done
  /// 0 = Transfer is in progress (or not active TA = 0).
  /// 1 = Transfer is complete. Cleared by writing more data to the TX FIFO or setting TA to 0.
  /// (RO)
  pub const CS_DONE: usize = 16;
  /// TE_EN Unused
  /// (RW) RESET: 0
  pub const CS_TE_EN: usize = 15;
  /// LMONO Unused
  /// (RW) RESET: 0
  pub const CS_LMONO: usize = 14;
  /// LoSSI Enable
  /// The serial interface is configured as a LoSSI master.
  /// 0 = The serial interface will behave as an SPI master.
  /// 1 = The serial interface will behave as a LoSSI master.
  /// (RW) RESET: 0
  pub const CS_LOSSI: usize = 13;
  /// Read Enable
  /// Read enable if you are using bidirectional mode.
  /// If this bit is set, the SPI peripheral will be able to
  /// send data to this device.
  /// 0 = We intend to write to the SPI peripheral.
  /// 1 = We intend to read from the SPI peripheral.
  /// (RW) RESET: 1
  pub const CS_REN: usize = 12;
  /// Automatically Deassert Chip Select
  /// 0 = Don't automatically deassert chip select at the end of a DMA transfer; chip select is
  /// manually controlled by software.
  /// 1 = Automatically deassert chip select at the end of a DMA transfer (as determined by [SPI_DLEN])
  /// (RW) RESET: 0
  pub const CS_ADCS: usize = 11;
  /// Interrupt on RXR
  /// 0 = Don't generate interrupts on RX FIFO condition.
  /// 1 = Generate interrupt while RXR = 1.
  /// (RW) RESET: 0
  pub const CS_INTR: usize = 10;
  /// Interrupt on Done
  /// 0 = Don't generate interrupt on transfer complete.
  /// 1 = Generate interrupt when DONE = 1.
  /// (RW) RESET: 0
  pub const CS_INTD: usize = 9;
  /// DMA Enable
  /// 0 = No DMA requests will be issued.
  /// 1 = Enable DMA operation.
  /// Peripheral generates data requests. These will be taken in four-byte words
  /// until the SPI_DLEN has been reached.
  /// (RW) RESET: 0
  pub const CS_DMAEN: usize = 8;
  /// Transfer Active
  /// 0 = Transfer not active.
  /// CS lines are all high (assuming CSPOL = 0). RXR and DONE are 0.
  /// Writes to SPI_FIFO write data into bits -0 of SPICS allowing DMA data blocks to set mode before sending data.
  /// 1 = Transfer active.
  /// CS lines are set according to CS bits and CSPOL. Writes to SPI_FIFO write data to TX_FIFO.
  /// TA is cleared by a dma_frame_end pulse from the DMA controller.
  /// (RW) RESET: 0
  pub const CS_TA: usize = 7;
  /// Chip Select Polarity
  /// 0 = Chip select lines are active low
  /// 1 = Chip select lines are active high
  /// (RW) RESET: 0
  pub const CS_CSPOL: usize = 6;
  // Clear RX FIFO
  // 1 = Clear RX FIFO. One shot operation.
  // 0 = No action
  // If CLEAR and TA are both set in the same
  // operation, the FIFOs are cleared before the new
  // frame is started. Read back as 0.
  // (RW) RESET: 0
  pub const CS_CLEAR_RX: usize = 5;
  // Clear TX FIFO
  // 1 = Clear TX FIFO. One shot operation.
  // 0 = No action
  // If CLEAR and TA are both set in the same
  // operation, the FIFOs are cleared before the new
  // frame is started. Read back as 0.
  // (RW) RESET: 0
  pub const CS_CLEAR_TX: usize = 4;
  /// Clock Polarity
  /// 0 = Rest state of clock = low.
  /// 1 = Rest state of clock = high.
  /// (RW) RESET: 0
  pub const CS_CPOL: usize = 3;
  /// Clock Phase
  /// 0 = First SCLK transition at middle of data bit.
  /// 1 = First SCLK transition at beginning of data bit.
  /// (RW) RESET: 0
  pub const CS_CPHA: usize = 2;
  /// Chip Select (0, 1, 2)
  /// 00 = Chip select 0
  /// 01 = Chip select 1
  /// 10 = Chip select 2
  /// 11 = Reserved
  /// (RW) RESET: 0
  pub const CS_CS_START: usize = 0;

  /// Chip Select (0, 1, 2)
  /// See CS_CS_START
  pub const CS_CS_END: usize = 1;
}