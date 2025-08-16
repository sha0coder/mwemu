use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn AreFileApiIsAnsi(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!AreFileApiIsAnsi {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 1;
}