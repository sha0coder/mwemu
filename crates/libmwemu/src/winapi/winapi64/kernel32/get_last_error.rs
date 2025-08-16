use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;
use crate::winapi::winapi64::kernel32::LAST_ERROR;

pub fn GetLastError(emu: &mut emu::Emu) {
    let err = LAST_ERROR.lock().unwrap();
    emu.regs_mut().rax = *err;
    log::info!(
        "{}** {} kernel32!GetLastError ={} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rax,
        emu.colors.nc
    );
}