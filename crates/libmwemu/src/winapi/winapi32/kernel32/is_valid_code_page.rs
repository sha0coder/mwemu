use crate::emu;

pub fn IsValidCodePage(emu: &mut emu::Emu) {
    let codepage = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!IsValidCodePage error geting codepage param");

    log::info!(
        "{}** {} kernel32!IsValidCodePage {} {}",
        emu.colors.light_red,
        emu.pos,
        codepage,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}