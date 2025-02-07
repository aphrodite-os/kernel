//! The main code for the kernel.
#![warn(missing_docs)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]

use core::alloc::{Allocator, Layout};

use crate::output::*;

const MEM_TEST_SIZES: [usize; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

/// The real entrypoint to the kernel. `internel/arch/*/entry.rs` files eventually call this.
#[allow(non_snake_case)]
pub fn _entry(display: Option<&dyn crate::display::TextDisplay>, BI: &crate::boot::BootInfo) -> ! {
    let mut mem_map = BI.memory_map.unwrap();
    let allocator = crate::mem::MemoryMapAlloc::new(&mut mem_map).unwrap();
    tdebugsln("Testing allocator...", display.unwrap());

    for size in MEM_TEST_SIZES {
        tdebugs("Allocating ", display.unwrap());
        tdebugbnp(&crate::usize_as_u8_slice(size), display.unwrap());
        tdebugsnpln(" bytes of memory...", display.unwrap());
        if let Err(_) = allocator.allocate(Layout::from_size_align(size, 1).unwrap()) {
            terrors("Failed to allocate: ",display.unwrap());
            unsafe { crate::mem::LAST_MEMMAP_ERR.unwrap_err().display_np(display.unwrap()) }
        }
    }
    loop {}
}