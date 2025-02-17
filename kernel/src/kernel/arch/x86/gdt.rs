//! GDT initalization.

use core::alloc::Layout;

use alloc::vec::Vec;

/// Writes a series of GDT entries to an allocated section of memory and returns a pointer.
pub unsafe fn write_gdt_entries(entries: Vec<GDTEntry>) -> Result<*const [u8], crate::Error<'static>> {
    let mut mem = unsafe { alloc::alloc::alloc(Layout::from_size_align(8*entries.len(), 1).unwrap()) };
    for ele in &entries {
        let ele: &GDTEntry = ele;
        unsafe { ele.write_to_addr(mem as *mut ())? }
        mem = (mem as usize + 8) as *mut u8;
    }

    Ok(core::ptr::from_raw_parts(mem, 8*entries.len()))
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

/// An error returned by [GDTEntry::write_to_addr] when the limit is greater than 0xFFFFF.
const GDT_WRITE_ADDR_INVALID_LIMIT: i16 = -1;

impl GDTEntry {
    const unsafe fn write_to_addr(self, ptr: *mut ()) -> Result<(), crate::Error<'static>> {
        if self.limit > 0xFFFFF {
            return Err(crate::Error::new("Invalid GDT entry limit(more than 0xFFFFF)", GDT_WRITE_ADDR_INVALID_LIMIT));
        }
        let mut serialized = (0u64).to_ne_bytes();

        serialized[0] = (self.limit & 0xFF) as u8;
        serialized[1] = ((self.limit >> 8) & 0xFF) as u8;
        serialized[6] = ((self.limit >> 16) & 0x0F) as u8;

        serialized[2] = (self.base & 0xFF) as u8;
        serialized[3] = ((self.base >> 8) & 0xFF) as u8;
        serialized[4] = ((self.base >> 16) & 0xFF) as u8;
        serialized[7] = ((self.base >> 24) & 0xFF) as u8;

        serialized[5] = self.access;

        serialized[6] |= self.flags << 4;

        unsafe {
            core::ptr::write(ptr as *mut [u8; 8], serialized);
        }
        
        Ok(())
    }
}