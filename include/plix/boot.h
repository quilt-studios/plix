#pragma once

#include <stdint.h>

typedef enum plix_arch {
    PLIX_ARCH_X86_64 = 0x8664,
    PLIX_ARCH_AARCH64 = 0xaa64,
    PLIX_ARCH_RISCV64 = 0x5064,
} plix_arch_t;

typedef struct plix_boot_info {
    plix_arch_t arch;
    uint64_t hart_or_cpu;
    uintptr_t firmware_pointer;
} plix_boot_info_t;

void plix_kernel_main(const plix_boot_info_t *boot_info);
void plix_halt_forever(void) __attribute__((noreturn));
