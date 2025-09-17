// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(unused, reason = "Constants may be unused, they should be declared regardless of usage.")]

use crate::util::mem::Register;

#[derive(Clone, Copy)]
pub enum PinFunction {
  INPUT,
  OUTPUT,
  ALT0,
  ALT1,
  ALT2,
  ALT3,
  ALT4,
  ALT5,
}

impl PinFunction {
  #[inline]
  pub fn value(&self) -> u32 {
    match *self {
      PinFunction::INPUT => 0b000,
      PinFunction::OUTPUT => 0b001,
      PinFunction::ALT0 => 0b100,
      PinFunction::ALT1 => 0b101,
      PinFunction::ALT2 => 0b110,
      PinFunction::ALT3 => 0b111,
      PinFunction::ALT4 => 0b011,
      PinFunction::ALT5 => 0b010,
    }
  }
}

const BASE: u32 = 0x7E200000;

/// Function Select 0 (pins 0-9)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL0: Register = Register::from_addr(BASE + 0x00);

/// Function Select 1 (pins 10-19)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL1: Register = Register::from_addr(BASE + 0x04);

/// Function Select 2 (pins 20-29)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL2: Register = Register::from_addr(BASE + 0x08);

/// Function Select 3 (pins 30-39)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL3: Register = Register::from_addr(BASE + 0x0C);

/// Function Select 4 (pins 40-49)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL4: Register = Register::from_addr(BASE + 0x10);

/// Function Select 5 (pins 50-59)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL5: Register = Register::from_addr(BASE + 0x14);

/// Pin Output Set 0 (pins 0-31)
///
/// The output set registers are used to set a GPIO pin.
///
/// The SET{n} bit defines the respective GPIO pin to set, writing a "0" to the bit
/// has no effect. If the GPIO pin is being used as in input (by default) then the value
/// in the SET{n} bit is ignored.
///
/// However, if the pin is subsequently defined as an output then the bit will be set
/// according to the last set/clear operation. Separating the set and clear functions
/// removes the need for read-modify-write operations
pub const GPIO_SET0: Register = Register::from_addr(BASE + 0x1C);

/// Pin Output Set 1 (pins 32-53)
///
/// The output set registers are used to set a GPIO pin.
///
/// The SET{n} bit defines the respective GPIO pin to set, writing a "0" to the bit
/// has no effect. If the GPIO pin is being used as in input (by default) then the value
/// in the SET{n} bit is ignored.
///
/// However, if the pin is subsequently defined as an output then the bit will be set
/// according to the last set/clear operation. Separating the set and clear functions
/// removes the need for read-modify-write operations
pub const GPIO_SET1: Register = Register::from_addr(BASE + 0x20);

/// Pin Output Clear 0 (pins 0-31)
///
/// The output clear registers are used to clear a GPIO pin. The CLR{n} bit defines
/// the respective GPIO pin to clear, writing a "0" to the bit has no effect. If the GPIO
/// pin is being used as in input (by default) then the value in the CLR{n} bit is
/// ignored. However, if the pin is subsequently defined as an output then the bit will
/// be set according to the last set/clear operation. Separating the set and clear
/// functions removes the need for read-modify-write operations.
pub const GPIO_CLR0: Register = Register::from_addr(BASE + 0x28);

/// Pin Output Clear 1 (pins 32-53)
///
/// The output clear registers are used to clear a GPIO pin. The CLR{n} bit defines
/// the respective GPIO pin to clear, writing a "0" to the bit has no effect. If the GPIO
/// pin is being used as in input (by default) then the value in the CLR{n} bit is
/// ignored. However, if the pin is subsequently defined as an output then the bit will
/// be set according to the last set/clear operation. Separating the set and clear
/// functions removes the need for read-modify-write operations.
pub const GPIO_CLR1: Register = Register::from_addr(BASE + 0x2C);
/// Pin Level 0 (pins 0-31)
///
/// The pin level registers return the actual value of the pin. The LEV{n} field gives the
/// value of the respective GPIO pin.
pub const GPIO_LEV0: Register = Register::from_addr(BASE + 0x34);
/// Pin Level 1 (pins 32-53)
///
/// The pin level registers return the actual value of the pin. The LEV{n} field gives the
/// value of the respective GPIO pin.
pub const GPIO_LEV1: Register = Register::from_addr(BASE + 0x38);

// TODO: Document constants below
/// Pin Event Detect Status 0
pub const GPIO_EDS0: Register = Register::from_addr(BASE + 0x40);
/// Pin Event Detect Status 1
pub const GPIO_EDS1: Register = Register::from_addr(BASE + 0x44);
/// Pin Rising Edge Detect Enable 0
pub const GPIO_REN0: Register = Register::from_addr(BASE + 0x4C);
/// Pin Rising Edge Detect Enable 1
pub const GPIO_REN1: Register = Register::from_addr(BASE + 0x50);
/// Pin Falling Edge Detect Enable 0
pub const GPIO_FEN0: Register = Register::from_addr(BASE + 0x58);
/// Pin Falling Edge Detect Enable 1
pub const GPIO_FEN1: Register = Register::from_addr(BASE + 0x5C);
/// Pin High Detect Enable 0
pub const GPIO_HEN0: Register = Register::from_addr(BASE + 0x64);
/// Pin High Detect Enable 1
pub const GPIO_HEN1: Register = Register::from_addr(BASE + 0x68);
/// Pin Low Detect Enable 0
pub const GPIO_LEN0: Register = Register::from_addr(BASE + 0x70);
/// Pin Low Detect Enable 1
pub const GPIO_LEN1: Register = Register::from_addr(BASE + 0x74);
/// Pin Async. Rising Edge Detect 0
pub const GPIO_AREN0: Register = Register::from_addr(BASE + 0x7C);
/// Pin Async. Rising Edge Detect 1
pub const GPIO_AREN1: Register = Register::from_addr(BASE + 0x80);
/// Pin Async. Falling Edge Detect 0
pub const GPIO_AFEN0: Register = Register::from_addr(BASE + 0x88);
/// Pin Async. Falling Edge Detect 1
pub const GPIO_AFEN1: Register = Register::from_addr(BASE + 0x8C);
/// Pin Pull-up/down Enable
pub const GPIO_PUD: Register = Register::from_addr(BASE + 0x94);
/// Pin Pull-up/down Enable Clock 0
pub const GPIO_PUDCLK0: Register = Register::from_addr(BASE + 0x98);
/// Pin Pull-up/down Enable Clock 1
pub const GPIO_PUDCLK1: Register = Register::from_addr(BASE + 0x9C);
