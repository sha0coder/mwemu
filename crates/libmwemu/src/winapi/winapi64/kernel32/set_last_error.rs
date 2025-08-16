use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;
use crate::winapi::winapi64::kernel32::LAST_ERROR;

pub fn SetLastError(emu: &mut emu::Emu) {
    let err_code = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!SetLastError err: {} {}",
        emu.colors.light_red,
        emu.pos,
        err_code,
        emu.colors.nc
    );
    let mut err = LAST_ERROR.lock().unwrap();
    *err = err_code;
}