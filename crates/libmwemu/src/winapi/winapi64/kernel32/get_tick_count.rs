use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetTickCount(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetTickCount {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // TODO: increment the tick?
    emu.regs_mut().rax = emu.tick as u64;
}