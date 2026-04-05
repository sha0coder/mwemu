use crate::constants;
use crate::emu;

pub fn UnregisterApplicationRestart(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!UnregisterApplicationRestart");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
