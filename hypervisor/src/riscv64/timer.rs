//! RISC-V timer support
//!
//! For QEMU without OpenSBI, we directly write to memory-mapped timer registers.
//! In QEMU virt machine, the CLINT (Core Local Interruptor) is at 0x02000000.

use spin::Mutex;

/// Timer ticks per second
pub const TICKS_PER_SEC: u64 = 100;
/// CPU clock frequency in Hz (10MHz for QEMU)
pub const CLOCK_FREQ: u64 = 10_000_000;
/// Interval between timer interrupts in CPU cycles
const TIMER_INTERVAL: u64 = CLOCK_FREQ / TICKS_PER_SEC;

/// CLINT base address in QEMU virt machine
const CLINT_BASE: usize = 0x0200_0000;
/// CLINT timer register offset for hart 0
const CLINT_MTIME_OFFSET: usize = 0xBFF8;
/// CLINT mtimecmp register offset for hart 0
const CLINT_MTIMECMP_OFFSET: usize = 0x4000;

static TIMER_COUNT: Mutex<u64> = Mutex::new(0);

/// Initialize the timer by setting the first timer interrupt
pub fn init() {
    let deadline = read_time() + TIMER_INTERVAL;
    write_mtimecmp(deadline);
}

/// Handle timer interrupt - increment counter and set next interrupt
pub fn handle_timer_interrupt() {
    let mut count = TIMER_COUNT.lock();
    *count += 1;
    drop(count);

    // Set next timer interrupt
    let deadline = read_time() + TIMER_INTERVAL;
    write_mtimecmp(deadline);
}

/// Get the current number of ticks
pub fn current_ticks() -> u64 {
    *TIMER_COUNT.lock()
}

/// Convert ticks to nanoseconds
pub fn ticks_to_nanos(ticks: u64) -> u64 {
    ticks * 1_000_000_000 / CLOCK_FREQ
}

/// Read the current time from CLINT mtime register
#[inline]
fn read_time() -> u64 {
    unsafe {
        core::ptr::read_volatile((CLINT_BASE + CLINT_MTIME_OFFSET) as *const u64)
    }
}

/// Write to CLINT mtimecmp register to set next timer interrupt
#[inline]
fn write_mtimecmp(deadline: u64) {
    unsafe {
        core::ptr::write_volatile((CLINT_BASE + CLINT_MTIMECMP_OFFSET) as *mut u64, deadline)
    }
}
