.section .text.boot,"ax"
.global _start
.extern plix_kernel_main
_start:
    la sp, stack_top
    la t0, boot_info
    sd a0, 8(t0)
    sd a1, 16(t0)
    mv a0, t0
    call plix_kernel_main

.global plix_halt_forever
plix_halt_forever:
1:  wfi
    j 1b

.section .data
.align 3
boot_info:
    .word 0x5064
    .word 0
    .quad 0
    .quad 0

.section .bss
.align 4
.skip 16384
stack_top:

.section .note.GNU-stack,"",@progbits
