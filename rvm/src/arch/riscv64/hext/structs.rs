use bit_field::BitField;
use bitflags::bitflags;

use crate::arch::csr::{Csr, CsrReadWrite};
use crate::mm::PhysFrame;
use crate::{HostPhysAddr, RvmHal, RvmResult};

#[derive(Debug)]
pub struct HextRegion<H: RvmHal> {
    frame: PhysFrame<H>,
}

impl<H: RvmHal> HextRegion<H> {
    pub const unsafe fn uninit() -> Self {
        Self {
            frame: PhysFrame::uninit(),
        }
    }

    pub fn new(revision_id: u32, shadow_indicator: bool) -> RvmResult<Self> {
        let frame = PhysFrame::alloc_zero()?;
        unsafe {
            (*(frame.as_mut_ptr() as *mut u32))
                .set_bits(0..=30, revision_id)
                .set_bit(31, shadow_indicator);
        }
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
        MachineISAFlags::from_bits_truncate(unsafe {
            Self::read_raw()
        })
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