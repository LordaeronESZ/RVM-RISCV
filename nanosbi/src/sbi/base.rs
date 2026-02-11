use crate::csr;
use crate::sbi::{SbiContext, SbiError, SbiFunction, SbiResult};

pub fn handle_base_call(function: usize, context: &mut SbiContext) -> SbiResult<()> {
    match function {
        x if x == SbiFunction::GetSpecVersion as usize => {
            context.a0 = SbiError::Success as usize;
            context.a1 = 0x2; // SBI v2.0
            Ok(())
        }
        x if x == SbiFunction::GetImplId as usize => {
            context.a0 = SbiError::Success as usize;
            context.a1 = 0x4E534249; // "NSBI" - nanoSBI
            Ok(())
        }
        x if x == SbiFunction::GetImplVersion as usize => {
            context.a0 = SbiError::Success as usize;
            context.a1 = 0x1; // version 1.0
            Ok(())
        }
        x if x == SbiFunction::ProbeExtension as usize => {
            let extension_id = context.a0;
            let supported = probe_extension(extension_id);
            context.a0 = SbiError::Success as usize;
            context.a1 = if supported { 1 } else { 0 };
            Ok(())
        }
        x if x == SbiFunction::GetMhartid as usize => {
            let hartid = csr::read_hart_id();
            context.a0 = SbiError::Success as usize;
            context.a1 = hartid;
            Ok(())
        }
        x if x == SbiFunction::GetMvendorid as usize => {
            let mvendorid = csr::read_mvendorid();
            context.a0 = SbiError::Success as usize;
            context.a1 = mvendorid;
            Ok(())
        }
        x if x == SbiFunction::GetMarchid as usize => {
            let marchid = csr::read_marchid();
            context.a0 = SbiError::Success as usize;
            context.a1 = marchid;
            Ok(())
        }
        x if x == SbiFunction::GetMimpid as usize => {
            let mimpid = csr::read_mimpid();
            context.a0 = SbiError::Success as usize;
            context.a1 = mimpid;
            Ok(())
        }
        _ => {
            context.a0 = SbiError::NotSupported as usize;
            context.a1 = 0;
            Ok(())
        }
    }
}

fn probe_extension(extension_id: usize) -> bool {
    use crate::sbi::SbiExtension;

    match extension_id {
        x if x == SbiExtension::Base as usize => true,
        x if x == SbiExtension::Timer as usize => true,
        x if x == SbiExtension::HardwareFeatures as usize => true,
        _ => false,
    }
}