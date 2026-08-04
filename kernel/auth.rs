use core::ffi::{c_char, c_int};

const FNV_OFFSET: u64 = 1_469_598_103_934_665_603;
const FNV_PRIME: u64 = 1_099_511_628_211;

#[repr(C)]
pub struct PlixUser {
    name: *const c_char,
    password_hash: u64,
    salt: u64,
    power_user: c_int,
}

#[repr(C)]
pub struct PlixSession {
    user: *const PlixUser,
    authenticated: c_int,
}

unsafe impl Sync for PlixUser {}

static ROOT_NAME: &[u8] = b"root\0";
static GUEST_NAME: &[u8] = b"guest\0";
static NONE_NAME: &[u8] = b"none\0";

static USERS: [PlixUser; 2] = [
    PlixUser {
        name: ROOT_NAME.as_ptr().cast(),
        password_hash: 0x3684_f671_2bed_fc93,
        salt: 0x706c_6978_726f_6f74,
        power_user: 1,
    },
    PlixUser {
        name: GUEST_NAME.as_ptr().cast(),
        password_hash: 0xc824_e1f3_3535_3f6e,
        salt: 0x706c_6978_6775_6573,
        power_user: 0,
    },
];

unsafe fn c_str_eq(mut left: *const c_char, mut right: *const c_char) -> bool {
    while *left != 0 && *right != 0 {
        if *left != *right {
            return false;
        }
        left = left.add(1);
        right = right.add(1);
    }
    *left == 0 && *right == 0
}

fn hash_byte(hash: u64, value: u8) -> u64 {
    (hash ^ u64::from(value)).wrapping_mul(FNV_PRIME)
}

#[no_mangle]
pub unsafe extern "C" fn plix_auth_hash_password(
    name: *const c_char,
    password: *const c_char,
    salt: u64,
) -> u64 {
    let mut hash = FNV_OFFSET;
    let mut shift = 0;
    while shift < 64 {
        hash = hash_byte(hash, (salt >> shift) as u8);
        shift += 8;
    }

    let mut name_at = name;
    while *name_at != 0 {
        hash = hash_byte(hash, *name_at as u8);
        name_at = name_at.add(1);
    }

    hash = hash_byte(hash, b':');

    let mut password_at = password;
    while *password_at != 0 {
        hash = hash_byte(hash, *password_at as u8);
        password_at = password_at.add(1);
    }

    hash
}

#[no_mangle]
pub unsafe extern "C" fn plix_auth_init(session: *mut PlixSession) {
    (*session).user = &USERS[1];
    (*session).authenticated = 1;
}

#[no_mangle]
pub unsafe extern "C" fn plix_auth_login(
    session: *mut PlixSession,
    name: *const c_char,
    password: *const c_char,
) -> c_int {
    for user in &USERS {
        if c_str_eq(user.name, name)
            && user.password_hash == plix_auth_hash_password(name, password, user.salt)
        {
            (*session).user = user;
            (*session).authenticated = 1;
            return 0;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn plix_auth_check_password(
    session: *const PlixSession,
    password: *const c_char,
) -> c_int {
    let user = (*session).user;
    if user.is_null() {
        return 0;
    }
    ((*user).password_hash == plix_auth_hash_password((*user).name, password, (*user).salt))
        as c_int
}

#[no_mangle]
pub unsafe extern "C" fn plix_auth_user_name(session: *const PlixSession) -> *const c_char {
    let user = (*session).user;
    if user.is_null() {
        NONE_NAME.as_ptr().cast()
    } else {
        (*user).name
    }
}

#[no_mangle]
pub unsafe extern "C" fn plix_auth_is_power_user(session: *const PlixSession) -> c_int {
    let user = (*session).user;
    (!user.is_null() && (*session).authenticated != 0 && (*user).power_user != 0) as c_int
}
