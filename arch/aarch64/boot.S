.section .text.boot,"ax"
.global _start
.extern plix_kernel_main
_start:
    ldr x1, =stack_top
    mov sp, x1
    ldr x0, =boot_info
    str xzr, [x0, #8]
    str x1, [x0, #16]
    bl plix_kernel_main

.global plix_halt_forever
plix_halt_forever:
1:  wfe
    b 1b

.section .data
.align 3
boot_info:
    .word 0xaa64
    .word 0
    .quad 0
    .quad 0

.section .bss
.align 4
.skip 16384
stack_top:

.section .note.GNU-stack,"",@progbits
