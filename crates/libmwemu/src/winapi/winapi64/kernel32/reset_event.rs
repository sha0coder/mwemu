
use crate::emu;

pub fn ResetEvent(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!ResetEvent {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // TODO: do something
    emu.regs_mut().rax = 1;
}