pub(crate) mod csr;

cfg_if::cfg_if! {
    if #[cfg(feature = "hext")] {
        mod hext;
        use hext as vender;
    }
}

pub use vender::{has_hardware_support, ArchPerCpuState};
