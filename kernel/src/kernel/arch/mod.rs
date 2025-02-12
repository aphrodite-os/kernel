//! Arch-specific code. This module re-exports all code from the architecture being used.
//! 
//! See [example_impl] for everything that has to be implemented by an architecture module.

mod x86;
pub mod example_impl;

pub use x86::*;