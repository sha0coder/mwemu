use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn SetThreadStackGuarantee(emu: &mut emu::Emu) {
    let stack_size_in_bytes = emu.regs().rcx as usize;
    log_red!(emu, "** {} kernel32!SetThreadStackGuarantee {:x}", emu.pos, stack_size_in_bytes);
    // TODO: implement this
    emu.regs_mut().rax = 1;
}