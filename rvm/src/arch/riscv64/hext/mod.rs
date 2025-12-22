mod structs;

use crate::arch::csr::Csr;
use crate::arch::riscv64::hext::structs::{HextRegion, MachineISA, MachineISAFlags};
use crate::error::{RvmError, RvmResult};
use crate::hal::RvmHal;

pub use self::HextPerCpuState as ArchPerCpuState;

pub fn has_hardware_support() -> bool {
    let has_hext = MachineISA::read().contains(MachineISAFlags::H);
    has_hext
}

pub struct HextPerCpuState<H: RvmHal> {
    hext_region: HextRegion<H>,
}

impl<H: RvmHal> HextPerCpuState<H> {
    pub const fn new() -> Self {
        Self {
            hext_region: unsafe { HextRegion::uninit() },
        }
    }

    pub fn is_enabled(&self) -> bool {
        false
    }

    pub fn hardware_enable(&mut self) -> RvmResult {
        if !has_hardware_support() {
            return rvm_err!(Unsupported, "CPU does not support feature H-Ext");
        }

        info!("[RVM] successed to turn on H-Ext.");

        Ok(())
    }

    pub fn hardware_disable(&mut self) -> RvmResult {
        info!("[RVM] successed to turn off H-Ext.");

        Ok(())
    }
}
