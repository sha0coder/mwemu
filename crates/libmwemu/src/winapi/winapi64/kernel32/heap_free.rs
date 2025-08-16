use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn HeapFree(emu: &mut emu::Emu) {
    let heap = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mem = emu.regs().r8;

    log::info!(
        "{}** {} kernel32!HeapFree mem: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        mem,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}