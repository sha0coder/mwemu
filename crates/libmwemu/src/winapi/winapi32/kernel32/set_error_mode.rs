use crate::emu;

pub fn SetErrorMode(emu: &mut emu::Emu) {
    let mode = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!SetErrorMode cannot read mode param");

    log::info!(
        "{}** {} kernel32!SetErrorMode 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        mode,
        emu.colors.nc
    );

    emu.stack_pop32(false);

    emu.regs_mut().rax = 0;
}