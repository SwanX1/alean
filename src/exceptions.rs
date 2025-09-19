use core::arch::{asm, naked_asm};

use crate::{peripheral::drivers::uart::uart_write_str, syscall::handle_syscall};

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn svc_handler() {
  naked_asm!(
    // r0 = call number (set by the SVC caller), r1..r3 = args 1..3, r12 = arg4
    // Preserve caller-saved regs we don't pass, keep 8-byte alignment, place r12 as 5th arg
    "push {{r1-r3, lr}}",    // save r1..r3 and lr (16 bytes)
    "sub sp, sp, #4",        // pad to maintain 8-byte alignment
    "push {{r12}}",          // push arg4 as the 5th parameter (stack arg 1)

    "bl {handle_syscall}",

    // Undo temporary stack space and restore registers
    "add sp, sp, #4",        // drop saved r12 (5th arg)
    "add sp, sp, #4",        // drop padding
    "pop {{r1-r3, lr}}",     // restore r1..r3 and lr

    // Return from exception: restores CPSR from SPSR_svc and branch back
    "subs pc, lr, #0",
    handle_syscall = sym handle_syscall,
  );
}

// TODO: add logging information for fatal exceptions (undef, pabort, dabort)

#[cold]
#[unsafe(no_mangle)]
pub extern "C" fn undef_handler() {
  uart_write_str("Undefined instruction exception\n");
  loop {
    unsafe { asm!("wfe"); }
  }
}

#[cold]
#[unsafe(no_mangle)]
pub extern "C" fn pabort_handler() {
  uart_write_str("Prefetch abort exception\n");
  loop {
    unsafe { asm!("wfe"); }
  }
}

#[cold]
#[unsafe(no_mangle)]
pub extern "C" fn dabort_handler() {
  uart_write_str("Data abort exception\n");
  loop {
    unsafe { asm!("wfe"); }
  }
}

#[cold]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn reserved_handler() {
  naked_asm!(
    "b {undef_handler}",
    undef_handler = sym undef_handler,
  );
}

// TODO: Create a sophisticates IRQ/FIQ handler that checks interrupt sources and handles them appropriately

#[unsafe(no_mangle)]
pub extern "C" fn irq_handler() {
  uart_write_str("IRQ exception\n");
  loop {
    unsafe { asm!("wfe"); }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn fiq_handler() {
  uart_write_str("FIQ exception\n");
  loop {
    unsafe { asm!("wfe"); }
  }
}
