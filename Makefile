ARCH ?= x86_64
BUILD_DIR ?= build/$(ARCH)
KERNEL := $(BUILD_DIR)/plix.elf
ISO := build/plix-x86_64.iso

COMMON_CFLAGS := -std=c11 -ffreestanding -fno-stack-protector -fno-pic -fno-pie -Wall -Wextra -Werror -Iinclude
RUSTC ?= rustc
RUSTFLAGS := --edition=2021 -C panic=abort -C opt-level=2
LDFLAGS := -T linker.ld -nostdlib -z max-page-size=0x1000

ifeq ($(ARCH),x86_64)
TRIPLE ?= x86_64-elf
CC ?= $(TRIPLE)-gcc
LD ?= $(TRIPLE)-ld
ARCH_BOOT := arch/x86_64/boot.S
ARCH_C := arch/x86_64/console.c
RUST_TARGET ?= x86_64-unknown-none
else ifeq ($(ARCH),aarch64)
TRIPLE ?= aarch64-none-elf
CC ?= $(TRIPLE)-gcc
LD ?= $(TRIPLE)-ld
ARCH_BOOT := arch/aarch64/boot.S
ARCH_C := arch/aarch64/console.c
RUST_TARGET ?= aarch64-unknown-none
else ifeq ($(ARCH),riscv64)
TRIPLE ?= riscv64-unknown-elf
CC ?= $(TRIPLE)-gcc
LD ?= $(TRIPLE)-ld
ARCH_BOOT := arch/riscv64/boot.S
ARCH_C := arch/riscv64/console.c
RUST_TARGET ?= riscv64gc-unknown-none-elf
COMMON_CFLAGS += -mcmodel=medany
else
$(error Unsupported ARCH '$(ARCH)')
endif

C_SOURCES :=
RUST_SOURCES := kernel/plix.rs
OBJECTS := $(patsubst %.c,$(BUILD_DIR)/%.o,$(C_SOURCES) $(ARCH_C)) $(patsubst %.rs,$(BUILD_DIR)/%.a,$(RUST_SOURCES)) $(BUILD_DIR)/$(ARCH_BOOT:.S=.o)

.PHONY: all clean check check-rust check-host check-tree iso run run-serial
all: $(KERNEL)

$(KERNEL): $(OBJECTS) linker.ld
	$(LD) $(LDFLAGS) -o $@ $(OBJECTS)

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(COMMON_CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.S
	@mkdir -p $(dir $@)
	$(CC) $(COMMON_CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.a: %.rs
	@mkdir -p $(dir $@)
	$(RUSTC) $(RUSTFLAGS) --target $(RUST_TARGET) --crate-type staticlib $< -o $@

iso: $(KERNEL) boot/grub/grub.cfg
ifeq ($(ARCH),x86_64)
	@mkdir -p build/iso/boot/grub
	cp $(KERNEL) build/iso/boot/plix.elf
	cp boot/grub/grub.cfg build/iso/boot/grub/grub.cfg
	grub-file --is-x86-multiboot2 $(KERNEL)
	grub-mkrescue -o $(ISO) build/iso
else
	@echo "ISO boot is currently supported only for ARCH=x86_64" >&2
	@exit 1
endif

run:
ifeq ($(ARCH),x86_64)
	$(MAKE) ARCH=x86_64 iso
	qemu-system-x86_64 -cdrom $(ISO) -serial stdio -display none -no-reboot -no-shutdown
else ifeq ($(ARCH),aarch64)
	$(MAKE) $(KERNEL)
	qemu-system-aarch64 -machine virt -cpu cortex-a57 -nographic -kernel $(KERNEL)
else ifeq ($(ARCH),riscv64)
	$(MAKE) $(KERNEL)
	qemu-system-riscv64 -machine virt -nographic -kernel $(KERNEL)
endif

run-serial: run

check-rust:
	@mkdir -p build/tests
	$(RUSTC) $(RUSTFLAGS) --crate-type staticlib kernel/plix.rs -o build/tests/libplix.a

check-host: check-rust
	cc -std=c11 -Wall -Wextra -Werror -Iinclude tests/cli_check.c tests/console_putc.c tests/halt_stub.c build/tests/libplix.a -o build/tests/cli_check
	build/tests/cli_check

check-tree:
	@for arch in x86_64 aarch64 riscv64; do \
		test -f arch/$$arch/boot.S || exit 1; \
		test -f arch/$$arch/console.c || exit 1; \
	done
	@test -f boot/grub/grub.cfg
	@test -f linker.ld
	@test -f include/plix/boot.h
	@test -f include/plix/cli.h
	@test -f include/plix/console.h
	@test -f include/plix/driver.h
	@test -f include/plix/auth.h
	@test -f include/plix/everyfile.h
	@test -f include/plix/auralattice.h

check: check-host check-tree

clean:
	rm -rf build
