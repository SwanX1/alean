use core::{alloc::{GlobalAlloc, Layout}, ptr::NonNull};

use crate::alloc::arbitrary_ptr::ArbitraryPtr;

unsafe extern "C" {
  // SAFETY: linker provides this symbol
  static __end: u8;
}

#[inline(always)]
const fn kernel_end() -> usize {
  let kernel_end = unsafe { __end as usize };
  kernel_end
}

/// Memory-Mapped I/O (MMIO) region start address for BCM2835.
/// This region should never be used for heap allocations.
const MMIO_START: usize = 0x2000_0000;

/// Address just after the MMIO region (0x2000_0000 to 0x20FF_FFFF) where allocations can resume.
const MMIO_SKIP_TO: usize = 0x2100_0000;

/// Maximum memory address for allocations.
/// This should ideally be determined dynamically based on available RAM.
const MEMORY_CAP: usize = 0x4000_0000;

/// The amount of regions to allocate space for when expanding the regions array.
const ALLOCATOR_REGION_INCREASE: usize = 1024;

#[derive(Clone)]
struct Region {
  start: ArbitraryPtr,
  size: usize,
}

impl Region {
  const fn new(start: ArbitraryPtr, size: usize) -> Self {
    Self {
      start,
      size,
    }
  }

  fn start(&self) -> ArbitraryPtr {
    self.start
  }

  fn end(&self) -> ArbitraryPtr {
    let addr= self.start;
    addr + self.size
  }

  fn contains(&self, ptr: ArbitraryPtr) -> bool {
    let addr = ptr;
    let start_addr = self.start;
    addr >= start_addr && addr < start_addr + self.size
  }

  fn overlaps(&self, other: &Region) -> bool {
    let self_start = self.start;
    let self_end = self.end();
    let other_start = other.start;
    let other_end = other.end();

    !(self_end <= other_start || other_end <= self_start)
  }
}

/// A sparse vector to track allocated regions.
/// May contain None entries for deallocated regions.
/// Does not shrink or compact on deallocation.
pub struct RegionSparseVec {
  // Note: This array also contains the Region for the regions array itself.
  regions: NonNull<Option<Region>>, // [Option<Region>; capacity]
  capacity: usize,
}

impl RegionSparseVec {
  /// SAFETY: Caller must ensure `capacity` is at least 1, reasonable and memory is available.
  /// Caller must also ensure that RegionVec is the only existing instance using this memory,
  /// otherwise soundness for the RegionVec's own allocated Regions is not guaranteed.
  /// This must be called only once during system initialization.
  unsafe fn new(capacity: usize) -> Self {
    debug_assert!(capacity > 0); // Capacity of 0 is not useful.
    
    let regions_layout = core::alloc::Layout::array::<Option<Region>>(capacity);
    // SAFETY: Caller must ensure `capacity` is reasonable and memory is available.
    let regions_layout = unsafe { regions_layout.unwrap_unchecked() };

    // SAFETY: kernel_end() is assumed to be non-null.
    let regions_ptr = unsafe { ArbitraryPtr::new_unchecked(kernel_end() as *mut ()) }.align(regions_layout);

    // This isn't inherently unsafe, but in future references in code,
    // self.regions will be dereferenced, so to avoid many safety comments:
    // SAFETY: Caller ensured that this memory is available by the contract of this function.
    let regions = regions_ptr.as_ptr().cast::<Option<Region>>();

    // Fill with nulls initially.
    for i in 0..capacity {
      // SAFETY: Size of Option<Region> is 8, while align is 4, so this pointer arithmetic
      // is safe and cannot misalign a pointer.
      let region_ptr = unsafe { regions.add(i) };
      // SAFETY: We ensure i < capacity, so this is safe.
      unsafe { region_ptr.write(None) };
    }

    // Insert region for the regions array itself.
    // SAFETY: regions_ptr is valid and properly aligned for Region.
    // There are no other contents of this array yet, so no overlap is possible.
    let regions_region = Region::new(regions_ptr, regions_layout.size());

    // SAFETY: We ensured capacity > 0, so this is safe.
    unsafe { regions.write(Some(regions_region)) };

    Self {
      regions,
      capacity,
    }
  }

  fn capacity(&self) -> usize {
    self.capacity
  }

  fn is_full(&self) -> bool {
    // If there's any null entry, we're not full.
    for i in 0..self.capacity {
      // SAFETY: We ensure i < self.capacity, so this is safe.
      let region = unsafe { &*self.regions.as_ptr().add(i) };
      if region.is_none() {
        return false;
      }
    }
    true
  }

  /// Resizes the internal storage to accommodate at least `additional` more elements.
  /// SAFETY: Caller must ensure that `additional` is reasonable and memory is available.
  unsafe fn reserve(&mut self, additional: usize) {
    let new_region_layout = core::alloc::Layout::array::<Option<Region>>(self.capacity + additional);
    // SAFETY: Caller ensures that `additional` is reasonable and memory is available.
    let new_region_layout = unsafe { new_region_layout.unwrap_unchecked() };

    let new_regions_ptr = self.next_available_address(new_region_layout);
    let new_regions_ptr = match new_regions_ptr {
      Some(ptr) => ptr,
      None => return, // Unable to find space for new region; do nothing.
    };

    // Replace the existing region for the regions array.
    unsafe {
      // SAFETY: We ensured that new_regions_ptr is valid and properly aligned for Region.
      // The first entry in the regions array is the region for the regions array itself,
      // which we are moving right now.
      self.regions.as_ptr().write(Some(Region::new(new_regions_ptr, new_region_layout.size())))
    };

    let new_regions = new_regions_ptr.as_ptr().cast::<Option<Region>>();
    // Copy existing entries to the new location.
    for i in 0..self.capacity {
      // SAFETY: We ensure i < self.capacity, so this is safe.
      let old_region_ptr = unsafe { self.regions.as_ptr().add(i) };
      let new_region_ptr = unsafe { new_regions.add(i) };
      // SAFETY: We ensure i < self.capacity, so this is safe.
      let region_opt = unsafe { &*old_region_ptr };
      // SAFETY: new_region_ptr is valid and properly aligned.
      unsafe { new_region_ptr.write(region_opt.clone()) };
    }

    // Fill the new entries with None.
    for i in self.capacity..(self.capacity + additional) {
      let new_region_ptr = unsafe { new_regions.add(i) };
      // SAFETY: new_region_ptr is valid and properly aligned.
      unsafe { new_region_ptr.write(None) };
    }

    self.regions = new_regions;
    self.capacity += additional;
  }

  fn remove(&mut self, index: usize) {
    if index >= self.capacity {
      return;
    }
    // SAFETY: We ensure index < self.capacity, so this is safe.
    unsafe { self.regions.as_ptr().add(index).write(None) };
  }

  fn get(&self, index: usize) -> Option<&Region> {
    if index >= self.capacity {
      return None;
    }
    // SAFETY: We ensure index < self.capacity, so this is safe.
    let region_opt = unsafe { &*self.regions.as_ptr().add(index) };
    region_opt.as_ref()
  }

  fn get_mut(&mut self, index: usize) -> Option<&mut Region> {
    if index >= self.capacity {
      return None;
    }
    // SAFETY: We ensure index < self.capacity, so this is safe.
    let region_opt = unsafe { &mut *self.regions.as_ptr().add(index) };
    region_opt.as_mut()
  }

  fn set(&mut self, index: usize, region: Region) -> bool {
    if index >= self.capacity {
      return false;
    }
    // SAFETY: We ensure index < self.capacity, so this is safe.
    unsafe { self.regions.as_ptr().add(index).write(Some(region)) };
    true
  }

  fn find_region_index(&self, ptr: ArbitraryPtr) -> Option<usize> {
    for i in 0..self.capacity {
      // SAFETY: We ensure i < self.capacity, so this is safe.
      let region_opt = unsafe { &*self.regions.as_ptr().add(i) };
      if let Some(region) = region_opt {
        if region.contains(ptr) {
          return Some(i);
        }
      }
    }
    None
  }

  // Finds the next available address that can fit a region of the given layout,
  // avoiding overlaps with existing allocated regions.
  // Returns None if no suitable address is found within MEMORY_CAP.
  // Doesn't check if the vec is full.
  fn next_available_address(&self, layout: Layout) -> Option<ArbitraryPtr> {
    let mut current_addr = unsafe { ArbitraryPtr::new_unchecked(kernel_end() as *mut ()) };
    current_addr.align_in_place(layout);

    while (Into::<usize>::into(current_addr) + layout.size()) <= MEMORY_CAP {
      let candidate_region = Region::new(current_addr, layout.size());

      // Check for overlaps with existing regions.
      let mut overlaps = false;
      for i in 0..self.capacity {
        // SAFETY: We ensure i < self.capacity, so this is safe.
        let region_opt = unsafe { &*self.regions.as_ptr().add(i) };
        if let Some(region) = region_opt {
          if candidate_region.overlaps(region) {
            overlaps = true;
            break;
          }
        }
      }

      if !overlaps {
        return Some(current_addr);
      }

      // Move to the next aligned address after the current candidate region.
      let next_addr = Into::<usize>::into(candidate_region.end());
      current_addr = ArbitraryPtr::from(next_addr).align(layout);
    }

    None
  }

  fn can_region_grow(&self, region: &Region, new_size: usize) -> bool {
    let new_region = Region::new(region.start(), new_size);

    for i in 0..self.capacity {
      // SAFETY: We ensure i < self.capacity, so this is safe.
      let region_opt = unsafe { &*self.regions.as_ptr().add(i) };
      if let Some(existing_region) = region_opt {
        // Skip checking against itself.
        if existing_region.start() == region.start() {
          continue;
        }
        if new_region.overlaps(existing_region) {
          return false; // Overlap detected, cannot grow.
        }
      }
    }

    true // No overlaps, can grow.
  }

  fn grow_region(&mut self, index: usize, new_size: usize) -> bool {
    if index >= self.capacity {
      return false;
    }
    // SAFETY: We ensure index < self.capacity, so this is safe.
    let region_opt = unsafe { &*self.regions.as_ptr().add(index) };
    let region = match region_opt {
      Some(r) => r,
      None => return false, // No region at this index to grow.
    };

    if !self.can_region_grow(region, new_size) {
      return false; // Cannot grow due to overlaps.
    }

    if let Some(region_mut) = self.get_mut(index) {
      region_mut.size = new_size;
    }

    true
  }
}

/// SAFETY: RegionSparseVec does not contain any non-Send types, except for NonNull.
unsafe impl Sync for RegionSparseVec {}

pub struct Allocator {
  // Option to allow for late initialization.
  regions_vec: Option<RegionSparseVec>,
}

impl Allocator {
  /// SAFETY: Caller must also ensure that Allocator is the only existing instance using this memory,
  /// otherwise soundness for the other instances is not guaranteed.
  /// This must be called only once during system initialization.
  const unsafe fn new() -> Self {
    Self { regions_vec: None }
  }

  fn regions_vec(&mut self) -> &mut RegionSparseVec {
    if self.regions_vec.is_none() {
      // SAFETY: This is called only once during system initialization, and ALLOCATOR_REGION_INCREASE is at least 1 and reasonable.
      self.regions_vec = Some(unsafe { RegionSparseVec::new(ALLOCATOR_REGION_INCREASE) });
      // Allocate MMIO skip region to avoid allocations there.
      let mmio_skip_layout = Layout::from_size_align(MMIO_SKIP_TO - MMIO_START, 4).unwrap();
      // SAFETY: MMIO_START is not null
      let mmio_skip_addr = unsafe { ArbitraryPtr::new_unchecked(MMIO_START as *mut ()) };
      let mmio_skip_region = Region::new(mmio_skip_addr, mmio_skip_layout.size());
      // Insert the MMIO skip region into the regions_vec.
      // Use index 1, as index 0 is used for the regions array itself.
      // SAFETY: We ensured regions_vec is Some above, so this is safe.
      unsafe { self.regions_vec.as_mut().unwrap_unchecked() }.set(1, mmio_skip_region);
    }
    // SAFETY: We ensured regions_vec is Some above, so this is safe.
    unsafe { self.regions_vec.as_mut().unwrap_unchecked() }
  }

  fn allocate(&mut self, layout: Layout) -> Option<ArbitraryPtr> {
    if self.regions_vec().is_full() {
      // Try to reserve more space.
      // SAFETY: ALLOCATOR_REGION_INCREASE is reasonable and memory is available.
      unsafe { self.regions_vec().reserve(ALLOCATOR_REGION_INCREASE) };
      if self.regions_vec().is_full() {
        return None; // Still full after trying to reserve more space.
      }
    }

    let addr = self.regions_vec().next_available_address(layout)?;
    let new_region = Region::new(addr, layout.size());

    // Insert the new region into the first available slot.
    for i in 0..self.regions_vec().capacity() {
      let region_opt = self.regions_vec().get(i);
      if region_opt.is_none() {
        self.regions_vec().set(i, new_region);
        return Some(addr);
      }
    }

    None // Should not reach here due to earlier checks.
  }

  fn deallocate(&mut self, ptr: ArbitraryPtr) {
    if let Some(index) = self.regions_vec().find_region_index(ptr) {
      self.regions_vec().remove(index);
    }
  }

  fn reallocate(&mut self, ptr: ArbitraryPtr, new_size: usize) -> Option<ArbitraryPtr> {
    let index = self.regions_vec().find_region_index(ptr)?;
    let region = self.regions_vec().get(index)?;
    if region.size >= new_size {
      return Some(region.start()); // Current region is already large enough.
    }

    if self.regions_vec().grow_region(index, new_size) {
      let region = self.regions_vec().get(index)?;
      return Some(region.start());
    }

    // Cannot grow in place; allocate a new region.
    let new_layout = Layout::from_size_align(new_size, 4).ok()?;
    let new_addr = self.allocate(new_layout)?;
    // Copy existing data to the new region.
    let old_region = self.regions_vec().get(index)?;

    // SAFETY: Both old_region and new_addr are valid for the sizes involved.
    // They do not overlap as new_addr is freshly allocated.
    unsafe {
      core::ptr::copy_nonoverlapping(
        old_region.start().as_ptr().as_ptr(),
        new_addr.as_ptr().as_ptr(),
        old_region.size.min(new_size),
      );
    }

    // Deallocate the old region.
    self.deallocate(ptr);

    Some(new_addr)
  }
}


#[global_allocator]
static ALLOC_WRAPPER: AllocWrapper = AllocWrapper;
// SAFETY: This is only called once, here.
static mut GLOBAL_ALLOCATOR: Allocator = unsafe { Allocator::new() };

struct AllocWrapper;

unsafe impl GlobalAlloc for AllocWrapper {
  unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
    #[allow(static_mut_refs)]
    unsafe { GLOBAL_ALLOCATOR.allocate(layout) }
    .map_or(core::ptr::null_mut(), |ptr| ptr.as_ptr().as_ptr() as *mut u8)
  }
  
  unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
    if ptr.is_null() {
      return;
    }
    let arbitrary_ptr = unsafe { ArbitraryPtr::new_unchecked(ptr as *mut ()) };
    #[allow(static_mut_refs)]
    unsafe { GLOBAL_ALLOCATOR.deallocate(arbitrary_ptr) };
  }
  
  unsafe fn realloc(
    &self,
    ptr: *mut u8,
    old_layout: core::alloc::Layout,
    new_size: usize,
  ) -> *mut u8 {
    if ptr.is_null() {
      // Equivalent to alloc.
      return unsafe { self.alloc(Layout::from_size_align_unchecked(new_size, old_layout.align())) };
    }
    let arbitrary_ptr = unsafe { ArbitraryPtr::new_unchecked(ptr as *mut ()) };
    #[allow(static_mut_refs)]
    let new_ptr = unsafe { GLOBAL_ALLOCATOR.reallocate(arbitrary_ptr, new_size) };
    new_ptr.map_or(core::ptr::null_mut(), |p| p.as_ptr().as_ptr() as *mut u8)
  }
}