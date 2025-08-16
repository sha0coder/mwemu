use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetVersionExW(emu: &mut emu::Emu) {
    let version_info_ptr = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!GetVersionExW 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        version_info_ptr,
        emu.colors.nc
    );

    let os_version_info = structures::OsVersionInfo::new();
    os_version_info.save(version_info_ptr, &mut emu.maps);

    emu.regs_mut().rax = 1;
}