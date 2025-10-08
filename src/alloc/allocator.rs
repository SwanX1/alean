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

/// Memory-Mapped I/O (MMIO) region start address for BCM2835.
/// This region should never be used for heap allocations.
const MMIO_START: usize = 0x2000_0000;

/// Address just after the MMIO region (0x2000_0000 to 0x20FF_FFFF) where allocations can resume.
const MMIO_SKIP_TO: usize = 0x2100_0000;

/// Maximum memory address for allocations.
/// This should ideally be determined dynamically based on available RAM.
const MEMORY_CAP: usize = 0x4000_0000;

/// The maximum number of blocks that can be allocated simultaneously.
/// Once this limit is reached, further allocations will fail.
const ALLOCATOR_BLOCKS: usize = 1024;

#[global_allocator]
pub (super) static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator;
static mut GLOBAL_ALLOCATOR_STATE: GlobalAllocatorState = GlobalAllocatorState {
  allocated_blocks: [const { None }; ALLOCATOR_BLOCKS],
};

/// Represents a single allocated memory block tracked by the allocator.
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
  
  /// Checks if this block overlaps with a memory region defined by ptr and size.
  fn overlaps(&self, ptr: ArbitraryPtr, size: usize) -> bool {
    let start = self.ptr;
    let end = start + self.size;
    let target_start = ptr;
    let target_end = target_start + size;
    !(target_end <= start || end <= target_start)
  }
}

/// Internal state for the global allocator.
/// This is a static mutable and should only be accessed within the GlobalAllocator implementation.
struct GlobalAllocatorState {
  allocated_blocks: [Option<AllocatedBlock>; ALLOCATOR_BLOCKS],
}

/// A simple bump allocator with a free list for the kernel.
/// 
/// This allocator maintains a list of allocated blocks and searches for free
/// memory regions by scanning through allocated blocks. It automatically skips
/// the MMIO region used by hardware peripherals.
pub struct GlobalAllocator;

impl GlobalAllocator {
  /// Find the index of an allocated block by its pointer and size
  fn find_block_index(&self, ptr: ArbitraryPtr, size: usize) -> Option<usize> {
    #[allow(static_mut_refs)] // SAFETY: We are the only ones accessing GLOBAL_ALLOCATOR_STATE here
    for (index, block_option) in unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks.iter().enumerate() } {
      if let Some(block) = block_option {
        if block.ptr == ptr && block.size == size {
          return Some(index);
        }
      }
    }
    None
  }

  fn find_free_block(&self, layout: Layout) -> Option<ArbitraryPtr> {
    // Start searching from the end of the kernel memory
    let mut current_address: ArbitraryPtr = (kernel_end() + 1).into();

    loop {
      // Align to alignment requirement
      current_address = (current_address + layout.align() - 1usize) & !(layout.align() - 1);
      
      // Check if we've exceeded the memory cap
      if current_address + layout.size() > MEMORY_CAP.into() {
        return None;
      }
      
      // Check for overlap in MMIO range
      // If current block would overlap with MMIO, skip to after MMIO range
      let current_end = current_address + layout.size();
      if usize::from(current_address) < MMIO_SKIP_TO && usize::from(current_end) > MMIO_START {
        // Overlaps with MMIO range, skip to after it
        current_address = MMIO_SKIP_TO.into();
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
      // No free slots available - return null instead of invalid pointer
      core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
      // Find the block and mark it as free
      let target_ptr: ArbitraryPtr = ptr.into();
      if let Some(index) = self.find_block_index(target_ptr, layout.size()) {
        #[allow(static_mut_refs)] // SAFETY: We are the only ones accessing GLOBAL_ALLOCATOR_STATE here
        unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks[index] = None };
      }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
      // Find the current block
      let current_ptr: ArbitraryPtr = ptr.into();
      let block_index = match self.find_block_index(current_ptr, layout.size()) {
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
      let extension_size = new_size - block.size;
      let can_extend = {
        // Check for overlap with other blocks and MMIO
        let mut can_extend = true;
        
        // Check MMIO range overlap
        let extended_end = next_address + extension_size;
        if usize::from(next_address) < MMIO_SKIP_TO && usize::from(extended_end) > MMIO_START {
          can_extend = false;
        }
        
        // Check for overlap with other allocated blocks
        if can_extend {
          #[allow(static_mut_refs)] // SAFETY: We are the only ones accessing GLOBAL_ALLOCATOR_STATE here
          for (index, other_block_option) in unsafe { GLOBAL_ALLOCATOR_STATE.allocated_blocks.iter().enumerate() } {
            if index == block_index {
              continue;
            }
            if let Some(other_block) = other_block_option {
              // Check if extension would overlap with this block
              let extension_end = next_address + extension_size;
              if next_address < other_block.ptr + other_block.size && extension_end > other_block.ptr {
                can_extend = false;
                break;
              }
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