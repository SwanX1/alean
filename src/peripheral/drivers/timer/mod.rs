#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

// TODO: THIS IS NOT FINISHED YET, DO NOT USE IT

pub mod constants;
pub mod util;

// Load a value into the ARM Timer's Load register
pub unsafe fn timer_load(val: u32) {
  write_volatile(constants::TIMER_LOAD, val);
}

// Read the ARM Timer's Value register
pub unsafe fn timer_value() -> u32 {
  return read_volatile(constants::TIMER_VALUE);
}

// Read the ARM Timer's Control register
pub unsafe fn timer_control() -> u32 {
  return read_volatile(constants::TIMER_CONTROL);
}

// Write a value to the ARM Timer's Control register
pub unsafe fn set_timer_control(val: u32) {
  write_volatile(constants::TIMER_CONTROL, val);
}

// Clear the ARM Timer's Interrupt Request (IRQ) register
pub unsafe fn clear_timer_irq() {
  write_volatile(constants::TIMER_IRQ_CLR, 0);
}

// Read the ARM Timer's Raw IRQ register
pub unsafe fn timer_raw_irq() -> u32 {
  return read_volatile(constants::TIMER_RAW_IRQ);
}

// Read the ARM Timer's Masked IRQ register
pub unsafe fn timer_masked_irq() -> u32 {
  return read_volatile(constants::TIMER_MASKED_IRQ);
}

// Write a value to the ARM Timer's Reload register
pub unsafe fn set_timer_reload(val: u32) {
  write_volatile(constants::TIMER_RELOAD, val);
}

// Write a value to the ARM Timer's pre-divider register
pub unsafe fn set_timer_predivider(val: u32) {
  write_volatile(constants::TIMER_PREDIVIDER, val);
}

// Read the ARM Timer's Free Running Counter register
pub unsafe fn timer_free_running_counter() -> u32 {
  return read_volatile(constants::TIMER_FREE_RUNNING_COUNTER);
}
