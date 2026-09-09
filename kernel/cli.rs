use core::ffi::{c_char, c_int};

const OUT_CAP: usize = 512;
const MAX_PATH: usize = 128;
const MAX_CHILDREN: usize = 8;
const AUTH_MAX_NAME: usize = 16;

#[repr(C)]
pub struct PlixCli {
    output: [c_char; OUT_CAP],
    output_len: usize,
}
#[repr(C)]
struct PlixSession {
    user: *const PlixUser,
    authenticated: c_int,
}
#[repr(C)]
struct PlixUser {
    name: *const c_char,
    password_hash: u64,
    salt: u64,
    power_user: c_int,
}
#[repr(C)]
struct PlixEveryfile {
    root: *mut PlixEveryfileNode,
    current: *mut PlixEveryfileNode,
    active_user: *mut PlixEveryfileNode,
}
#[repr(C)]
struct PlixEveryfileNode {
    _private: [u8; 0],
}
#[repr(C)]
struct PlixDriverOps {
    write: Option<unsafe extern "C" fn(*const c_char)>,
    read: Option<unsafe extern "C" fn(*mut c_char, usize) -> c_int>,
}
#[repr(C)]
struct PlixDriver {
    name: *const c_char,
    bus: u32,
    device_class: u32,
    compatible: u64,
    base: u64,
    size: u64,
    ops: *const PlixDriverOps,
    initialized: c_int,
}

extern "C" {
    fn plix_console_putc(value: c_char);
    fn plix_everyfile_init(fs: *mut PlixEveryfile);
    fn plix_everyfile_enter_user(fs: *mut PlixEveryfile, name: *const c_char) -> c_int;
    fn plix_everyfile_goto(fs: *mut PlixEveryfile, path: *const c_char) -> c_int;
    fn plix_everyfile_show(
        fs: *const PlixEveryfile,
        names: *mut *const c_char,
        capacity: usize,
    ) -> usize;
    fn plix_everyfile_path(
        fs: *const PlixEveryfile,
        buffer: *mut c_char,
        capacity: usize,
    ) -> usize;
    fn plix_auth_init(session: *mut PlixSession);
    fn plix_auth_login(
        session: *mut PlixSession,
        name: *const c_char,
        password: *const c_char,
    ) -> c_int;
    fn plix_auth_check_password(session: *const PlixSession, password: *const c_char) -> c_int;
    fn plix_auth_user_name(session: *const PlixSession) -> *const c_char;
    fn plix_auth_is_power_user(session: *const PlixSession) -> c_int;
    fn plix_driver_count() -> usize;
    fn plix_driver_get(index: usize) -> *const PlixDriver;
    fn plix_driver_bus_name(bus: u32) -> *const c_char;
    fn plix_driver_class_name(device_class: u32) -> *const c_char;
}

static mut FS: PlixEveryfile = PlixEveryfile {
    root: core::ptr::null_mut(),
    current: core::ptr::null_mut(),
    active_user: core::ptr::null_mut(),
};
static mut SESSION: PlixSession = PlixSession {
    user: core::ptr::null(),
    authenticated: 0,
};
static mut BOOT_CLI: PlixCli = PlixCli {
    output: [0; OUT_CAP],
    output_len: 0,
};

unsafe fn streq(mut left: *const c_char, mut right: *const c_char) -> bool {
    while *left != 0 && *right != 0 {
        if *left != *right {
            return false;
        }
        left = left.add(1);
        right = right.add(1);
    }
    *left == 0 && *right == 0
}

unsafe fn append_char(cli: *mut PlixCli, value: c_char) {
    if (*cli).output_len + 1 < OUT_CAP {
        (*cli).output[(*cli).output_len] = value;
        (*cli).output_len += 1;
        (*cli).output[(*cli).output_len] = 0;
        plix_console_putc(value);
    }
}

unsafe fn append(cli: *mut PlixCli, text: *const c_char) {
    let mut i = 0;
    while *text.add(i) != 0 {
        append_char(cli, *text.add(i));
        i += 1;
    }
}

unsafe fn append_current_path(cli: *mut PlixCli) {
    let mut path = [0 as c_char; MAX_PATH];
    let _ = plix_everyfile_path(core::ptr::addr_of!(FS), path.as_mut_ptr(), path.len());
    append(cli, path.as_ptr());
}

unsafe fn skip_spaces(mut text: *const c_char) -> *const c_char {
    while *text == b' ' as c_char || *text == b'\t' as c_char {
        text = text.add(1);
    }
    text
}

unsafe fn token_len(text: *const c_char) -> usize {
    let mut len = 0;
    while *text.add(len) != 0
        && *text.add(len) != b' ' as c_char
        && *text.add(len) != b'\t' as c_char
    {
        len += 1;
    }
    len
}

unsafe fn token_is(token: *const c_char, len: usize, expected: &'static [u8]) -> bool {
    let mut i = 0;
    while i < len && expected[i] != 0 && *token.add(i) == expected[i] as c_char {
        i += 1;
    }
    i == len && expected[i] == 0
}

unsafe fn copy_token(dst: &mut [c_char], src: *const c_char, len: usize) {
    let copied = if len < dst.len() - 1 {
        len
    } else {
        dst.len() - 1
    };
    for i in 0..copied {
        dst[i] = *src.add(i);
    }
    dst[copied] = 0;
}

#[no_mangle]
pub unsafe extern "C" fn plix_cli_init(cli: *mut PlixCli) {
    plix_everyfile_init(core::ptr::addr_of_mut!(FS));
    plix_auth_init(core::ptr::addr_of_mut!(SESSION));
    let _ = plix_everyfile_enter_user(
        core::ptr::addr_of_mut!(FS),
        plix_auth_user_name(core::ptr::addr_of!(SESSION)),
    );
    (*cli).output_len = 0;
    (*cli).output[0] = 0;
    append(cli, b"plix cli ready at \0".as_ptr().cast());
    append_current_path(cli);
    append(cli, b"\nuse help for commands\n\0".as_ptr().cast());
}

#[no_mangle]
pub unsafe extern "C" fn plix_cli_execute(cli: *mut PlixCli, line: *const c_char) -> c_int {
    if cli.is_null() || line.is_null() {
        return -1;
    }

    let command = skip_spaces(line);
    let command_len = token_len(command);
    let argument = skip_spaces(command.add(command_len));

    if token_is(command, command_len, b"help\0") || token_is(command, command_len, b"?\0") {
        append(
            cli,
            b"help/?  pwd/pw  who  goto/gt <path>  show/sw  drivers/drv  login <user> <pass>  pudo <pass> <command>  status/st\n\0"
                .as_ptr()
                .cast(),
        );
        return 0;
    }

    if token_is(command, command_len, b"pwd\0") || token_is(command, command_len, b"pw\0") {
        append_current_path(cli);
        append(cli, b"\n\0".as_ptr().cast());
        return 0;
    }

    if token_is(command, command_len, b"status\0") || token_is(command, command_len, b"st\0") {
        append(cli, b"user=\0".as_ptr().cast());
        append(cli, plix_auth_user_name(core::ptr::addr_of!(SESSION)));
        append(cli, b" path=\0".as_ptr().cast());
        append_current_path(cli);
        append(cli, b" drivers=\0".as_ptr().cast());
        let count = plix_driver_count();
        if count == 0 {
            append(cli, b"0\n\0".as_ptr().cast());
        } else {
            for _ in 0..count {
                append(cli, b"+\0".as_ptr().cast());
            }
            append(cli, b"\n\0".as_ptr().cast());
        }
        return 0;
    }

    if token_is(command, command_len, b"login\0") {
        let name = argument;
        let name_len = token_len(name);
        let password = skip_spaces(name.add(name_len));
        if name_len == 0 || *password == 0 {
            append(cli, b"login braucht user und passwort\n\0".as_ptr().cast());
            return -1;
        }
        let mut user_name = [0 as c_char; AUTH_MAX_NAME];
        copy_token(&mut user_name, name, name_len);
        if plix_auth_login(
            core::ptr::addr_of_mut!(SESSION),
            user_name.as_ptr(),
            password,
        ) != 0
            || plix_everyfile_enter_user(core::ptr::addr_of_mut!(FS), user_name.as_ptr()) != 0
        {
            append(cli, b"login fehlgeschlagen\n\0".as_ptr().cast());
            return -1;
        }
        append(cli, b"angemeldet als \0".as_ptr().cast());
        append(cli, plix_auth_user_name(core::ptr::addr_of!(SESSION)));
        append(cli, b"\n\0".as_ptr().cast());
        return 0;
    }

    if token_is(command, command_len, b"who\0") {
        append(cli, plix_auth_user_name(core::ptr::addr_of!(SESSION)));
        append(cli, b"\n\0".as_ptr().cast());
        return 0;
    }

    if token_is(command, command_len, b"goto\0") || token_is(command, command_len, b"gt\0") {
        if *argument == 0 {
            append(cli, b"goto braucht ein ziel\n\0".as_ptr().cast());
            return -1;
        }
        if plix_everyfile_goto(core::ptr::addr_of_mut!(FS), argument) != 0 {
            append(cli, b"ziel nicht gefunden\n\0".as_ptr().cast());
            return -1;
        }
        append(cli, b"jetzt in \0".as_ptr().cast());
        append_current_path(cli);
        append(cli, b"\n\0".as_ptr().cast());
        return 0;
    }

    if token_is(command, command_len, b"show\0") || token_is(command, command_len, b"sw\0") {
        let mut names = [core::ptr::null(); MAX_CHILDREN];
        let count = plix_everyfile_show(core::ptr::addr_of!(FS), names.as_mut_ptr(), MAX_CHILDREN);
        let stop = if count < MAX_CHILDREN { count } else { MAX_CHILDREN };
        for name in names.iter().take(stop) {
            append(cli, *name);
            append(cli, b"\n\0".as_ptr().cast());
        }
        return 0;
    }

    if token_is(command, command_len, b"drivers\0") || token_is(command, command_len, b"drv\0") {
        let count = plix_driver_count();
        if count == 0 {
            append(cli, b"keine linux treiber geladen\n\0".as_ptr().cast());
            return 0;
        }
        for i in 0..count {
            let driver = plix_driver_get(i);
            if !driver.is_null() {
                append(cli, (*driver).name);
                append(cli, b" \0".as_ptr().cast());
                append(cli, plix_driver_bus_name((*driver).bus));
                append(cli, b" \0".as_ptr().cast());
                append(cli, plix_driver_class_name((*driver).device_class));
                append(cli, b"\n\0".as_ptr().cast());
            }
        }
        return 0;
    }

    if token_is(command, command_len, b"pudo\0") {
        let password = argument;
        let password_len = token_len(password);
        let power_command = skip_spaces(password.add(password_len));
        let mut pudo_password = [0 as c_char; AUTH_MAX_NAME];
        copy_token(&mut pudo_password, password, password_len);
        if plix_auth_is_power_user(core::ptr::addr_of!(SESSION)) == 0
            || password_len == 0
            || plix_auth_check_password(core::ptr::addr_of!(SESSION), pudo_password.as_ptr()) == 0
        {
            append(cli, b"pudo verweigert\n\0".as_ptr().cast());
            return -1;
        }
        append(cli, b"pudo erlaubt: \0".as_ptr().cast());
        append(
            cli,
            if *power_command == 0 {
                b"<leer>\0".as_ptr().cast()
            } else {
                power_command
            },
        );
        append(cli, b"\n\0".as_ptr().cast());
        return 0;
    }

    if streq(command, b"\0".as_ptr().cast()) {
        return 0;
    }

    append(cli, b"unbekannter befehl; nutze help\n\0".as_ptr().cast());
    -1
}

#[no_mangle]
pub unsafe extern "C" fn plix_cli_output(cli: *const PlixCli) -> *const c_char {
    if cli.is_null() {
        core::ptr::null()
    } else {
        (*cli).output.as_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn plix_cli_boot() {
    plix_cli_init(core::ptr::addr_of_mut!(BOOT_CLI));
    let _ = plix_cli_execute(
        core::ptr::addr_of_mut!(BOOT_CLI),
        b"status\0".as_ptr().cast(),
    );
    let _ = plix_cli_execute(
        core::ptr::addr_of_mut!(BOOT_CLI),
        b"drivers\0".as_ptr().cast(),
    );
    let _ = plix_cli_execute(core::ptr::addr_of_mut!(BOOT_CLI), b"show\0".as_ptr().cast());
}
