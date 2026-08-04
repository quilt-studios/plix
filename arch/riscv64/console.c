#include <plix/console.h>

#define UART0 ((volatile unsigned char *)0x10000000u)

void plix_console_putc(char value) {
    *UART0 = (unsigned char)value;
}
