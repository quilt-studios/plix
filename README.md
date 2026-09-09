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

The default build directory is `build/`. Cross compilers can be overridden with `CC` and `LD`. A Rust compiler is used for the freestanding kernel components; override it with `RUSTC`, and install or override the per-architecture `RUST_TARGET` (`x86_64-unknown-none`, `aarch64-unknown-none`, or `riscv64gc-unknown-none-elf`).

## Automated checks

The repository now runs GitHub Actions on pushes to `main` and on pull requests. CI executes the host regression suite and builds the x86_64 kernel ELF with the freestanding Rust target.

Run the same host checks locally with:

```sh
make check
```

The check target is split into:

```sh
make check-rust
make check-host
make check-tree
```

`check-host` builds the Rust kernel library for the host and links the CLI regression test. `check-tree` verifies that the architecture entry points, consoles, linker script, GRUB config, and public headers are present.

## Boot

### x86_64

The x86_64 kernel uses a Multiboot2 header and is booted through GRUB. Install GRUB rescue-image tooling, xorriso, and QEMU, then run:

```sh
make ARCH=x86_64 iso
qemu-system-x86_64 -cdrom build/plix-x86_64.iso -serial stdio -display none
```

Or use the convenience target:

```sh
make ARCH=x86_64 run
```

The ISO target checks the generated ELF with `grub-file --is-x86-multiboot2` before creating the image.

### AArch64 and RISC-V

The generated ELF is suitable for direct firmware or QEMU loading once the matching cross toolchain and emulator are installed:

```sh
qemu-system-aarch64 -machine virt -cpu cortex-a57 -nographic -kernel build/aarch64/plix.elf
qemu-system-riscv64 -machine virt -nographic -kernel build/riscv64/plix.elf
```

The same commands are available through:

```sh
make ARCH=aarch64 run
make ARCH=riscv64 run
```

## Plix CLI

After the architecture-specific boot handoff, the kernel initializes the Plix CLI on top of the `everyfile` filesystem. The boot transcript now includes status, loaded drivers, and the current directory listing.

Available commands:

- `help` / `?`: show the command overview.
- `pwd` / `pw`: show the current Everyfile path.
- `status` / `st`: show the active user, current path, and loaded-driver count marker.
- `who`: show the active user.
- `goto` / `gt <path>`: move to a directory, replacing `cd`.
- `show` / `sw`: show directory entries, replacing `ls`.
- `drivers` / `drv`: list initialized drivers with bus and class.
- `login <user> <password>`: switch the active prototype user.
- `pudo <password> <command>`: validate power-user authorization for a command request.

`everyfile` is user-centric. Every user owns a tree below `/users/<name>/`; global configuration is under `main` instead of `etc`, and personal files are under `house` instead of `home`. Each known user gets `main` and `house`; the boot session starts in `/users/guest`.

## Authentication prototype

The current authentication layer is intentionally small and is **not production security**. It ships with fixed prototype users and salted FNV-1a password hashes embedded in the kernel image. This is suitable for exercising privilege and session flows during kernel development, but it should be replaced before Plix is used for real security boundaries.

The current test users are:

- `guest` / `guest`
- power user `root` / `plixroot`

## Linux-style driver registry

Plix has a small in-kernel driver registry using platform/MMIO/PIO metadata and compatible identifiers. The first console drivers are:

- `linux-8250-serial` for the x86_64 COM1/8250-compatible serial port.
- `linux-amba-pl011` for the AArch64 `virt` AMBA PL011 UART.
- `linux-8250-mmio` for the RISC-V `virt` MMIO 8250-compatible UART.

Use `drivers` or `drv` in the Plix CLI to list initialized drivers with their bus and class.

## Console output

The kernel writes boot and CLI output to QEMU-friendly architecture consoles:

- x86_64: COM1 serial.
- AArch64: PL011 UART0 on `virt`.
- RISC-V: NS16550 UART0 on `virt`.

The `run` and `run-serial` targets launch the matching QEMU path when the required toolchain and emulator are installed.

## Project status

Plix is still a kernel prototype rather than a complete general-purpose operating system. Current emphasis is on a consistent cross-architecture boot contract, Auralattice messaging, Everyfile semantics, authentication experiments, a minimal CLI, driver discovery, and reproducible automated checks.
