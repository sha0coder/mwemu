
use crate::emu;

pub fn SetThreadLocale(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!SetThreadLocale {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // TODO: do something
    emu.regs_mut().rax = 1;
}