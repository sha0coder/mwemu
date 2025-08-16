use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetEnvironmentStringsW {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    let addr = emu.alloc("environment", 1024);
    emu.maps
        .write_wide_string(addr, "PATH=c:\\Windows\\System32");
    emu.regs_mut().rax = addr;
}