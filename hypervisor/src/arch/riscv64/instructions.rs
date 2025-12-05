#![allow(dead_code)]

use core::arch::asm;

// ============================================================================
// Interrupt Control
// ============================================================================

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

// ============================================================================
// Interrupt Enable Bits
// ============================================================================

/// Enable machine software interrupt
#[inline]
pub fn enable_msie() {
    unsafe {
        asm!("csrsi mie, {}", const 0x8);  // MSIE = bit 3
    }
}

/// Enable machine timer interrupt
#[inline]
pub fn enable_mtie() {
    let mie = read_mie();
    write_mie(mie | 0x80);  // MTIE = bit 7
}

/// Enable machine external interrupt
#[inline]
pub fn enable_meie() {
    let mie = read_mie();
    write_mie(mie | 0x800);  // MEIE = bit 11
}

/// Disable machine software interrupt
#[inline]
pub fn disable_msie() {
    unsafe {
        asm!("csrci mie, {}", const 0x8);
    }
}

/// Disable machine timer interrupt
#[inline]
pub fn disable_mtie() {
    let mie = read_mie();
    write_mie(mie & !0x80);
}

/// Disable machine external interrupt
#[inline]
pub fn disable_meie() {
    let mie = read_mie();
    write_mie(mie & !0x800);
}

// ============================================================================
// System Control
// ============================================================================

/// Read machine status register
#[inline]
pub fn read_mstatus() -> usize {
    let mstatus: usize;
    unsafe {
        asm!("csrr {}, mstatus", out(reg) mstatus);
    }
    mstatus
}

/// Write machine status register
#[inline]
pub fn write_mstatus(value: usize) {
    unsafe {
        asm!("csrw mstatus, {}", in(reg) value);
    }
}

/// Read machine cause register
#[inline]
pub fn read_mcause() -> usize {
    let mcause: usize;
    unsafe {
        asm!("csrr {}, mcause", out(reg) mcause);
    }
    mcause
}

/// Read machine trap value register
#[inline]
pub fn read_mtval() -> usize {
    let mtval: usize;
    unsafe {
        asm!("csrr {}, mtval", out(reg) mtval);
    }
    mtval
}

/// Read machine trap program counter
#[inline]
pub fn read_mepc() -> usize {
    let mepc: usize;
    unsafe {
        asm!("csrr {}, mepc", out(reg) mepc);
    }
    mepc
}

/// Read machine interrupt enable register
#[inline]
pub fn read_mie() -> usize {
    let mie: usize;
    unsafe {
        asm!("csrr {}, mie", out(reg) mie);
    }
    mie
}

/// Write machine interrupt enable register
#[inline]
pub fn write_mie(value: usize) {
    unsafe {
        asm!("csrw mie, {}", in(reg) value);
    }
}

// ============================================================================
// Hart (Hardware Thread) Control
// ============================================================================

/// Read hart ID
#[inline]
pub fn read_hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("csrr {}, mhartid", out(reg) hart_id);
    }
    hart_id
}
