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
else ifeq ($(ARCH),aarch64)
TRIPLE ?= aarch64-none-elf
CC ?= $(TRIPLE)-gcc
LD ?= $(TRIPLE)-ld
ARCH_BOOT := arch/aarch64/boot.S
else ifeq ($(ARCH),riscv64)
TRIPLE ?= riscv64-unknown-elf
CC ?= $(TRIPLE)-gcc
LD ?= $(TRIPLE)-ld
ARCH_BOOT := arch/riscv64/boot.S
COMMON_CFLAGS += -mcmodel=medany
else
$(error Unsupported ARCH '$(ARCH)')
endif

C_SOURCES := kernel/main.c kernel/auralattice.c
OBJECTS := $(patsubst %.c,$(BUILD_DIR)/%.o,$(C_SOURCES)) $(BUILD_DIR)/$(ARCH_BOOT:.S=.o)

.PHONY: all clean check iso
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

check:
	$(CC) $(COMMON_CFLAGS) -fsyntax-only $(C_SOURCES)
	@for arch in x86_64 aarch64 riscv64; do \
		test -f arch/$$arch/boot.S || exit 1; \
	done

clean:
	rm -rf build
