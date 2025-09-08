use crate::{constants, emu};

pub fn GetUserDefaultLCID(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetUserDefaultLCID");
    emu.regs_mut().rax = constants::LOCALE_USER_DEFAULT;
}
