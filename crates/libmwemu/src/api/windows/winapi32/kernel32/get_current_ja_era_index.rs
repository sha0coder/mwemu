use crate::constants;
use crate::emu;

pub fn GetCurrentJaEraIndex(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentJaEraIndex");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
