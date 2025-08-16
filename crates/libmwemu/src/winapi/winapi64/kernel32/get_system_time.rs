use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::{emu, structures};

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let out_time = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!GetSystemTime ptr: 0x{:x}' {}",
        emu.colors.light_red,
        emu.pos,
        out_time,
        emu.colors.nc
    );

    let systime = structures::SystemTime::now();
    systime.save(out_time, &mut emu.maps);
}