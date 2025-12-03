#![allow(dead_code)]

use core::arch::asm;

/// Enable interrupts (set MIE bit in mstatus)
#[inline]
pub fn enable_irqs() {
    unsafe {
        asm!("csrsi mstatus, 0x8");
    }
}

/// Disable interrupts (clear MIE bit in mstatus)
#[inline]
pub fn disable_irqs() {
    unsafe {
        asm!("csrci mstatus, 0x8");
    }
}

/// Check if interrupts are disabled
#[inline]
pub fn irqs_disabled() -> bool {
    let mstatus: usize;
    unsafe {
        asm!("csrr {}, mstatus", out(reg) mstatus);
    }
    (mstatus & 0x8) == 0
}

/// Wait for interrupts using WFI instruction
#[inline]
pub fn wait_for_ints() {
    if !irqs_disabled() {
        unsafe {
            asm!("wfi");
        }
    }
}
