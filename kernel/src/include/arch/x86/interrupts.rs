//! Provides interrupt-related functions
#![cfg(any(target_arch = "x86"))]

use core::arch::asm;

/// Returns whether interrupts are enabled or not.
pub fn interrupts_enabled() -> bool {
    let flags: u32;
    unsafe {
        asm!(
            "pushf",
            "pop {0:e}", out(reg) flags
        )
    }
    (flags & (1 << 9)) == 0
}

/// Disables interrupts and returns the value of them.
pub fn pop_irq() -> u32 {
    let flags: u32;
    unsafe {
        asm!(
            "pushf",
            "cli",
            "pop {0:e}", out(reg) flags
        )
    }
    flags
}

/// Restores interrupts after a [pop_irq] call.
pub fn restore_irq(flags: u32) {
    unsafe {
        asm!(
            "push {0:e}", in(reg) flags
        );
        asm!(
            "popf"
        );
    }
}

/// The IDTR. Used internally in [load_idt].
#[repr(C)]
struct IDTR {
    base: *const u8,
    size: usize
}

/// Loads an interrupt descriptor table.
pub fn load_idt(base: *const u8, size: usize) {
    let idtr = IDTR {
        base,
        size
    };
    unsafe {
        asm!(
            "lidt {}", in(reg) &idtr
        )
    }
}