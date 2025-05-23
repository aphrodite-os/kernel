//! Provides interrupt-related functions
#![cfg(target_arch = "x86")]
#![allow(static_mut_refs)]

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

/// Disables interrupts.
pub fn disable_interrupts() { unsafe { asm!("cli") } }

/// Enables interrupts.
pub fn enable_interrupts() { unsafe { asm!("sti") } }

/// PoppedInterrupts implements drop and restores the interrupts upon being
/// dropped. This is useful in functions where you need interrupts disabled
/// during it but also want to use functions like [Result::unwrap] or
/// [Option::unwrap].
#[derive(Clone)]
pub struct PoppedInterrupts(u32);

impl Drop for PoppedInterrupts {
    fn drop(&mut self) { restore_irq(self.clone()); }
}

/// Disables interrupts and returns the value of them.
pub fn pop_irq() -> PoppedInterrupts {
    let flags: u32;
    unsafe {
        asm!(
            "pushf",
            "cli",
            "pop {0:e}", out(reg) flags
        )
    }
    PoppedInterrupts(flags)
}

/// Restores interrupts after a [pop_irq] call.
pub fn restore_irq(flags: PoppedInterrupts) {
    let flags = flags.0;
    unsafe {
        asm!(
            "push {0:e}", in(reg) flags
        );
        asm!("popf");
    }
}

/// The IDTR. Used internally in [load_idt].
#[repr(C, packed)]
struct Idtr {
    base: *const u8,
    size: usize,
}

/// Loads an interrupt descriptor table.
#[inline(always)]
pub unsafe fn load_idt(base: *const u8, size: usize) {
    unsafe {
        asm!(
            "mov [3f], ax", // load limit
            "mov [3f+2], ebx", // load base
            "lidt [3f]", // load IDT
            "jmp 2f", // jump past the data
            "3:", // IDT data
            "nop; nop; nop; nop; nop; nop",
            "2:", // end
            in("ax") size as u16,
            in("ebx") base as usize as u32,
            options(readonly)
        )
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    pub offset_high: u16,
    pub segment: u16,
    filler: u8,
    pub attrs: u8,
    pub offset_low: u16,
}

impl IdtEntry {
    pub fn from_data(func: usize, user_callable: bool, exception: bool) -> Self {
        let func = func as u32;
        let mut entry = Self {
            offset_high: (func >> 16) as u16,
            segment: super::gdt::GDT_KERNEL_CODE_SEGMENT,
            attrs: 0b10000000,
            filler: 0,
            offset_low: (func & 0xFFFF) as u16,
        };
        if user_callable {
            entry.attrs |= 0b1100000;
        }
        if exception {
            entry.attrs |= 0b1111;
        } else {
            entry.attrs |= 0b1110;
        }
        entry
    }
}

/// An Interrupt Descriptor Table.
pub type Idt = [IdtEntry; 256];

pub fn new_idt_zeroed() -> Idt {
    [IdtEntry::from_data(super::interrupt_impls::intdefault as usize, false, false); 256]
}
