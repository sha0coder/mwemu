use crate::emu;
use crate::winapi::helper;
use crate::constants;
use crate::structures;

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