use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn IsBadReadPtr(emu: &mut emu::Emu) {
    let lp = emu.regs().rcx as usize;
    let ucb = emu.regs().rdx as usize;
    log_red!(emu, "** {} kernel32!IsBadReadPtr {:x} {:x}", emu.pos, lp, ucb);
    // TODO: implement this
    emu.regs_mut().rax = 0;
}