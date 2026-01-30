use crate::constants;
use crate::emu;

pub fn IsFusionFullySupported(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsFusionFullySupported");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
