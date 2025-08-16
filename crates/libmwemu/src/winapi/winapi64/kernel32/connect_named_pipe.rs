use crate::emu;
use crate::winapi::helper;

pub fn ConnectNamedPipe(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let overlapped = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!ConnectNamedPipe hndl: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        emu.colors.nc
    );

    if !helper::handler_exist(handle) {
        log::info!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1;
}