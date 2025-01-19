//! The entrypoint to the kernel; placed before all other code.
#![no_std]
#![no_main]
#![warn(missing_docs)]

use core::{arch::asm, ffi::CStr, hint::unreachable_unchecked, panic::PanicInfo};
use aphrodite::multiboot2::{BootInfo, CString, RootTag, Tag};

#[unsafe(link_section = ".multiboot2")]
static MULTIBOOT_HEADER: [u16; 14] = [
    // Magic fields
    0xE852, 0x50D6, // Magic number
    0x0000, 0x0000, // Architecture, 0=i386
    0x0000, 0x000E, // length of MULTIBOOT_HEADER
    0x17AD, 0xAF1C, // checksum=all magic field excluding this+this=0
    
    // Framebuffer tag- empty flags, no preference for width, height, or bit depth
    0x0005, 0x0000,
    0x0014, 0x0000,
    0x0000, 0x0000,
];

// The root tag, provided directly from the multiboot bootloader.
static mut RT: *const RootTag = core::ptr::null();
// The boot info struct, created from all of the tags.
static mut BI: BootInfo = BootInfo {
    mem_lower: None,
    mem_upper: None,
    cmdline: None,
    modules: None,
    memory_map: None,
    bootloader_name: None,
    framebuffer_info: None
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
                                panic!("Size of ending tag != 8");
                            }
                            break
                        },
                        4 => { // Basic memory information
                            if (*current_tag).tag_len != 16 { // Unexpected size, something is probably up
                                panic!("Size of basic memory information tag != 16");
                            }

                            BI.mem_lower = Some(*((current_tag as usize + 8) as *const u32));
                            BI.mem_upper = Some(*((current_tag as usize + 12) as *const u32));
                            // The end result of the above is adding an offset to a pointer and retrieving the value at that pointer

                            current_tag = (current_tag as usize + 16) as *const Tag;
                        },
                        1 => { // Command line
                            if (*current_tag).tag_len < 8 { // Unexpected size, something is probably up
                                panic!("Size of command line tag < 8");
                            }
                            let cstring = CStr::from_ptr((current_tag as usize + 8) as *const i8);
                            // creates a &core::ffi::CStr from the start of the command line...

                            let cstring = CString {
                                ptr: cstring.as_ptr() as *const u8,
                                len: cstring.to_bytes().len()
                            };
                            // ...which can then be converted to a aphrodite::multiboot2::CString...

                            current_tag = (current_tag as usize + 8 + cstring.len) as *const Tag;
                            // ...before the current_tag is incremented to prevent ownership issues...

                            BI.cmdline = Some(cstring);
                            // ...before lastly the BootInfo's commandline is set.
                        },
                        _ => { // Unknown tag type
                            todo!("Implement tag");
                        }
                    }
                }
            },
            _ => { // Unknown bootloader, triple fault
                asm!(
                    "lidt 0", // Make interrupt table invalid(may or may not be invalid, depending on the bootloader, but we don't know)
                    "int 0h" // Try to perform an interrupt
                    // CPU then triple faults, thus restarting it
                )
            }
        }
    }
    loop {}
}

#[unsafe(link_section = ".panic")]
#[panic_handler]
fn handle_panic(_: &PanicInfo) -> ! {
    unsafe {
        asm!("hlt");
        unreachable_unchecked();
    }
}