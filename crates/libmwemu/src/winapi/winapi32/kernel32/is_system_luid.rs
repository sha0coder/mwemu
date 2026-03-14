use crate::constants;
use crate::emu;

pub fn IsSystemLUID(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsSystemLUID");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
