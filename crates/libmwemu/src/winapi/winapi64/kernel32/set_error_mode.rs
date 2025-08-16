
use crate::emu;

pub fn SetErrorMode(emu: &mut emu::Emu) {
    let mode = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!SetErrorMode 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        mode,
        emu.colors.nc
    );

    emu.regs_mut().rax = 0;
}