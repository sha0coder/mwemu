
use crate::emu;

pub fn ExitProcess(emu: &mut emu::Emu) {
    let code = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!ExitProcess code: {} {}",
        emu.colors.light_red,
        emu.pos,
        code,
        emu.colors.nc
    );
    std::process::exit(1);
}