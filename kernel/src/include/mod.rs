//! This provides syscalls(for userspace programs) and types(for userspace and kernelspace programs) for the Aphrodite kernel.
#![no_std]
#![warn(missing_docs)]
#![feature(ptr_metadata)]
#![feature(const_trait_impl)]

mod constants;
mod util;
pub mod multiboot2;
pub mod arch;
mod errors;
pub mod _entry;
mod traits;
pub mod output;
pub mod boot;

#[allow(unused_imports)] // if there are no constants, then it gives a warning
pub use constants::*;

pub use errors::*;
pub use util::*;
pub use traits::*;