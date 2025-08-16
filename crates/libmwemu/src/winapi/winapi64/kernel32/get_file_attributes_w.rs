use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let filename = emu.maps.read_wide_string(filename_ptr);

    log::info!(
        "{}** {} kernel32!GetFileAttributesW file: {} {}",
        emu.colors.light_red,
        emu.pos,
        filename,
        emu.colors.nc
    );
    emu.regs_mut().rax = 0x123;
}