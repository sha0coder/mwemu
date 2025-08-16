use crate::emu;
use crate::winapi::helper;
use crate::constants;
use crate::structures;

pub fn GetLogicalDrives(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetLogicalDrives {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 0xc;
}