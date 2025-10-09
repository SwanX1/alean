use core::{alloc::Layout, ops::{Add, BitAnd, BitOr, Sub}, ptr::NonNull};

/// Represents a pointer to an arbitrary non-null memory location.
/// This type is a thin wrapper around a raw pointer (`NonNull<()>`),
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
pub(in crate::alloc) struct ArbitraryPtr(NonNull<()>);
impl ArbitraryPtr {
  pub const fn new(ptr: *mut ()) -> Self {
    // SAFETY: Caller must ensure that ptr is non-null.
    debug_assert!(!ptr.is_null());
    unsafe { Self::new_unchecked(ptr) }
  }

  /// SAFETY: Caller must ensure that `ptr` is non-null.
  pub const unsafe fn new_unchecked(ptr: *mut ()) -> Self {
    Self(unsafe { NonNull::new_unchecked(ptr) })
  }

  pub const fn as_ptr(&self) -> NonNull<()> {
    self.0
  }

  /// Aligns the pointer to the specified layout's alignment.
  #[inline]
  pub fn align(&self, layout: Layout) -> Self {
    let mut new_ptr = Self(self.0);
    new_ptr.align_in_place(layout);
    new_ptr
  }
  
  pub fn align_in_place(&mut self, layout: Layout) -> () {
    let align = layout.align();
    let addr = self.0.as_ptr() as usize;
    let aligned_addr = (addr + align - 1) & !(align - 1);
    // SAFETY: aligned_addr is guaranteed to be non-null because align is a power of two
    self.0 = unsafe { NonNull::new_unchecked(aligned_addr as *mut ()) };
  }
}

impl From<ArbitraryPtr> for usize {
  fn from(ptr: ArbitraryPtr) -> usize {
    ptr.0.as_ptr() as usize
  }
}

impl From<usize> for ArbitraryPtr {
  fn from(addr: usize) -> Self {
    // SAFETY: addr is assumed to be a valid address, but we cannot guarantee it's non-null.
    // The caller must ensure that addr is non-zero.
    debug_assert!(addr != 0);
    unsafe { Self::new_unchecked(addr as *mut ()) }
  }
}

impl Add<usize> for ArbitraryPtr {
  type Output = Self;

  fn add(self, rhs: usize) -> Self::Output {
    (Into::<usize>::into(self) + rhs).into()
  }
}

impl Add<ArbitraryPtr> for ArbitraryPtr {
  type Output = Self;

  fn add(self, rhs: ArbitraryPtr) -> Self::Output {
    (self + Into::<usize>::into(rhs)).into()
  }
}

impl Sub<usize> for ArbitraryPtr {
  type Output = Self;

  fn sub(self, rhs: usize) -> Self::Output {
    (Into::<usize>::into(self) - rhs).into()
  }
}

impl Sub<ArbitraryPtr> for ArbitraryPtr {
  type Output = Self;

  fn sub(self, rhs: ArbitraryPtr) -> Self::Output {
    (self - Into::<usize>::into(rhs)).into()
  }
}

impl BitAnd<usize> for ArbitraryPtr {
  type Output = Self;

  fn bitand(self, rhs: usize) -> Self::Output {
    (Into::<usize>::into(self) & rhs).into()
  }
}

impl BitAnd<ArbitraryPtr> for ArbitraryPtr {
  type Output = Self;

  fn bitand(self, rhs: ArbitraryPtr) -> Self::Output {
    (self & Into::<usize>::into(rhs)).into()
  }
}

impl BitOr<usize> for ArbitraryPtr {
  type Output = Self;

  fn bitor(self, rhs: usize) -> Self::Output {
    (Into::<usize>::into(self) | rhs).into()
  }
}

impl BitOr<ArbitraryPtr> for ArbitraryPtr {
  type Output = Self;

  fn bitor(self, rhs: ArbitraryPtr) -> Self::Output {
    (self | Into::<usize>::into(rhs)).into()
  }
}

// SAFETY: ArbitraryPtr is used only as a value holder. The value itself is "the address",
// and does not have any inherent thread affinity. The pointer it holds is not dereferenced or mutated directly;
// all operations on it are done through safe abstractions that ensure proper synchronization and memory safety,
// hence this struct is private to this module.
unsafe impl Send for ArbitraryPtr {}
unsafe impl Sync for ArbitraryPtr {}