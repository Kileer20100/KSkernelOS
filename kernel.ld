SECTIONS {
    . = 1M;
    .text : { *(.text*) }
    .rodata : { *(.rodata*) }
    .data : { *(.data*) }
    .bss : { *(.bss*) }
    
    _heap_start = .;
    . += 1M;         /* 1 MB heap */
    _heap_end = .;
}
