use crate::emu;

pub fn FreeLibrary(emu: &mut emu::Emu) {
    let hmod = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FreeLibrary: error reading param") as u64;

    log::info!(
        "{}** {} kernel32!FreeLibrary   {:x} {}",
        emu.colors.light_red,
        emu.pos,
        hmod,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
    emu.stack_pop32(false);
}