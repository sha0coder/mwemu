use crate::constants;
use crate::emu;

pub fn WerpCheckOkToRegister(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!WerpCheckOkToRegister");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
