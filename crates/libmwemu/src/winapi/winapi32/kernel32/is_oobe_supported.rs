use crate::constants;
use crate::emu;

pub fn IsOOBESupported(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsOOBESupported");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
