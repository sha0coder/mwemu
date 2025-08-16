use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn LoadResource(emu: &mut emu::Emu) {
    let hModule = emu.regs().rcx;
    let hResInfo = emu.regs().rdx as u64;

    log_red!(emu, "** {} kernel32!LoadResource {:x} {:x}", emu.pos, hModule, hResInfo);

    emu.regs_mut().rax = hResInfo;
}