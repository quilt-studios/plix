#include <plix/console.h>

#define PL011_UART0 ((volatile unsigned int *)0x09000000u)

void plix_console_putc(char value) {
    PL011_UART0[0] = (unsigned int)value;
}
