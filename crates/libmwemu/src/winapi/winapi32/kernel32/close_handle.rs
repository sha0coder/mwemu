use crate::emu;
use crate::winapi::helper;

pub fn CloseHandle(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CloseHandle cannot read the handle") as u64;

    log::info!(
        "{}** {} kernel32!CloseHandle 0x{:X} {}",
        emu.colors.light_red,
        emu.pos,
        hndl,
        emu.colors.nc
    );

    if !helper::handler_close(hndl) {
        log::info!("\tinvalid handle.")
    }
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}