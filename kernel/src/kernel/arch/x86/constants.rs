//! Constants used throughout kernel code.
#![cfg(any(target_arch = "x86"))]

/// The assembly port number to output debug messages to.
pub(super) const DEBUG_PORT: u16 = 0xE9;
