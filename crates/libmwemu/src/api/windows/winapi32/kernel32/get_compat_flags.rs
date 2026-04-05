use crate::constants;
use crate::emu;

pub fn GetCompatFlags(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCompatFlags");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
