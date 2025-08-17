
use crate::emu;

pub fn GetThreadLocale(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetThreadLocale {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    //emu.regs_mut().rax = constants::LOCALE_USER_DEFAULT; // TODO: 0x400 LOCALE_USER_DEFAULT or 0x409?
    emu.regs_mut().rax = 0x409; // English (United States)
}