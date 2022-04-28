INCLUDE defaults.x

ENTRY(_start);

SECTIONS
{
    _stack_size = 2K;

    .text ORIGIN(REGION_TEXT) : ALIGN(4)
    {
        _stext = .;
        KEEP(*(.vectors));
        KEEP(*(.init .init.*));
        . = ALIGN(4);

        *(.text .text.*);

        . = ALIGN(4);
        _etext = .;
    } > REGION_TEXT

    .rodata : ALIGN(4)
    {̦
        *(.srodata .srodata.*);
        *(.rodata .rodata.*);

        . = ALIGN(4);
    } > REGION_RODATA

    .stack (NOLOAD) : ALIGN(16)
    {
        _stack_bottom = .;
        . = . + _stack_size;
        . = ALIGN(16);
        _stack_top = .;
    } > REGION_STACK

    .data : ALIGN(4)
    {
        _sidata = LOADADDR(.data);
        _sdata = .;

        PROVIDE(__global_pointer$ = . + 0x800);

        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*);

        . = ALIGN(4);
        _edata = .;
    } > REGION_DATA AT > REGION_RODATA

    .bss (NOLOAD) : ALIGN(4)
    {
        _sbss = .;

        *(.sbss .sbss.*);
        *(.bss .bss.*);

        . = ALIGN(4);
        _ebss = .;
    } > REGION_BSS

    .heap (NOLOAD) : ALIGN(4)
    {
        _sheap = .;
        . = ORIGIN(RAM) + LENGTH(RAM);
        _eheap = .;
    } > REGION_HEAP

    .got (INFO) :
    {
        KEEP(*(.got .got.*));
    }

    .eh_frame (INFO) : { KEEP(*(.eh_frame)); }
    .eh_frame_hdr (INFO) : { *(.eh_frame_hdr); }

    ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
    ERROR: The start of the REGION_TEXT must be 4-byte aligned")

    ASSERT(ORIGIN(REGION_RODATA) % 4 == 0, "
    ERROR: The start of the REGION_RODATA must be 4-byte aligned")

    ASSERT(ORIGIN(REGION_DATA) % 4 == 0, "
    ERROR: the start of the REGION_DATA must be 4-byte aligned")

    ASSERT(ORIGIN(REGION_HEAP) % 4 == 0, "
    ERROR: the start of the REGION_HEAP must be 4-byte aligned")

    ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
    ERROR: the start of the REGION_TEXT must be 4-byte aligned")

    ASSERT(ORIGIN(REGION_STACK) % 4 == 0, "
    ERROR: the start of the REGION_STACK must be 4-byte aligned")

    ASSERT(_stext % 4 == 0, "
    ERROR: `_stext` must be 4-byte aligned")

    ASSERT(_sdata % 4 == 0 && _edata % 4 == 0, "
    BUG: .data is not 4-byte aligned")

    ASSERT(_sidata % 4 == 0, "
    BUG: the LMA of .data is not 4-byte aligned")

    ASSERT(_sbss % 4 == 0 && _ebss % 4 == 0, "
    BUG: .bss is not 4-byte aligned")

    ASSERT(_sheap % 4 == 0, "
    BUG: start of .heap is not 4-byte aligned")

    ASSERT(_stack_top % 16 == 0, "
    BUG: Stack pointer is not 16-byte aligned")

    ASSERT(_stext + SIZEOF(.text) < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT), "
    ERROR: The .text section must be placed inside the REGION_TEXT region.
    Set _stext to an address smaller than 'ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT)'")

    ASSERT(SIZEOF(.got) == 0, "
    .got section detected in the input files. Dynamic relocations are not
    supported. If you are linking to C code compiled using the `gcc` crate
    then modify your build script to compile the C code _without_ the
    -fPIC flag. See the documentation of the `gcc::Config.fpic` method for
    details.")
}
