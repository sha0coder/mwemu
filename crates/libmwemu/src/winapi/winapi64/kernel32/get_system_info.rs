use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let out_sysinfo = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!GetSystemInfo sysinfo: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        out_sysinfo,
        emu.colors.nc
    );

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(out_sysinfo, &mut emu.maps);
}