#include <plix/auralattice.h>
#include <plix/boot.h>

static plix_auralattice_t fabric;
volatile uint64_t plix_last_boot_arch;
volatile uint64_t plix_boot_pulses;

void plix_kernel_main(const plix_boot_info_t *boot_info) {
    plix_auralattice_init(&fabric);

    plix_pulse_t first_pulse = {
        .source = 0,
        .target = 0,
        .verb = boot_info->arch,
        .payload = {boot_info->hart_or_cpu, boot_info->firmware_pointer, 0, 0},
    };

    (void)plix_auralattice_emit(&fabric, &first_pulse);
    plix_last_boot_arch = boot_info->arch;
    plix_boot_pulses = fabric.realms[0].pulse_count;

    plix_halt_forever();
}
