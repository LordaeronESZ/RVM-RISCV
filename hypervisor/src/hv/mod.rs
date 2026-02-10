pub mod error;

pub use error::{RvmError, RvmResult};
pub use crate::mm::{GuestPhysAddr, GuestVirtAddr, HostPhysAddr, HostVirtAddr};
pub use crate::riscv64::hext::{HextPerCpuState, has_hardware_support};

/// Host per-CPU states to run the guest.
pub struct RvmPerCpu {
    _cpu_id: usize,
    hext: HextPerCpuState,
}

impl RvmPerCpu {
    /// Create an uninitialized instance.
    pub fn new(cpu_id: usize) -> Self {
        Self {
            _cpu_id: cpu_id,
            hext: HextPerCpuState::new(),
        }
    }

    /// Whether the current CPU has hardware virtualization enabled.
    pub fn is_enabled(&self) -> bool {
        self.hext.is_enabled()
    }

    /// Enable hardware virtualization on the current CPU.
    pub fn hardware_enable(&mut self) -> RvmResult {
        self.hext.hardware_enable()
    }

    /// Disable hardware virtualization on the current CPU.
    pub fn hardware_disable(&mut self) -> RvmResult {
        self.hext.hardware_disable()
    }
}

impl Drop for RvmPerCpu {
    fn drop(&mut self) {
        if self.is_enabled() {
            self.hardware_disable().unwrap();
        }
    }
}

pub fn run() {
    println!("Starting virtualization...");
    println!("Hardware support: {:?}", has_hardware_support());

    let mut percpu = RvmPerCpu::new(0);
    let res = percpu.hardware_enable();
    println!("Hardware enable: {:?}", res);
}
