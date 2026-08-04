#include <plix/console.h>

#define COM1 0x3f8u

static void outb(unsigned short port, unsigned char value) {
    __asm__ volatile ("outb %0, %1" : : "a"(value), "Nd"(port));
}

static unsigned char inb(unsigned short port) {
    unsigned char value;
    __asm__ volatile ("inb %1, %0" : "=a"(value) : "Nd"(port));
    return value;
}

static void serial_init(void) {
    static int initialized;
    if (initialized != 0) {
        return;
    }
    outb(COM1 + 1u, 0x00);
    outb(COM1 + 3u, 0x80);
    outb(COM1 + 0u, 0x03);
    outb(COM1 + 1u, 0x00);
    outb(COM1 + 3u, 0x03);
    outb(COM1 + 2u, 0xc7);
    outb(COM1 + 4u, 0x0b);
    initialized = 1;
}

void plix_console_putc(char value) {
    serial_init();
    while ((inb(COM1 + 5u) & 0x20u) == 0u) {
    }
    outb(COM1, (unsigned char)value);
}
