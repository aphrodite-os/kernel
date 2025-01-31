//! This provides raw methods for internal kernel usage for the Aphrodite kernel. See aphrodite_user for userspace.
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
pub mod psfont;
pub mod display;

#[allow(unused_imports)] // if there are no constants, then it gives a warning
pub use constants::*;

pub use errors::*;
pub use util::*;

#[allow(unused_imports)] // if there are no traits, then it gives a warning
pub use traits::*;