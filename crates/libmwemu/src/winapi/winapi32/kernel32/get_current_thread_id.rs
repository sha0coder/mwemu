use crate::emu;

pub fn GetCurrentThreadId(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetCurrentThreadId {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 0x111; //TODO: track pids and tids
}