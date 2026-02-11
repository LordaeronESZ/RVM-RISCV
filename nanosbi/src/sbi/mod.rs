pub mod base;
pub mod timer;

use crate::trap::SbiContext;
use crate::csr::read_misa;

/// SBI extension ID
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbiExtension {
    Base = 0x10,
    Timer = 0x54494D45,
    HardwareFeatures = 0x48574446,
}

/// SBI function ID
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbiFunction {
    // Base extension functions (0x10)
    GetSpecVersion = 0,
    GetImplId = 1,
    GetImplVersion = 2,
    ProbeExtension = 3,
    GetMhartid = 4,
    GetMvendorid = 5,
    GetMarchid = 6,
    GetMimpid = 7,

    // Timer extension functions (0x54494D45)
    SetTimer = 0x100,

    // Hardware feature extension functions (0x48574446)
    GetHardwareFeatures = 0x200,
}

/// SBI error code
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbiError {
    Success = 0,
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
}

/// SBI calling result
pub type SbiResult<T> = Result<T, SbiError>;

/// SBI handler
pub fn handle_sbi_call(context: &mut SbiContext) -> SbiResult<()> {
    let extension = context.a7;
    let function = context.a6;

    match extension {
        x if x == SbiExtension::Base as usize => base::handle_base_call(function, context),
        x if x == SbiExtension::Timer as usize => timer::handle_timer_call(function, context),
        x if x == SbiExtension::HardwareFeatures as usize => {
            handle_hardware_features_call(function, context)
        }
        _ => {
            context.a0 = SbiError::NotSupported as usize;
            context.a1 = 0;
            Ok(())
        }
    }
}

fn handle_hardware_features_call(function: usize, context: &mut SbiContext) -> SbiResult<()> {
    match function {
        x if x == SbiFunction::GetHardwareFeatures as usize => {
            let features = get_hardware_features();
            context.a0 = SbiError::Success as usize;
            context.a1 = features;
            Ok(())
        }
        _ => {
            context.a0 = SbiError::NotSupported as usize;
            context.a1 = 0;
            Ok(())
        }
    }
}

fn get_hardware_features() -> usize {
    let misa = read_misa();
    let mut features = 0;

    features
}