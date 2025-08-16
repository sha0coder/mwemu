use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let filename = emu.maps.read_string(filename_ptr);

    log::info!(
        "{}** {} kernel32!GetFileAttributesA file: {} {}",
        emu.colors.light_red,
        emu.pos,
        filename,
        emu.colors.nc
    );
    emu.regs_mut().rax = 0x123;
}