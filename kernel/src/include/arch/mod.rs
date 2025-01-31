//! Arch-specific code. This module re-exports all code from the architecture being used.

mod x86_asmp;

pub use x86_asmp::*;