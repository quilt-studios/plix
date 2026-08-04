use core::ffi::c_char;

extern "C" {
    fn plix_console_putc(value: c_char);
}

#[no_mangle]
pub unsafe extern "C" fn plix_console_write(text: *const c_char) {
    let mut at = text;
    while *at != 0 {
        plix_console_putc(*at);
        at = at.add(1);
    }
}
