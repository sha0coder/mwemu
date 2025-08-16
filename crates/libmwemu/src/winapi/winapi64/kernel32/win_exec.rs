use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn WinExec(emu: &mut emu::Emu) {
    let cmdline_ptr = emu.regs().rcx;
    let cmdline = emu.maps.read_string(cmdline_ptr);

    log::info!(
        "{}** {} kernel32!WinExec  '{}'  {}",
        emu.colors.light_red,
        emu.pos,
        cmdline,
        emu.colors.nc
    );

    emu.regs_mut().rax = 32;
}