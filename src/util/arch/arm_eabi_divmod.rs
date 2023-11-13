// Purpose: Implements the __aeabi_uidivmod function for ARM EABI targets.

// struct qr - stores qutient/remainder to handle divmod EABI interfaces.
struct QR {
  /// computed quotient
  q: u32,
  /// computed remainder
  r: u32,
}

#[no_mangle]
pub extern "system" fn __aeabi_uidiv(numerator: u32, denominator: u32) -> u32 {
  let qr = &mut QR { q: 0, r: 0 };

  uint_div_qr(numerator, denominator, qr);

  return qr.q;
}

fn uint_div_qr(numerator: u32, denominator: u32, qr: &mut QR) -> () {
  division_qr(numerator, denominator, qr);
}

fn division_qr(n: u32, p: u32, qr: &mut QR) -> () {
  let mut i: u32 = 1;
  let mut q: u32 = 0;
  let mut p: u32 = p;
  let mut n: u32 = n;

  if p == 0 {
    qr.r = 0xFFFFFFFF; // division by 0
    return;
  }

  while (p >> 31) == 0 {
    i = i << 1; // count the max division steps
    p = p << 1; // increase p until it has maximum size
  }

  while i > 0 {
    q = q << 1; // write bit in q at index (size-1)
    if n >= p {
      n -= p;
      q += 1;
    }
    p = p >> 1; // decrease p
    i = i >> 1; // decrease remaining size in q
  }
  qr.r = n;
  qr.q = q;
}
