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