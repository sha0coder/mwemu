use crate::emu;

pub fn Sleep(emu: &mut emu::Emu) {
    let millis = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!Sleep cannot read millis");

    log::info!(
        "{}** {} kernel32!Sleep millis: {} {}",
        emu.colors.light_red,
        emu.pos,
        millis,
        emu.colors.nc
    );

    emu.tick += millis as usize;

    emu.stack_pop32(false);
}