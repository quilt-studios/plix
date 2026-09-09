#include <plix/boot.h>
#include <plix/cli.h>
#include <plix/driver.h>

static int contains(const char *haystack, const char *needle) {
    for (unsigned long i = 0; haystack[i] != '\0'; ++i) {
        unsigned long j = 0;
        while (needle[j] != '\0' && haystack[i + j] == needle[j]) {
            ++j;
        }
        if (needle[j] == '\0') {
            return 1;
        }
    }
    return 0;
}

int main(void) {
    plix_driver_init_arch(PLIX_ARCH_X86_64);

    plix_cli_t cli;
    plix_cli_init(&cli);

    if (!contains(plix_cli_output(&cli), "/users/guest")) {
        return 1;
    }
    if (!contains(plix_cli_output(&cli), "use help for commands")) {
        return 2;
    }
    if (plix_cli_execute(&cli, "help") != 0 || !contains(plix_cli_output(&cli), "pwd/pw")) {
        return 3;
    }
    if (plix_cli_execute(&cli, "pwd") != 0 || !contains(plix_cli_output(&cli), "/users/guest")) {
        return 4;
    }
    if (plix_cli_execute(&cli, "status") != 0 || !contains(plix_cli_output(&cli), "user=guest")) {
        return 5;
    }
    if (plix_cli_execute(&cli, "show") != 0) {
        return 6;
    }
    if (!contains(plix_cli_output(&cli), "main") || !contains(plix_cli_output(&cli), "house")) {
        return 7;
    }
    if (plix_cli_execute(&cli, "gt house") != 0) {
        return 8;
    }
    if (!contains(plix_cli_output(&cli), "/users/guest/house")) {
        return 9;
    }
    if (plix_cli_execute(&cli, "sw") != 0) {
        return 10;
    }
    if (!contains(plix_cli_output(&cli), "welcome")) {
        return 11;
    }
    if (plix_cli_execute(&cli, "drivers") != 0) {
        return 12;
    }
    if (!contains(plix_cli_output(&cli), "linux-8250-serial") || !contains(plix_cli_output(&cli), "console")) {
        return 13;
    }
    if (plix_cli_execute(&cli, "pudo guest show") == 0) {
        return 14;
    }
    if (plix_cli_execute(&cli, "login root plixroot") != 0) {
        return 15;
    }
    if (!contains(plix_cli_output(&cli), "angemeldet als root")) {
        return 16;
    }
    if (plix_cli_execute(&cli, "who") != 0 || !contains(plix_cli_output(&cli), "root")) {
        return 17;
    }
    if (plix_cli_execute(&cli, "pudo plixroot show") != 0) {
        return 18;
    }
    if (!contains(plix_cli_output(&cli), "pudo erlaubt")) {
        return 19;
    }
    if (plix_cli_execute(&cli, "definitely-not-a-command") == 0) {
        return 20;
    }
    if (!contains(plix_cli_output(&cli), "nutze help")) {
        return 21;
    }
    return 0;
}
