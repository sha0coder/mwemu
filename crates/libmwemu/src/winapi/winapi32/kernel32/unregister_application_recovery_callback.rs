use crate::constants;
use crate::emu;

pub fn UnregisterApplicationRecoveryCallback(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!UnregisterApplicationRecoveryCallback");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
