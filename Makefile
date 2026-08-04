ARCH ?= x86_64
BUILD_DIR ?= build/$(ARCH)
KERNEL := $(BUILD_DIR)/plix.elf

COMMON_CFLAGS := -std=c11 -ffreestanding -fno-stack-protector -fno-pic -fno-pie -Wall -Wextra -Werror -Iinclude
LDFLAGS := -T linker.ld -nostdlib -z max-page-size=0x1000

ifeq ($(ARCH),x86_64)
TRIPLE ?= x86_64-elf
CC ?= $(TRIPLE)-gcc
LD ?= $(TRIPLE)-ld
ARCH_BOOT := arch/x86_64/boot.S
ARCH_C := arch/x86_64/console.c
else ifeq ($(ARCH),aarch64)
TRIPLE ?= aarch64-none-elf
CC ?= $(TRIPLE)-gcc
LD ?= $(TRIPLE)-ld
ARCH_BOOT := arch/aarch64/boot.S
ARCH_C := arch/aarch64/console.c
else ifeq ($(ARCH),riscv64)
TRIPLE ?= riscv64-unknown-elf
CC ?= $(TRIPLE)-gcc
LD ?= $(TRIPLE)-ld
ARCH_BOOT := arch/riscv64/boot.S
ARCH_C := arch/riscv64/console.c
COMMON_CFLAGS += -mcmodel=medany
else
$(error Unsupported ARCH '$(ARCH)')
endif

C_SOURCES := kernel/main.c kernel/auralattice.c kernel/everyfile.c kernel/cli.c kernel/auth.c kernel/driver.c kernel/console.c
OBJECTS := $(patsubst %.c,$(BUILD_DIR)/%.o,$(C_SOURCES) $(ARCH_C)) $(BUILD_DIR)/$(ARCH_BOOT:.S=.o)

.PHONY: all clean check iso run run-serial
all: $(KERNEL)

$(KERNEL): $(OBJECTS) linker.ld
	$(LD) $(LDFLAGS) -o $@ $(OBJECTS)

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(COMMON_CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.S
	@mkdir -p $(dir $@)
	$(CC) $(COMMON_CFLAGS) -c $< -o $@

iso: $(KERNEL) boot/grub/grub.cfg
	@mkdir -p build/iso/boot/grub
	cp $(KERNEL) build/iso/boot/plix.elf
	cp boot/grub/grub.cfg build/iso/boot/grub/grub.cfg
	grub-mkrescue -o build/plix-$(ARCH).iso build/iso

run: $(KERNEL)
ifeq ($(ARCH),x86_64)
	qemu-system-x86_64 -kernel $(KERNEL) -serial stdio -display none
else ifeq ($(ARCH),aarch64)
	qemu-system-aarch64 -machine virt -cpu cortex-a57 -nographic -kernel $(KERNEL)
else ifeq ($(ARCH),riscv64)
	qemu-system-riscv64 -machine virt -nographic -kernel $(KERNEL)
endif

run-serial: run

check:
	$(CC) $(COMMON_CFLAGS) -fsyntax-only $(C_SOURCES)
	@mkdir -p build/tests
	cc -std=c11 -Wall -Wextra -Werror -Iinclude tests/cli_check.c kernel/everyfile.c kernel/cli.c kernel/auth.c kernel/driver.c kernel/console.c -o build/tests/cli_check
	build/tests/cli_check
	@for arch in x86_64 aarch64 riscv64; do \
		test -f arch/$$arch/boot.S || exit 1; \
	done

clean:
	rm -rf build
