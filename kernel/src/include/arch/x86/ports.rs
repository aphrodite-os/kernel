//! Provides utilities for interacting with assembly ports
#![cfg(any(target_arch = "x86"))]

use core::arch::asm;

/// Outputs a byte to an IO port
#[inline(always)]
pub fn outb(port: u16, val: u8) {
    unsafe {
        asm!(
            "out dx, al", in("dx") port, in("al") val
        )
    }
}

/// Outputs an arbitrary number of bytes to an IO port
pub fn outbs(port: u16, val: &[u8]) {
    for ele in val {
        outb(port, *ele);
    }
}

/// Reads a byte from an IO port
#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let out;
    unsafe {
        asm!(
            "in {}, {1:x}", out(reg_byte) out, in(reg) port
        )
    }
    out
}

/// Wait a short, indeterminable time
#[inline(always)]
pub fn io_wait() {
    outb(0x80, 0);
}