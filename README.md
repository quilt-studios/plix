# Plix Kernel

Plix is an experimental, freestanding kernel prototype built around the **Auralattice** architecture: a small message-fabric core where every CPU architecture enters the same kernel through a capability-oriented boot contract.

The tree currently supports boot objects for:

- `x86_64` via a Multiboot2-compatible entry point.
- `aarch64` via a flat QEMU/firmware-style kernel entry.
- `riscv64` via the standard machine-mode register handoff used by QEMU `virt`.

## Build

```sh
make ARCH=x86_64
make ARCH=aarch64
make ARCH=riscv64
```

The default build directory is `build/`. Cross compilers can be overridden with `CC`, `LD`, `OBJCOPY`, and `OBJDUMP`.

## Boot

For x86_64, install GRUB tooling and run:

```sh
make ARCH=x86_64 iso
qemu-system-x86_64 -cdrom build/plix-x86_64.iso
```

For AArch64 and RISC-V, the generated ELF is suitable for direct firmware or QEMU loading once the matching cross toolchain and emulator are installed:

```sh
qemu-system-aarch64 -machine virt -cpu cortex-a57 -nographic -kernel build/aarch64/plix.elf
qemu-system-riscv64 -machine virt -nographic -kernel build/riscv64/plix.elf
```

## Checks

```sh
make check
```
