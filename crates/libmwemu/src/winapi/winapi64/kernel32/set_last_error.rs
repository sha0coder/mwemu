
use crate::emu;
use crate::winapi::winapi64::kernel32::set_last_error;

pub fn SetLastError(emu: &mut emu::Emu) {
    let err_code = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!SetLastError err: {} {}",
        emu.colors.light_red,
        emu.pos,
        err_code,
        emu.colors.nc
    );
    set_last_error(err_code);
}