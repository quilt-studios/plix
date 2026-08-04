#pragma once

#include <stddef.h>

#define PLIX_CLI_OUTPUT_CAPACITY 512u

typedef struct plix_cli {
    char output[PLIX_CLI_OUTPUT_CAPACITY];
    size_t output_len;
} plix_cli_t;

void plix_cli_boot(void);
void plix_cli_init(plix_cli_t *cli);
int plix_cli_execute(plix_cli_t *cli, const char *line);
const char *plix_cli_output(const plix_cli_t *cli);
