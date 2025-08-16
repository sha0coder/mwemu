use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!SetUnhandledExceptionFilter  callback: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        callback,
        emu.colors.nc
    );

    emu.regs_mut().rax = emu.seh();
    emu.set_seh(callback);
}