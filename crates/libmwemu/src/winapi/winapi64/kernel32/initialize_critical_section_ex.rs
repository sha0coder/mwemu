use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn InitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu.regs().rcx;
    let spin_count = emu.regs().rdx;
    let flags = emu.regs().r9;

    log::info!(
        "{}** {} kernel32!InitializeCriticalSectionEx ptr: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr_crit_sect,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}