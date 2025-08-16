use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn FlsAlloc(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!FlsAlloc callback: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        callback,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}