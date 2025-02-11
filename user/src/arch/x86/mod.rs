//! x86 syscall method.

/// Syscall method.
#[macro_export]
macro_rules! syscall {
    ($id: expr) => {
        unsafe {
            let out = 0u32;
            asm!(
                "int 0xA0",
                id = in("eax") const $id,
                out("eax") out
            )
            out
        }
    };
    ($id: expr, $d0: expr) => {
        unsafe {
            let out = 0u32;
            let d0 = $d0;
            asm!(
                "int 0xA0",
                id = in("eax") const $id,
                d0 = in("ebx") $d0,
                out("eax") out
            )
            out
        }
    };
    ($id: expr, $d0: expr, $d1: expr) => {
        unsafe {
            let out = 0u32;
            let d0 = $d0;
            let d1 = $d1;
            asm!(
                "int 0xA0",
                id = in("eax") const $id,
                d0 = in("ebx") $d0,
                d1 = in("ecx") $d1,
                out("eax") out
            )
            out
        }
    };
    ($id: expr, $d0: expr, $d1: expr, $d2: expr) => {
        unsafe {
            let out = 0u32;
            let d0 = $d0;
            let d1 = $d1;
            let d2 = $d2;
            asm!(
                "int 0xA0",
                id = in("eax") const $id,
                d0 = in("ebx") $d0,
                d1 = in("ecx") $d1,
                d2 = in("edx") $d2,
                out("eax") out
            )
            out
        }
    }
}