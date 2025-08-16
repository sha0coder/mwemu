use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn InitializeCriticalSection(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!InitializeCriticalSection ptr: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr_crit_sect,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}