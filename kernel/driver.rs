use core::ffi::{c_char, c_int};

const PLIX_ARCH_X86_64: u64 = 0x8664;
const PLIX_ARCH_AARCH64: u64 = 0xaa64;
const PLIX_ARCH_RISCV64: u64 = 0x5064;
const PLIX_DRIVER_MAX_DEVICES: usize = 8;
const PLIX_DRIVER_BUS_MMIO: u32 = 1;
const PLIX_DRIVER_BUS_PIO: u32 = 2;
const PLIX_DRIVER_CLASS_CONSOLE: u32 = 0;
const PLIX_COMPAT_X86_16550: u64 = 0x7838_3631_3635_3530;
const PLIX_COMPAT_ARM_PL011: u64 = 0x6172_6d70_6c30_3131;
const PLIX_COMPAT_RV_16550: u64 = 0x7276_3136_3535_3000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PlixDriverOps {
    write: Option<unsafe extern "C" fn(*const c_char)>,
    read: Option<unsafe extern "C" fn(*mut c_char, usize) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PlixDriver {
    name: *const c_char,
    bus: u32,
    device_class: u32,
    compatible: u64,
    base: u64,
    size: u64,
    ops: *const PlixDriverOps,
    initialized: c_int,
}

unsafe impl Sync for PlixDriverOps {}
unsafe impl Sync for PlixDriver {}

extern "C" {
    fn plix_console_write(text: *const c_char);
}

static mut REGISTRY: [PlixDriver; PLIX_DRIVER_MAX_DEVICES] = [const {
    PlixDriver {
        name: core::ptr::null(),
        bus: 0,
        device_class: 0,
        compatible: 0,
        base: 0,
        size: 0,
        ops: core::ptr::null(),
        initialized: 0,
    }
}; PLIX_DRIVER_MAX_DEVICES];
static mut REGISTRY_COUNT: usize = 0;

static X86_NAME: &[u8] = b"linux-8250-serial\0";
static ARM_NAME: &[u8] = b"linux-amba-pl011\0";
static RV_NAME: &[u8] = b"linux-8250-mmio\0";
static PLATFORM_NAME: &[u8] = b"platform\0";
static MMIO_NAME: &[u8] = b"mmio\0";
static PIO_NAME: &[u8] = b"pio\0";
static CONSOLE_NAME: &[u8] = b"console\0";
static BLOCK_NAME: &[u8] = b"block\0";
static NET_NAME: &[u8] = b"net\0";
static INPUT_NAME: &[u8] = b"input\0";
static UNKNOWN_NAME: &[u8] = b"unknown\0";

unsafe extern "C" fn console_write(text: *const c_char) {
    plix_console_write(text);
}

unsafe extern "C" fn no_input(_buffer: *mut c_char, _capacity: usize) -> c_int {
    0
}

static CONSOLE_OPS: PlixDriverOps = PlixDriverOps {
    write: Some(console_write),
    read: Some(no_input),
};

#[no_mangle]
pub unsafe extern "C" fn plix_driver_registry_init() {
    REGISTRY_COUNT = 0;
    for i in 0..PLIX_DRIVER_MAX_DEVICES {
        REGISTRY[i].name = core::ptr::null();
        REGISTRY[i].initialized = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn plix_driver_register(driver: *const PlixDriver) -> c_int {
    if driver.is_null()
        || (*driver).name.is_null()
        || (*driver).ops.is_null()
        || REGISTRY_COUNT >= PLIX_DRIVER_MAX_DEVICES
    {
        return -1;
    }
    REGISTRY[REGISTRY_COUNT] = *driver;
    REGISTRY[REGISTRY_COUNT].initialized = 1;
    REGISTRY_COUNT += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn plix_driver_count() -> usize {
    REGISTRY_COUNT
}

#[no_mangle]
pub unsafe extern "C" fn plix_driver_get(index: usize) -> *const PlixDriver {
    if index < REGISTRY_COUNT {
        &REGISTRY[index]
    } else {
        core::ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn plix_driver_find_console() -> *const PlixDriver {
    for i in 0..REGISTRY_COUNT {
        if REGISTRY[i].device_class == PLIX_DRIVER_CLASS_CONSOLE && REGISTRY[i].initialized != 0 {
            return &REGISTRY[i];
        }
    }
    core::ptr::null()
}

unsafe fn register_arch_driver(
    name: &'static [u8],
    bus: u32,
    compatible: u64,
    base: u64,
    size: u64,
) {
    let driver = PlixDriver {
        name: name.as_ptr().cast(),
        bus,
        device_class: PLIX_DRIVER_CLASS_CONSOLE,
        compatible,
        base,
        size,
        ops: &CONSOLE_OPS,
        initialized: 0,
    };
    let _ = plix_driver_register(&driver);
}

#[no_mangle]
pub unsafe extern "C" fn plix_driver_init_arch(arch: u64) {
    plix_driver_registry_init();
    if arch == PLIX_ARCH_X86_64 {
        register_arch_driver(
            X86_NAME,
            PLIX_DRIVER_BUS_PIO,
            PLIX_COMPAT_X86_16550,
            0x3f8,
            8,
        );
    } else if arch == PLIX_ARCH_AARCH64 {
        register_arch_driver(
            ARM_NAME,
            PLIX_DRIVER_BUS_MMIO,
            PLIX_COMPAT_ARM_PL011,
            0x0900_0000,
            0x1000,
        );
    } else if arch == PLIX_ARCH_RISCV64 {
        register_arch_driver(
            RV_NAME,
            PLIX_DRIVER_BUS_MMIO,
            PLIX_COMPAT_RV_16550,
            0x1000_0000,
            0x100,
        );
    }
}

#[no_mangle]
pub extern "C" fn plix_driver_bus_name(bus: u32) -> *const c_char {
    match bus {
        0 => PLATFORM_NAME.as_ptr().cast(),
        1 => MMIO_NAME.as_ptr().cast(),
        2 => PIO_NAME.as_ptr().cast(),
        _ => UNKNOWN_NAME.as_ptr().cast(),
    }
}

#[no_mangle]
pub extern "C" fn plix_driver_class_name(device_class: u32) -> *const c_char {
    match device_class {
        0 => CONSOLE_NAME.as_ptr().cast(),
        1 => BLOCK_NAME.as_ptr().cast(),
        2 => NET_NAME.as_ptr().cast(),
        3 => INPUT_NAME.as_ptr().cast(),
        _ => UNKNOWN_NAME.as_ptr().cast(),
    }
}
