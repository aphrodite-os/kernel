//! General x86 functions
#![cfg(target_arch = "x86")]

use core::arch::asm;

pub mod egatext;
mod gdt;
pub mod interrupts;
pub mod memory;
pub mod output;
pub mod paging;
pub mod ports;

mod constants;

use alloc::vec;
use constants::*;
use gdt::GDTEntry;
use interrupts::{pop_irq, restore_irq};
use ports::{inb, outb};

/// Returns the most specific architecture available.
pub const fn get_arch() -> super::Architecture { super::Architecture::X86 }

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

/// Returns whether the a20 gate is enabled.
pub fn test_a20() -> bool {
    let addr0: usize;
    let addr1: usize;
    unsafe {
        asm!(
            "mov edi, 0x112345",
            "mov esi, 0x012345",
            "mov [esi], esi",
            "mov [edi], edi",
            "mov eax, esi",
            out("eax") addr0, out("edi") addr1
        )
    }
    addr0 != addr1
}

/// Waits for a keyboard command to complete.
pub fn wait_for_keyboard_cmd() { while inb(0x64) & 0b10 > 1 {} }

/// Waits for there to be data to read from the keyboard.
pub fn wait_for_keyboard_data() { while inb(0x64) & 0b1 == 0 {} }

/// Sends a keyboard command.
pub fn send_keyboard_cmd(byte: u8) { outb(0x64, byte); }

/// Gets data from the keyboard.
pub fn get_keyboard_data() -> u8 { inb(0x60) }

/// Sends data to the keyboard.
pub fn send_keyboard_data(data: u8) { outb(0x60, data); }

static mut RTC_INITALIZED: bool = false;

pub fn initalize_rtc() {
    if unsafe { RTC_INITALIZED } {
        return;
    }
    let irq = pop_irq();
    outb(0x70, 0x8A);
    outb(0x71, 0x20);
    restore_irq(irq);
    unsafe { RTC_INITALIZED = true }
}

pub fn sleep(seconds: u32) { initalize_rtc(); }

pub fn alloc_available_boot() {
    let irq = pop_irq();
    let mut entries = vec![];
    entries.push(gdt::GDT_NULL_ENTRY);
    entries.push(GDTEntry {
        limit: 0,
        base: 0,
        access: 0b10011011,
        flags: 0b1100,
    }); // kernel code segment
    entries.push(GDTEntry {
        limit: 0,
        base: 0,
        access: 0b10010011,
        flags: 0b1100,
    }); //

    unsafe {
        gdt::activate_gdt(gdt::write_gdt_entries(entries).unwrap());
    }
    restore_irq(irq);
}
