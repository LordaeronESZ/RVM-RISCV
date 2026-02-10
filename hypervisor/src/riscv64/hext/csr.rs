use core::arch::asm;

/// RISC-V control and status registers.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Csr {
    MISA = 0x301,
}

impl Csr {
    /// Read 64 bits csr register.
    #[inline(always)]
    pub unsafe fn read(self) -> u64 {
        let value: u64;
        match self {
            Csr::MISA => {
                unsafe {
                    asm!("csrr {}, misa", out(reg) value);
                }
            }
        }
        value
    }

    /// Write 64 bits to csr register.
    ///
    /// # Safety
    ///
    /// The caller must ensure that this write operation has no unsafe side
    /// effects.
    #[inline(always)]
    pub unsafe fn write(self, value: u64) {
        match self {
            Csr::MISA => {
                asm!("csrw misa, {}", in(reg) value);
            }
        }
    }
}

pub(super) trait CsrReadWrite {
    const CSR: Csr;

    unsafe fn read_raw() -> u64 {
        Self::CSR.read()
    }

    unsafe fn write_raw(flags: u64) {
        Self::CSR.write(flags);
    }
}
