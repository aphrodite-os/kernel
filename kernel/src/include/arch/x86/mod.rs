//! General x86 functions
#![cfg(any(target_arch = "x86"))]

use core::arch::asm;

pub mod interrupts;
pub mod ports;
pub mod output;
pub mod egatext;

mod constants;

pub use constants::*;

/// Returns information from the CPUID command in the form
/// (ebx, edx, ecx).
pub fn cpuid(id: u32) -> (u32, u32, u32) {
    let mut out = (0u32, 0u32, 0u32);
    unsafe {
        asm!(
            "cpuid", in("eax") id, out("ebx") out.0, out("edx") out.1, out("ecx") out.2
        )
    }
    out
}

/// Returns whether extended functions are available
/// (more specifically, 0x80000001 or higher)
pub fn cpuid_extended_functions() -> bool {
    let out: u32;
    unsafe {
        asm!(
            "mov eax, 0x80000000",
            "cpuid", out("eax") out
        )
    }
    out >= 0x80000001
}