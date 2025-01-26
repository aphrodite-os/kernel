//! The main code for the kernel.
#![warn(missing_docs)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]

use crate::arch::x86::output::*;
use crate::arch::x86::egatext as egatext;
use crate::multiboot2::BootInfo;
use egatext::*;

/// The real entrypoint to the kernel. `internel/arch/*/entry.rs` files eventually call this.
#[allow(non_snake_case)]
pub fn _entry(ega: Option<crate::arch::x86::egatext::FramebufferInfo>, BI: &BootInfo) -> ! {
    if ega.is_some() {
        let ega = ega.unwrap();
        ega.clear_screen(WHITE_ON_BLACK);
        sreset();

        let extended_functions = crate::arch::x86::cpuid_extended_functions();

        if extended_functions { 
            binfosln("This CPU supports extended functions", ega).unwrap();

            let longmode_support = crate::arch::x86::cpuid(0x80000001).1 & (1<<29) > 1;
            if longmode_support {
                binfosln("This CPU supports long mode", ega).unwrap();
            } else {
                binfosln("This CPU does NOT support long mode!", ega).unwrap();
                bdebugs("Long mode CPUID: ", ega).unwrap();
                bdebugbnpln(&crate::u32_as_u8_slice(crate::arch::x86::cpuid(0x80000001).1), ega).unwrap();
            }
        } else {
            binfosln("This CPU does NOT support extended functions or long mode!", ega).unwrap();
        }
        if BI.bootloader_name.is_some() {
            binfos("Kernel booted by ", ega).unwrap();
            binfosnpln(BI.bootloader_name.unwrap().into(), ega).unwrap();
        }
        if BI.cmdline.is_some() {
            binfos("Command line passed: \"", ega).unwrap();
            binfosnp(BI.cmdline.unwrap().into(), ega).unwrap();
            binfosnpln("\"", ega).unwrap();
        }
        if BI.mem_lower.is_some() {
            binfos("Amount of lower memory: ", ega).unwrap();
            binfobnpln(&crate::u32_as_u8_slice(BI.mem_lower.unwrap()), ega).unwrap();
        }
        if BI.mem_upper.is_some() {
            binfos("Amount of upper memory: ", ega).unwrap();
            binfobnpln(&crate::u32_as_u8_slice(BI.mem_upper.unwrap()), ega).unwrap();
        }
        if BI.memory_map.is_some() {
            binfos("Recieved memory map from bootloader with ", ega).unwrap();
            binfobnp(&crate::usize_as_u8_slice(BI.memory_map.unwrap().sections.len()), ega).unwrap();
            binfosnpln(" sections", ega).unwrap();
            let mut i = 0;
            for ele in BI.memory_map.unwrap().sections {
                binfos("Section #", ega).unwrap();
                binfobnp(&crate::usize_as_u8_slice(i), ega).unwrap();
                binfosnp(": ", ega).unwrap();
                match ele.mem_type {
                    1 => {
                        binfosnp("Available RAM", ega).unwrap();
                    },
                    2 => {
                        binfosnp("Reserved by hardware", ega).unwrap();
                    }
                    3 => {
                        binfosnp("ACPI information", ega).unwrap();
                    },
                    4 => {
                        binfosnp("Reserved memory", ega).unwrap();
                    },
                    5 => {
                        binfosnp("Defective", ega).unwrap();
                    },
                    _ => {
                        binfosnp("Reserved/unknown (type=", ega).unwrap();
                        binfobnp(&crate::u32_as_u8_slice(ele.mem_type), ega).unwrap();
                        binfosnp(")", ega).unwrap();
                    }
                }
                binfosnp(", starting at ", ega).unwrap();
                binfobnp(&crate::u64_as_u8_slice(ele.base_addr), ega).unwrap();
                binfosnp(" and running for ", ega).unwrap();
                binfobnp(&crate::u64_as_u8_slice(ele.length), ega).unwrap();
                binfosnpln(" bytes", ega).unwrap();

                i += 1;
            }
        }
    } else {
        if BI.bootloader_name.is_some() {
            sinfos("Kernel booted by ");
            sinfosnpln(BI.bootloader_name.unwrap().into());
        }
        if BI.cmdline.is_some() {
            sinfos("Command line passed: \"");
            sinfosnp(BI.cmdline.unwrap().into());
            sinfosnpln("\"");
        }
        if BI.memory_map.is_some() {
            sinfosln("Recieved memory map from bootloader");
        }
        if BI.mem_lower.is_some() {
            sinfos("Amount of lower memory: ");
            sinfobnpln(&crate::u32_as_u8_slice(BI.mem_lower.unwrap()));
        }
        if BI.mem_upper.is_some() {
            sinfos("Amount of upper memory: ");
            sinfobnpln(&crate::u32_as_u8_slice(BI.mem_upper.unwrap()));
        }
    }
    loop {}
}