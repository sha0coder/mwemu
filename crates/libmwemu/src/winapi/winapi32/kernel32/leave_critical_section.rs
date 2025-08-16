use crate::emu;

pub fn LeaveCriticalSection(emu: &mut emu::Emu) {
    let crit_sect = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!LeaveCriticalSection cannot read crit_sect");

    log::info!(
        "{}** {} kernel32!LeaveCriticalSection {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 1;
    emu.stack_pop32(false);
}