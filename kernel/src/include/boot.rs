//! General bootloader-independent stuff.

/// A type of memory, for use in [MemoryMapping]s.
/// The memory allocator will ignore all memory
/// except for memory with type [MemoryType::Free]
/// or [MemoryType::HardwareSpecific] memory with
/// the boolean argument set.
#[derive(Clone, Copy, PartialEq, Eq)]
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
    Permanent,
}

/// A single memory mapping for [MemoryMap].
#[derive(Clone, Copy)]
pub struct MemoryMapping {
    /// Returns the type of the memory.
    pub mem_type: MemoryType,
    /// Returns the beginning of the memory.
    pub start: u64,
    /// Returns the length of the memory.
    pub len: u64,
}

#[derive(Clone, Copy)]
pub struct MemoryMap {
    pub len: u64,
    pub size_pages: u64,
    pub page_size: u64,

    /// All sections.
    pub sections: &'static [MemoryMapping],

    /// Iterator's index.
    pub idx: usize,
}

impl MemoryMap {
    pub fn len(&self) -> u64 {
        self.sections.len() as u64
    }
    pub fn reset_iter(&mut self) {
        self.idx = 0;
    }
    pub fn mem_size(&mut self) -> u64 {
        let curr_idx = self.idx;
        self.reset_iter();
        let mut out = 0u64;
        for ele in self.sections {
            if ele.mem_type == crate::boot::MemoryType::Free {
                out += ele.len;
            } else if let crate::boot::MemoryType::HardwareSpecific(_, free) = ele.mem_type {
                if free {
                    out += ele.len;
                }
            }
        }
        self.idx = curr_idx;
        out
    }
}

impl core::ops::Index<usize> for MemoryMap {
    type Output = MemoryMapping;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sections[index]
    }
}

impl core::iter::Iterator for MemoryMap {
    type Item = MemoryMapping;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if self.sections.len()<=self.idx-1 {
            return None;
        }
        Some(self.sections[self.idx-1].into())
    }
}

/// Bootloader-independent information.
#[derive(Clone)]
pub struct BootInfo<'a> {
    /// The commandline of the kernel.
    /// See <https://aphrodite-os.github.io/book/command-line.html> for the format.
    pub cmdline: Option<&'static str>,

    /// The memory map provided by the bootloader. If None, the kernel will attempt to generate it.
    pub memory_map: Option<MemoryMap>,

    /// The name of the bootloader(for example, "GRUB 2.12").
    pub bootloader_name: Option<&'static str>,

    /// Provides a way to display text.
    pub output: Option<&'a dyn crate::display::TextDisplay>,
}
