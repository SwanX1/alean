// Copyright (c) 2022 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::peripheral::drivers::constants::apply_mask;

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

/// Function Select 0 (pins 0-9)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL0: *mut u32 = apply_mask(0x7E200000) as *mut u32;

/// Function Select 1 (pins 10-19)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL1: *mut u32 = apply_mask(0x7E200004) as *mut u32;

/// Function Select 2 (pins 20-29)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL2: *mut u32 = apply_mask(0x7E200008) as *mut u32;

/// Function Select 3 (pins 30-39)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL3: *mut u32 = apply_mask(0x7E20000C) as *mut u32;

/// Function Select 4 (pins 40-49)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL4: *mut u32 = apply_mask(0x7E200010) as *mut u32;

/// Function Select 5 (pins 50-59)
///
/// The function select registers are used to define the operation of the general-purpose I/O
/// pins. Each of the 54 GPIO pins has at least two alternative functions.
///
/// The `FSEL{n}` field determines the functionality of the nth GPIO pin. All unused
/// alternative function lines are tied to ground and will output a "0" if selected. All pins reset
/// to normal GPIO input operation.
pub const GPIO_FSEL5: *mut u32 = apply_mask(0x7E200014) as *mut u32;

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
pub const GPIO_SET0: *mut u32 = apply_mask(0x7E20001C) as *mut u32;

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
pub const GPIO_SET1: *mut u32 = apply_mask(0x7E200020) as *mut u32;

/// Pin Output Clear 0 (pins 0-31)
///
/// The output clear registers are used to clear a GPIO pin. The CLR{n} bit defines
/// the respective GPIO pin to clear, writing a "0" to the bit has no effect. If the GPIO
/// pin is being used as in input (by default) then the value in the CLR{n} bit is
/// ignored. However, if the pin is subsequently defined as an output then the bit will
/// be set according to the last set/clear operation. Separating the set and clear
/// functions removes the need for read-modify-write operations.
pub const GPIO_CLR0: *mut u32 = apply_mask(0x7E200028) as *mut u32;

/// Pin Output Clear 1 (pins 32-53)
///
/// The output clear registers are used to clear a GPIO pin. The CLR{n} bit defines
/// the respective GPIO pin to clear, writing a "0" to the bit has no effect. If the GPIO
/// pin is being used as in input (by default) then the value in the CLR{n} bit is
/// ignored. However, if the pin is subsequently defined as an output then the bit will
/// be set according to the last set/clear operation. Separating the set and clear
/// functions removes the need for read-modify-write operations.
pub const GPIO_CLR1: *mut u32 = apply_mask(0x7E20002C) as *mut u32;
/// Pin Level 0 (pins 0-31)
///
/// The pin level registers return the actual value of the pin. The LEV{n} field gives the
/// value of the respective GPIO pin.
pub const GPIO_LEV0: *mut u32 = apply_mask(0x7E200034) as *mut u32;
/// Pin Level 1 (pins 32-53)
///
/// The pin level registers return the actual value of the pin. The LEV{n} field gives the
/// value of the respective GPIO pin.
pub const GPIO_LEV1: *mut u32 = apply_mask(0x7E200038) as *mut u32;

// TODO: Document constants below
/// Pin Event Detect Status 0
pub const GPIO_EDS0: *mut u32 = apply_mask(0x7E200040) as *mut u32;
/// Pin Event Detect Status 1
pub const GPIO_EDS1: *mut u32 = apply_mask(0x7E200044) as *mut u32;
/// Pin Rising Edge Detect Enable 0
pub const GPIO_REN0: *mut u32 = apply_mask(0x7E20004C) as *mut u32;
/// Pin Rising Edge Detect Enable 1
pub const GPIO_REN1: *mut u32 = apply_mask(0x7E200050) as *mut u32;
/// Pin Falling Edge Detect Enable 0
pub const GPIO_FEN0: *mut u32 = apply_mask(0x7E200058) as *mut u32;
/// Pin Falling Edge Detect Enable 1
pub const GPIO_FEN1: *mut u32 = apply_mask(0x7E20005C) as *mut u32;
/// Pin High Detect Enable 0
pub const GPIO_HEN0: *mut u32 = apply_mask(0x7E200064) as *mut u32;
/// Pin High Detect Enable 1
pub const GPIO_HEN1: *mut u32 = apply_mask(0x7E200068) as *mut u32;
/// Pin Low Detect Enable 0
pub const GPIO_LEN0: *mut u32 = apply_mask(0x7E200070) as *mut u32;
/// Pin Low Detect Enable 1
pub const GPIO_LEN1: *mut u32 = apply_mask(0x7E200074) as *mut u32;
/// Pin Async. Rising Edge Detect 0
pub const GPIO_AREN0: *mut u32 = apply_mask(0x7E20007C) as *mut u32;
/// Pin Async. Rising Edge Detect 1
pub const GPIO_AREN1: *mut u32 = apply_mask(0x7E200080) as *mut u32;
/// Pin Async. Falling Edge Detect 0
pub const GPIO_AFEN0: *mut u32 = apply_mask(0x7E200088) as *mut u32;
/// Pin Async. Falling Edge Detect 1
pub const GPIO_AFEN1: *mut u32 = apply_mask(0x7E20008C) as *mut u32;
/// Pin Pull-up/down Enable
pub const GPIO_PUD: *mut u32 = apply_mask(0x7E200094) as *mut u32;
/// Pin Pull-up/down Enable Clock 0
pub const GPIO_PUDCLK0: *mut u32 = apply_mask(0x7E200098) as *mut u32;
/// Pin Pull-up/down Enable Clock 1
pub const GPIO_PUDCLK1: *mut u32 = apply_mask(0x7E20009C) as *mut u32;
