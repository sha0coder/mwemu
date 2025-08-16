
use crate::emu;

pub fn GetCurrentProcessId(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetCurrentProcessId {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 0x123;
}