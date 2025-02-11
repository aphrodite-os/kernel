//! Architecture-specific stuff, mainly syscall methods.

#[macro_use]
mod x86;

pub use x86::*;