use crate::emu;
use crate::winapi::helper;

pub fn FindClose(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!FindClose {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    helper::handler_close(hndl);
    emu.regs_mut().rax = 1;
}