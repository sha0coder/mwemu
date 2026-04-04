use crate::constants;
use crate::emu;

pub fn GetCorruptDetectionState(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCorruptDetectionState");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
