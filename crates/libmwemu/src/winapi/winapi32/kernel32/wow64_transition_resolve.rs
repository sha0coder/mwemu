use crate::constants;
use crate::emu;

pub fn Wow64TransitionResolve(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!Wow64TransitionResolve");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
