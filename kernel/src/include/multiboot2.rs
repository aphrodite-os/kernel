//! Definitions of structs for multiboot2 information. Mostly used during pre-userspace.

#![warn(missing_docs)]

/// Used when a CString is passed. Move into separate file?
#[derive(Clone)]
pub struct CString {
    /// The raw pointer to the string.
    pub ptr: *const u8,
    /// The length of the string, excluding the null byte(\0) at the end.
    pub len: usize,
}

impl core::ops::Index<usize> for CString {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            if index>self.len {
                panic!("index into CString too large");
            }
            let mut ptr = self.ptr as usize;
            ptr += index * size_of::<u8>();
            let ptr = ptr as *const u8;
            &*ptr
        }
    }
}

/// Used for Multiboot2 tags. This shouldn't be used after a [BootInfo] struct has been initalized, but it still can be used.
#[repr(C)]
#[repr(align(1))] // may or may not be necessary, but I'm not taking chances
#[derive(Clone)]
pub struct Tag {
    /// The type of the tag.
    pub tag_type: u32,
    /// The length of the tag.
    pub tag_len: u32
}

/// The root tag. The official Multiboot2 name is literally the "fixed part" of the tags, so I made a better name.
#[repr(C)]
#[repr(align(1))] // may or may not be necessary, but I'm not taking chances
#[derive(Clone)]
pub struct RootTag {
    /// The total length between the root tag and the terminating tag.
    /// You can also search for the tag that has a type of 0 and a length of 8.
    pub total_len: u32,
    /// Reserved space. Unused for anything.
    reserved: u32,
}

/// A Multiboot2 module. See https://github.com/AverseABFun/aphrodite/wiki/Plan/#Bootloader-modules (remember to update link later!).
#[derive(Clone)]
pub struct Module {
    /// A pointer to the start of the module
    pub mod_start: *const u8,
    /// A pointer to the end of the module
    pub mod_end: *const u8,
    /// A string that should be in the format `module_name (command line arguments)`.
    /// See https://github.com/AverseABFun/aphrodite/wiki/Plan/#Bootloader-modules (remember to update link later!).
    pub mod_str: CString
}

/// All modules provided by the bootloader. Very similar to [CString].
#[derive(Clone)]
pub struct Modules {
    /// A pointer to the first module. All modules should be consecutive.
    pub ptr: *const Module,
    /// The number of modules. If zero, [ptr](Modules::ptr) should not be trusted!
    pub modules_num: usize
}

impl core::ops::Index<usize> for Modules {
    type Output = Module;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            if index>self.modules_num {
                panic!("index into Modules too large");
            }
            let mut ptr = self.ptr as usize;
            ptr += index * size_of::<Module>();
            let ptr = ptr as *const Module;
            &*ptr
        }
    }
}

/// One memory section provided by a Multiboot2 bootloader.
#[repr(C)]
#[repr(align(1))] // may or may not be necessary, but I'm not taking chances
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

/// A full memory map provided by a Multiboot2 bootloader.
#[derive(Clone)]
pub struct MemoryMap {
    /// The version of the memory map. Should be disregarded as it's 0.
    pub version: u32, // currently is 0, future Multiboot2 versions may increment
    /// Size of one entry(one [MemorySection] for Aphrodite)
    pub entry_size: u32,
    /// A pointer to the first section.
    pub sections: *const MemorySection,
    /// The number of sections.
    pub sections_len: usize
}

/// A color descriptor for [ColorInfo::Palette].
#[repr(C)]
#[repr(align(1))] // may or may not be necessary, but I'm not taking chances
pub struct PaletteColorDescriptor {
    /// The red value
    pub red: u8,
    /// The green value
    pub green: u8,
    /// The blue value
    pub blue: u8
}

/// Information about color, for use in [FramebufferInfo].
#[repr(C)]
#[repr(align(1))] // may or may not be necessary, but I'm not taking chances
#[derive(Clone)]
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
    }
}

/// Information about the framebuffer.
#[repr(C)]
#[repr(align(1))] // may or may not be necessary, but I'm not taking chances
#[derive(Clone)]
pub struct FramebufferInfo {
    /// A pointer to the framebuffer.
    pub address: *mut u8,
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
    /// Color info. None if [fb_type](FramebufferInfo::fb_type) is 2.
    pub color_info: Option<ColorInfo>
}

/// Boot info collected from provided [Tag]s.
#[derive(Clone)]
pub struct BootInfo {
    /// See https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Basic-memory-information.
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
    /// See https://github.com/AverseABFun/aphrodite/wiki/Plan#bootloader (remember to update link later!) for the format.
    pub cmdline: Option<CString>,

    /// All modules provided by the bootloader.
    pub modules: Option<Modules>,

    // Multiboot2 bootloaders may provide us with ELF symbols, but I'm feeling lazy and right now the kernel is a 
    // flat binary, so I don't care. Sorry if you are affected by this.
    
    /// The memory map provided by the bootloader.
    pub memory_map: Option<MemoryMap>,

    /// The name of the bootloader(for example, "GRUB"). C-style UTF-8(null-terminated UTF-8) string.
    /// This should contain the original pointer provided by the bootloader.
    pub bootloader_name: Option<CString>,

    // APM table is ignored as APM has been superseded by ACPI. If your system doesn't support ACPI, good luck.

    // VBE table is ignored for a similar reason to above: it's deprecated. Good luck if you need it.

    /// Provides information on the framebuffer.
    pub framebuffer_info: Option<FramebufferInfo>,

    // Even though SMBIOS is documented for Multiboot2, we're not using it and will instead search for it ourselves.
    // This is because right now I cannot figure out what format it provides the SMBIOS table in.

    // EFI memory map and image handle pointers are not included for portability.

    // "Image load base physical address" is not included as at the moment the kernel is not relocatable.
}