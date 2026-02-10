use bit_field::BitField;
use bitflags::bitflags;

use crate::riscv64::hext::csr::{Csr, CsrReadWrite};
use crate::hv::RvmResult;
use crate::hv::HostPhysAddr;
use crate::mm::frame;
use crate::rvm_err_type;

/// A 4K-sized contiguous physical memory page, it will deallocate the page
/// automatically on drop.
#[derive(Debug)]
pub struct PhysFrame {
    start_paddr: HostPhysAddr,
}

impl PhysFrame {
    pub fn alloc() -> RvmResult<Self> {
        let start_paddr = unsafe { frame::alloc_page() }
            .ok_or_else(|| rvm_err_type!(OutOfMemory, "allocate physical frame failed"))?;
        assert_ne!(start_paddr, 0);
        debug!("[RVM] allocated PhysFrame({:#x})", start_paddr);
        Ok(Self { start_paddr })
    }

    pub fn alloc_zero() -> RvmResult<Self> {
        let mut f = Self::alloc()?;
        f.fill(0);
        Ok(f)
    }

    pub const unsafe fn uninit() -> Self {
        Self { start_paddr: 0 }
    }

    pub fn start_paddr(&self) -> HostPhysAddr {
        self.start_paddr
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        crate::mm::address::phys_to_virt(self.start_paddr) as *mut u8
    }

    pub fn fill(&mut self, byte: u8) {
        unsafe { core::ptr::write_bytes(self.as_mut_ptr(), byte, crate::mm::PAGE_SIZE) }
    }
}

impl Drop for PhysFrame {
    fn drop(&mut self) {
        if self.start_paddr > 0 {
            unsafe { frame::dealloc_page(self.start_paddr) };
            debug!("[RVM] deallocated PhysFrame({:#x})", self.start_paddr);
        }
    }
}

#[derive(Debug)]
pub struct HextRegion {
    frame: PhysFrame,
}

impl HextRegion {
    pub const unsafe fn uninit() -> Self {
        Self {
            frame: PhysFrame::uninit(),
        }
    }

    pub fn new() -> RvmResult<Self> {
        let frame = PhysFrame::alloc_zero()?;
        Ok(Self { frame })
    }

    pub fn phys_addr(&self) -> HostPhysAddr {
        self.frame.start_paddr()
    }
}

bitflags! {
    /// RISC-V Machine ISA flags
    /// We only care about H-extension for now
    pub struct MachineISAFlags: u64 {
        /// Hypervisor extension
        const H = 1 << 7;
    }
}

/// Machine ISA in RISC-V Processor
pub struct MachineISA;

impl CsrReadWrite for MachineISA {
    const CSR: Csr = Csr::MISA;
}

impl MachineISA {
    pub fn read() -> MachineISAFlags {
        MachineISAFlags::from_bits_truncate(unsafe { Self::read_raw() })
    }

    pub fn write(flags: MachineISAFlags) {
        let old_value = unsafe { Self::read_raw() };
        let reserved = old_value & !(MachineISAFlags::all().bits());
        let new_value = reserved | flags.bits();
        unsafe {
            Self::write_raw(new_value);
        }
    }
}
