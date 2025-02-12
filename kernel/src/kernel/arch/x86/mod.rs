//! General x86 functions
#![cfg(any(target_arch = "x86"))]

use core::arch::asm;

pub mod interrupts;
pub mod ports;
pub mod output;
pub mod egatext;
pub mod paging;

mod constants;

pub(self) use constants::*;
use interrupts::{pop_irq, restore_irq};
use ports::{inb, outb};

#[aphrodite_proc_macros::kernel_item(PagingAvailabe)]
pub fn paging_available() -> bool {
    true
}

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
pub fn wait_for_keyboard_cmd() {
    while inb(0x64)&0b10 > 1 {}
}

/// Waits for there to be data to read from the keyboard.
pub fn wait_for_keyboard_data() {
    while inb(0x64)&0b1 == 0 {}
}

/// Sends a keyboard command.
pub fn send_keyboard_cmd(byte: u8) {
    outb(0x64, byte);
}

/// Gets data from the keyboard.
pub fn get_keyboard_data() -> u8 {
    inb(0x60)
}

/// Sends data to the keyboard.
pub fn send_keyboard_data(data: u8) {
    outb(0x60, data);
}

/// Tries to enable the a20 gate via the keyboard controller method.
pub fn enable_a20_keyboard() {
    let irq = pop_irq();

    wait_for_keyboard_cmd();
    send_keyboard_cmd(0xAD); // disable keyboard

    wait_for_keyboard_cmd();
    send_keyboard_cmd(0xD0); // read from input

    wait_for_keyboard_cmd();
    wait_for_keyboard_data();
    let a = get_keyboard_data();

    wait_for_keyboard_cmd();
    send_keyboard_cmd(0xD1); // write to output

    wait_for_keyboard_cmd();
    send_keyboard_data(a|2);

    wait_for_keyboard_cmd();
    send_keyboard_cmd(0xAE); // enable keyboard

    restore_irq(irq);
}

/// Tries to enable the a20 gate via fast a20.
/// Note that this may not work or do something unexpected.
pub fn enable_a20_fasta20() {
    let mut a = inb(0x92);
    if a&0b10 > 0 {
        return
    }
    a |= 0b10;
    a &= 0xFE;
    outb(0x92, a);
}

/// Tries to enable the a20 gate by reading from port 0xee.
pub fn enable_a20_ee_port() {
    inb(0xee);
}

/// Tries to enable the a20 gate by trying many different methods
/// and seeing what sticks.
pub fn enable_a20() -> bool {
    if test_a20() {
        return true;
    }

    enable_a20_keyboard();
    let mut i = 0u32;
    while (!test_a20()) && i<10000 {
        i += 1;
    }

    if test_a20() {
        return true;
    }

    enable_a20_ee_port();
    let mut i = 0u32;
    while (!test_a20()) && i<10000 {
        i += 1;
    }

    if test_a20() {
        return true;
    }

    enable_a20_fasta20();
    let mut i = 0u32;
    while (!test_a20()) && i<10000 {
        i += 1;
    }

    return test_a20();
}
