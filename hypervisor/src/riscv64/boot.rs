// RISC-V boot code
// For QEMU, the bootloader sets:
// - a0 = hart_id
// - a1 = device_tree_blob
// - pc = 0x80000000 (default load address)

use core::arch::global_asm;

global_asm!(
    r#"
.section .text.boot
.globl _start
_start:
    // Load the kernel stack pointer
    la sp, boot_stack_top

    // Jump to main
    j main
"#
);
