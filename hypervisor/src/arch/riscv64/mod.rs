mod boot;

pub mod instructions;
pub mod timer;
pub mod trap;
pub mod uart;


pub fn init_early() {
    uart::init();
}

pub fn init() {
    trap::init();
    timer::init();
}
