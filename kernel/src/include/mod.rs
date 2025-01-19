//! This provides syscalls(for userspace programs) and types(for userspace and kernelspace programs) for the Aphrodite kernel.

#![no_std]
#![warn(missing_docs)]

mod constants;
pub mod multiboot2;

pub use constants::*;