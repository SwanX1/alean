use core::alloc::GlobalAlloc as _;

pub mod allocator;
pub(in crate::alloc) mod arbitrary_ptr;

pub struct Alloc;
#[allow(unused)]
impl Alloc {
  pub unsafe fn alloc(layout: core::alloc::Layout) -> *mut u8 {
    unsafe { allocator::GLOBAL_ALLOCATOR.alloc(layout) }
  }
  pub unsafe fn dealloc(ptr: *mut u8, layout: core::alloc::Layout) {
    unsafe { allocator::GLOBAL_ALLOCATOR.dealloc(ptr, layout) }
  }
  pub unsafe fn realloc(ptr: *mut u8, old_layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
    unsafe { allocator::GLOBAL_ALLOCATOR.realloc(ptr, old_layout, new_size) }
  }
  pub unsafe fn alloc_zeroed(layout: core::alloc::Layout) -> *mut u8 {
    unsafe { allocator::GLOBAL_ALLOCATOR.alloc_zeroed(layout) }
  }
}