#include <plix/console.h>

__attribute__((weak)) void plix_console_putc(char value) {
    (void)value;
}

void plix_console_write(const char *text) {
    for (unsigned i = 0; text[i] != '\0'; ++i) {
        plix_console_putc(text[i]);
    }
}
