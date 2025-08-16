use crate::emu;
use crate::constants;

pub fn GetThreadUILanguage(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetThreadUILanguage (0x0409 en_US) {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = constants::EN_US_LOCALE as u64;
}