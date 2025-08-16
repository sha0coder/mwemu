use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetCurrentThread(emu: &mut emu::Emu) {
    log_red!(emu, "** {} kernel32!GetCurrentThread", emu.pos);
    // TODO: implement this
    emu.regs_mut().rax = 3;
}