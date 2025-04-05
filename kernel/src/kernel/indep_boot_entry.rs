//! The main code for the kernel.
#![warn(missing_docs)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]

use crate::arch::output::*;
use crate::display::{COLOR_DEFAULT, NoneTextDisplay};
use crate::output::*;

use aphrodite_proc_macros::*;

/// The real entrypoint to the kernel. `internel/arch/*/entry.rs` files
/// eventually call this.
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

    let display = display.unwrap_or(&NoneTextDisplay {});

    display.clear_screen(COLOR_DEFAULT);
    sreset();

    let mem_map = BI.memory_map.unwrap();
    crate::mem::MemMapAllocInit(mem_map).unwrap();

    crate::arch::alloc_available_boot();

    if cfg!(not(CONFIG_POWERON_TESTS = "false")) {
        crate::power_on_tests::run(display);

        tinfosln("Successfully ran all configured power on tests", display).unwrap();
    }

    if cfg!(CONFIG_PREUSER_OUTPUT_DEBUG = "true") {
        if let Some(load_base) = BI.load_base {
            sdebugs("Image load base address is ");
            sdebugbnpln(&crate::u32_as_u8_slice(load_base));
        } else {
            sdebugsln("Image load base address was not provided");
        }
    }

    loop {}
}
