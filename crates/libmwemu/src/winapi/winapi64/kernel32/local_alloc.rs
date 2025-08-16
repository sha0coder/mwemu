use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn LocalAlloc(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let bytes = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!LocalAlloc flags: {:x} sz: {} {}",
        emu.colors.light_red,
        emu.pos,
        flags,
        bytes,
        emu.colors.nc
    );

    let base = emu
        .maps
        .alloc(bytes)
        .expect("kernel32!LocalAlloc out of memory");
    emu.maps
        .create_map(format!("alloc_{:x}", base).as_str(), base, bytes)
        .expect("kernel32!LocalAlloc out of memory");

    emu.regs_mut().rax = base;
}