use core::ffi::{c_char, c_int};

const MAX_CHILDREN: usize = 8;
const MAX_PATH: usize = 128;
const DIRECTORY: u32 = 0;
const TEXT: u32 = 1;

#[repr(C)]
pub struct PlixEveryfileNode {
    name: *const c_char,
    kind: u32,
    text: *const c_char,
    parent: *mut PlixEveryfileNode,
    children: [*mut PlixEveryfileNode; MAX_CHILDREN],
    child_count: usize,
}

#[repr(C)]
pub struct PlixEveryfile {
    root: *mut PlixEveryfileNode,
    current: *mut PlixEveryfileNode,
    active_user: *mut PlixEveryfileNode,
}

unsafe impl Sync for PlixEveryfileNode {}

const NULL_NODE: *mut PlixEveryfileNode = core::ptr::null_mut();
static mut ROOT: PlixEveryfileNode = node();
static mut USERS: PlixEveryfileNode = node();
static mut ROOT_USER: PlixEveryfileNode = node();
static mut ROOT_MAIN_DIR: PlixEveryfileNode = node();
static mut ROOT_HOUSE_DIR: PlixEveryfileNode = node();
static mut ROOT_SYSTEM_NOTE: PlixEveryfileNode = node();
static mut ROOT_WELCOME_NOTE: PlixEveryfileNode = node();
static mut GUEST_USER: PlixEveryfileNode = node();
static mut GUEST_MAIN_DIR: PlixEveryfileNode = node();
static mut GUEST_HOUSE_DIR: PlixEveryfileNode = node();
static mut GUEST_SYSTEM_NOTE: PlixEveryfileNode = node();
static mut GUEST_WELCOME_NOTE: PlixEveryfileNode = node();

const fn node() -> PlixEveryfileNode {
    PlixEveryfileNode {
        name: core::ptr::null(),
        kind: DIRECTORY,
        text: core::ptr::null(),
        parent: core::ptr::null_mut(),
        children: [NULL_NODE; MAX_CHILDREN],
        child_count: 0,
    }
}

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

unsafe fn node_init(
    node: *mut PlixEveryfileNode,
    name: &'static [u8],
    kind: u32,
    text: *const c_char,
) {
    (*node).name = name.as_ptr().cast();
    (*node).kind = kind;
    (*node).text = text;
    (*node).parent = core::ptr::null_mut();
    (*node).child_count = 0;
    (*node).children = [NULL_NODE; MAX_CHILDREN];
}

unsafe fn attach(parent: *mut PlixEveryfileNode, child: *mut PlixEveryfileNode) {
    if (*parent).child_count < MAX_CHILDREN {
        (*parent).children[(*parent).child_count] = child;
        (*parent).child_count += 1;
        (*child).parent = parent;
    }
}

#[no_mangle]
pub unsafe extern "C" fn plix_everyfile_init(fs: *mut PlixEveryfile) {
    node_init(
        core::ptr::addr_of_mut!(ROOT),
        b"\0",
        DIRECTORY,
        core::ptr::null(),
    );
    node_init(
        core::ptr::addr_of_mut!(USERS),
        b"users\0",
        DIRECTORY,
        core::ptr::null(),
    );
    node_init(
        core::ptr::addr_of_mut!(ROOT_USER),
        b"root\0",
        DIRECTORY,
        core::ptr::null(),
    );
    node_init(
        core::ptr::addr_of_mut!(ROOT_MAIN_DIR),
        b"main\0",
        DIRECTORY,
        core::ptr::null(),
    );
    node_init(
        core::ptr::addr_of_mut!(ROOT_HOUSE_DIR),
        b"house\0",
        DIRECTORY,
        core::ptr::null(),
    );
    node_init(
        core::ptr::addr_of_mut!(ROOT_SYSTEM_NOTE),
        b"system\0",
        TEXT,
        b"main ersetzt etc fuer globale Konfiguration\0"
            .as_ptr()
            .cast(),
    );
    node_init(
        core::ptr::addr_of_mut!(ROOT_WELCOME_NOTE),
        b"welcome\0",
        TEXT,
        b"house ersetzt home fuer Benutzerdaten\0".as_ptr().cast(),
    );
    node_init(
        core::ptr::addr_of_mut!(GUEST_USER),
        b"guest\0",
        DIRECTORY,
        core::ptr::null(),
    );
    node_init(
        core::ptr::addr_of_mut!(GUEST_MAIN_DIR),
        b"main\0",
        DIRECTORY,
        core::ptr::null(),
    );
    node_init(
        core::ptr::addr_of_mut!(GUEST_HOUSE_DIR),
        b"house\0",
        DIRECTORY,
        core::ptr::null(),
    );
    node_init(
        core::ptr::addr_of_mut!(GUEST_SYSTEM_NOTE),
        b"system\0",
        TEXT,
        b"guest main konfiguration\0".as_ptr().cast(),
    );
    node_init(
        core::ptr::addr_of_mut!(GUEST_WELCOME_NOTE),
        b"welcome\0",
        TEXT,
        b"guest house daten\0".as_ptr().cast(),
    );
    attach(
        core::ptr::addr_of_mut!(ROOT),
        core::ptr::addr_of_mut!(USERS),
    );
    attach(
        core::ptr::addr_of_mut!(USERS),
        core::ptr::addr_of_mut!(ROOT_USER),
    );
    attach(
        core::ptr::addr_of_mut!(USERS),
        core::ptr::addr_of_mut!(GUEST_USER),
    );
    attach(
        core::ptr::addr_of_mut!(ROOT_USER),
        core::ptr::addr_of_mut!(ROOT_MAIN_DIR),
    );
    attach(
        core::ptr::addr_of_mut!(ROOT_USER),
        core::ptr::addr_of_mut!(ROOT_HOUSE_DIR),
    );
    attach(
        core::ptr::addr_of_mut!(ROOT_MAIN_DIR),
        core::ptr::addr_of_mut!(ROOT_SYSTEM_NOTE),
    );
    attach(
        core::ptr::addr_of_mut!(ROOT_HOUSE_DIR),
        core::ptr::addr_of_mut!(ROOT_WELCOME_NOTE),
    );
    attach(
        core::ptr::addr_of_mut!(GUEST_USER),
        core::ptr::addr_of_mut!(GUEST_MAIN_DIR),
    );
    attach(
        core::ptr::addr_of_mut!(GUEST_USER),
        core::ptr::addr_of_mut!(GUEST_HOUSE_DIR),
    );
    attach(
        core::ptr::addr_of_mut!(GUEST_MAIN_DIR),
        core::ptr::addr_of_mut!(GUEST_SYSTEM_NOTE),
    );
    attach(
        core::ptr::addr_of_mut!(GUEST_HOUSE_DIR),
        core::ptr::addr_of_mut!(GUEST_WELCOME_NOTE),
    );
    (*fs).root = core::ptr::addr_of_mut!(ROOT);
    (*fs).active_user = core::ptr::addr_of_mut!(ROOT_USER);
    (*fs).current = core::ptr::addr_of_mut!(ROOT_USER);
}

#[no_mangle]
pub unsafe extern "C" fn plix_everyfile_current(
    fs: *const PlixEveryfile,
) -> *const PlixEveryfileNode {
    (*fs).current
}

unsafe fn find_child(
    node: *mut PlixEveryfileNode,
    name: *const c_char,
    length: usize,
) -> *mut PlixEveryfileNode {
    for i in 0..(*node).child_count {
        let candidate = (*(*node).children[i]).name;
        let mut j = 0;
        while j < length && *candidate.add(j) != 0 && *candidate.add(j) == *name.add(j) {
            j += 1;
        }
        if j == length && *candidate.add(j) == 0 {
            return (*node).children[i];
        }
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn plix_everyfile_goto(fs: *mut PlixEveryfile, path: *const c_char) -> c_int {
    let mut cursor = if *path == b'/' as c_char {
        (*fs).root
    } else {
        (*fs).current
    };
    let mut start = if *path == b'/' as c_char { 1 } else { 0 };
    if *path.add(start) == 0 {
        (*fs).current = cursor;
        return 0;
    }
    while *path.add(start) != 0 {
        let mut end = start;
        while *path.add(end) != b'/' as c_char && *path.add(end) != 0 {
            end += 1;
        }
        if end == start || (end - start == 1 && *path.add(start) == b'.' as c_char) {
        } else if end - start == 2
            && *path.add(start) == b'.' as c_char
            && *path.add(start + 1) == b'.' as c_char
        {
            if !(*cursor).parent.is_null() {
                cursor = (*cursor).parent;
            }
        } else {
            let next = find_child(cursor, path.add(start), end - start);
            if next.is_null() || (*next).kind != DIRECTORY {
                return -1;
            }
            cursor = next;
        }
        start = if *path.add(end) == b'/' as c_char {
            end + 1
        } else {
            end
        };
    }
    (*fs).current = cursor;
    0
}

#[no_mangle]
pub unsafe extern "C" fn plix_everyfile_show(
    fs: *const PlixEveryfile,
    names: *mut *const c_char,
    capacity: usize,
) -> usize {
    let count = (*(*fs).current).child_count;
    let copied = if count < capacity { count } else { capacity };
    for i in 0..copied {
        *names.add(i) = (*(*(*fs).current).children[i]).name;
    }
    count
}

#[no_mangle]
pub unsafe extern "C" fn plix_everyfile_path(
    fs: *const PlixEveryfile,
    buffer: *mut c_char,
    capacity: usize,
) -> usize {
    let mut stack: [*const PlixEveryfileNode; MAX_PATH / 2] = [core::ptr::null(); MAX_PATH / 2];
    let mut depth = 0;
    let mut cursor = (*fs).current as *const PlixEveryfileNode;
    while !cursor.is_null() && depth < MAX_PATH / 2 {
        stack[depth] = cursor;
        depth += 1;
        cursor = (*cursor).parent;
    }
    let mut written = 0;
    if capacity > 0 {
        *buffer.add(written) = b'/' as c_char;
        written += 1;
    }
    while depth > 0 && written < capacity {
        depth -= 1;
        let name = (*stack[depth]).name;
        if *name == 0 {
            continue;
        }
        let mut i = 0;
        while *name.add(i) != 0 && written + 1 < capacity {
            *buffer.add(written) = *name.add(i);
            written += 1;
            i += 1;
        }
        if depth > 0 && written + 1 < capacity {
            *buffer.add(written) = b'/' as c_char;
            written += 1;
        }
    }
    if capacity > 0 {
        if written >= capacity {
            written = capacity - 1;
        }
        *buffer.add(written) = 0;
    }
    written
}

#[no_mangle]
pub unsafe extern "C" fn plix_everyfile_name_is_reserved(name: *const c_char) -> c_int {
    (streq(name, b"etc\0".as_ptr().cast()) || streq(name, b"home\0".as_ptr().cast())) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn plix_everyfile_enter_user(
    fs: *mut PlixEveryfile,
    name: *const c_char,
) -> c_int {
    for i in 0..USERS.child_count {
        let candidate = (*USERS.children[i]).name;
        if streq(candidate, name) {
            (*fs).active_user = USERS.children[i];
            (*fs).current = USERS.children[i];
            return 0;
        }
    }
    -1
}
