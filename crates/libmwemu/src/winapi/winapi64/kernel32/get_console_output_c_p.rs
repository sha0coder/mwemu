use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetConsoleOutputCP(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetConsoleOutputCP {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 0x00000409;
}