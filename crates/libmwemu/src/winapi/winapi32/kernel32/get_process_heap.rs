use crate::emu;
use crate::winapi::helper;

pub fn GetProcessHeap(emu: &mut emu::Emu) {
    emu.regs_mut().rax = helper::handler_create("process heap");
    log::info!(
        "{}** {} kernel32!GetProcessHeap =0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rax as u32,
        emu.colors.nc
    );
}