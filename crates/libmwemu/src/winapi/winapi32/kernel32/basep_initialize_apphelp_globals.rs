use crate::constants;
use crate::emu;

pub fn BasepInitializeApphelpGlobals(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepInitializeApphelpGlobals");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
