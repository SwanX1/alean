use core::ops::{Add, BitAnd, BitOr, Sub};

/// Represents a pointer to an arbitrary memory location.
/// This type is a thin wrapper around a raw pointer (`*mut u8`),
/// providing basic pointer arithmetic and conversions.
/// 
/// This type is primarily intended for "this is an address" semantics,
/// but also "we don't care about the type at that address, or even if it's valid".
/// 
/// # Safety
/// This type does not guarantee the validity of the pointer it holds.
/// It is the responsibility of the user to ensure that the pointer is valid
/// for the intended operations. The pointer may point to any memory location,
/// including invalid or unallocated memory.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub(in crate::alloc) struct ArbitraryPtr(*mut u8);
impl ArbitraryPtr {
  pub const fn new(ptr: *mut u8) -> Self {
    Self(ptr)
  }

  pub const fn as_ptr(&self) -> *mut u8 {
    self.0
  }
}

impl From<ArbitraryPtr> for *mut u8 {
  fn from(ptr: ArbitraryPtr) -> *mut u8 {
    ptr.0
  }
}

impl From<ArbitraryPtr> for *const u8 {
  fn from(ptr: ArbitraryPtr) -> *const u8 {
    ptr.0 as *const u8
  }
}

impl From<ArbitraryPtr> for usize {
  fn from(ptr: ArbitraryPtr) -> usize {
    ptr.0 as usize
  }
}

impl From<*mut u8> for ArbitraryPtr {
  fn from(ptr: *mut u8) -> Self {
    Self(ptr)
  }
}

impl From<*const u8> for ArbitraryPtr {
  fn from(ptr: *const u8) -> Self {
    Self(ptr as *mut u8)
  }
}

impl From<usize> for ArbitraryPtr {
  fn from(addr: usize) -> Self {
    Self(addr as *mut u8)
  }
}

impl Add<usize> for ArbitraryPtr {
  type Output = Self;

  fn add(self, rhs: usize) -> Self::Output {
    Self(self.0.wrapping_byte_add(rhs))
  }
}

impl Add<ArbitraryPtr> for ArbitraryPtr {
  type Output = Self;

  fn add(self, rhs: ArbitraryPtr) -> Self::Output {
    Self(self.0.wrapping_byte_add(rhs.0 as usize))
  }
}

impl Sub<usize> for ArbitraryPtr {
  type Output = Self;

  fn sub(self, rhs: usize) -> Self::Output {
    Self(self.0.wrapping_byte_sub(rhs))
  }
}

impl Sub<ArbitraryPtr> for ArbitraryPtr {
  type Output = Self;

  fn sub(self, rhs: ArbitraryPtr) -> Self::Output {
    Self(self.0.wrapping_byte_sub(rhs.0 as usize))
  }
}

impl BitAnd<usize> for ArbitraryPtr {
  type Output = Self;

  fn bitand(self, rhs: usize) -> Self::Output {
    Self((self.0 as usize & rhs) as *mut u8)
  }
}

impl BitOr<usize> for ArbitraryPtr {
  type Output = Self;

  fn bitor(self, rhs: usize) -> Self::Output {
    Self((self.0 as usize | rhs) as *mut u8)
  }
}

// SAFETY: ArbitraryPtr is used only as a value holder. The value itself is "the address",
// and does not have any inherent thread affinity. The pointer it holds is not dereferenced or mutated directly;
// all operations on it are done through safe abstractions that ensure proper synchronization and memory safety,
// hence this struct is private to this module.
unsafe impl Send for ArbitraryPtr {}
unsafe impl Sync for ArbitraryPtr {}