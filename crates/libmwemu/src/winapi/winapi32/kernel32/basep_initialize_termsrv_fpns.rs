use crate::constants;
use crate::emu;

pub fn BasepInitializeTermsrvFpns(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepInitializeTermsrvFpns");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
