//! Arch-specific code. This module re-exports all code from the architecture being used.

mod x86;

pub use x86::*;