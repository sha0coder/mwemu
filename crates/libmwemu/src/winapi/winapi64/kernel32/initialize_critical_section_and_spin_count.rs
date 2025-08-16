use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn InitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = emu.regs().rcx;
    let spin_count = emu.regs().rdx;

    log::info!("{}** {} kernel32!InitializeCriticalSectionAndSpinCount crit_sect: 0x{:x} spin_count: {} {}", emu.colors.light_red,
        emu.pos, crit_sect, spin_count, emu.colors.nc);

    emu.regs_mut().rax = 1;
}