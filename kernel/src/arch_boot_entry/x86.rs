//! The entrypoint to the kernel; placed before all other code.
#![cfg(target_arch = "x86")]
#![no_std]
#![no_main]
#![warn(missing_docs)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]
#![feature(ptr_metadata)]
#![feature(cfg_match)]
#![feature(formatting_options)]

use core::arch::asm;
use core::ffi::CStr;
use core::fmt::Debug;
use core::panic::PanicInfo;

use aphrodite::arch::egatext;
use aphrodite::arch::output::*;
use aphrodite::boot::{BootInfo, MemoryMapping};
use aphrodite::multiboot2::{
    FramebufferInfo, MemoryMap, MemorySection, RawMemoryMap, RootTag, Tag,
};

#[cfg(not(CONFIG_DISABLE_MULTIBOOT2_SUPPORT))]
#[unsafe(link_section = ".bootheader")]
#[unsafe(no_mangle)]
static MULTIBOOT2_HEADER: [u8; 48] = [
    0xd6, 0x50, 0x52, 0xe8, // Magic number
    0x00, 0x00, 0x00, 0x00, // Architecture
    0x18, 0x00, 0x00, 0x00, // Size
    0x12, 0xaf, 0xad, 0x17, // Checksum
    0x0A, 0x00, // Relocatable tag
    0x00, 0x00, // Flags,
    0x18, 0x00, 0x00, 0x00, // Size of tag
    0x00, 0x00, 0x00, 0x00, // Starting minimum location
    0xFF, 0xFF, 0xFF, 0xFF, // Ending maximum location: End of 32-bit address space
    0x00, 0x00, 0x00, 0x00, // Image alignment
    0x01, 0x00, 0x00, 0x00, // Loading preference: lowest possible
    0x00, 0x00, // End tag
    0x00, 0x00, // Flags
    0x08, 0x00, 0x00, 0x00, // Size
];

// The root tag, provided directly from the multiboot2 bootloader.
static mut RT: *const RootTag = core::ptr::null();

// The raw pointer to bootloader-specific data.
static mut O: *const u8 = core::ptr::null();

static mut MM: MemoryMap = MemoryMap {
    entry_size: 0,
    version: 0,
    sections: &[],
};

static mut FBI: aphrodite::arch::egatext::FramebufferInfo =
    aphrodite::arch::egatext::FramebufferInfo {
        address: 0,
        pitch: 0,
        width: 0,
        height: 0,
        bpp: 0,
        change_cursor: false,
    };

// The magic number in eax. 0x36D76289 for multiboot2.
static mut MAGIC: u32 = 0xFFFFFFFF;

#[unsafe(link_section = ".start")]
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    unsafe {
        // Copy values provided by the bootloader out
        // Aphrodite bootloaders pass values in eax and ebx, however rust doesn't know
        // that it can't overwrite those. (if necessary, we'll store all of the
        // registers for other bootloaders and identify which one it is later)
        // we force using ebx and eax as the output of an empty assembly block to let it
        // know.
        asm!(
            "", out("ebx") O, // Bootloader-specific data(ebx)
                out("eax") MAGIC, // Magic number(eax)
            options(nomem, nostack, preserves_flags, pure)
        );
    }
    #[allow(non_snake_case)]
    let mut BI: BootInfo<'static> = BootInfo {
        cmdline: None,
        memory_map: None,
        bootloader_name: None,
        output: None,
        load_base: None,
    };
    unsafe {
        match MAGIC {
            #[cfg(not(CONFIG_DISABLE_MULTIBOOT2_SUPPORT))]
            0x36D76289 => {
                // Multiboot2
                RT = O as *const RootTag; // This is unsafe rust! We can do whatever we want! *manical laughter*

                sdebugs("Total boot info length is ");
                sdebugbnp(&aphrodite::u32_as_u8_slice((*RT).total_len));
                sdebugunp(b'\n');

                sdebugs("Root tag address is ");
                sdebugbnp(&aphrodite::usize_as_u8_slice(O as usize));
                sdebugunp(b'\n');

                if (*RT).total_len < 16 {
                    // Size of root tag+size of terminating tag. Something's up.
                    panic!("total length < 16")
                }

                let end_addr = O as usize + (*RT).total_len as usize;

                sdebugunp(b'\n');

                let mut ptr = O as usize;
                ptr += size_of::<RootTag>();

                let mut current_tag = core::ptr::read_volatile(ptr as *const Tag);

                loop {
                    sdebugs("Tag address is ");
                    sdebugbnpln(&aphrodite::usize_as_u8_slice(ptr));

                    sdebugs("Tag type is ");
                    sdebugbnpln(&aphrodite::u32_as_u8_slice(current_tag.tag_type));

                    sdebugs("Tag length is ");
                    sdebugbnpln(&aphrodite::u32_as_u8_slice(current_tag.tag_len));

                    match current_tag.tag_type {
                        0 => {
                            // Ending tag
                            if current_tag.tag_len != 8 {
                                // Unexpected size, something is probably up
                                panic!("size of ending tag != 8");
                            }
                            break;
                        },
                        4 => {
                            // Basic memory information, ignore
                            if current_tag.tag_len != 16 {
                                // Unexpected size, something is probably up
                                panic!("size of basic memory information tag != 16");
                            }
                        },
                        5 => {
                            // BIOS boot device, ignore
                            if current_tag.tag_len != 20 {
                                // Unexpected size, something is probably up
                                panic!("size of bios boot device tag != 20");
                            }
                        },
                        1 => {
                            // Command line
                            if current_tag.tag_len < 8 {
                                // Unexpected size, something is probably up
                                panic!("size of command line tag < 8");
                            }
                            let cstring = CStr::from_ptr((ptr + 8) as *const i8);
                            // creates a &core::ffi::CStr from the start of the command line...

                            BI.cmdline = Some(cstring.to_str().unwrap());
                            // ...before the BootInfo's commandline is set.
                        },
                        6 => {
                            // Memory map tag
                            if current_tag.tag_len < 16 {
                                // Unexpected size, something is probably up
                                panic!("size of memory map tag < 16");
                            }
                            let rawmemorymap: *mut RawMemoryMap = core::ptr::from_raw_parts_mut(
                                ptr as *mut u8,
                                (current_tag.tag_len / *((ptr + 8usize) as *const u32)) as usize,
                            );
                            // The end result of the above is creating a *const RawMemoryMap that
                            // has the same address as current_tag
                            // and has all of the [aphrodite::multiboot2::MemorySection]s for the
                            // memory map

                            let memorysections: &'static mut [aphrodite::multiboot2::MemorySection] = &mut *core::ptr::from_raw_parts_mut((&mut (*rawmemorymap).sections[0]) as &mut MemorySection, (*rawmemorymap).sections.len());
                            // Above is a bit hard to understand, but what it does is transmute
                            // rawmemorymap's sections into a pointer to those sections

                            for ele in &mut *memorysections {
                                (*ele) = core::mem::transmute::<
                                    aphrodite::boot::MemoryMapping,
                                    aphrodite::multiboot2::MemorySection,
                                >(
                                    Into::<MemoryMapping>::into(*ele)
                                )
                            }

                            MM = MemoryMap {
                                version: (*rawmemorymap).entry_version,
                                entry_size: (*rawmemorymap).entry_size,
                                sections: core::mem::transmute::<
                                    &mut [aphrodite::multiboot2::MemorySection],
                                    &[aphrodite::boot::MemoryMapping],
                                >(memorysections),
                            };
                            let mm2 = aphrodite::boot::MemoryMap {
                                len: MM.sections.len() as u64,
                                size_pages: 1,
                                page_size: MM.mem_size(),
                                sections: MM.sections,
                                idx: 0,
                            };
                            BI.memory_map = Some(mm2);
                        },
                        2 => {
                            // Bootloader name
                            if current_tag.tag_len < 8 {
                                // Unexpected size, something is probably up
                                panic!("size of command line tag < 8");
                            }
                            let cstring = CStr::from_ptr((ptr + 8) as *const i8);
                            // creates a &core::ffi::CStr from the start of the bootloader name...

                            BI.bootloader_name = Some(cstring.to_str().unwrap());
                            // ...before the BootInfo's bootloader_name is set.
                        },
                        8 => {
                            // Framebuffer info
                            if current_tag.tag_len < 32 {
                                // Unexpected size, something is probably up
                                panic!("size of framebuffer info tag < 32");
                            }
                            let framebufferinfo: *const FramebufferInfo =
                                (ptr + size_of::<Tag>()) as *const FramebufferInfo;
                            match (*framebufferinfo).fb_type {
                                0 => {
                                    // Indexed
                                    panic!("Indexed color is unimplemented");
                                },
                                1 => {
                                    // RGB
                                    panic!("RGB color is unimplemented");
                                },
                                2 => { // EGA Text  
                                },
                                _ => {
                                    panic!("unknown color info type")
                                },
                            }
                            let framebuffer_info = (*framebufferinfo).clone();

                            FBI = egatext::FramebufferInfo {
                                address: framebuffer_info.address,
                                pitch: framebuffer_info.pitch,
                                width: framebuffer_info.width,
                                height: framebuffer_info.height,
                                bpp: framebuffer_info.bpp,
                                change_cursor: false,
                            };
                            BI.output = Some(&FBI)
                        },
                        21 => {
                            // Image load base physical address
                            if current_tag.tag_len != 12 {
                                panic!("size of image load base physical address tag != 12");
                            }
                            let ptr = (ptr + size_of::<Tag>()) as *const u32;

                            sdebugs("ptr: ");
                            sdebugbnp(&aphrodite::usize_as_u8_slice(ptr as usize));
                            sdebugsnp(" value: ");
                            sdebugbnpln(&aphrodite::u32_as_u8_slice(*ptr));

                            BI.load_base = Some(*ptr);
                        },
                        _ => {
                            // Unknown/unimplemented tag type, ignore
                            swarnings("Unknown tag type ");
                            swarningbnpln(&aphrodite::u32_as_u8_slice(current_tag.tag_type));
                        },
                    }
                    sinfounp(b'\n');
                    ptr = (ptr + current_tag.tag_len as usize + 7) & !7;
                    if ptr > end_addr {
                        cfg_match! {
                            all(CONFIG_PREUSER_ERROR_ON_INVALID_LENGTH = "true", CONFIG_PREUSER_PANIC_ON_INVALID_LENGTH = "false") => {
                                serrorsln("Current tag length would put pointer out-of-bounds; CONFIG_PREUSER_ERROR_ON_INVALID_LENGTH is set, continuing");
                            }
                            all(CONFIG_PREUSER_WARN_ON_INVALID_LENGTH = "true", CONFIG_PREUSER_PANIC_ON_INVALID_LENGTH = "false") => {
                                swarningsln("Current tag length would put pointer out-of-bounds; CONFIG_PREUSER_WARN_ON_INVALID_LENGTH is set, continuing");
                            }
                        }
                        cfg_match! {
                            not(CONFIG_PREUSER_PANIC_ON_INVALID_LENGTH = "false") => {
                                panic!("current tag length would put pointer out-of-bounds")
                            }
                            CONFIG_PREUSER_EXIT_LOOP_ON_INVALID_LENGTH = "true" => {
                                sinfos("Exiting loop as current tag length would put pointer out-of-bounds ");
                                sinfosnpln("and CONFIG_PREUSER_EXIT_LOOP_ON_INVALID_LENGTH is set");
                                break;
                            }
                        }
                    }
                    current_tag = core::ptr::read_volatile(ptr as *const Tag);
                }
            },
            _ => {
                // Unknown bootloader, panic
                panic!("unknown bootloader");
            },
        }
    }
    sdebugsln("Bootloader information has been successfully loaded");
    sdebugunp(b'\n');

    // aphrodite::arch::initalize_rtc();

    unsafe {
        if BI.output.clone().is_some() {
            let framebuffer_info = FBI;

            sdebugs("Framebuffer width: ");
            sdebugbnpln(&aphrodite::u32_as_u8_slice(framebuffer_info.width));
            sdebugs("Framebuffer height: ");
            sdebugbnpln(&aphrodite::u32_as_u8_slice(framebuffer_info.height));
            sdebugs("Framebuffer pitch: ");
            sdebugbnpln(&aphrodite::u32_as_u8_slice(framebuffer_info.pitch));
            sdebugs("Framebuffer address: ");
            sdebugbnpln(&aphrodite::usize_as_u8_slice(
                framebuffer_info.address as usize,
            ));
            sdebugs("Framebuffer bpp: ");
            sdebugbnpln(&aphrodite::u8_as_u8_slice(framebuffer_info.bpp));

            let ega: &dyn aphrodite::display::TextDisplay = &framebuffer_info;

            aphrodite::indep_boot_entry::indep_boot_entry(Some(ega), &BI);
        }
    }

    aphrodite::indep_boot_entry::indep_boot_entry(None, &BI);
}

#[unsafe(link_section = ".panic")]
#[panic_handler]
#[cfg(not(CONFIG_HALT_ON_PANIC = "false"))]
fn halt_on_panic(info: &PanicInfo) -> ! {
    use core::fmt::FormattingOptions;

    if info.location().is_some() {
        sfatals("Panic at ");
        sfatalsnp(info.location().unwrap().file());
        sfatalsnp(":");
        sfatalbnp(&aphrodite::u32_as_u8_slice(info.location().unwrap().line()));
        sfatalsnp(":");
        sfatalbnpln(&aphrodite::u32_as_u8_slice(
            info.location().unwrap().column(),
        ));
    } else {
        sfatals("Panic: ");
    }
    let mut formatter = FormattingOptions::new().create_formatter(unsafe { &mut FBI });
    let _ = info.message().fmt(&mut formatter);
    aphrodite::arch::interrupts::disable_interrupts();
    unsafe {
        asm!("hlt", options(noreturn));
    }
}

#[unsafe(link_section = ".panic")]
#[panic_handler]
#[cfg(all(CONFIG_SPIN_ON_PANIC = "true", CONFIG_HALT_ON_PANIC = "false"))]
fn spin_on_panic(info: &PanicInfo) -> ! {
    if info.location().is_some() {
        sfatals("Panic at ");
        sfatalsnp(info.location().unwrap().file());
        sfatalsnp(":");
        sfatalbnp(&aphrodite::u32_as_u8_slice(info.location().unwrap().line()));
        sfatalsnp(":");
        sfatalbnp(&aphrodite::u32_as_u8_slice(
            info.location().unwrap().column(),
        ));
        sfatalsnp(": ");
    } else {
        sfatals("Panic: ");
    }
    let message = info.message().as_str().unwrap_or("");
    if message != "" {
        sfatalsnpln(message);
    } else {
        sfatalsnp("\n");
    }
    aphrodite::arch::interrupts::disable_interrupts();
    #[allow(clippy::empty_loop)]
    loop {}
}
