//! RISC-V trap (interrupt/exception) handler
//!
//! Handles machine-mode traps including interrupts and exceptions.
#![allow(dead_code)]

use core::arch::asm;
use core::arch::global_asm;
use log::{info, warn};

use crate::arch::instructions;
use crate::arch::timer;

// Declare the trap handler assembly function
extern "C" {
    fn trap_handler();
}

global_asm!(
    r#"
.section .text.trap
.align 4
.globl trap_handler
trap_handler:
    // Save all general-purpose registers
    addi sp, sp, -32*8

    // Save x1-x31 (x0 is hardwired to 0, no need to save)
    sd x1,  1*8(sp)
    sd x2,  2*8(sp)
    sd x3,  3*8(sp)
    sd x4,  4*8(sp)
    sd x5,  5*8(sp)
    sd x6,  6*8(sp)
    sd x7,  7*8(sp)
    sd x8,  8*8(sp)
    sd x9,  9*8(sp)
    sd x10, 10*8(sp)
    sd x11, 11*8(sp)
    sd x12, 12*8(sp)
    sd x13, 13*8(sp)
    sd x14, 14*8(sp)
    sd x15, 15*8(sp)
    sd x16, 16*8(sp)
    sd x17, 17*8(sp)
    sd x18, 18*8(sp)
    sd x19, 19*8(sp)
    sd x20, 20*8(sp)
    sd x21, 21*8(sp)
    sd x22, 22*8(sp)
    sd x23, 23*8(sp)
    sd x24, 24*8(sp)
    sd x25, 25*8(sp)
    sd x26, 26*8(sp)
    sd x27, 27*8(sp)
    sd x28, 28*8(sp)
    sd x29, 29*8(sp)
    sd x30, 30*8(sp)
    sd x31, 31*8(sp)

    // Call the Rust trap handler
    mv a0, sp
    call trap_handler_rust

    // Restore all general-purpose registers
    ld x1,  1*8(sp)
    ld x2,  2*8(sp)
    ld x3,  3*8(sp)
    ld x4,  4*8(sp)
    ld x5,  5*8(sp)
    ld x6,  6*8(sp)
    ld x7,  7*8(sp)
    ld x8,  8*8(sp)
    ld x9,  9*8(sp)
    ld x10, 10*8(sp)
    ld x11, 11*8(sp)
    ld x12, 12*8(sp)
    ld x13, 13*8(sp)
    ld x14, 14*8(sp)
    ld x15, 15*8(sp)
    ld x16, 16*8(sp)
    ld x17, 17*8(sp)
    ld x18, 18*8(sp)
    ld x19, 19*8(sp)
    ld x20, 20*8(sp)
    ld x21, 21*8(sp)
    ld x22, 22*8(sp)
    ld x23, 23*8(sp)
    ld x24, 24*8(sp)
    ld x25, 25*8(sp)
    ld x26, 26*8(sp)
    ld x27, 27*8(sp)
    ld x28, 28*8(sp)
    ld x29, 29*8(sp)
    ld x30, 30*8(sp)
    ld x31, 31*8(sp)

    addi sp, sp, 32*8
    mret
"#
);

/// Interrupt causes
pub const INTERRUPT_U_SOFT: usize = 0;
pub const INTERRUPT_S_SOFT: usize = 1;
pub const INTERRUPT_M_SOFT: usize = 3;
pub const INTERRUPT_U_TIMER: usize = 4;
pub const INTERRUPT_S_TIMER: usize = 5;
pub const INTERRUPT_M_TIMER: usize = 7;
pub const INTERRUPT_U_EXT: usize = 8;
pub const INTERRUPT_S_EXT: usize = 9;
pub const INTERRUPT_M_EXT: usize = 11;

/// Exception causes
pub const EXCEPTION_INST_ADDR_MISALIGNED: usize = 0;
pub const EXCEPTION_INST_ACCESS_FAULT: usize = 1;
pub const EXCEPTION_ILLEGAL_INST: usize = 2;
pub const EXCEPTION_BREAKPOINT: usize = 3;
pub const EXCEPTION_LOAD_ADDR_MISALIGNED: usize = 4;
pub const EXCEPTION_LOAD_ACCESS_FAULT: usize = 5;
pub const EXCEPTION_STORE_ADDR_MISALIGNED: usize = 6;
pub const EXCEPTION_STORE_ACCESS_FAULT: usize = 7;
pub const EXCEPTION_ECALL_FROM_U: usize = 8;
pub const EXCEPTION_ECALL_FROM_S: usize = 9;
pub const EXCEPTION_ECALL_FROM_M: usize = 11;
pub const EXCEPTION_INST_PAGE_FAULT: usize = 12;
pub const EXCEPTION_LOAD_PAGE_FAULT: usize = 13;
pub const EXCEPTION_STORE_PAGE_FAULT: usize = 15;

/// Trap context (register state)
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapFrame {
    // general-purpose registers
    pub zero: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}

/// Rust trap handler
#[no_mangle]
pub extern "C" fn trap_handler_rust(_tf: *const TrapFrame) {
    unsafe {
        trace!("trap : {:#x?}", *_tf);
    }
    let mcause = instructions::read_mcause();
    let is_interrupt = (mcause & 0x8000_0000_0000_0000) != 0;
    let cause_code = mcause & 0x7fff_ffff_ffff_ffff;

    if is_interrupt {
        handle_interrupt(cause_code);
    } else {
        handle_exception(cause_code);
    }
}

/// Handle interrupts
#[inline]
fn handle_interrupt(cause: usize) {
    match cause {
        INTERRUPT_M_TIMER => {
            timer::handle_timer_interrupt();
        }
        INTERRUPT_M_SOFT => {
            // Software interrupt - can be used for IPI
        }
        INTERRUPT_M_EXT => {
            // External interrupt
        }
        _ => {
            warn!("Unknown interrupt: {}", cause);
        }
    }
}

/// Handle exceptions
#[inline]
fn handle_exception(cause: usize) {
    match cause {
        EXCEPTION_ECALL_FROM_M => {
            // Handle system calls
            warn!("System call from machine mode (not expected)");
        }
        EXCEPTION_ILLEGAL_INST => {
            warn!("Illegal instruction at {:#x}", instructions::read_mepc());
        }
        _ => {
            warn!(
                "Unhandled exception {}: mepc={:#x}, mtval={:#x}",
                cause,
                instructions::read_mepc(),
                instructions::read_mtval()
            );
        }
    }
}

/// Set up the trap handler vector
pub fn init() {
    // Disable interrupts before setting up trap handler
    instructions::disable_irqs();

    // Set trap handler address
    unsafe {
        asm!("csrw mtvec, {}", in(reg) trap_handler as usize);
    }

    // Enable timer interrupt
    instructions::enable_mtie();

    // Enable global interrupts
    instructions::enable_irqs();

    info!("Trap handler initialized at {:#x}", trap_handler as usize);
}
