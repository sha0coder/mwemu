
use crate::emu;
use crate::winapi::helper;

pub fn GetProcessHeap(emu: &mut emu::Emu) {
    emu.regs_mut().rax = helper::handler_create("heap");

    log::info!(
        "{}** {} kernel32!GetProcessHeap ={} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rax,
        emu.colors.nc
    );
}