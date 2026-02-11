use crate::csr;
use crate::sbi::{SbiContext, SbiError, SbiFunction, SbiResult};

pub fn handle_timer_call(function: usize, context: &mut SbiContext) -> SbiResult<()> {
    match function {
        x if x == SbiFunction::SetTimer as usize => {
            let time = context.a0;
            set_timer(time);
            context.a0 = SbiError::Success as usize;
            context.a1 = 0;
            Ok(())
        }
        _ => {
            context.a0 = SbiError::NotSupported as usize;
            context.a1 = 0;
            Ok(())
        }
    }
}

fn set_timer(time: usize) {
    csr::write_timecmp(time);
    csr::enable_timer_interrupt();
}