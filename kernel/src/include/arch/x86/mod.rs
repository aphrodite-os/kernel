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
        // ebx is moved into eax as apparently ebx is used internally by LLVM
        asm!(
            "cpuid",
            "mov eax, ebx", in("eax") id, lateout("eax") out.0, out("edx") out.1, out("ecx") out.2
        )
    }
    out
}