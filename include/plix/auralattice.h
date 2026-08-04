#pragma once

#include <stdint.h>

#define PLIX_AURALATTICE_MAGIC 0x415552414c415454ULL
#define PLIX_MAX_REALMS 64u

typedef uint64_t plix_capability_t;

typedef struct plix_pulse {
    plix_capability_t source;
    plix_capability_t target;
    uint64_t verb;
    uint64_t payload[4];
} plix_pulse_t;

typedef struct plix_realm {
    plix_capability_t root;
    uint64_t epoch;
    uint64_t pulse_count;
} plix_realm_t;

typedef struct plix_auralattice {
    uint64_t magic;
    uint32_t realm_count;
    uint32_t reserved;
    plix_realm_t realms[PLIX_MAX_REALMS];
} plix_auralattice_t;

void plix_auralattice_init(plix_auralattice_t *fabric);
int plix_auralattice_emit(plix_auralattice_t *fabric, const plix_pulse_t *pulse);
