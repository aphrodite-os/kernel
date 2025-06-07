//! Implementations of interrupts.
#![cfg(target_arch = "x86")]

use core::arch::asm;

pub struct InterruptStackFrame {
    ip: usize,
    cs: usize,
    flags: usize,
    sp: usize,
    ss: usize,
}

pub unsafe extern "x86-interrupt" fn int0(_stack_frame: InterruptStackFrame) {
    super::output::sdebugsln("Interrupt handler #0 ran");
}

pub unsafe extern "x86-interrupt" fn int8(
    _stack_frame: InterruptStackFrame,
    _error_code: usize,
) -> ! {
    super::output::sfatalsln("Double fault encountered; halting system!");
    unsafe {
        asm!("cli", "hlt", options(noreturn));
    }
}

pub unsafe extern "x86-interrupt" fn intdefault(_stack_frame: InterruptStackFrame) -> ! {
    super::output::sfatalsln("Unimplemented interrupt! Halting system.");
    unsafe {
        asm!("cli", "hlt", options(noreturn));
    }
}
