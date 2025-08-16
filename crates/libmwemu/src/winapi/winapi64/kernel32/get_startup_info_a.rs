use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetStartupInfoA(emu: &mut emu::Emu) {
    let startup_info_ptr = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!GetStartupInfoA {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    if startup_info_ptr > 0 {
        let startupinfo = structures::StartupInfo64::new();
        startupinfo.save(startup_info_ptr, &mut emu.maps);
    }
}