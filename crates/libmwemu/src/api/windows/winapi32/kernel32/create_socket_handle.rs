use crate::constants;
use crate::emu;

pub fn CreateSocketHandle(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateSocketHandle");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
