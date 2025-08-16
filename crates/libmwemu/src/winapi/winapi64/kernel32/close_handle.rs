use crate::emu;
use crate::winapi::helper;

pub fn CloseHandle(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!CloseHandle 0x{:X} {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        emu.colors.nc
    );

    if !helper::handler_close(handle) {
        panic!("\tinvalid handle.")
    }

    emu.regs_mut().rax = 1;
}