#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[path = "auralattice.rs"]
mod auralattice;
#[path = "auth.rs"]
mod auth;
#[path = "cli.rs"]
mod cli;
#[path = "console.rs"]
mod console;
#[path = "driver.rs"]
mod driver;
#[path = "everyfile.rs"]
mod everyfile;
#[path = "main.rs"]
mod main;
