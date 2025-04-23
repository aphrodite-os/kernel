//! Functions and types related to paging.
#![cfg(target_arch = "x86")]

use core::{arch::asm, mem::MaybeUninit};

use super::cpuid;

/// One page directory entry. Use [PageDirectoryEntry::create_fourmb] or
/// [PageDirectoryEntry::create_other] to make these.
pub enum PageDirectoryEntry {
    /// A four megabyte page.
    FourMb(u32),
    /// A smaller page.
    Other(u32),
}

impl PageDirectoryEntry {
    pub const fn create_fourmb(
        mut bits32to22: u16,
        bits39to32: u8,
        pat: bool,
        mut available: u8,
        global: bool,
        dirty: bool,
        accessed: bool,
        disable_cache: bool,
        write_through: bool,
        user: bool,
        can_write: bool,
        present: bool,
    ) -> Self {
        let mut out = 0u32;
        if present {
            out |= 1 << 0;
        }
        if can_write {
            out |= 1 << 1;
        }
        if user {
            out |= 1 << 2;
        }
        if write_through {
            out |= 1 << 3;
        }
        if disable_cache {
            out |= 1 << 4;
        }
        if accessed {
            out |= 1 << 5;
        }
        if dirty {
            out |= 1 << 6;
        }
        out |= 1 << 7;
        if global {
            out |= 1 << 8;
        }
        available &= 0b111;
        out |= (available as u32) << 9;
        if pat {
            out |= 1 << 12;
        }
        out |= (bits39to32 as u32) << 13;
        bits32to22 &= 0b1111111111;
        out |= (bits32to22 as u32) << 22;
        Self::FourMb(out)
    }

    pub const fn create_other(
        mut bits31to12: u32,
        pat: bool,
        mut available: u8,
        global: bool,
        accessed: bool,
        disable_cache: bool,
        write_through: bool,
        user: bool,
        can_write: bool,
        present: bool,
    ) -> Self {
        let mut out = 0u32;
        if present {
            out |= 1 << 0;
        }
        if can_write {
            out |= 1 << 1;
        }
        if user {
            out |= 1 << 2;
        }
        if write_through {
            out |= 1 << 3;
        }
        if disable_cache {
            out |= 1 << 4;
        }
        if accessed {
            out |= 1 << 5;
        }
        if available & 1 != 0 {
            out |= 1 << 6;
        }
        out |= 0 << 7;
        if global {
            out |= 1 << 8;
        }
        available &= 0b11110;
        out |= (available as u32) << 8;
        if pat {
            out |= 1 << 12;
        }
        bits31to12 &= 0b1111111111111111111;
        out |= bits31to12 << 13;
        Self::Other(out)
    }
}

static mut PAGE_DIRECTORY: MaybeUninit<PageDirectoryEntry> = MaybeUninit::uninit();

/// Initalize paging.
pub fn initalize_paging(pae: bool) {
    if cpuid(1).1 & 1<<6 != 0 && pae {
        unsafe {
            asm!(
                "mov eax, cr4",
                "or eax, 0b100000",
                "mov cr4, eax",
                out("eax") _
            )
        }
    }
    #[allow(static_mut_refs)]
    unsafe {
        asm!(
            "mov cr3, eax",
            "mov eax, cr0",
            "or eax, 0x80000001",
            "mov cr0, eax",
            in("eax") PAGE_DIRECTORY.as_ptr(), lateout("eax") _
        )
    }
}

/// Disables paging by clearing bit 31 in the cr0 register.
pub fn disable_paging() {
    unsafe {
        asm!(
            "mov eax, cr0",
            "and eax, 01111111111111111111111111111111b",
            "mov cr0, eax"
        )
    }
}
