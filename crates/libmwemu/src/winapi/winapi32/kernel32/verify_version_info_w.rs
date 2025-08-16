use crate::emu;

pub fn VerifyVersionInfoW(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!VerifyVersionInfoW {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0xffff;
}