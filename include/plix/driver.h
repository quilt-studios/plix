#pragma once

#include <stddef.h>
#include <stdint.h>

#define PLIX_DRIVER_MAX_DEVICES 8u
#define PLIX_DRIVER_MAX_NAME 32u

typedef enum plix_driver_bus {
    PLIX_DRIVER_BUS_PLATFORM,
    PLIX_DRIVER_BUS_MMIO,
    PLIX_DRIVER_BUS_PIO,
} plix_driver_bus_t;

typedef enum plix_driver_class {
    PLIX_DRIVER_CLASS_CONSOLE,
    PLIX_DRIVER_CLASS_BLOCK,
    PLIX_DRIVER_CLASS_NET,
    PLIX_DRIVER_CLASS_INPUT,
} plix_driver_class_t;

typedef struct plix_driver_ops {
    void (*write)(const char *text);
    int (*read)(char *buffer, size_t capacity);
} plix_driver_ops_t;

typedef struct plix_driver {
    const char *name;
    plix_driver_bus_t bus;
    plix_driver_class_t device_class;
    uint64_t compatible;
    uint64_t base;
    uint64_t size;
    const plix_driver_ops_t *ops;
    int initialized;
} plix_driver_t;

void plix_driver_registry_init(void);
int plix_driver_register(const plix_driver_t *driver);
size_t plix_driver_count(void);
const plix_driver_t *plix_driver_get(size_t index);
const plix_driver_t *plix_driver_find_console(void);
void plix_driver_init_arch(uint64_t arch);
const char *plix_driver_bus_name(plix_driver_bus_t bus);
const char *plix_driver_class_name(plix_driver_class_t device_class);
