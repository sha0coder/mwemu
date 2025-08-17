use crate::emu;

pub fn GetTickCount(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetTickCount {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = emu.tick as u64;
}