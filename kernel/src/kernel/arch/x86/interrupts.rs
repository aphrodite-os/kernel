//! Provides interrupt-related functions
#![cfg(any(target_arch = "x86"))]
#![allow(static_mut_refs)]

use core::{alloc::{Allocator, Layout}, arch::asm, mem::MaybeUninit};

/// The syscall vector.
pub const USER_SYSCALL_VECTOR: u16 = 0xA0;

/// Returns whether interrupts are enabled or not.
#[aphrodite_proc_macros::kernel_item(InterruptsCheck)]
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
#[aphrodite_proc_macros::kernel_item(InterruptsDisable)]
pub fn disable_interrupts() {
    unsafe {
        asm!("cli")
    }
}

/// Disables interrupts and returns the value of them.
#[aphrodite_proc_macros::kernel_item(InterruptsPop)]
pub fn pop_irq() -> u64 {
    let flags: u32;
    unsafe {
        asm!(
            "pushf",
            "cli",
            "pop {0:e}", out(reg) flags
        )
    }
    flags as u64
}

/// Restores interrupts after a [pop_irq] call.
#[aphrodite_proc_macros::kernel_item(InterruptsRestore)]
pub fn restore_irq(flags: u64) {
    let flags = flags as u32;
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
#[repr(packed)]
#[repr(C)]
struct IDTR {
    base: *const u8,
    size: usize
}

unsafe impl Send for IDTR {}
unsafe impl Sync for IDTR {}

/// Loads an interrupt descriptor table.
fn load_idt(base: *const u8, size: usize) {
    static mut IDTR: MaybeUninit<IDTR> = MaybeUninit::uninit();
    unsafe {
        IDTR.write(IDTR {
            base,
            size
        });
    }
    unsafe {
        asm!("lidt {}", in(reg) IDTR.as_ptr() as usize)
    }
}

/// Activate an IDT.
#[aphrodite_proc_macros::kernel_item(ActivateIDT)]
fn activate_idt(idt: Idt, alloc: crate::mem::MemoryMapAlloc) {
    let mem = alloc.allocate(unsafe { Layout::from_size_align_unchecked(8*idt.len, 1) }).unwrap().as_mut_ptr();
    for i in 0..idt.len {
        let vector = idt.vectors[i];
        let func = unsafe { idt.funcs[i].assume_init() } as usize as u32;
        let user_callable = idt.user_callable[i];
        let output: u64 = (func & 0b1111111111111111) as u64;

    }
}

#[derive(Clone, Copy)]
pub struct Idt {
    vectors: [u16; 256],
    funcs: [MaybeUninit<fn ()>; 256],
    user_callable: [bool; 256],
    len: usize,
}

#[derive(Clone, Copy)]
pub struct IdtBuilder {
    vectors: [u16; 256],
    funcs: [MaybeUninit<fn ()>; 256],
    user_callable: [bool; 256],
    idx: usize,
}

impl IdtBuilder {
    pub fn new() -> Self {
        IdtBuilder { 
            vectors: [0; 256],
            funcs: [MaybeUninit::uninit(); 256],
            user_callable: [false; 256],
            idx: 0,
        }
    }
    pub fn add_fn(&mut self, vector: u16, func: fn(), user_callable: bool) -> &mut Self {
        self.vectors[self.idx] = vector;
        self.funcs[self.idx].write(func);
        self.user_callable[self.idx] = user_callable;
        self.idx += 1;
        self
    }
    pub fn finish(&self) -> Idt {
        Idt {
            vectors: self.vectors,
            funcs: self.funcs,
            user_callable: self.user_callable,
            len: self.idx
        }
    }
}