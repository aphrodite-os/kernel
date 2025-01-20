//! The entrypoint to the kernel; placed before all other code.
#![no_std]
#![no_main]
#![warn(missing_docs)]
#![feature(ptr_metadata)]

use core::{arch::asm, ffi::CStr, panic::PanicInfo};
use aphrodite::multiboot2::{BootInfo, CString, ColorInfo, FramebufferInfo, MemoryMap, PaletteColorDescriptor, RawMemoryMap, RootTag, Tag};

// The root tag, provided directly from the multiboot bootloader.
static mut RT: *const RootTag = core::ptr::null();
// The boot info struct, created from all of the tags.
static mut BI: BootInfo = BootInfo {
    mem_lower: None,
    mem_upper: None,
    cmdline: None,
    memory_map: None,
    bootloader_name: None,
    framebuffer_info: None,
    color_info: None,
};

// The raw pointer to bootloader-specific data.
static mut O: *const u8 = core::ptr::null();

// The magic number in eax. 0x36D76289 for multiboot2.
static mut MAGIC: u32 = 0xFFFFFFFF;

#[unsafe(link_section = ".start")]
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    unsafe { // Copy values provided by the bootloader out
        asm!(
            "mov {0:e}, eax", out(reg) MAGIC // Magic number
        );
        asm!(
            "mov {0:e}, ebx", out(reg) O // Bootloader-specific data
        );
    }
    unsafe {
        match MAGIC {
            0x36D76289 => { // Multiboot2
                RT = O as *const RootTag; // This is unsafe rust! We can do whatever we want! *manical laughter*

                let mut ptr = O as usize;
                ptr += size_of::<RootTag>();

                let mut current_tag = ptr as *const Tag;
                
                loop {
                    match (*current_tag).tag_type {
                        0 => { // Ending tag
                            if (*current_tag).tag_len != 8 { // Unexpected size, something is probably up
                                panic!("size of ending tag != 8");
                            }
                            break
                        },
                        4 => { // Basic memory information
                            if (*current_tag).tag_len != 16 { // Unexpected size, something is probably up
                                panic!("size of basic memory information tag != 16");
                            }

                            BI.mem_lower = Some(*((current_tag as usize + 8) as *const u32));
                            BI.mem_upper = Some(*((current_tag as usize + 12) as *const u32));
                            // The end result of the above is adding an offset to a pointer and retrieving the value at that pointer
                        },
                        5 => { // BIOS boot device, ignore
                            if (*current_tag).tag_len != 20 { // Unexpected size, something is probably up
                                panic!("size of bios boot device tag != 20");
                            }
                        },
                        1 => { // Command line
                            if (*current_tag).tag_len < 8 { // Unexpected size, something is probably up
                                panic!("size of command line tag < 8");
                            }
                            let cstring = CStr::from_ptr((current_tag as usize + 8) as *const i8);
                            // creates a &core::ffi::CStr from the start of the command line...

                            let cstring = CString {
                                ptr: cstring.as_ptr() as *const u8,
                                len: cstring.to_bytes().len()
                            };
                            // ...which can then be converted to a aphrodite::multiboot2::CString...

                            BI.cmdline = Some(cstring);
                            // ...before the BootInfo's commandline is set.
                        },
                        6 => { // Memory map tag
                            if (*current_tag).tag_len < 16 { // Unexpected size, something is probably up
                                panic!("size of memory map tag < 16");
                            }
                            let rawmemorymap: *const RawMemoryMap = core::ptr::from_raw_parts(
                                current_tag, ((*current_tag).tag_len / *((current_tag as usize + 8usize) as *const u32)) as usize
                            );
                            // The end result of the above is creating a *const RawMemoryMap that has the same address as current_tag
                            // and has all of the [aphrodite::multiboot2::MemorySection]s for the memory map

                            BI.memory_map = Some(MemoryMap {
                                version: (*rawmemorymap).entry_version,
                                entry_size: (*rawmemorymap).entry_size,
                                sections: &(*rawmemorymap).sections[0],
                                sections_len: (*rawmemorymap).sections.len()
                            });
                        },
                        2 => { // Bootloader name
                            if (*current_tag).tag_len < 8 { // Unexpected size, something is probably up
                                panic!("size of command line tag < 8");
                            }
                            let cstring = CStr::from_ptr((current_tag as usize + 8) as *const i8);
                            // creates a &core::ffi::CStr from the start of the bootloader name...

                            let cstring = CString {
                                ptr: cstring.as_ptr() as *const u8,
                                len: cstring.to_bytes().len()
                            };
                            // ...which can then be converted to a aphrodite::multiboot2::CString...

                            BI.bootloader_name = Some(cstring);
                            // ...before the BootInfo's bootloader_name is set.
                        },
                        8 => { // Framebuffer info
                            if (*current_tag).tag_len < 40 { // Unexpected size, something is probably up
                                panic!("size of framebuffer info tag < 40");
                            }
                            let framebufferinfo: *const FramebufferInfo = current_tag as *const FramebufferInfo;
                            let colorinfo: ColorInfo;
                            match (*framebufferinfo).fb_type {
                                0 => { // Indexed
                                    colorinfo = ColorInfo::Palette {
                                        num_colors: *((current_tag as usize + 40) as *const u32),
                                        palette: (current_tag as usize + 44) as *const PaletteColorDescriptor
                                    };
                                },
                                1 => { // RGB
                                    colorinfo = ColorInfo::RGBColor {
                                        red_field_position: *((current_tag as usize + 40) as *const u8),
                                        red_mask_size: *((current_tag as usize + 41) as *const u8),
                                        green_field_position: *((current_tag as usize + 42) as *const u8),
                                        green_mask_size: *((current_tag as usize + 43) as *const u8),
                                        blue_field_position: *((current_tag as usize + 44) as *const u8),
                                        blue_mask_size: *((current_tag as usize + 45) as *const u8)
                                    }
                                },
                                2 => { // EGA Text
                                    colorinfo = ColorInfo::EGAText;
                                },
                                _ => {
                                    unreachable!();
                                }
                            }
                            BI.framebuffer_info = Some((*framebufferinfo).clone());
                            BI.color_info = Some(colorinfo);
                        },
                        _ => { // Unknown/unimplemented tag type, ignore
                            // TODO: Add info message
                        }
                    }
                    current_tag = (current_tag as usize + (*current_tag).tag_len as usize) as *const Tag;
                }
            },
            _ => { // Unknown bootloader, panic
                panic!("unknown bootloader");
            }
        }
    }

    panic!("kernel exited");
}

#[unsafe(link_section = ".panic")]
#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    let message = info.message().as_str().unwrap_or("");
    if message != "" {
        aphrodite::arch::x86::output::sfatals(message);
        aphrodite::arch::x86::ports::outb(aphrodite::arch::x86::DEBUG_PORT, b'\n');
    }
    unsafe {
        asm!("hlt", options(noreturn));
    }
}