use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetStdHandle(emu: &mut emu::Emu) {
    let nstd = emu.regs().rcx as usize;  // Parameter passed in RCX in x64
    log_red!(emu, "** {} kernel32!GetStdHandle nstd: {}",
        emu.pos,
        nstd
    );
    emu.regs_mut().rax = nstd as u64;
}