use crate::constants;
use crate::emu;

pub fn LoadAppInitDllsImplementation(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LoadAppInitDllsImplementation");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
