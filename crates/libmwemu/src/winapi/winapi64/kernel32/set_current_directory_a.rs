use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn SetCurrentDirectoryA(emu: &mut emu::Emu) {
    let lp_path_name = emu.regs().rcx as usize;
    log_red!(emu, "** {} kernel32!SetCurrentDirectoryA lp_path_name: 0x{:x}", emu.pos, lp_path_name);
    // TODO: Implement this
    emu.regs_mut().rax = 1;
}