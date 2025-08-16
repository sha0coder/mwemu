use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn CreateProcessW(emu: &mut emu::Emu) {
    let appname_ptr = emu.regs().rcx;
    let cmdline_ptr = emu.regs().rdx;
    let appname = emu.maps.read_wide_string(appname_ptr);
    let cmdline = emu.maps.read_wide_string(cmdline_ptr);

    log::info!(
        "{}** {} kernel32!CreateProcessW  {} {} {}",
        emu.colors.light_red,
        emu.pos,
        appname,
        cmdline,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}