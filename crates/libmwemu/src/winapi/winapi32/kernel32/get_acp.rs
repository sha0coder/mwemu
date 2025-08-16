use crate::emu;
use crate::winapi::helper;
use crate::constants;
use crate::structures;

pub fn GetAcp(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetAcp {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 1252;
}