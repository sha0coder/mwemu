use crate::constants;
use crate::emu;

pub fn GetSystemDEPPolicy(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetSystemDEPPolicy");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
