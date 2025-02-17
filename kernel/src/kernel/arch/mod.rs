//! Arch-specific code. This module re-exports all code from the architecture being used.
//!
//! See [example_impl] for everything that has to be implemented by an architecture module.

pub mod example_impl;
mod x86;

pub use x86::*;

/// The enum returned by arch::*::get_arch.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Architecture {
    /// Returned by [example_impl]. If this is returned by arch::*::get_arch, something
    /// is incredibly wrong and a panic should occur immediately.
    #[default]
    ExampleDummy,
    /// 32-bit x86.
    X86,
}
