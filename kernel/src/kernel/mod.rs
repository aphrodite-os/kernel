//! This provides raw methods for internal kernel usage for the Aphrodite kernel. See aphrodite_user for userspace.
#![no_std]
#![warn(missing_docs)]
// tidy-alphabetical-start
#![feature(ptr_metadata)]
#![feature(const_trait_impl)]
#![feature(f128)]
#![feature(ptr_alignment_type)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(nonnull_provenance)]
#![feature(min_specialization)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
// tidy-alphabetical-end

pub mod indep_boot_entry;
pub mod arch;
pub mod boot;
pub mod cmdline;
mod constants;
pub mod display;
mod errors;
pub mod mem;
pub mod multiboot2;
pub mod output;
pub mod psfont;
mod traits;
mod util;

#[macro_use]
mod cfg;

#[allow(unused_imports)] // if there are no constants, then it gives a warning
pub use constants::*;

pub use errors::*;
pub use util::*;

#[allow(unused_imports)] // if there are no traits, then it gives a warning
pub use traits::*;
