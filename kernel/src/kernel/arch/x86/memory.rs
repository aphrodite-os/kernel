//! Hardware-level memory sections. Unimplemented for certain hardware, x86
//! implements with GDT.
#![cfg(target_arch = "x86")]

use core::arch::asm;

use alloc::vec;
use alloc::vec::Vec;

use crate::memsections::*;

use super::gdt::{GDTEntry, write_gdt_entries};

/// A list of memory sections. Create one with [MemorySectionBuilder].
pub struct MemorySections {
    sections: Vec<MemorySection>,
}

#[repr(packed)]
struct GDTR {
    address: u32,
    size: u16,
}

unsafe impl crate::memsections::MemorySections for MemorySections {
    unsafe fn write(self) -> Result<(), crate::Error<'static>> {
        let mut entries: Vec<GDTEntry> = vec![];

        for section in self.sections {
            let mut section: MemorySection = section;
            // rust-analyzer doesn't want to cooperate and recognize that section is already
            // MemorySection, so I'm telling it here.
            fn make_entry(section: &mut MemorySection, entries: &mut Vec<GDTEntry>) {
                if section.length == 0 {
                    return;
                }
                let mut len = section.length as u32;
                while len > 0xFFFFF {
                    len -= 0xFFFFF;
                }
                let mut access = 0b10000001u8;
                match section.owner {
                    Owner::Kernelspace => {
                        access |= 0b0000000;
                    },
                    Owner::Modulespace => {
                        access |= 0b0100000;
                    },
                    Owner::Userspace => {
                        access |= 0b1100000;
                    },
                }
                if let SectionType::TaskSection { busy } = section.section_type {
                    access |= 0b00000;
                    if busy {
                        access |= 0x9;
                    } else {
                        access |= 0xB;
                    }
                } else {
                    access |= 0b10000;
                    if let SectionType::CodeSection {
                        can_powerful_sections_jump,
                    } = section.section_type
                    {
                        access |= 0b1000;
                        if can_powerful_sections_jump {
                            access |= 0b100;
                        }
                        if section.readable {
                            access |= 0b10;
                        }
                    } else if section.section_type == SectionType::DataSection {
                        access |= 0b0000;
                        if section.writable {
                            access |= 0b10;
                        }
                    }
                }

                let flags = 0b1100u8;

                let entry = GDTEntry {
                    limit: len,
                    base: section.address as u32,
                    access,
                    flags,
                };
                if section.length > 0xFFFFF {
                    section.length -= 0xFFFFF;
                }
                entries.push(entry);
            }
            while section.length > 0xFFFFF {
                make_entry(&mut section, &mut entries);
            }
            make_entry(&mut section, &mut entries);
        }
        unsafe {
            let _ = super::interrupts::pop_irq();

            let segment_entries: Vec<GDTEntry> = entries.clone();

            let ptr = write_gdt_entries(entries)?;

            let gdtr = GDTR {
                address: ptr as *const u8 as usize as u32,
                size: (ptr.len() - 1) as u16,
            };

            let addr = &gdtr as *const GDTR as *const () as usize as u32;

            asm!(
                "lgdt eax",
                in("eax") addr
            );

            let mut code_segment = 0u16;
            let mut code_set = false;
            let mut data_segment = 0u16;
            let mut data_set = false;

            let mut i = 0;
            for entry in segment_entries {
                let entry: GDTEntry = entry;
                i += 1;
                if code_set && data_set {
                    break;
                }

                if entry.access & 0b11000 == 0b11000 && !code_set {
                    code_segment = i - 1;
                    code_set = true;
                } else if entry.access & 0b10000 == 0b10000 && !data_set {
                    data_segment = i - 1;
                    data_set = true;
                }
            }

            asm!(
                "jmp   bx:2 ; `#[deny(named_asm_labels)]` on by default; see <https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html#labels>
                 2:           ; ax is already loaded with the correct value from rustland
                 mov   ds, ax
                 mov   es, ax
                 mov   fs, ax
                 mov   gs, ax
                 mov   ss, ax",
                 in("bx") code_segment,
                 in("ax") data_segment,
                 options(preserves_flags, nomem, nostack)
            );
        }

        Ok(())
    }
}

/// A memory section builder.
pub struct MemorySectionBuilder {
    sections: Vec<MemorySection>,
}

impl MemorySectionBuilder {
    /// Create a new MemorySectionBuilder.
    pub fn new() -> Self { MemorySectionBuilder { sections: vec![] } }

    /// Adds a section to this MemorySectionBuilder.
    pub fn add_section(&mut self, section: MemorySection) -> &mut Self {
        self.sections.push(section);

        self
    }

    /// Finishes this MemorySectionBuilder and returns a MemorySections.
    pub fn finish(self) -> MemorySections {
        MemorySections {
            sections: self.sections,
        }
    }
}
