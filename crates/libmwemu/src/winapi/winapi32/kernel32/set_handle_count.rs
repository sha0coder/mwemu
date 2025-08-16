use crate::emu;

pub fn SetHandleCount(emu: &mut emu::Emu) {
    let num = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!SetHandleCount error getting num param");

    log::info!(
        "{}** {} kernel32!SetHandleCount {} {}",
        emu.colors.light_red,
        emu.pos,
        num,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = num as u64;
}