mod csr;
mod structs;

pub use structs::{HextRegion, MachineISA, MachineISAFlags, PhysFrame};

use crate::hv::error::RvmResult;

pub fn has_hardware_support() -> bool {
    MachineISA::read().contains(MachineISAFlags::H)
}

pub struct HextPerCpuState {
    hext_region: HextRegion,
}

impl HextPerCpuState {
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
