//! Memory allocation.

use core::{
    alloc::{Allocator, GlobalAlloc},
    fmt::Debug,
    mem::MaybeUninit,
    num::NonZero,
    ops::Range,
    ptr::{NonNull, null_mut},
};

use crate::boot::{MemoryMap, MemoryType};

use aphrodite_proc_macros::*;

#[derive(Clone, Copy)]
struct Allocation {
    /// Whether this allocation is used. This is used so that the
    /// entire allocation table doesn't need to be shifted back
    /// on every allocation.
    pub used: bool,
    /// The starting address of the allocation.
    pub addr: u64,
    /// The length of the allocation.
    pub len: u64,
}

#[derive(Clone, Copy)]
struct AllocationHeader {
    /// Whether this allocation table is used. Kept for parity with [Allocation]s.
    #[allow(dead_code)]
    pub used: bool,
    /// The starting address of the allocation table.
    #[allow(dead_code)]
    pub addr: u64,
    /// The length in bytes of the allocation table.
    pub len: u64,
    /// The number of allocations in the allocation table.
    pub num_allocations: u64,
}

struct AllocationIter {
    ptr: *const Allocation,
    num_allocations: u64,
    idx: u64,
}

impl Iterator for AllocationIter {
    type Item = *mut Allocation;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.idx += 1;
        if self.idx > self.num_allocations {
            return None;
        }
        crate::arch::output::sdebugsln("Providing allocation from iterator");

        Some(&unsafe {
            *((self.ptr as usize + (size_of::<Allocation>() * (self.idx as usize - 1)))
                as *const Allocation)
        } as *const Allocation as *mut Allocation)
    }
}

#[global_allocator]
static mut ALLOCATOR: MaybeMemoryMapAlloc<'static> = MaybeMemoryMapAlloc::new(None);
static mut ALLOCATOR_MEMMAP: MaybeUninit<MemoryMap> = MaybeUninit::uninit();
static mut ALLOCATOR_INITALIZED: bool = false;

#[kernel_item(MemMapAlloc)]
fn get_allocator() -> Option<&'static MemoryMapAlloc<'static>> {
    if unsafe { ALLOCATOR_INITALIZED } {
        #[allow(static_mut_refs)]
        return Some(unsafe { ALLOCATOR.assume_init_ref() });
    } else {
        return None;
    }
}

/// The unsafe counterpart of [MemMapAlloc()]. Doesn't check if the allocator is initalized.
/// Internally, uses [MaybeUninit::assume_init_ref].
///
/// # Safety
///
/// Calling this instead of [MemMapAlloc] or when the allocator is uninitalized causes
/// undefined behavior; check [MaybeUninit::assume_init_ref] for safety guarantees.
pub unsafe fn get_allocator_unchecked() -> &'static MemoryMapAlloc<'static> {
    #[allow(static_mut_refs)]
    unsafe {
        ALLOCATOR.assume_init_ref()
    }
}

#[kernel_item(MemMapAllocInit)]
fn memory_map_alloc_init(memmap: crate::boot::MemoryMap) -> Result<(), crate::Error<'static>> {
    #[allow(static_mut_refs)]
    unsafe {
        ALLOCATOR_MEMMAP.write(memmap);
    }
    #[allow(static_mut_refs)]
    let alloc = MemoryMapAlloc::new(unsafe { ALLOCATOR_MEMMAP.assume_init_mut() })?;

    unsafe {
        #[allow(static_mut_refs)]
        ALLOCATOR.add_alloc(alloc);
    }
    unsafe {
        ALLOCATOR_INITALIZED = true;
    }

    Ok(())
}

/// A implementation of a physical memory allocator that uses a [crate::boot::MemoryMap].
pub struct MemoryMapAlloc<'a> {
    /// The memory map to use to allocate memory.
    pub memory_map: &'a mut crate::boot::MemoryMap,

    allocationheader: *mut AllocationHeader,
    allocations: *mut Allocation,
    max_allocations_size: u64,
}

/// Too many allocations have been created, pushing the size of [MemoryMapAlloc::allocations] over [MemoryMapAlloc::max_allocations_size].
pub const TOO_MANY_ALLOCATIONS: i16 = -2;

/// There isn't enough space for 32 allocations(the minimum available).
pub const ALLOCATIONS_NOT_ENOUGH_SPACE: i16 = -3;

/// The index provided to [MemoryMapAlloc::extend_allocation] is too big.
pub const EXTEND_ALLOCATION_INVALID_INDEX: i16 = -4;

/// The allocation provided to [MemoryMapAlloc::extend_allocation] is unused.
pub const EXTEND_ALLOCATION_ALLOCATION_UNUSED: i16 = -5;

/// The allocation provided to [MemoryMapAlloc::extend_allocation], if extended, would extend into another allocation.
pub const EXTEND_ALLOCATION_OTHER_ALLOCATION: i16 = -6;

impl<'a> Debug for MemoryMapAlloc<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("MemoryMapAlloc with ")?;
        f.write_str(
            core::str::from_utf8(&crate::u64_as_u8_slice(
                unsafe { *self.allocationheader }.num_allocations,
            ))
            .unwrap(),
        )?;
        f.write_str(" allocations")?;
        Ok(())
    }
}

impl<'a> MemoryMapAlloc<'a> {
    /// Creates a new [MemoryMapAlloc]. Please call this method instead of creating it manually!
    ///
    /// This method internally stores the memory map in the outputted MemoryMapAlloc.
    ///
    /// Note that this function will return an error only if there isn't enough allocatable space
    /// for at least 32 allocations.
    pub fn new(
        memory_map: &'a mut crate::boot::MemoryMap,
    ) -> Result<MemoryMapAlloc<'a>, crate::Error<'a>> {
        let mut out = MemoryMapAlloc {
            memory_map,
            allocations: core::ptr::null_mut(),
            allocationheader: core::ptr::null_mut(),
            max_allocations_size: 0,
        };
        out.memory_map.reset_iter();
        for mapping in &mut *out.memory_map {
            mapping.output();
            crate::arch::output::sdebugsnpln("");
            if mapping.len < (size_of::<Allocation>() * 32) as u64 {
                continue;
            }
            if mapping.mem_type == MemoryType::Free {
                out.allocationheader = core::ptr::from_raw_parts_mut(
                    core::ptr::without_provenance_mut::<()>(mapping.start as usize),
                    (),
                );
                out.allocations = core::ptr::from_raw_parts_mut(
                    core::ptr::without_provenance_mut::<()>(
                        mapping.start as usize + size_of::<AllocationHeader>(),
                    ),
                    (),
                );
                out.max_allocations_size = mapping.len;
            } else if let MemoryType::HardwareSpecific(_, allocatable) = mapping.mem_type {
                if allocatable {
                    out.allocationheader = core::ptr::from_raw_parts_mut(
                        core::ptr::without_provenance_mut::<()>(mapping.start as usize),
                        (),
                    );
                    out.allocations = core::ptr::from_raw_parts_mut(
                        core::ptr::without_provenance_mut::<()>(
                            mapping.start as usize + size_of::<AllocationHeader>(),
                        ),
                        (),
                    );
                    out.max_allocations_size = mapping.len;
                }
            }
        }
        if out.allocations == core::ptr::null_mut() {
            return Err(crate::Error::new(
                "no free memory with space for 32 allocations",
                ALLOCATIONS_NOT_ENOUGH_SPACE,
            ));
        }
        unsafe {
            (*out.allocations) = Allocation {
                used: false,
                addr: 0,
                len: 0,
            };
            (*out.allocationheader) = AllocationHeader {
                used: true,
                addr: out.allocations as usize as u64,
                len: (size_of::<Allocation>() * 32) as u64,
                num_allocations: 1,
            }
        }
        Ok(out)
    }

    /// Returns the number of allocations.
    pub fn number_of_allocations(&self) -> u64 {
        unsafe { *self.allocationheader }.num_allocations
    }

    /// Creates a [AllocationIter] to iterate over the current allocations.
    fn allocations_iter(&self) -> AllocationIter {
        AllocationIter {
            ptr: self.allocations,
            num_allocations: unsafe { *self.allocationheader }.num_allocations,
            idx: 0,
        }
    }

    /// Check to see if any allocations contain the given address. Returns true if so.
    fn check_addr(&self, addr: u64) -> bool {
        if cfg!(CONFIG_MEMORY_UNION_ALL = "true") {
            return false;
        }
        if addr >= (self.allocationheader as u64)
            && addr < (self.allocationheader as u64 + unsafe { *self.allocationheader }.len)
        {
            return true;
        }
        for ele in self.allocations_iter() {
            let alloc = unsafe { *ele };
            if addr >= alloc.addr && addr < alloc.addr + alloc.len {
                return true;
            }
        }
        false
    }

    /// Check to see if a range of addresses have any allocations within. Returns true if so.
    fn check_range(&self, addr: Range<u64>) -> bool {
        if cfg!(CONFIG_MEMORY_UNION_ALL = "true") {
            return false;
        }
        for addr in addr {
            // REALLY inefficient, but I don't think there's a better way.
            if self.check_addr(addr) {
                return true;
            }
        }
        false
    }

    #[allow(unused)]
    fn output_number(&self, num: u64, prefix: &str) {
        crate::arch::output::sdebugs(prefix);
        crate::arch::output::sdebugb(&crate::u64_as_u8_slice(num));
    }

    /// Print debug info about an allocation
    #[allow(unused)]
    fn debug_allocation_info(&self, allocation: &Allocation) {
        self.output_number(allocation.addr, "Allocation at 0x");
        self.output_number(allocation.len, " with length 0x");
        crate::arch::output::sdebugs(" is ");
        crate::arch::output::sdebugsnpln(if allocation.used { "used" } else { "free" });
    }

    /// Zero out a memory region
    #[allow(unused)]
    unsafe fn zero_memory_region(&self, addr: u64, len: u64) {
        unsafe {
            core::ptr::write_bytes(addr as *mut u8, 0, len as usize);
        }
    }

    /// Try to merge adjacent free blocks
    #[allow(unused)]
    fn try_merge_blocks(&self) {
        if self.allocations.is_null() {
            return;
        }

        let num_allocs = unsafe { (*self.allocationheader).num_allocations };
        let mut i = 0;
        while i < num_allocs {
            let current = unsafe {
                &mut *((self.allocations as usize + size_of::<Allocation>() * (i as usize)) 
                    as *mut Allocation)
            };

            if current.used {
                i += 1;
                continue;
            }

            let mut merged = false;
            let mut j = i + 1;
            while j < num_allocs {
                let next = unsafe {
                    &mut *((self.allocations as usize + size_of::<Allocation>() * (j as usize))
                        as *mut Allocation)
                };

                if next.used {
                    break;
                }

                if current.addr + current.len == next.addr {
                    // Merge the blocks
                    current.len += next.len;
                    next.used = true; // Mark as merged
                    merged = true;
                }
                j += 1;
            }

            if !merged {
                i += 1;
            }
        }
    }

    /// Finds a free block of memory that can fit the requested size and alignment
    fn find_free_block(&self, size: u64, align: usize) -> Option<u64> {
        for mapping in self.memory_map.clone() {
            if mapping.len < size {
                continue;
            }

            let mut allocatable = false;
            if mapping.mem_type == MemoryType::Free {
                allocatable = true;
            } else if let MemoryType::HardwareSpecific(_, alloc) = mapping.mem_type {
                allocatable = alloc;
            }

            if !allocatable {
                continue;
            }

            // Try to find space from the end of the region
            let mut addr = mapping.start + mapping.len - size;
            while addr >= mapping.start {
                if addr % align as u64 == 0 && !self.check_range(addr..addr + size) {
                    return Some(addr);
                }
                addr -= size;
            }
        }
        None
    }

    /// Track a new allocation in the allocation table
    fn track_allocation(&self, addr: u64, size: u64) -> Result<(), crate::Error<'static>> {
        let allocation = Allocation {
            used: true,
            addr,
            len: size,
        };

        // First try to find an unused slot
        for alloc in self.allocations_iter() {
            if unsafe { !(*alloc).used } {
                unsafe { *alloc = allocation };
                return Ok(());
            }
        }

        // Need to add new slot
        unsafe { (*self.allocationheader).num_allocations += 1 };
        let num_allocs = unsafe { (*self.allocationheader).num_allocations };

        if num_allocs as usize * size_of::<Allocation>() > self.max_allocations_size as usize {
            unsafe { (*self.allocationheader).num_allocations -= 1 };
            return Err(crate::Error::new(
                "allocation table full",
                TOO_MANY_ALLOCATIONS,
            ));
        }

        let new_alloc = unsafe {
            &mut *((self.allocations as usize + size_of::<Allocation>() * (num_allocs as usize - 1))
                as *mut Allocation)
        };
        *new_alloc = allocation;

        Ok(())
    }

    /// Merge adjacent free blocks to reduce fragmentation
    fn merge_free_blocks(&self) {
        if self.allocations.is_null() {
            return;
        }

        let mut i = 0;
        while i < unsafe { (*self.allocationheader).num_allocations } {
            let current = unsafe {
                &mut *((self.allocations as usize + size_of::<Allocation>() * i as usize)
                    as *mut Allocation)
            };

            if current.used {
                i += 1;
                continue;
            }

            // Look ahead for adjacent free blocks to merge
            let mut j = i + 1;
            while j < unsafe { (*self.allocationheader).num_allocations } {
                let next = unsafe {
                    &mut *((self.allocations as usize + size_of::<Allocation>() * j as usize)
                        as *mut Allocation)
                };

                if next.used {
                    break;
                }

                // Merge if blocks are contiguous
                if current.addr + current.len == next.addr {
                    current.len += next.len;
                    next.used = true; // Mark as merged
                    next.addr = 0;
                    next.len = 0;
                }

                j += 1;
            }
            i += 1;
        }
    }

    /// Merge contiguous free memory blocks to reduce fragmentation.
    /// This should be called periodically to keep memory efficient.
    pub fn merge_contiguous_allocations(&self) {
        self.merge_free_blocks();
    }
}

unsafe impl<'a> Allocator for MemoryMapAlloc<'a> {
    fn allocate(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        unsafe { LAST_MEMMAP_ERR = Ok(()) };

        if self.allocations.is_null() {
            unsafe {
                LAST_MEMMAP_ERR = Err(crate::Error::new(
                    "allocator not initialized",
                    FREE_MEMORY_UNAVAILABLE,
                ))
            };
            return Err(core::alloc::AllocError);
        }

        // Try to find a suitable memory block
        let addr = match self.find_free_block(layout.size() as u64, layout.align()) {
            Some(addr) => addr,
            None => {
                unsafe {
                    LAST_MEMMAP_ERR = Err(crate::Error::new(
                        "no suitable memory block found",
                        FREE_MEMORY_UNAVAILABLE,
                    ))
                };
                return Err(core::alloc::AllocError);
            }
        };

        // Track the allocation
        if let Err(err) = self.track_allocation(addr, layout.size() as u64) {
            unsafe { LAST_MEMMAP_ERR = Err(err) };
            return Err(core::alloc::AllocError);
        }

        Ok(NonNull::from_raw_parts(
            NonNull::<u8>::without_provenance(NonZero::new(addr as usize).unwrap()),
            layout.size(),
        ))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: core::alloc::Layout) {
        let addr = ptr.addr().get() as u64;

        if self.allocations.is_null() {
            unsafe {
                LAST_MEMMAP_ERR = Err(crate::Error::new(
                    "allocator not initialized",
                    FREE_MEMORY_UNAVAILABLE,
                ))
            };
            return;
        }

        // Find the allocation
        let mut found = false;
        for i in 0..unsafe { (*self.allocationheader).num_allocations } {
            let alloc = unsafe {
                &mut *((self.allocations as usize + size_of::<Allocation>() * i as usize)
                    as *mut Allocation)
            };

            if alloc.used && alloc.addr == addr {
                // Zero the memory
                unsafe { core::ptr::write_bytes(addr as *mut u8, 0, alloc.len as usize) };
                
                // Mark as free
                alloc.used = false;
                found = true;
                break;
            }
        }

        if !found {
            unsafe {
                LAST_MEMMAP_ERR = Err(crate::Error::new(
                    "memory not allocated",
                    MEMORY_NOT_ALLOCATED,
                ))
            };
            return;
        }

        // Try to merge adjacent free blocks
        self.merge_free_blocks();
    }
}

/// Error returned when free memory is not available.
pub const FREE_MEMORY_UNAVAILABLE: i16 = -1;

/// Error returned when memory wasn't allocated.
pub const MEMORY_NOT_ALLOCATED: i16 = -7;

/// Error returned when the [MaybeMemoryMapAlloc] doesn't have
/// an initalized allocator.
pub const MAYBE_MEMORY_MAP_ALLOC_UNINITALIZED: i16 = -8;

struct MaybeMemoryMapAlloc<'a> {
    alloc: MaybeUninit<MemoryMapAlloc<'a>>,
    initalized: bool,
}
impl<'a> MaybeMemoryMapAlloc<'a> {
    const fn new(alloc: Option<MemoryMapAlloc<'a>>) -> Self {
        if alloc.is_none() {
            return MaybeMemoryMapAlloc {
                alloc: MaybeUninit::uninit(),
                initalized: false,
            };
        }
        MaybeMemoryMapAlloc {
            alloc: MaybeUninit::new(alloc.unwrap()),
            initalized: true,
        }
    }
    const unsafe fn assume_init_ref(&self) -> &MemoryMapAlloc<'a> {
        unsafe { self.alloc.assume_init_ref() }
    }
    /// Note that if the allocator isn't initalized then this will do nothing.
    const fn add_alloc(&mut self, alloc: MemoryMapAlloc<'a>) {
        if self.initalized {
            return;
        }
        self.alloc.write(alloc);
        self.initalized = true;
    }
}

unsafe impl<'a> GlobalAlloc for MaybeMemoryMapAlloc<'a> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if self.initalized {
            unsafe {
                LAST_MEMMAP_ERR = Err(crate::Error::new(
                    "MaybeMemoryMapAlloc not initalized",
                    MAYBE_MEMORY_MAP_ALLOC_UNINITALIZED,
                ))
            }
            return null_mut();
        }
        unsafe { self.alloc.assume_init_ref().alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        if self.initalized {
            unsafe {
                LAST_MEMMAP_ERR = Err(crate::Error::new(
                    "MaybeMemoryMapAlloc not initalized",
                    MAYBE_MEMORY_MAP_ALLOC_UNINITALIZED,
                ))
            }
            return;
        }
        unsafe { self.alloc.assume_init_ref().dealloc(ptr, layout) }
    }
}

unsafe impl<'a> Allocator for MaybeMemoryMapAlloc<'a> {
    fn allocate(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
        if !self.initalized {
            unsafe {
                LAST_MEMMAP_ERR = Err(crate::Error::new(
                    "MaybeMemoryMapAlloc not initalized",
                    MAYBE_MEMORY_MAP_ALLOC_UNINITALIZED,
                ))
            }
            return Err(core::alloc::AllocError {});
        }
        unsafe { self.alloc.assume_init_ref() }.allocate(layout)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: core::alloc::Layout) {
        if !self.initalized {
            unsafe {
                LAST_MEMMAP_ERR = Err(crate::Error::new(
                    "MaybeMemoryMapAlloc not initalized",
                    MAYBE_MEMORY_MAP_ALLOC_UNINITALIZED,
                ))
            }
            return;
        }
        unsafe { self.alloc.assume_init_ref().deallocate(ptr, layout) }
    }
}

unsafe impl<'a> GlobalAlloc for MemoryMapAlloc<'a> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let result = self.allocate(layout);
        if result.is_err() {
            return null_mut();
        }
        result.unwrap().as_mut_ptr()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        unsafe {
            self.deallocate(
                NonNull::without_provenance(NonZero::new(ptr as usize).unwrap()),
                layout,
            );
        }
    }
}

/// The last status of memory allocation or deallocation for a [MemoryMapAlloc].
/// This can be used for more insight to why an allocation or deallocation failed.
pub static mut LAST_MEMMAP_ERR: Result<(), crate::Error<'static>> = Ok(());