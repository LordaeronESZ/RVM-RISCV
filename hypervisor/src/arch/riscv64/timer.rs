//! RISC-V timer support

use spin::Mutex;

const TICKS_PER_SEC: u64 = 100;
const CLOCK_FREQ: u64 = 10_000_000; // 10MHz for QEMU

static TIMER_COUNT: Mutex<u64> = Mutex::new(0);

pub fn init() {
    // Timer initialization would be done here
    // For now, just a placeholder
}

pub fn tick() {
    let mut count = TIMER_COUNT.lock();
    *count += 1;
}

pub fn get_ticks() -> u64 {
    *TIMER_COUNT.lock()
}

pub fn current_ticks() -> u64 {
    *TIMER_COUNT.lock()
}

pub fn ticks_to_nanos(ticks: u64) -> u64 {
    ticks * 1_000_000_000 / CLOCK_FREQ
}
