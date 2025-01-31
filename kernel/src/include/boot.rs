//! General bootloader-independent stuff.

/// A type of memory, for use in [MemoryMapping]s.
/// The memory allocator will ignore all memory
/// except for memory with type [MemoryType::Free]
/// or [MemoryType::HardwareSpecific] memory with
/// the boolean argument set.
#[derive(Clone, Copy)]
pub enum MemoryType {
    /// Free RAM with no use.
    Free,
    /// RAM used by the kernel
    Kernel,
    /// Reserved by something.
    Reserved,
    /// Reserved by something on the hardware.
    HardwareReserved,
    /// Faulty RAM modules.
    Faulty,
    /// Unknown use.
    Unknown,
    /// Hardware-specific use. The boolean argument states
    /// whether memory can be allocated in this region.
    HardwareSpecific(u32, bool),
    /// Flash/semi-permanent memory. Generally used in embedded systems.
    Permanent
}

/// A single memory mapping for [MemoryMap].
pub trait MemoryMapping {
    /// Returns the type of the memory.
    fn get_type(&self) -> MemoryType;
    /// Returns the beginning of the memory.
    fn get_start(&self) -> u64;
    /// Returns the length of the memory.
    fn get_length(&self) -> u64;
}

/// Memory mapping.
pub trait _MemoryMap: core::iter::Iterator<Item = &'static dyn MemoryMapping> + core::ops::Index<usize, Output = dyn MemoryMapping> {
    /// Returns the number of [MemoryMapping]s in the MemoryMap. This is total, not remainder.
    fn len(&self) -> usize;
}

/// Memory mapping. Used so that we can downcast.
pub trait MemoryMap: _MemoryMap + core::any::Any {}

/// Bootloader-independent information.
#[derive(Clone)]
pub struct BootInfo<'a> {
    /// The commandline of the kernel.
    /// See <https://aphrodite-os.github.io/book/command-line.html> for the format.
    pub cmdline: Option<&'static str>,
    
    /// The memory map provided by the bootloader. If None, the kernel will attempt to generate it.
    pub memory_map: Option<&'a dyn MemoryMap>,

    /// The name of the bootloader(for example, "GRUB 2.12").
    pub bootloader_name: Option<&'static str>,

    /// Provides a way to display text.
    pub output: Option<&'a dyn crate::display::TextDisplay>,
}