//! The main code for the kernel.
#![warn(missing_docs)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]

use core::alloc::{Allocator, Layout};

use crate::{
    display::{COLOR_DEFAULT, NoneTextDisplay},
    output::*,
};

use aphrodite_proc_macros::*;

const MEM_TEST_SIZES: [usize; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

/// The real entrypoint to the kernel. `internel/arch/*/entry.rs` files eventually call this.
#[kernel_item(IndepBootEntry)]
fn indep_boot_entry(
    display: Option<&dyn crate::display::TextDisplay>,
    #[allow(non_snake_case)] BI: &crate::boot::BootInfo,
) -> ! {
    assert_ne!(
        crate::arch::get_arch(),
        crate::arch::Architecture::ExampleDummy,
        "Somehow the kernel successfully booted into IndepBootEntry with a dummy architecture"
    );
    crate::arch::output::sdebugsln("IndepBootEntry called");

    let display = display.unwrap_or(&NoneTextDisplay {});

    display.clear_screen(COLOR_DEFAULT);
    sreset();

    let mem_map = BI.memory_map.unwrap();
    crate::mem::MemMapAllocInit(mem_map).unwrap();
    let allocator = crate::mem::MemMapAlloc().unwrap();

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
            panic!("Allocation failure");
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
                allocator.merge_contiguous_allocations();
            }
        }
        tdebugsln("", display).unwrap();
    }
    loop {}
}
