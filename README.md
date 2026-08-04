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


## Plix CLI

After the architecture-specific boot handoff, the kernel initializes the Plix CLI on top of the `everyfile` filesystem. Commands always have a full and short spelling where applicable:

- `goto` / `gt`: go to a directory, replacing `cd`.
- `show` / `sw`: show directory entries, replacing `ls`.
- `pudo`: power-user do, the Plix administrative command prefix replacing `sudo`.

`everyfile` is user-centric. Every user owns a full tree below `/users/<name>/`; global configuration is under `main` instead of `etc`, and personal files are under `house` instead of `home`. Each known user gets `main` and `house`; the authenticated boot session starts in `/users/guest`, and `root` can be reached with `login root plixroot`.

## Users and passwords

The boot session starts as `guest` in `/users/guest`. Use `login <user> <password>` to switch users. The prototype ships with `guest` / `guest` and power user `root` / `plixroot`. Passwords are stored as salted FNV-1a hashes in the kernel image rather than plaintext. `pudo <password> <command>` only succeeds for an authenticated power user with the correct password.

## QEMU

The kernel writes CLI boot output to the QEMU-friendly debug console for each architecture: COM1 serial on x86_64, PL011 UART0 on `virt` AArch64, and NS16550 UART0 on `virt` RISC-V. The Makefile provides `run` and `run-serial` targets so the same build can be launched directly in QEMU once the matching emulator is installed.
