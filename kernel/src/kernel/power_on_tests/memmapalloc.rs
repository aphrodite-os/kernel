#![cfg(all(
    not(CONFIG_POWERON_TESTS = "false"),
    not(CONFIG_POWERON_TEST_ALLOC = "false")
))]

use crate::display::TextDisplay;
use crate::output::*;

use core::alloc::{Allocator, Layout};

const MEM_TEST_SIZES: [usize; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

pub fn run(display: &dyn TextDisplay) {
    let allocator = crate::mem::get_allocator().unwrap();
    tdebugsln("Testing allocator...", display).unwrap();

    for size in MEM_TEST_SIZES {
        tdebugs("Number of allocations: ", display).unwrap();
        tdebugbnpln(
            &crate::u64_as_u8_slice(allocator.number_of_allocations()),
            display,
        )
        .unwrap();

        tdebugs("Allocating ", display).unwrap();
        tdebugbnp(&crate::usize_as_u8_slice(size), display).unwrap();
        tdebugsnpln(" byte(s) of memory...", display).unwrap();

        let allocation = allocator.allocate(Layout::from_size_align(size, 1).unwrap());
        if let Err(_) = allocation {
            terrors("Failed to allocate: ", display).unwrap();
            unsafe { crate::mem::LAST_MEMMAP_ERR.unwrap_err().display_np(display) }
            panic!("Allocator test failure");
        } else if let Ok(ptr) = allocation {
            tdebugs("Successfully allocated! Address is ", display).unwrap();
            tdebugbnp(&crate::usize_as_u8_slice(ptr.addr().get()), display).unwrap();
            tdebugsnpln(".", display).unwrap();
            tdebugsln("", display).unwrap();
            tdebugsln("Deallocating memory...", display).unwrap();
            unsafe {
                allocator.deallocate(
                    ptr.as_non_null_ptr(),
                    Layout::from_size_align(size, 1).unwrap(),
                )
            }
            if let Err(err) = unsafe { crate::mem::LAST_MEMMAP_ERR } {
                terrors("Failed to deallocate: ", display).unwrap();
                err.display_np(display);
                panic!("Deallocation failure");
            } else {
                tdebugsln("Successfully deallocated!", display).unwrap();
            }
        }
        tdebugsln("", display).unwrap();
    }
}
