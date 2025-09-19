use crate::peripheral::drivers::uart::uart_write_str;

pub fn handle_syscall(svc_number: u32, _arg1: u32, _arg2: u32, _arg3: u32, _arg4: u32) -> u32 {
  match svc_number {
    0 => uart_write_str("HANDLING SYSCALL 0\n"),
    _ => uart_write_str("UNKNOWN SYSCALL\n"),
  }

  0
}

// TODO: document safety

#[inline(always)]
pub unsafe fn syscall0(call: u32) -> u32 {
  // SAFETY: ensured by caller
  unsafe { syscall4(call, 0, 0, 0, 0) }
}

#[inline(always)]
pub unsafe fn syscall1(call: u32, arg1: u32) -> u32 {
  // SAFETY: ensured by caller
  unsafe { syscall4(call, arg1, 0, 0, 0) }
}

#[inline(always)]
pub unsafe fn syscall2(call: u32, arg1: u32, arg2: u32) -> u32 {
  // SAFETY: ensured by caller
  unsafe { syscall4(call, arg1, arg2, 0, 0) }
}

#[inline(always)]
pub unsafe fn syscall3(call: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
  // SAFETY: ensured by caller
  unsafe { syscall4(call, arg1, arg2, arg3, 0) }
}

#[inline(always)]
pub unsafe fn syscall4(call: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> u32 {
  // Call into the kernel via SVC. Convention used:
  //  r0 = call number
  //  r1 = arg1, r2 = arg2, r3 = arg3
  //  r12 = arg4 (moved to stack by the SVC handler as 5th parameter)
  // Return value is in r0.
  let mut ret: u32 = call;
  unsafe {
    core::arch::asm!(
      "svc 0", // TODO: use constant generic parameter for SVC immediate
      inlateout("r0") ret,          // r0: call number in, return value out
      in("r1") arg1,
      in("r2") arg2,
      in("r3") arg3,
      in("r12") arg4,
      options(nostack)
    );
  }
  ret
}
