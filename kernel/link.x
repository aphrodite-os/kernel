ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
    .text : {
        . = ALIGN(8);
        KEEP(*(.bootheader))
        KEEP(*(.start))
        KEEP(*(.text))
        KEEP(*(.panic))
    }
}
