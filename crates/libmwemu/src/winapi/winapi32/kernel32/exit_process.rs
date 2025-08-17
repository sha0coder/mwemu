use crate::emu;

pub fn ExitProcess(emu: &mut emu::Emu) {
    let code = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!ExitProcess cannot read the exit code");

    log::info!(
        "{}** {} kernel32!ExitProcess code: {} {}",
        emu.colors.light_red,
        emu.pos,
        code,
        emu.colors.nc
    );
    emu.stack_pop32(false);

    std::process::exit(1);
}