SECTIONS
{
    . = 0x80000;

    .text : 
    {
        *(.text._start) *(.text.atomic_cmpset) *(.text*)
    }

    .rodata : 
    { 
        *(.rodata*)
    }
    
    .data : 
    {
        *(.data*)
    }

    . = ALIGN(0x8);
    __bss_start = .;
    .bss : 
    {
        *(.bss*)
    }
    __bss_end = .;

    /DISCARD/ : { *(.comment*) }

}