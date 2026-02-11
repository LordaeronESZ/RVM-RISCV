// RISC-V nanoSBI entry point
// For QEMU, the bootloader sets:
// - pc = 0x80000000 (default load address)

#![no_std]
#![no_main]

mod csr;
mod sbi;
mod trap;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    r#"
.section .text.entry
.globl _start
_start:
    // Set stack pointer
    la sp, stack_top

    // Clear BSS section
    la t0, sbss
    la t1, ebss
    bgeu t0, t1, clear_bss_done

clear_bss_loop:
    sd zero, (t0)
    addi t0, t0, 8
    bltu t0, t1, clear_bss_loop

clear_bss_done:
    // Jump to Rust entry point
    call _start_rust

    // Never return
    j .

.section .bss.stack
.align 4
stack_bottom:
    .space 0x1000  # 4KB stack space
stack_top:

.section .sbss
.align 4
sbss:

.section .ebss
.align 4
ebss:
"#
);

pub const HYPERVISOR_ENTRY: usize = 0x8002_0000;

#[no_mangle]
pub extern "C" fn _start_rust() -> ! {
    // Initialize trap handler
    trap::init();

    switch_to_hs_mode_simple();

    loop {
        core::hint::spin_loop();
    }
}

/// Switch to HS-mode
fn switch_to_hs_mode_simple() {
    use core::arch::asm;

    // Set hypervisor entry address
    unsafe {
        asm!("csrw mepc, {}", in(reg) HYPERVISOR_ENTRY);
    }

    // Configure mstatus register
    // MPP = 01 (HS-mode), MIE = 0
    let mstatus_value = 0x0000_0000_0000_0800;
    unsafe {
        asm!("csrw mstatus, {}", in(reg) mstatus_value);
    }

    // Disable paging
    unsafe {
        asm!("csrw satp, zero");
    }

    // Execute switching
    unsafe {
        asm!("mret");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}