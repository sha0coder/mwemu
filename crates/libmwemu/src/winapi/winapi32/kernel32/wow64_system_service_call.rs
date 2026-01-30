use crate::constants;
use crate::emu;

pub fn Wow64SystemServiceCall(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!Wow64SystemServiceCall");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
