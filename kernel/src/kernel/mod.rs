//! This provides raw methods for internal kernel usage for the Aphrodite
//! kernel. See aphrodite_user for userspace.
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(incomplete_features)]
#![feature(ptr_metadata)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(nonnull_provenance)]
#![allow(internal_features)]
#![feature(generic_const_exprs)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod arch;
pub mod boot;
pub mod cmdline;
mod constants;
pub mod display;
mod errors;
pub mod indep_boot_entry;
pub mod mem;
pub mod memsections;
pub mod multiboot2;
pub mod output;
pub mod psfont;
mod traits;
mod util;

pub(crate) mod power_on_tests;

#[macro_use]
pub(crate) mod cfg;

#[allow(unused_imports)] // if there are no constants, then it gives a warning
pub use constants::*;

pub use errors::*;
pub use util::*;

#[allow(unused_imports)] // if there are no traits, then it gives a warning
pub use traits::*;

/// Returns the version of aphrodite.
pub const fn version() -> &'static str { env!("VERSION") }

/// Returns the version of the config for aphrodite.
pub const fn cfg_version() -> &'static str { env!("CFG_VERSION") }
