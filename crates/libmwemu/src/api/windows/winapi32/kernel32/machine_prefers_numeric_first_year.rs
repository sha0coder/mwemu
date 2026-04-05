use crate::constants;
use crate::emu;

pub fn MachinePrefersNumericFirstYear(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!MachinePrefersNumericFirstYear");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
