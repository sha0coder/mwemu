use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn ActivateActCtx(emu: &mut emu::Emu) {
    let h_act_ctx = emu.regs().rcx;
    let lp_cookie = emu.regs().rdx as usize;
    log_red!(emu, "** {} kernel32!ActivateActCtx {:x} {:x}", emu.pos, h_act_ctx, lp_cookie);
    // TODO: implement this
    emu.regs_mut().rax = 1;
}