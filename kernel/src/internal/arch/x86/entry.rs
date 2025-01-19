//! The entrypoint to the kernel; placed before all other code.
#![no_std]
#![no_main]
#![warn(missing_docs)]

use core::{arch::asm, hint::unreachable_unchecked, panic::PanicInfo};
use aphrodite::multiboot2::{BootInfo, RootTag, Tag};

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

static mut RT: *const RootTag = core::ptr::null();
static mut BI: *const BootInfo = core::ptr::null();
static mut O: *const u8 = core::ptr::null();
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
            0x36d76289 => { // Multiboot2
                RT = O as *const RootTag; // This is unsafe rust! We can do whatever we want! *manical laughter*


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