use crate::constants;
use crate::emu;

pub fn CptpQuirksInitOnce(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CptpQuirksInitOnce");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
