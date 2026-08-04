#include <plix/driver.h>
#include <plix/boot.h>
#include <plix/console.h>

#define PLIX_COMPAT_X86_16550 0x7838363136353530ULL
#define PLIX_COMPAT_ARM_PL011 0x61726d706c303131ULL
#define PLIX_COMPAT_RV_16550 0x7276313635353000ULL

static plix_driver_t registry[PLIX_DRIVER_MAX_DEVICES];
static size_t registry_count;

static void console_write(const char *text) {
    plix_console_write(text);
}

static int no_input(char *buffer, size_t capacity) {
    (void)buffer;
    (void)capacity;
    return 0;
}

static const plix_driver_ops_t console_ops = {
    .write = console_write,
    .read = no_input,
};

void plix_driver_registry_init(void) {
    registry_count = 0;
    for (size_t i = 0; i < PLIX_DRIVER_MAX_DEVICES; ++i) {
        registry[i].name = 0;
        registry[i].initialized = 0;
    }
}

int plix_driver_register(const plix_driver_t *driver) {
    if (driver == 0 || driver->name == 0 || driver->ops == 0 || registry_count >= PLIX_DRIVER_MAX_DEVICES) {
        return -1;
    }
    registry[registry_count] = *driver;
    registry[registry_count].initialized = 1;
    ++registry_count;
    return 0;
}

size_t plix_driver_count(void) {
    return registry_count;
}

const plix_driver_t *plix_driver_get(size_t index) {
    return index < registry_count ? &registry[index] : 0;
}

const plix_driver_t *plix_driver_find_console(void) {
    for (size_t i = 0; i < registry_count; ++i) {
        if (registry[i].device_class == PLIX_DRIVER_CLASS_CONSOLE && registry[i].initialized != 0) {
            return &registry[i];
        }
    }
    return 0;
}

void plix_driver_init_arch(uint64_t arch) {
    plix_driver_registry_init();

    if (arch == PLIX_ARCH_X86_64) {
        const plix_driver_t driver = {"linux-8250-serial", PLIX_DRIVER_BUS_PIO, PLIX_DRIVER_CLASS_CONSOLE, PLIX_COMPAT_X86_16550, 0x3f8u, 8u, &console_ops, 0};
        (void)plix_driver_register(&driver);
    } else if (arch == PLIX_ARCH_AARCH64) {
        const plix_driver_t driver = {"linux-amba-pl011", PLIX_DRIVER_BUS_MMIO, PLIX_DRIVER_CLASS_CONSOLE, PLIX_COMPAT_ARM_PL011, 0x09000000u, 0x1000u, &console_ops, 0};
        (void)plix_driver_register(&driver);
    } else if (arch == PLIX_ARCH_RISCV64) {
        const plix_driver_t driver = {"linux-8250-mmio", PLIX_DRIVER_BUS_MMIO, PLIX_DRIVER_CLASS_CONSOLE, PLIX_COMPAT_RV_16550, 0x10000000u, 0x100u, &console_ops, 0};
        (void)plix_driver_register(&driver);
    }
}

const char *plix_driver_bus_name(plix_driver_bus_t bus) {
    switch (bus) {
    case PLIX_DRIVER_BUS_PLATFORM:
        return "platform";
    case PLIX_DRIVER_BUS_MMIO:
        return "mmio";
    case PLIX_DRIVER_BUS_PIO:
        return "pio";
    }
    return "unknown";
}

const char *plix_driver_class_name(plix_driver_class_t device_class) {
    switch (device_class) {
    case PLIX_DRIVER_CLASS_CONSOLE:
        return "console";
    case PLIX_DRIVER_CLASS_BLOCK:
        return "block";
    case PLIX_DRIVER_CLASS_NET:
        return "net";
    case PLIX_DRIVER_CLASS_INPUT:
        return "input";
    }
    return "unknown";
}
