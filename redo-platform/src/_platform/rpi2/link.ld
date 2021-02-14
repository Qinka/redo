
/* Redo Kernel
 * Link file for armv7
 * Copyright (c) 2021 Johann Li <me@qinka.pro>
 */

ENTRY(_boot)

SECTIONS
{
    /* Start at LOADER_ADDR */
    . = 0x8000;

    __ro_start = .;
    .text :
    {
        *(.text._boot)
        *(.text._boot_config)
        *(.text*)
    }

    .exception_vectors :
    {
        *(.exception_vectors*)
    }

    .rodata :
    {
        *(.rodata*)
    }
    . = ALIGN(65536); /* Fill up to 64 KiB */
    __ro_end = .;

    .data :
    {
        *(.data*)
    }

    /* Section is zeroed in u64 chunks, align start and end to 8 bytes */
    .bss ALIGN(8):
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(8);

        /* Fill for the bss == 0 case, so that __bss_start <= __bss_end_inclusive holds */
        . += 8;
        __bss_end = . - 8;
    }
    . = ALIGN(65536);
    __data_end = .;

    __ro_size = __ro_end - __ro_start;
    __data_size = __data_end - __ro_end;

    /DISCARD/ : { *(.comment*) }
}


