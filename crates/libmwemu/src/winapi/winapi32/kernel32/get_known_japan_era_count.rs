use crate::constants;
use crate::emu;

pub fn GetKnownJapanEraCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetKnownJapanEraCount");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
