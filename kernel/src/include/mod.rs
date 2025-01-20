//! This provides syscalls(for userspace programs) and types(for userspace and kernelspace programs) for the Aphrodite kernel.
#![no_std]
#![warn(missing_docs)]
#![feature(ptr_metadata)]

mod constants;
pub mod multiboot2;
pub mod arch;
mod errors;

#[allow(unused_imports)] // if there are no constants, then it gives a warning
pub use constants::*;

pub use errors::*;