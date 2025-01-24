ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
    .text : {
        . = ALIGN(8);
        KEEP(*(.multiboot2))
        KEEP(*(.start))
        KEEP(*(.text))
        KEEP(*(.panic))
    }
}
