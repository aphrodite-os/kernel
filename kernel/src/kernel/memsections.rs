//! Architecture-independt memory section stuff.
//! 
//! arch::*::memory is the architecture-dependent counterpart.

/// Section types for [MemorySection].
/// If the architecture doesn't support, for example, writing
/// to a code section, then this should take precedence over
/// [MemorySection] attributes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    /// A code section. Generally at least one of these for the kernel.
    CodeSection {
        /// Whether more powerful owners can jump to this if the owner
        /// is less powerful.
        can_powerful_sections_jump: bool
    },
    /// A data section. Generally at least one of these for the kernel.
    DataSection,
    /// A [task state section](https://wiki.osdev.org/Task_State_Segment).
    /// These can be interpreted as [DataSection](SectionType::DataSection)s
    /// if they aren't directly supported by the hardware.
    TaskSection {
        /// Whether the section is busy.
        busy: bool
    },
}

/// The owner of a [MemorySection].
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /// Userspace.
    Userspace,
    /// Kernelspace.
    Kernelspace,
    /// Modulespace.
    Modulespace,
}

/// A memory section.
#[derive(Clone, Copy)]
pub struct MemorySection {
    /// The type of section.
    pub section_type: SectionType,
    /// The owner of the section.
    pub owner: Owner,
    /// Whether the kernel should minimize reads to the section.
    pub minimal_read: bool,
    /// Whether the segment is readable.
    pub readable: bool,
    /// Whether the segment is writable.
    pub writable: bool,
    /// The base address.
    pub address: u64,
    /// The length. If the implementation has a maximum length of
    /// sections, it should be automatically split into however many sections are necessary.
    pub length: u64,
}

/// Implemented by arch::*::memory::MemorySections. Note to implementers:
/// Copy should NOT be implemented. That would lead to issues where a
/// struct implementing this trait could be used after [write](MemorySections::write)
/// is called, which is not supposed to happen.
pub unsafe trait MemorySections {
    /// Write the sections to an allocated region and then activate them.
    /// 
    /// This intentionally takes ownership of the MemorySections as it
    /// shouldn't be used after this is called.
    unsafe fn write(self) -> Result<(), crate::Error<'static>>;
}