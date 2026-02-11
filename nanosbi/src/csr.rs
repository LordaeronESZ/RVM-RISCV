use core::arch::asm;

// ============================================================================
// Hart Identification
// ============================================================================

/// Read hart ID from mhartid CSR
#[inline]
pub fn read_hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("csrr {}, mhartid", out(reg) hart_id);
    }
    hart_id
}

// ============================================================================
// Machine ISA
// ============================================================================

/// Read Machine ISA register
#[inline]
pub fn read_misa() -> usize {
    let misa: usize;
    unsafe {
        asm!("csrr {}, misa", out(reg) misa);
    }
    misa
}

// ============================================================================
// Machine Vendor/Architecture/Implementation IDs
// ============================================================================

/// Read machine vendor ID
#[inline]
pub fn read_mvendorid() -> usize {
    let vendorid: usize;
    unsafe {
        asm!("csrr {}, mvendorid", out(reg) vendorid);
    }
    vendorid
}

/// Read machine architecture ID
#[inline]
pub fn read_marchid() -> usize {
    let marchid: usize;
    unsafe {
        asm!("csrr {}, marchid", out(reg) marchid);
    }
    marchid
}

/// Read machine implementation ID
#[inline]
pub fn read_mimpid() -> usize {
    let mimpid: usize;
    unsafe {
        asm!("csrr {}, mimpid", out(reg) mimpid);
    }
    mimpid
}

// ============================================================================
// Timer
// ============================================================================

/// CLINT (Core-Local Interruptor) base address for QEMU virt machine
const CLINT_BASE: usize = 0x0200_0000;
/// mtimecmp offset for hart 0
const MTIMECMP_HART0: usize = 0x4000;
/// mtime offset
const MTIME: usize = 0xBFF8;

/// Read time CSR
#[inline]
pub fn read_time() -> usize {
    let time: usize;
    unsafe {
        asm!("csrr {}, time", out(reg) time);
    }
    time
}

/// Read timecmp (mtimecmp) memory-mapped register
#[inline]
pub fn read_timecmp() -> usize {
    unsafe {
        ((CLINT_BASE + MTIMECMP_HART0) as *const u64).read_volatile() as usize
    }
}

/// Write timecmp (mtimecmp) memory-mapped register
#[inline]
pub fn write_timecmp(value: usize) {
    unsafe {
        ((CLINT_BASE + MTIMECMP_HART0) as *mut u64).write_volatile(value as u64);
    }
}

// ============================================================================
// Machine Status (mstatus)
// ============================================================================

/// Enable machine-mode interrupts (set MIE bit in mstatus)
#[inline]
pub fn enable_m_interrupts() {
    unsafe {
        asm!("csrsi mstatus, 0x8"); // MIE = bit 3
    }
}

/// Disable machine-mode interrupts (clear MIE bit in mstatus)
#[inline]
pub fn disable_m_interrupts() {
    unsafe {
        asm!("csrci mstatus, 0x8"); // MIE = bit 3
    }
}

// ============================================================================
// Machine Interrupt Enable (mie)
// ============================================================================

/// Enable machine timer interrupt (set MTIE bit in mie)
#[inline]
pub fn enable_timer_interrupt() {
    let mie: usize;
    unsafe {
        asm!("csrr {}, mie", out(reg) mie);
        asm!("csrw mie, {}", in(reg) (mie | 0x80)); // MTIE = bit 7
    }
}

/// Disable machine timer interrupt (clear MTIE bit in mie)
#[inline]
pub fn disable_timer_interrupt() {
    let mie: usize;
    unsafe {
        asm!("csrr {}, mie", out(reg) mie);
        asm!("csrw mie, {}", in(reg) (mie & !0x80)); // MTIE = bit 7
    }
}

// ============================================================================
// Machine Trap Vector (mtvec)
// ============================================================================

/// Write machine trap vector base address
/// MODE is set to Direct (bits[1:0] = 0)
#[inline]
pub fn write_mtvec(value: usize) {
    let mtvec_value = value & !0x3; // Clear mode bits for Direct mode
    unsafe {
        asm!("csrw mtvec, {}", in(reg) mtvec_value);
    }
}

/// Read machine trap vector base address
/// Returns only the base address, not the mode
#[inline]
pub fn read_mtvec() -> usize {
    let mtvec: usize;
    unsafe {
        asm!("csrr {}, mtvec", out(reg) mtvec);
    }
    mtvec & !0x3 // Clear mode bits
}

// ============================================================================
// Machine Exception Handling
// ============================================================================

/// Read machine cause register
#[inline]
pub fn read_mcause() -> usize {
    let mcause: usize;
    unsafe {
        asm!("csrr {}, mcause", out(reg) mcause);
    }
    mcause
}

/// Read machine exception program counter
#[inline]
pub fn read_mepc() -> usize {
    let mepc: usize;
    unsafe {
        asm!("csrr {}, mepc", out(reg) mepc);
    }
    mepc
}

/// Write machine exception program counter
#[inline]
pub fn write_mepc(value: usize) {
    unsafe {
        asm!("csrw mepc, {}", in(reg) value);
    }
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

/// Write machine trap value register
#[inline]
pub fn write_mtval(value: usize) {
    unsafe {
        asm!("csrw mtval, {}", in(reg) value);
    }
}
