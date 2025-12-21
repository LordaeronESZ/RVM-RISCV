/// RISC-V control and status registers.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Csr {
    PLACE_HOLDER = 0x0,
}

impl Csr {
    /// Read 64 bits csr register.
    #[inline(always)]
    pub fn read(self) -> u64 {
        0
    }

    /// Write 64 bits to csr register.
    ///
    /// # Safety
    ///
    /// The caller must ensure that this write operation has no unsafe side
    /// effects.
    #[inline(always)]
    pub unsafe fn write(self, value: u64) {
        // TODO
    }
}

pub(super) trait CsrReadWrite {
    const CSR: Csr;

    fn read_raw() -> u64 {
        Self::CSR.read()
    }

    unsafe fn write_raw(flags: u64) {
        Self::CSR.write(flags);
    }
}
