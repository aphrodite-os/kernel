ENTRY(_start)
OUTPUT_FORMAT(binary)

SECTIONS {
    .text : {
        . = ALIGN(8);
        KEEP(*(.multiboot2))
        KEEP(*(.start))
        KEEP(*(.text))
        KEEP(*(.panic))
    }
}
