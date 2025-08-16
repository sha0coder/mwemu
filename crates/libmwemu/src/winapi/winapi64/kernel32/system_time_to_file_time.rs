use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn SystemTimeToFileTime(emu: &mut emu::Emu) {
    let in_ptr = emu.regs().rcx;
    let out_ptr = emu.regs().rdx;

    let now = structures::SystemTime::now();
    now.save(out_ptr, &mut emu.maps);

    log::info!(
        "{}** {} kernel32!SystemTimeToFileTime  {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
}