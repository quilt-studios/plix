.section .multiboot2,"a"
.align 8
header_start:
.long 0xe85250d6
.long 0
.long header_end - header_start
.long -(0xe85250d6 + 0 + (header_end - header_start))
.align 8
.short 0
.short 0
.long 8
header_end:

.section .text.boot,"ax"
.global _start
.extern plix_kernel_main
_start:
    cli
    mov $stack_top, %rsp
    xor %rbp, %rbp
    lea boot_info(%rip), %rdi
    mov %rbx, boot_info+16(%rip)
    call plix_kernel_main

.global plix_halt_forever
plix_halt_forever:
    cli
1:  hlt
    jmp 1b

.section .data
.align 8
boot_info:
    .long 0x8664
    .long 0
    .quad 0
    .quad 0

.section .bss
.align 16
.skip 16384
stack_top:

.section .note.GNU-stack,"",@progbits
