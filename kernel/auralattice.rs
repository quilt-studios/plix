use core::ffi::c_int;

const PLIX_AURALATTICE_MAGIC: u64 = 0x4155_5241_4c41_5454;
const PLIX_MAX_REALMS: usize = 64;

#[repr(C)]
pub struct PlixPulse {
    source: u64,
    target: u64,
    verb: u64,
    payload: [u64; 4],
}

#[repr(C)]
pub struct PlixRealm {
    root: u64,
    epoch: u64,
    pulse_count: u64,
}

#[repr(C)]
pub struct PlixAuralattice {
    magic: u64,
    realm_count: u32,
    reserved: u32,
    realms: [PlixRealm; PLIX_MAX_REALMS],
}

#[no_mangle]
pub unsafe extern "C" fn plix_auralattice_init(fabric: *mut PlixAuralattice) {
    (*fabric).magic = PLIX_AURALATTICE_MAGIC;
    (*fabric).realm_count = 1;
    (*fabric).reserved = 0;
    for i in 0..PLIX_MAX_REALMS {
        (*fabric).realms[i].root = if i == 0 { 1 } else { 0 };
        (*fabric).realms[i].epoch = 0;
        (*fabric).realms[i].pulse_count = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn plix_auralattice_emit(
    fabric: *mut PlixAuralattice,
    pulse: *const PlixPulse,
) -> c_int {
    if (*fabric).magic != PLIX_AURALATTICE_MAGIC || (*pulse).target >= PLIX_MAX_REALMS as u64 {
        return -1;
    }

    let realm = &mut (*fabric).realms[(*pulse).target as usize];
    if realm.root == 0 {
        realm.root = (*pulse).target + 1;
        (*fabric).realm_count += 1;
    }

    realm.epoch += 1;
    realm.pulse_count += 1;
    0
}
