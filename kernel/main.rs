#[repr(C)]
pub struct PlixBootInfo {
    arch: u32,
    hart_or_cpu: u64,
    firmware_pointer: usize,
}

#[repr(C)]
struct PlixPulse {
    source: u64,
    target: u64,
    verb: u64,
    payload: [u64; 4],
}

#[repr(C)]
struct PlixRealm {
    root: u64,
    epoch: u64,
    pulse_count: u64,
}

#[repr(C)]
struct PlixAuralattice {
    magic: u64,
    realm_count: u32,
    reserved: u32,
    realms: [PlixRealm; 64],
}

extern "C" {
    fn plix_auralattice_init(fabric: *mut PlixAuralattice);
    fn plix_auralattice_emit(fabric: *mut PlixAuralattice, pulse: *const PlixPulse) -> i32;
    fn plix_driver_init_arch(arch: u64);
    fn plix_cli_boot();
    fn plix_halt_forever() -> !;
}

static mut FABRIC: PlixAuralattice = PlixAuralattice {
    magic: 0,
    realm_count: 0,
    reserved: 0,
    realms: [const {
        PlixRealm {
            root: 0,
            epoch: 0,
            pulse_count: 0,
        }
    }; 64],
};

#[no_mangle]
pub static mut plix_last_boot_arch: u64 = 0;
#[no_mangle]
pub static mut plix_boot_pulses: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn plix_kernel_main(boot_info: *const PlixBootInfo) {
    plix_auralattice_init(core::ptr::addr_of_mut!(FABRIC));

    let first_pulse = PlixPulse {
        source: 0,
        target: 0,
        verb: (*boot_info).arch as u64,
        payload: [
            (*boot_info).hart_or_cpu,
            (*boot_info).firmware_pointer as u64,
            0,
            0,
        ],
    };

    let _ = plix_auralattice_emit(core::ptr::addr_of_mut!(FABRIC), &first_pulse);
    plix_last_boot_arch = (*boot_info).arch as u64;
    plix_boot_pulses = FABRIC.realms[0].pulse_count;

    plix_driver_init_arch((*boot_info).arch as u64);
    plix_cli_boot();
    plix_halt_forever();
}
