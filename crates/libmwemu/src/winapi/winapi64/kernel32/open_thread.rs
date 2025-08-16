use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;
use crate::winapi::helper;

pub fn OpenThread(emu: &mut emu::Emu) {
    let access = emu.regs().rcx;
    let inherit = emu.regs().rdx;
    let tid = emu.regs().r8;

    log::info!(
        "{}** {} kernel32!OpenThread tid: {} {}",
        emu.colors.light_red,
        emu.pos,
        tid,
        emu.colors.nc
    );

    let uri = format!("tid://{}", tid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}