OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY {
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128K
}

SECTIONS {
    . = 0x80000000;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    } > RAM

    .rodata : {
        *(.rodata .rodata.*)
    } > RAM

    .data : {
        *(.data .data.*)
    } > RAM

    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(.ebss .ebss.*)
    } > RAM

    .stack : {
        . = ALIGN(16);
        *(.bss.stack)
    } > RAM

    /DISCARD/ : {
        *(.comment)
        *(.note.*)
    }

    /* symbol definition */
    PROVIDE(__stack_top = ORIGIN(RAM) + LENGTH(RAM));
    PROVIDE(__heap_start = .);
    PROVIDE(__heap_end = ORIGIN(RAM) + LENGTH(RAM));
}