//! Provides interrupt-related functions
#![cfg(target_arch = "x86")]
#![allow(static_mut_refs)]

use core::arch::asm;
use core::mem::MaybeUninit;

use crate::arch::x86::*;

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
unsafe fn load_idt(base: *const u8, size: usize) {
    static mut IDTR: Idtr = Idtr {
        base: 0 as *const u8,
        size: 0,
    };
    unsafe {
        IDTR = Idtr { base, size };
    }
    unsafe { asm!("lidt {}", sym IDTR) }
}

#[derive(Clone, Copy)]
pub struct IdtEntry {
    pub offset_high: u16,
    pub data: u16,
    pub segment: u16,
    pub offset_low: u16,
    pub vector: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct RawIdtEntry {
    pub offset_high: u16,
    pub data: u16,
    pub segment: u16,
    pub offset_low: u16,
}

impl From<IdtEntry> for RawIdtEntry {
    fn from(value: IdtEntry) -> Self {
        RawIdtEntry {
            offset_high: value.offset_high,
            data: value.data,
            segment: value.segment,
            offset_low: value.offset_low,
        }
    }
}

/// Activate an IDT. Requires that all handlers can properly handle the calling
/// convention and are in GDT segment gdt::GDT_KERNEL_CODE_SEGMENT.
pub unsafe fn activate_idt(idt: Idt) {
    let mut entries = [IdtEntry {
        offset_high: 0,
        data: 0,
        segment: 0,
        offset_low: 0,
        vector: 0,
    }; 256];
    let mut f = 0;
    for i in 0..idt.len {
        if idt.using_raw[i] {
            entries[f] = idt.raw_entries[i];
            f += 1;
            continue;
        }
        let vector = idt.vectors[i];
        let func = unsafe { idt.funcs[i].assume_init() } as usize as u32;
        let user_callable = idt.user_callable[i];
        let exception = idt.exception[i];

        let mut entry = IdtEntry {
            offset_high: (func >> 16) as u16,
            data: 0b1000000000000000,
            segment: super::gdt::GDT_KERNEL_CODE_SEGMENT,
            offset_low: (func & 0xFFFF) as u16,
            vector,
        };
        if user_callable {
            entry.data |= 0b110000000000000;
        }
        if exception {
            entry.data |= 0b111100000000;
        } else {
            entry.data |= 0b111000000000;
        }
        entries[f] = entry;
        sdebugs("Entry vector=");
        sdebugbnp(&crate::u16_as_u8_slice(entries[f].vector));
        sdebugsnp(" f=");
        sdebugbnpln(&crate::usize_as_u8_slice(f));
        f += 1;
    }

    (&mut entries[..idt.len]).sort_by_key(|ele: &IdtEntry| ele.vector);
    // as entries is a 256 element array, if we don't take a slice of it it will shove all of the
    // non-existant entries at the start(right after valid entry 0, if it exists)

    sdebugsln("IDT entries prepared and sorted");
    let mut last_vector = 0u16;
    let mut start = true;

    let mut entries2 = [IdtEntry {
        offset_high: 0,
        data: 0,
        segment: 0,
        offset_low: 0,
        vector: 0,
    }; 256];

    f = 0;

    for entry in &entries[..idt.len] {
        sdebugs("Entry vector=");
        sdebugbnp(&crate::u16_as_u8_slice(entry.vector));
        sdebugsnp(" f=");
        sdebugbnp(&crate::usize_as_u8_slice(f));
        if entry.data>>15 & 1 == 1 {
            sdebugsnpln(" valid");
        } else {
            sdebugsnpln(" not valid");
        }
        if start {
            let mut vector = entry.vector;
            while vector > 1 {
                entries2[f] = IdtEntry {
                    offset_high: 0,
                    data: 0,
                    segment: 0,
                    offset_low: 0,
                    vector: 0,
                };
                vector -= 1;
                f += 1;
            }
            last_vector = entry.vector;
            entries2[f] = *entry;
            f += 1;
            start = false;
            continue;
        }
        if entry.vector - last_vector > 0 {
            let mut vector = entry.vector - last_vector;
            while vector > 1 {
                entries2[f] = IdtEntry {
                    offset_high: 0,
                    data: 0,
                    segment: 0,
                    offset_low: 0,
                    vector: 0,
                };
                vector -= 1;
                f += 1;
            }
        }
        last_vector = entry.vector;
        entries2[f] = *entry;
        f += 1;
    }

    sdebugs("Number of entries counting filler: ");
    sdebugbnpln(&crate::usize_as_u8_slice(f));

    let mut raw_entries = [RawIdtEntry {
        offset_high: 0,
        data: 0,
        segment: 0,
        offset_low: 0,
    }; 256];
    f = 0;
    for entry in &entries2 {
        sdebugs("Entry vector=");
        sdebugbnp(&crate::u16_as_u8_slice(entry.vector));
        sdebugsnp(" f=");
        sdebugbnp(&crate::usize_as_u8_slice(f));
        if entry.data>>15 & 1 == 1 {
            sdebugsnpln(" valid");
        } else {
            sdebugsnpln(" not valid");
        }
        raw_entries[f] = RawIdtEntry::from(*entry);
        f += 1;
    }

    unsafe {
        load_idt(
            (&raw_entries) as *const [RawIdtEntry; 256] as *const u8,
            (idt.len * 8) - 1,
        );
    }
}

/// An Interrupt Descriptor Table.
#[derive(Clone, Copy)]
pub struct Idt {
    vectors: [u16; 256],
    funcs: [MaybeUninit<unsafe fn()>; 256],
    user_callable: [bool; 256],
    exception: [bool; 256],
    raw_entries: [IdtEntry; 256],
    using_raw: [bool; 256],
    len: usize,
}

/// A builder of an [Idt].
#[derive(Clone, Copy)]
pub struct IdtBuilder {
    vectors: [u16; 256],
    funcs: [MaybeUninit<unsafe fn()>; 256],
    user_callable: [bool; 256],
    exception: [bool; 256],
    raw_entries: [IdtEntry; 256],
    using_raw: [bool; 256],
    idx: usize,
}

impl IdtBuilder {
    /// Create a new IdtBuilder.
    pub fn new() -> Self {
        IdtBuilder {
            vectors: [0; 256],
            funcs: [MaybeUninit::uninit(); 256],
            user_callable: [false; 256],
            exception: [false; 256],
            raw_entries: [IdtEntry {
                offset_high: 0,
                data: 0,
                segment: 0,
                offset_low: 0,
                vector: 0,
            }; 256],
            using_raw: [false; 256],
            idx: 0,
        }
    }
    /// Add a function to this IdtBuilder.
    pub fn add_fn(
        &mut self,
        vector: u16,
        func: unsafe fn(),
        user_callable: bool,
        exception: bool,
    ) -> &mut Self {
        self.vectors[self.idx] = vector;
        self.funcs[self.idx].write(func);
        self.user_callable[self.idx] = user_callable;
        self.exception[self.idx] = exception;
        self.using_raw[self.idx] = false;
        self.idx += 1;
        self
    }
    pub fn add_raw(&mut self, vector: u16, raw_entry: IdtEntry) -> &mut Self {
        self.vectors[self.idx] = vector;
        self.raw_entries[self.idx] = raw_entry;
        self.using_raw[self.idx] = true;
        self.idx += 1;
        self
    }
    /// Finish creating this IdtBuilder and return an [Idt].
    pub fn finish(&self) -> Idt {
        Idt {
            vectors: self.vectors,
            funcs: self.funcs,
            user_callable: self.user_callable,
            raw_entries: self.raw_entries,
            using_raw: self.using_raw,
            exception: self.exception,
            len: self.idx,
        }
    }
}

impl Default for IdtBuilder {
    fn default() -> Self { Self::new() }
}
