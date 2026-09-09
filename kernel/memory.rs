use core::ffi::{c_int, c_void};

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut c_void, value: c_int, count: usize) -> *mut c_void {
    let bytes = dest.cast::<u8>();
    for i in 0..count {
        bytes.add(i).write(value as u8);
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(
    dest: *mut c_void,
    src: *const c_void,
    count: usize,
) -> *mut c_void {
    let out = dest.cast::<u8>();
    let input = src.cast::<u8>();
    for i in 0..count {
        out.add(i).write(input.add(i).read());
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(
    dest: *mut c_void,
    src: *const c_void,
    count: usize,
) -> *mut c_void {
    let out = dest.cast::<u8>();
    let input = src.cast::<u8>();

    if (out as usize) <= (input as usize) {
        for i in 0..count {
            out.add(i).write(input.add(i).read());
        }
    } else {
        let mut i = count;
        while i > 0 {
            i -= 1;
            out.add(i).write(input.add(i).read());
        }
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(left: *const c_void, right: *const c_void, count: usize) -> c_int {
    let a = left.cast::<u8>();
    let b = right.cast::<u8>();
    for i in 0..count {
        let av = a.add(i).read();
        let bv = b.add(i).read();
        if av != bv {
            return av as c_int - bv as c_int;
        }
    }
    0
}
