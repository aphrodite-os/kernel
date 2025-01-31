//! The main code for the kernel.
#![warn(missing_docs)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]

use crate::output::*;

/// The real entrypoint to the kernel. `internel/arch/*/entry.rs` files eventually call this.
#[allow(non_snake_case)]
pub fn _entry(display: Option<&dyn crate::display::TextDisplay>, BI: &crate::boot::BootInfo) -> ! {
    loop {}
}