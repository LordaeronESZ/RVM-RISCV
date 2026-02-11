use crate::csr;
use crate::sbi;

/// SBI call context
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SbiContext {
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
}

pub fn init() {
    csr::write_mtvec(trap_entry as usize);
    csr::enable_m_interrupts();
}

#[unsafe(naked)]
#[no_mangle]
unsafe extern "C" fn trap_entry() {
    core::arch::naked_asm!(
        "addi sp, sp, -256",
        "sd ra, 0(sp)",
        "sd gp, 8(sp)",
        "sd tp, 16(sp)",
        "sd t0, 24(sp)",
        "sd t1, 32(sp)",
        "sd t2, 40(sp)",
        "sd s0, 48(sp)",
        "sd s1, 56(sp)",
        "sd a0, 64(sp)",
        "sd a1, 72(sp)",
        "sd a2, 80(sp)",
        "sd a3, 88(sp)",
        "sd a4, 96(sp)",
        "sd a5, 104(sp)",
        "sd a6, 112(sp)",
        "sd a7, 120(sp)",
        "sd s2, 128(sp)",
        "sd s3, 136(sp)",
        "sd s4, 144(sp)",
        "sd s5, 152(sp)",
        "sd s6, 160(sp)",
        "sd s7, 168(sp)",
        "sd s8, 176(sp)",
        "sd s9, 184(sp)",
        "sd s10, 192(sp)",
        "sd s11, 200(sp)",
        "sd t3, 208(sp)",
        "sd t4, 216(sp)",
        "sd t5, 224(sp)",
        "sd t6, 232(sp)",

        "call trap_handler",

        "ld ra, 0(sp)",
        "ld gp, 8(sp)",
        "ld tp, 16(sp)",
        "ld t0, 24(sp)",
        "ld t1, 32(sp)",
        "ld t2, 40(sp)",
        "ld s0, 48(sp)",
        "ld s1, 56(sp)",
        "ld a0, 64(sp)",
        "ld a1, 72(sp)",
        "ld a2, 80(sp)",
        "ld a3, 88(sp)",
        "ld a4, 96(sp)",
        "ld a5, 104(sp)",
        "ld a6, 112(sp)",
        "ld a7, 120(sp)",
        "ld s2, 128(sp)",
        "ld s3, 136(sp)",
        "ld s4, 144(sp)",
        "ld s5, 152(sp)",
        "ld s6, 160(sp)",
        "ld s7, 168(sp)",
        "ld s8, 176(sp)",
        "ld s9, 184(sp)",
        "ld s10, 192(sp)",
        "ld s11, 200(sp)",
        "ld t3, 208(sp)",
        "ld t4, 216(sp)",
        "ld t5, 224(sp)",
        "ld t6, 232(sp)",
        "addi sp, sp, 256",

        "mret",
    );
}

/// Interrupt causes
pub const INTERRUPT_M_SOFT: usize = 3;
pub const INTERRUPT_M_TIMER: usize = 7;
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

#[no_mangle]
extern "C" fn trap_handler() {
    let mcause = csr::read_mcause();
    let mepc = csr::read_mepc();
    let mtval = csr::read_mtval();

    let is_interrupt = (mcause & 0x8000_0000_0000_0000) != 0;
    let cause_code = mcause & 0x7fff_ffff_ffff_ffff;

    if is_interrupt {
        handle_interrupt(cause_code);
    } else {
        handle_exception(cause_code, mepc, mtval);
    }
}

fn handle_interrupt(cause: usize) {
    match cause {
        INTERRUPT_M_SOFT => handle_m_software_interrupt(),
        INTERRUPT_M_TIMER => handle_m_timer_interrupt(),
        INTERRUPT_M_EXT => handle_m_external_interrupt(),
        _ => {
            loop {
                core::hint::spin_loop();
            }
        }
    }
}

fn handle_exception(code: usize, mepc: usize, mtval: usize) {
    match code {
        EXCEPTION_ECALL_FROM_U => handle_ecall_from_u_mode(mepc),
        EXCEPTION_ECALL_FROM_S => handle_ecall_from_s_mode(mepc),
        EXCEPTION_ECALL_FROM_M => handle_ecall_from_m_mode(mepc),
        _ => {
            loop {
                core::hint::spin_loop();
            }
        }
    }
}

fn handle_m_timer_interrupt() {
    csr::disable_timer_interrupt();

    // Set next timer interrupt
    let time = csr::read_time();
    let next_time = time.wrapping_add(10_000_000); // 100ms
    csr::write_timecmp(next_time);

    csr::enable_timer_interrupt();
}

fn handle_m_external_interrupt() {
    loop {
        core::hint::spin_loop();
    }
}

fn handle_m_software_interrupt() {
    loop {
        core::hint::spin_loop();
    }
}

fn handle_ecall_from_u_mode(mepc: usize) {
    loop {
        core::hint::spin_loop();
    }
}

fn handle_ecall_from_s_mode(mepc: usize) {
    loop {
        core::hint::spin_loop();
    }
}

fn handle_ecall_from_m_mode(mepc: usize) {
    let mut context = SbiContext {
        a0: read_register(10), // a0
        a1: read_register(11), // a1
        a2: read_register(12), // a2
        a3: read_register(13), // a3
        a4: read_register(14), // a4
        a5: read_register(15), // a5
        a6: read_register(16), // a6
        a7: read_register(17), // a7
    };

    let result = sbi::handle_sbi_call(&mut context);

    write_register(10, context.a0); // a0
    write_register(11, context.a1); // a1

    csr::write_mepc(mepc + 4);

    if let Err(e) = result {
        loop {
            core::hint::spin_loop();
        }
    }
}

fn read_register(reg: usize) -> usize {
    let value: usize;
    unsafe {
        match reg {
            10 => core::arch::asm!("mv {}, a0", out(reg) value),
            11 => core::arch::asm!("mv {}, a1", out(reg) value),
            12 => core::arch::asm!("mv {}, a2", out(reg) value),
            13 => core::arch::asm!("mv {}, a3", out(reg) value),
            14 => core::arch::asm!("mv {}, a4", out(reg) value),
            15 => core::arch::asm!("mv {}, a5", out(reg) value),
            16 => core::arch::asm!("mv {}, a6", out(reg) value),
            17 => core::arch::asm!("mv {}, a7", out(reg) value),
            _ => panic!("Invalid register"),
        }
    }
    value
}

fn write_register(reg: usize, value: usize) {
    unsafe {
        match reg {
            10 => core::arch::asm!("mv a0, {}", in(reg) value),
            11 => core::arch::asm!("mv a1, {}", in(reg) value),
            _ => panic!("Invalid register"),
        }
    }
}