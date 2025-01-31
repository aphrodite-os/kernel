//! Definitions of structs for multiboot2 information. Mostly used during pre-userspace.

/// Used for Multiboot2 tags. This shouldn't be used after a [crate::boot::BootInfo] struct has been initalized, but it still can be used.
#[repr(C)]
#[derive(Clone)]
pub struct Tag {
    /// The type of the tag.
    pub tag_type: u32,
    /// The length of the tag.
    pub tag_len: u32
}

/// The root tag. The official Multiboot2 name is literally the "fixed part" of the tags, so I made a better name.
#[repr(C)]
#[derive(Clone)]
pub struct RootTag {
    /// The total length between the root tag and the terminating tag.
    /// You can also search for the tag that has a type of 0 and a length of 8.
    pub total_len: u32,
    /// Reserved space. Unused for anything.
    reserved: u32,
}

/// A Multiboot2 module. See <https://aphrodite-os.github.io/book/bootloader-modules.html>.
#[derive(Clone)]
pub struct Module {
    /// A pointer to the start of the module
    pub mod_start: *const u8,
    /// A pointer to the end of the module
    pub mod_end: *const u8,
    /// A string that should be in the format `module_name (command line arguments)`.
    /// See <https://aphrodite-os.github.io/book/bootloader-modules.html>.
    pub mod_str: &'static core::ffi::CStr
}

/// One memory section provided by a Multiboot2 bootloader.
#[repr(C)]
#[derive(Clone)]
pub struct MemorySection {
    /// The base address of the section.
    pub base_addr: u64,
    /// The length of the section.
    pub length: u64,
    /// The type of the section. Name is changed from the one provided in the Multiboot2 docs
    /// as "type" is a keyword in rust.
    pub mem_type: u32,
    /// Reserved space. Should be ignored.
    reserved: u32,
}

impl crate::boot::MemoryMapping for MemorySection {
    fn get_type(&self) -> crate::boot::MemoryType {
        match self.mem_type {
            1 => crate::boot::MemoryType::Free,
            2 => crate::boot::MemoryType::HardwareReserved,
            3 => crate::boot::MemoryType::HardwareSpecific(3, false),
            5 => crate::boot::MemoryType::Faulty,
            _ => crate::boot::MemoryType::Reserved
        }
    }
    fn get_start(&self) -> u64 {
        self.base_addr
    }
    fn get_length(&self) -> u64 {
        self.length
    }
}

/// The raw memory map provided by a Multiboot2 bootloader. This is interpreted
/// into a [MemoryMap].
#[repr(C)]
pub struct RawMemoryMap {
    /// The type of the tag.
    pub tag_type: u32,
    /// The length of the tag.
    pub tag_len: u32,
    /// Size of one entry(one [MemorySection] for Aphrodite)
    pub entry_size: u32,
    /// The version of the memory map. Should be disregarded as it's 0.
    pub entry_version: u32, // currently is 0, future Multiboot2 versions may increment
    /// The sections. This is the reason that [Clone] can't be implemented for [RawMemoryMap].
    pub sections: [MemorySection]
}

/// A full memory map provided by a Multiboot2 bootloader.
#[derive(Clone, Copy)]
pub struct MemoryMap {
    /// The version of the memory map. Should be disregarded as it's 0.
    pub version: u32, // currently is 0, future Multiboot2 versions may increment
    /// Size of one entry(one [MemorySection] for Aphrodite)
    pub entry_size: u32,
    /// All sections.
    pub sections: &'static [MemorySection],

    /// Iterator's index.
    pub idx: usize,
}

impl crate::boot::MemoryMap for MemoryMap {}

impl crate::boot::_MemoryMap for MemoryMap {
    fn len(&self) -> usize {
        self.sections.len()
    }
}

impl core::ops::Index<usize> for MemoryMap {
    type Output = dyn crate::boot::MemoryMapping;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sections[index] as &'static dyn crate::boot::MemoryMapping
    }
}

impl core::iter::Iterator for MemoryMap {
    type Item = &'static dyn crate::boot::MemoryMapping;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if self.sections.len()<=self.idx-1 {
            return None;
        }
        Some(&self.sections[self.idx-1])
    }
}

/// A color descriptor for [ColorInfo::Palette].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PaletteColorDescriptor {
    /// The red value
    pub red: u8,
    /// The green value
    pub green: u8,
    /// The blue value
    pub blue: u8
}

/// Information about color, for use in [FramebufferInfo].
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ColorInfo {
    /// The palette for use on the framebuffer.
    Palette {
        /// The number of colors in the palette.
        num_colors: u32,
        /// The first color in the palette.
        palette: *const PaletteColorDescriptor
    },
    /// RGB information for use on the framebuffer.
    RGBColor {
        /// Red color information.
        red_field_position: u8,
        /// See above.
        red_mask_size: u8,

        /// Green color information.
        green_field_position: u8,
        /// See above.
        green_mask_size: u8,

        /// Blue color information.
        blue_field_position: u8,
        /// See above.
        blue_mask_size: u8,
    },
    /// Text information, no metadata
    EGAText
}

/// Information about the framebuffer.
#[repr(C)]
#[derive(Clone)]
pub struct FramebufferInfo {
    /// A pointer to the framebuffer.
    pub address: u64,
    /// The pitch of the framebuffer (i.e. the number of bytes in each row).
    pub pitch: u32,
    /// The width of the framebuffer.
    pub width: u32,
    /// The height of the framebuffer.
    pub height: u32,
    /// Bits per pixel.
    pub bpp: u8,
    /// The type of the framebuffer. 0=indexed, 1=RGB, 2=text.
    pub fb_type: u8,
    /// Reserved space. Ignore.
    reserved: u8,

    // Color info after this; we need separate structs for each colorinfo as
    // we have to understand the format the bootloader gives us.
}

/// Boot info collected from provided [Tag]s.
#[derive(Clone)]
pub struct Multiboot2BootInfo {
    /// See <https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Basic-memory-information>.
    /// Tl;dr: mem_lower indicates the amount of "lower memory"
    /// and mem_upper the amount of "upper memory".
    pub mem_lower: Option<u32>,
    /// See above
    pub mem_upper: Option<u32>,

    // Multiboot2 bootloaders may provide us with the BIOS device and partition, but we're not interested.
    // To ensure future developers don't get any ideas, I'm leaving it out here.
    // If you need it, good luck.

    /// We're provided with a C-style UTF-8(null-terminated UTF-8) string. This should contain the original pointer provided by
    /// the bootloader.
    /// See <https://aphrodite-os.github.io/book/command-line.html> for the format.
    pub cmdline: Option<&'static core::ffi::CStr>,

    // Due to the way modules work, it's not easily possible to make a struct that contains all the modules.
    // Therefore, they are loaded on the fly.

    // Multiboot2 bootloaders may provide us with ELF symbols, but I'm feeling lazy and right now it's mostly
    // unnecessary, so I don't care. Sorry if you are affected by this.
    
    /// The memory map provided by the bootloader.
    pub memory_map: Option<MemoryMap>,

    /// The name of the bootloader(for example, "GRUB 2.12"). C-style UTF-8(null-terminated UTF-8) string.
    /// This should contain the original pointer provided by the bootloader.
    pub bootloader_name: Option<&'static core::ffi::CStr>,

    // APM table is ignored as APM has been superseded by ACPI. If your system doesn't support ACPI, good luck.

    // VBE table is ignored for a similar reason to above: it's deprecated. Good luck if you need it.

    /// Provides information on the framebuffer.
    pub framebuffer_info: Option<FramebufferInfo>,
    /// Color info, stored separately from [FramebufferInfo] because rust
    pub color_info: Option<ColorInfo>,

    // Even though SMBIOS is documented for Multiboot2, we're not using it and will instead search for it ourselves.
    // This is because right now I cannot figure out what format it provides the SMBIOS table in.

    // EFI memory map and image handle pointers are not included for portability.

    // "Image load base physical address" is not included as at the moment the kernel is not relocatable.
}
