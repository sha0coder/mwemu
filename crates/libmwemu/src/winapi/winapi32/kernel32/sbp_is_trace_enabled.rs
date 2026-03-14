use crate::constants;
use crate::emu;

pub fn SbpIsTraceEnabled(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!SbpIsTraceEnabled");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
