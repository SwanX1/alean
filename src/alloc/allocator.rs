pub(crate) use core::alloc::{GlobalAlloc, Layout};
use crate::alloc::arbitrary_ptr::ArbitraryPtr;

unsafe extern "C" {
  // SAFETY: linker provides this symbol
  static __end: u8;
}

#[inline(always)]
const fn kernel_end() -> usize {
  let kernel_end = unsafe { __end as usize };
  // Align to next page boundary (4KiB)
  (kernel_end + 0xFFF) & !0xFFF
}

const ALLOCATOR_BLOCKS: usize = 1024;

#[global_allocator]
pub (super) static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator;
static mut GLOBAL_ALLOCATOR_STATE: GlobalAllocatorState = GlobalAllocatorState {
  allocated_blocks: [const { None }; ALLOCATOR_BLOCKS],
};

#[derive(Clone)]
struct AllocatedBlock {
  ptr: ArbitraryPtr,
  size: usize,
}

impl AllocatedBlock {
  const fn new(ptr: ArbitraryPtr, size: usize) -> Self {
    Self {
      ptr,
      size,
    }
  }
  
  fn overlaps(&self, ptr: ArbitraryPtr, size: usize) -> bool {
    let start = self.ptr;
    let end = start + self.size;
    let target_start = ptr.into();
    let target_end = target_start + size;
    !(target_end <= start || end <= target_start)
  }
}

struct GlobalAllocatorState {
  allocated_blocks: [Option<AllocatedBlock>; ALLOCATOR_BLOCKS],
}
pub struct GlobalAllocator;

impl GlobalAllocator {
  fn find_free_block(&self, layout: Layout) -> Option<ArbitraryPtr> {
    // Start searching from the end of the kernel memory
    let mut current_address: ArbitraryPtr = (kernel_end() + 1).into();

    loop {
      // Align to alignment requirement
      current_address = (current_address + layout.align() - 1usize) & !(layout.align() - 1);
      
      // Memory cap arbitrary here
      // TODO: Find a better memory cap here
      if current_address + layout.size() > 0x4000_0000usize.into() {
        return None;
      }
      
      // Check for overlap in MMIO range (0x2000_0000 to 0x20FF_FFFF)
      if 0x2000_0000usize <= current_address.into() && current_address <= 0x20FF_FFFF.into() {
        continue;
      }
      
      if 0x2000_0000usize <= (current_address + layout.size()).into() && (current_address + layout.size()) <= 0x20FF_FFFF.into() {
        continue;
      }
      
      if current_address <= 0x2000_0000.into() && 0x20FF_FFFFusize <= (current_address + layout.size()).into() {
        continue;
      }

      // Get the nearest allocated block that is after current_address
      let mut nearest_block_index: Option<(usize, ArbitraryPtr)> = None;
      #[allow(static_mut_refs)] // SAFETY: We are the only ones accessing GLOBAL_ALLOCATOR_STATE here
      for (index, block_option) in unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks.iter().enumerate() } {
        if let Some(block) = block_option {
          if block.ptr >= current_address {
            match nearest_block_index {
              Some((_, nearest_ptr)) => {
                if block.ptr < nearest_ptr {
                  nearest_block_index = Some((index, block.ptr));
                }
              }
              None => {
                nearest_block_index = Some((index, block.ptr));
              }
            }
          }
        }
      }

      // Check if overlaps with nearest block
      let nearest_block = match nearest_block_index {
        Some((i, _)) => {
          // SAFETY: We checked that i is valid within the loop before
          unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks[i].clone().unwrap_unchecked() }
        }
        None => {
          // No blocks allocated yet, so no overlap
          return Some(current_address);
        }
      };
      
      if !nearest_block.overlaps(current_address, layout.size()) {
        return Some(current_address);
      }
      
      // Move current_address to the end of the nearest block
      current_address = nearest_block.ptr + nearest_block.size;

      // Try again.
    }
  }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
      let addr = self.find_free_block(layout).unwrap_or(ArbitraryPtr::new(core::ptr::null_mut())).as_ptr();
      if addr.is_null() {
        return core::ptr::null_mut();
      }

      // Find a free slot in allocated_blocks
      #[allow(static_mut_refs)] // SAFETY: We are the only ones accessing GLOBAL_ALLOCATOR_STATE here
      for block_option in unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks.iter_mut() } {
        if block_option.is_none() {
          *block_option = Some(AllocatedBlock::new(addr.into(), layout.size()));
          return addr;
        }
      }
      addr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
      // Find the block and mark it as free
      let target_ptr: ArbitraryPtr = ptr.into();
      #[allow(static_mut_refs)] // SAFETY: We are the only ones accessing GLOBAL_ALLOCATOR_STATE here
      for block_option in unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks.iter_mut() } {
        if let Some(block) = block_option {
          if block.ptr == target_ptr && block.size == layout.size() {
            *block_option = None;
            return;
          }
        }
      }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
      // Check if the current block can accommodate the new size
      let current_ptr: ArbitraryPtr = ptr.into();
      let mut found_block_index: Option<usize> = None;
      #[allow(static_mut_refs)] // SAFETY: We are the only ones accessing GLOBAL_ALLOCATOR_STATE here
      for (index, block_option) in unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks.iter().enumerate() } {
        if let Some(block) = block_option {
          if block.ptr == current_ptr && block.size == layout.size() {
            found_block_index = Some(index);
            break;
          }
        }
      }
      let block_index = match found_block_index {
        Some(i) => i,
        None => {
          // Block not found, cannot realloc
          return core::ptr::null_mut();
        }
      };

      let block = unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks[block_index].as_mut().unwrap_unchecked() };

      if new_size <= block.size {
        // Current block is sufficient, shrink in place
        block.size = new_size;
        return ptr;
      }

      // Need to allocate a new block

      // Check if we can extend the current block
      let next_address = block.ptr + block.size;
      let can_extend = {
        // Check for overlap with other blocks
        let mut can_extend = true;
        #[allow(static_mut_refs)] // SAFETY: We are the only ones accessing GLOBAL_ALLOCATOR_STATE here
        for (index, other_block_option) in unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks.iter().enumerate() } {
          if index == block_index {
            continue;
          }
          if let Some(other_block) = other_block_option {
            if other_block.ptr < next_address + (new_size - block.size) && next_address < other_block.ptr + other_block.size {
              can_extend = false;
              break;
            }
          }
        }
        can_extend
      };

      if can_extend {
        // Extend in place
        block.size = new_size;
        return ptr;
      }

      // Allocate a new block
      let new_ptr = unsafe { self.alloc(Layout::from_size_align_unchecked(new_size, layout.align())) };
      if new_ptr.is_null() {
        return core::ptr::null_mut();
      }

      // Copy old data to new block
      unsafe { core::ptr::copy_nonoverlapping(ptr, new_ptr, layout.size().min(new_size)) };

      // Free old block
      unsafe { self.dealloc(ptr, layout) };

      new_ptr
    }
}