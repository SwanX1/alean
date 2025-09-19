// Copyright (c) 2025 Kārlis Čerņavskis, licensed under GNU AGPL v3.0
// This file is included in lib.rs via global_asm!

// To keep this in the first portion of the binary.
.section ".text.boot"
 
// Make _start global.
.globl _start
  .org 0x8000

// Entry point for the kernel.
// r15 -> should begin execution at 0x8000.
// r0 -> 0x00000000
// r1 -> 0x00000C42 - machine id
// r2 -> 0x00000100 - start of ATAGS
// preserve these registers as argument for kernel_main

_start:
  // Setup the stack (SVC mode on reset)
  mov sp, #0x8000
 
  // Clear out bss.
  ldr r4, =__bss_start
  ldr r9, =__bss_end
  mov r5, #0
  mov r6, #0
  mov r7, #0
  mov r8, #0
  b       2f
 
1:
  // store multiple at r4.
  stmia r4!, {r5-r8}
 
  // If we are still below bss_end, loop.
2:
  cmp r4, r9
  blo 1b
 
  // Install exception vector table at 0x00000000
  push {r0-r2, lr}
  bl install_vectors
  pop {r0-r2, lr}

  // Switch to System mode so SVC has its own banked SP/LR and can safely return
  mrs r0, cpsr
  bic r0, r0, #0x1F         // clear mode bits
  orr r0, r0, #0x1F         // set System mode (0b11111)
  msr cpsr_c, r0
  // Set up a System-mode stack separate from SVC stack
  mov sp, #0x7000

  // Call kernel_main in System mode
  ldr r3, =kernel_main
  bx r3
 
// halt
halt:
  wfe // This works because we are the only core running.
  b halt

// This block is copied to 0x00000000 by install_vectors.
// Each entry loads PC from the literal table starting 0x20 bytes after the base.
// Layout follows ARM ARM recommendations for ARM state.
.align 5
.globl __vectors_start
__vectors_start:
  ldr pc, [pc, #0x18]   // Reset
  ldr pc, [pc, #0x18]   // Undefined
  ldr pc, [pc, #0x18]   // SVC
  ldr pc, [pc, #0x18]   // Prefetch abort
  ldr pc, [pc, #0x18]   // Data abort
  ldr pc, [pc, #0x18]   // Reserved
  ldr pc, [pc, #0x18]   // IRQ
  ldr pc, [pc, #0x18]   // FIQ
  // Literal targets (absolute addresses filled by linker)
  .word _start // Reset maps to _start
  .word undef_handler
  .word svc_handler
  .word pabort_handler
  .word dabort_handler
  .word reserved_handler
  .word irq_handler
  .word fiq_handler
.globl __vectors_end
__vectors_end:

// Copy the vectors to 0x00000000
.globl install_vectors
install_vectors:
  ldr r0, =0x00000000        // dest
  ldr r1, =__vectors_start   // src
  ldr r2, =__vectors_end     // end
1:
  cmp r1, r2
  ldrcc r3, [r1], #4
  strcc r3, [r0], #4
  bcc 1b
  bx lr
