#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(panic_info_message, alloc_error_handler)]

#[macro_use]
extern crate log;

#[macro_use]
mod logging;

mod riscv64;
mod config;
mod hv;
mod mm;
mod timer;

#[cfg(not(test))]
mod lang_items;

use core::sync::atomic::{AtomicBool, Ordering};

static INIT_OK: AtomicBool = AtomicBool::new(false);

const LOGO: &str = r"
 ______     ____  __       ____  ___ ____   ______     __
|  _ \ \   / /  \/  |     |  _ \|_ _/ ___| / ___\ \   / /
| |_) \ \ / /| |\/| |_____| |_) || |\___ \| |    \ \ / /
|  _ < \ V / | |  | |_____|  _ < | | ___) | |___  \ V /
|_| \_\ \_/  |_|  |_|     |_| \_\___|____/ \____|  \_/

";

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

pub fn init_ok() -> bool {
    INIT_OK.load(Ordering::SeqCst)
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    clear_bss();
    riscv64::init_early();
    println!("{}", LOGO);
    println!(
        "\
        arch = {}\n\
        build_mode = {}\n\
        log_level = {}\n\
        ",
        option_env!("ARCH").unwrap_or(""),
        option_env!("MODE").unwrap_or(""),
        option_env!("LOG").unwrap_or(""),
    );

    mm::init_heap_early();
    logging::init();
    info!("Logging is enabled.");

    riscv64::init();
    mm::init();
    INIT_OK.store(true, Ordering::SeqCst);
    println!("Initialization completed.\n");

    hv::run();
    println!("Run OK!");

    riscv64::instructions::enable_irqs();
    loop {
        riscv64::instructions::wait_for_ints();
    }
}
