//! GDT initalization.
#![cfg(target_arch = "x86")]

use core::alloc::Layout;
use core::arch::asm;

pub const GDT_KERNEL_CODE_SEGMENT: u16 = 0x08;
pub const GDT_KERNEL_DATA_SEGMENT: u16 = 0x10;
pub const GDT_USER_CODE_SEGMENT: u16 = 0x18;
pub const GDT_USER_DATA_SEGMENT: u16 = 0x20;
pub const GDT_OTHER_DATA_SEGMENT: u16 = 0x28;

/// The GDTR. Used internally in [activate_gdt].
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Gdtr {
    // raw pointer to the GDT
    base: u32,
    // size of the GDT in bytes
    size: u16,
}

unsafe impl Sync for Gdtr {}

/// Activates the GDT using `lgdt`. Does NOT, I repeat, does NOT change the
/// segment registers!
pub unsafe fn activate_gdt(ptr: *const [u8]) {
    unsafe {
        asm!(
            "mov [3f], ax", // load limit
            "mov [3f+2], ebx", // load base
            "xor ax, ax", // clear ax
            "lldt ax", // deactivate LDT
            "lgdt [3f]", // load GDT
            "jmp 2f", // jump past the data
            "3:", // GDT data
            "nop; nop; nop; nop; nop; nop",
            "2:", // end
            in("ax") ptr.len() as u16,
            in("ebx") ptr as *const u8 as usize as u32,
            options(readonly)
        )
        // super::output::sdebugs("base: ");
        // super::output::sdebugbnp(&crate::u32_as_u8_slice(GDTR.base));
        // super::output::sdebugsnp(" size: ");
        // super::output::sdebugbnpln(&crate::u16_as_u8_slice(GDTR.size));
    }
}

/// Writes a series of GDT entries to an allocated section of memory and returns
/// a pointer.
pub unsafe fn write_gdt_entries(
    entries: &[GDTEntry],
) -> Result<*const [u8], crate::Error<'static>> {
    let mut mem =
        unsafe { alloc::alloc::alloc(Layout::from_size_align(8 * entries.len(), 8).unwrap()) };
    for ele in entries {
        let serialized = ele.serialize()?;
        unsafe {
            core::ptr::write(mem as *mut [u8; 8], serialized);
        }
        mem = (mem as usize + 8) as *mut u8;
    }

    Ok(core::ptr::from_raw_parts(mem, 8 * entries.len()))
}

const fn concat_arrays<T, const M: usize, const N: usize>(a: [T; M], b: [T; N]) -> [T; M + N] {
    let mut result = core::mem::MaybeUninit::uninit();
    let dest = result.as_mut_ptr() as *mut T;
    unsafe {
        core::ptr::copy_nonoverlapping(a.as_ptr(), dest, M);
        core::ptr::copy_nonoverlapping(b.as_ptr(), dest.add(M), N);
        core::mem::forget(a);
        core::mem::forget(b);
        result.assume_init()
    }
}

pub const fn serialize_gdt_entries(entries: [GDTEntry; 6]) -> [u8; 6 * 8] {
    concat_arrays(
        concat_arrays(
            concat_arrays(
                concat_arrays(
                    concat_arrays(
                        entries[0].serialize_panicing(),
                        entries[1].serialize_panicing(),
                    ),
                    entries[2].serialize_panicing(),
                ),
                entries[3].serialize_panicing(),
            ),
            entries[4].serialize_panicing(),
        ),
        entries[5].serialize_panicing(),
    )
}

/// A GDT entry.
#[derive(Clone, Copy)]
pub struct GDTEntry {
    /// The size of the entry. Has to be less than 0xFFFFF.
    pub limit: u32,
    /// The base address of the entry.
    pub base: u32,
    /// The access byte of the entry.
    pub access: u8,
    /// The flags of the entry.
    pub flags: u8,
}

pub const GDT_NULL_ENTRY: GDTEntry = GDTEntry {
    limit: 0,
    base: 0,
    access: 0,
    flags: 0,
};

/// An error returned by [GDTEntry::write_to_addr] when the limit is greater
/// than 0xFFFFF.
pub const GDT_WRITE_ADDR_INVALID_LIMIT: i16 = -1;

impl GDTEntry {
    const fn serialize(self) -> Result<[u8; 8], crate::Error<'static>> {
        if self.limit > 0xFFFFF {
            return Err(crate::Error::new(
                "Invalid GDT entry limit(more than 0xFFFFF)",
                GDT_WRITE_ADDR_INVALID_LIMIT,
            ));
        }
        let mut out = [0u8; 8];

        out[0] = (self.limit & 0xFF) as u8;
        out[1] = ((self.limit >> 8) & 0xFF) as u8;
        out[6] = ((self.limit >> 16) & 0x0F) as u8;

        out[2] = (self.base & 0xFF) as u8;
        out[3] = ((self.base >> 8) & 0xFF) as u8;
        out[4] = ((self.base >> 16) & 0xFF) as u8;
        out[7] = ((self.base >> 24) & 0xFF) as u8;

        out[5] = self.access;

        out[6] |= self.flags << 4;

        Ok(out)
    }
    const fn serialize_panicing(self) -> [u8; 8] {
        if self.limit > 0xFFFFF {
            panic!("Invalid GDT entry limit(more than 0xFFFFF)");
        }
        let mut out = [0u8; 8];

        out[0] = (self.limit & 0xFF) as u8;
        out[1] = ((self.limit >> 8) & 0xFF) as u8;
        out[6] = ((self.limit >> 16) & 0x0F) as u8;

        out[2] = (self.base & 0xFF) as u8;
        out[3] = ((self.base >> 8) & 0xFF) as u8;
        out[4] = ((self.base >> 16) & 0xFF) as u8;
        out[7] = ((self.base >> 24) & 0xFF) as u8;

        out[5] = self.access;

        out[6] |= self.flags << 4;

        out
    }
}
