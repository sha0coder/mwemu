use crate::emu;

pub fn GetACP(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetAcp {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 1252;
}