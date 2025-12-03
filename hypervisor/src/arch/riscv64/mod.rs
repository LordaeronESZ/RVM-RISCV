mod boot;

pub mod instructions;
pub mod timer;
pub mod uart;

pub use uart as console;

pub fn init_early() {
    uart::init();
}

pub fn init() {
    timer::init();
}
