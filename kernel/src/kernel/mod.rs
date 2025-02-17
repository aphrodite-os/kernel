//! This provides raw methods for internal kernel usage for the Aphrodite kernel. See aphrodite_user for userspace.
#![no_std]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
// tidy-alphabetical-start
#![feature(ptr_metadata)]
#![feature(const_trait_impl)]
#![feature(f128)]
#![feature(ptr_alignment_type)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(stmt_expr_attributes)]
#![feature(nonnull_provenance)]
#![feature(min_specialization)]
#![feature(ptr_as_uninit)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
// tidy-alphabetical-end

extern crate alloc;

pub mod arch;
pub mod boot;
pub mod cmdline;
mod constants;
pub mod display;
mod errors;
pub mod indep_boot_entry;
pub mod mem;
pub mod multiboot2;
pub mod output;
pub mod psfont;
mod traits;
mod util;
pub mod memsections;

#[macro_use]
pub(crate) mod cfg;

#[allow(unused_imports)] // if there are no constants, then it gives a warning
pub use constants::*;

pub use errors::*;
pub use util::*;

#[allow(unused_imports)] // if there are no traits, then it gives a warning
pub use traits::*;
