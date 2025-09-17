// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
#![allow(unused, reason = "Constants may be unused, they should be declared regardless of usage.")]

use crate::util::mem::Register;

pub const BASE: u32 = 0x7E201000;

/// Data Register
/// 
/// For words to be transmitted:
/// if the FIFOs are enabled, data written to this location is pushed onto the transmit FIFO.
/// if the FIFOs are not enabled, data is stored in the transmitter holding register (the bottom word of the transmit FIFO).
/// The write operation initiates transmission from the UART. The data is prefixed with a start bit,
/// appended with the appropriate parity bit (if parity is enabled), and a stop bit.
/// The resultant word is then transmitted.
/// 
/// For received words:
/// if the FIFOs are enabled, the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO
/// if the FIFOs are not enabled, the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO).
pub const UART_DR: Register = Register::from_addr(BASE + 0x00);
/// 
pub const UART_RSRECR: Register = Register::from_addr(BASE + 0x04);
/// Flag register
pub const UART_FR: Register = Register::from_addr(BASE + 0x18);
/// not in use
pub const UART_ILPR: Register = Register::from_addr(BASE + 0x20);
/// Integer Baud rate divisor
pub const UART_IBRD: Register = Register::from_addr(BASE + 0x24);
/// Fractional Baud rate divisor
pub const UART_FBRD: Register = Register::from_addr(BASE + 0x28);
/// Line Control register
pub const UART_LCRH: Register = Register::from_addr(BASE + 0x2c);
/// Control register
pub const UART_CR: Register = Register::from_addr(BASE + 0x30);
/// Interupt FIFO Level Select Register
pub const UART_IFLS: Register = Register::from_addr(BASE + 0x34);
/// Interupt Mask Set Clear Register
pub const UART_IMSC: Register = Register::from_addr(BASE + 0x38);
/// Raw Interupt Status Register
pub const UART_RIS: Register = Register::from_addr(BASE + 0x3c);
/// Masked Interupt Status Register
pub const UART_MIS: Register = Register::from_addr(BASE + 0x40);
/// Interupt Clear Register
pub const UART_ICR: Register = Register::from_addr(BASE + 0x44);
/// DMA Control Register
pub const UART_DMACR: Register = Register::from_addr(BASE + 0x48);
/// Test Control register
pub const UART_ITCR: Register = Register::from_addr(BASE + 0x80);
/// Integration test input reg
pub const UART_ITIP: Register = Register::from_addr(BASE + 0x84);
/// Integration test output reg
pub const UART_ITOP: Register = Register::from_addr(BASE + 0x88);
/// Test Data reg
pub const UART_TDR: Register = Register::from_addr(BASE + 0x8c);
