use crate::emu;

pub fn TlsFree(emu: &mut emu::Emu) {
    let idx = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!TlsFree cannot read idx");

    log::info!(
        "{}** {} kernel32!TlsFree idx: {} {}",
        emu.colors.light_red,
        emu.pos,
        idx,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}