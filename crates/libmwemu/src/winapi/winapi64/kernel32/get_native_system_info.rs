use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::{emu, structures};

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let ptr_sysinfo = emu.regs().rcx;

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(ptr_sysinfo, &mut emu.maps);

    log::info!("{}** {} kernel32!GetNativeSysteminfo {:?}  {}",
        emu.colors.light_red,
        emu.pos,
        sysinfo,
        emu.colors.nc
    );

    log::info!(
        "{}** {} kernel32!GetNativeSysteminfo 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        ptr_sysinfo,
        emu.colors.nc
    );
}