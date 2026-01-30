use crate::constants;
use crate::emu;

pub fn WerpInitPEBStore(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!WerpInitPEBStore");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
