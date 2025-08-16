use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn lstrlenA(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx;

    let s = emu.maps.read_string(s_ptr);
    let len = s.len() as u64;

    log::info!(
        "{}** {} kernel32!lstrlen '{}' ={} {}",
        emu.colors.light_red,
        emu.pos,
        s,
        len,
        emu.colors.nc
    );

    emu.regs_mut().rax = len;
}