
use crate::emu;

pub fn GetConsoleCP(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetConsoleCP {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 0x00000409;
}