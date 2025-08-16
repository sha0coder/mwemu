use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;
use crate::winapi::helper;

pub fn Process32Next(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let lppe = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!Process32Next hndl: {:x} lppe: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        lppe,
        emu.colors.nc
    );

    emu.maps.write_string(lppe + 44, "explorer.exe\x00");

    if !helper::handler_exist(handle) {
        emu.regs_mut().rax = 0;
        return;
    }

    emu.regs_mut().rax = 0; // trigger exit loop
}