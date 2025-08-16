use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn HeapCreate(emu: &mut emu::Emu) {
    let opts = emu.regs().rcx;
    let initSZ = emu.regs().rdx;
    let maxSZ = emu.regs().r8;

    log::info!(
        "{}** {} kernel32!HeapCreate maxSZ:{} {}",
        emu.colors.light_red,
        emu.pos,
        maxSZ,
        emu.colors.nc
    );

    let uri = format!("HeapCreate://{}", maxSZ);
    emu.regs_mut().rax = helper::handler_create(&uri);
}