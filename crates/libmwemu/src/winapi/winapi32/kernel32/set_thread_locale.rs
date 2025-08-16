use crate::emu;

pub fn SetThreadLocale(emu: &mut emu::Emu) {
    let locale = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!SetThreadLocale cannot read locale param");

    log::info!(
        "{}** {} kernel32!SetThreadLocale {} {}",
        emu.colors.light_red,
        emu.pos,
        locale,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}