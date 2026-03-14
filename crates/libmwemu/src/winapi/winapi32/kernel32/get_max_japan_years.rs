use crate::constants;
use crate::emu;

pub fn GetMaxJapanYears(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaxJapanYears");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
