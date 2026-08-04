#include <plix/auralattice.h>

void plix_auralattice_init(plix_auralattice_t *fabric) {
    fabric->magic = PLIX_AURALATTICE_MAGIC;
    fabric->realm_count = 1;
    fabric->reserved = 0;
    for (uint32_t i = 0; i < PLIX_MAX_REALMS; ++i) {
        fabric->realms[i].root = i == 0 ? 1u : 0u;
        fabric->realms[i].epoch = 0;
        fabric->realms[i].pulse_count = 0;
    }
}

int plix_auralattice_emit(plix_auralattice_t *fabric, const plix_pulse_t *pulse) {
    if (fabric->magic != PLIX_AURALATTICE_MAGIC || pulse->target >= PLIX_MAX_REALMS) {
        return -1;
    }

    plix_realm_t *realm = &fabric->realms[pulse->target];
    if (realm->root == 0) {
        realm->root = pulse->target + 1u;
        ++fabric->realm_count;
    }

    ++realm->epoch;
    ++realm->pulse_count;
    return 0;
}
