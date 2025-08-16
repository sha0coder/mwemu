use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;
use crate::winapi::winapi64::kernel32::load_library;

pub fn LoadLibraryExA(emu: &mut emu::Emu) {
    let dllptr = emu.regs().rcx;
    let dll = emu.maps.read_string(dllptr);

    emu.regs_mut().rax = load_library(emu, &dll);

    log::info!(
        "{}** {} kernel32!LoadLibraryExA  '{}' =0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        dll,
        emu.regs().rax,
        emu.colors.nc
    );
}