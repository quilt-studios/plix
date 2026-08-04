#include <plix/cli.h>

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
    plix_cli_t cli;
    plix_cli_init(&cli);

    if (!contains(plix_cli_output(&cli), "/users/guest")) {
        return 1;
    }
    if (plix_cli_execute(&cli, "show") != 0) {
        return 2;
    }
    if (!contains(plix_cli_output(&cli), "main") || !contains(plix_cli_output(&cli), "house")) {
        return 3;
    }
    if (plix_cli_execute(&cli, "gt house") != 0) {
        return 4;
    }
    if (!contains(plix_cli_output(&cli), "/users/guest/house")) {
        return 5;
    }
    if (plix_cli_execute(&cli, "sw") != 0) {
        return 6;
    }
    if (!contains(plix_cli_output(&cli), "welcome")) {
        return 7;
    }
    if (plix_cli_execute(&cli, "pudo guest show") == 0) {
        return 8;
    }
    if (plix_cli_execute(&cli, "login root plixroot") != 0) {
        return 9;
    }
    if (!contains(plix_cli_output(&cli), "angemeldet als root")) {
        return 10;
    }
    if (plix_cli_execute(&cli, "pudo plixroot show") != 0) {
        return 11;
    }
    if (!contains(plix_cli_output(&cli), "pudo erlaubt")) {
        return 12;
    }
    return 0;
}
