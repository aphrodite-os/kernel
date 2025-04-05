//! An example implementation of an architecture. DO NOT use this module!
//! Everything must be implemented via either kernel items, or for constants
//! making them public.
//!
//! This is commented out for obvious reasons, but make sure to have this at
//! the top of the all files in your arch(with "arch" replaced with the
//! actual architecture, of course):
//! #![cfg(any(target_arch = "arch"))]

// DO NOT have this in your actual code:
#![allow(dead_code)]

/// Returns the most specific architecture available.
pub const fn get_arch() -> super::Architecture { super::Architecture::ExampleDummy }

pub mod interrupts {
    //! Interrupt-related functions.

    use core::mem::MaybeUninit;

    /// Must be a u16 or castable to a u16.
    /// Value used in x86 shown here as an example.
    pub const USER_SYSCALL_VECTOR: u16 = 0xA0;

    /// Returns whether interrupts are enabled or not.
    fn interrupts_enabled() -> bool { false }

    /// Enables interrupts.
    fn enable_interrupts() {}

    /// Disables interrupts.
    fn disable_interrupts() {}

    /// Disables interrupts and a value that can be used to restore them
    /// with [restore_irq].
    fn pop_irq() -> u64 { 0 }

    /// Restores interrupts after a [pop_irq] call.
    fn restore_irq(_irq: u64) {}

    /// Activates an IDT.
    fn activate_idt(_idt: Idt) {}

    /// An IDT.
    #[derive(Clone, Copy)]
    pub struct Idt {
        vectors: [u16; 256],
        funcs: [MaybeUninit<fn()>; 256],
        len: usize,
    }

    /// An IDT builder. The only way to create
    /// an IDT.
    #[derive(Clone, Copy)]
    pub struct IdtBuilder {
        vectors: [u16; 256],
        funcs: [MaybeUninit<fn()>; 256],
        idx: usize,
    }

    impl IdtBuilder {
        /// Start creating a new IDT.
        pub fn new() -> Self {
            IdtBuilder {
                vectors: [0; 256],
                funcs: [MaybeUninit::uninit(); 256],
                idx: 0,
            }
        }
        /// Add a function to the IDT.
        pub fn add_fn(&mut self, vector: u16, func: fn()) -> &mut Self {
            self.vectors[self.idx] = vector;
            self.funcs[self.idx].write(func);
            self.idx += 1;
            self
        }
        /// Create the IDT from the IDT builder.
        pub fn finish(&self) -> Idt {
            Idt {
                vectors: self.vectors,
                funcs: self.funcs,
                len: self.idx,
            }
        }
    }

    impl Default for IdtBuilder {
        fn default() -> Self {
            IdtBuilder {
                vectors: [0; 256],
                funcs: [MaybeUninit::uninit(); 256],
                idx: 0,
            }
        }
    }
}

pub mod output {
    //! Not shown here(see [crate::arch::x86] for an example), but a
    //! LOT of output functions must be implemented. Using macros to
    //! implement these is HIGHLY recommended.
}
