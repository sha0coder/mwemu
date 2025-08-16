
use crate::emu;
use crate::winapi::helper;

pub fn GetCurrentProcess(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetCurrentProcess {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = helper::handler_create("current process");
}