use crate::constants;
use crate::emu;

pub fn timeGetTime(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!timeGetTime");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
