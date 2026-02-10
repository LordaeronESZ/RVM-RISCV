mod boot;

pub mod instructions;
pub mod timer;
pub mod trap;
pub mod uart;

pub mod hext;


pub fn init_early() {
    uart::init();
}

pub fn init() {
    trap::init();
    timer::init();
}
