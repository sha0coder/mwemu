use crate::constants;
use crate::emu;

pub fn GetUserDefaultUILanguage(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetUserDefaultUILanguage (0x0409 en_US)");
    emu.regs_mut().rax = constants::EN_US_LOCALE as u64;
}
