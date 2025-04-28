//! General x86 functions
#![cfg(target_arch = "x86")]

use core::arch::asm;

pub mod egatext;
mod gdt;
mod interrupt_impls;
pub mod interrupts;
pub mod output;
pub mod paging;
pub mod ports;

mod constants;

use constants::*;
use gdt::GDTEntry;
use interrupts::{disable_interrupts, enable_interrupts, pop_irq, restore_irq};
use output::*;
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

pub fn alloc_available_boot() {
    disable_interrupts();
    {
        // GDT
        sdebugsln("Setting up GDT");

        let entries = gdt::serialize_gdt_entries([
            gdt::GDT_NULL_ENTRY,
            GDTEntry {
                // kernel code segment, segment 0x08
                limit: 0xFFFFF,
                base: 0,
                access: 0x9A,
                flags: 0xC,
            },
            GDTEntry {
                // kernel data segment, segment 0x10
                limit: 0xFFFFF,
                base: 0,
                access: 0x92,
                flags: 0xC,
            },
            GDTEntry {
                // user code segment, segment 0x18
                limit: 0xFFFFF,
                base: 0,
                access: 0xFA,
                flags: 0xC,
            },
            GDTEntry {
                // user data segment, segment 0x20
                limit: 0xFFFFF,
                base: 0,
                access: 0xF2,
                flags: 0xC,
            },
            GDTEntry {
                // Video RAM segment, segment 0x28
                limit: 0xFFFFF,
                base: 0,
                access: 0x92,
                flags: 0xC,
            },
        ]);

        sdebugsln("GDT prepared");

        unsafe {
            gdt::activate_gdt(&entries);
        }

        sdebugsln("GDT successfully activated; resetting segment registers");

        unsafe {
            asm!(
                "call reloadSegments", // I hate rust's inline assembly
                out("ax") _
            );
        }
        sdebugsln("Segment registers reset");
    }
    {
        // IDT
        sdebugsln("Setting up IDT");

        let mut idt = self::interrupts::new_idt_zeroed();
        idt[0] = self::interrupts::IdtEntry::from_data(
            get_actual_address(interrupt_impls::int0 as usize),
            false,
            true,
        );
        idt[8] = self::interrupts::IdtEntry::from_data(
            get_actual_address(interrupt_impls::int8 as usize),
            false,
            true,
        );

        sdebugsln("Prepared IDT");
        unsafe {
            interrupts::load_idt(
                (&idt) as *const [self::interrupts::IdtEntry; 256] as *const u8,
                2047,
            );
        }
        sdebugsln("IDT activated; enabling interrupts");
        enable_interrupts();
        unsafe {
            asm!("xchg bx, bx");
        }
        sdebugsln("IDT successfully loaded");
    }
}

fn get_actual_address(addr: usize) -> usize {
    let out;
    unsafe {
        asm!(
            "mov ebx, get_addr_actual",
            "call get_addr_actual",
            in("ecx") addr, out("eax") _, out("ebx") _, lateout("ecx") out
        );
    }
    sdebugs("actual address: passed: ");
    sdebugbnp(&crate::usize_as_u8_slice(addr));
    sdebugsnp(" actual: ");
    sdebugbnpln(&crate::usize_as_u8_slice(out));
    out
}
