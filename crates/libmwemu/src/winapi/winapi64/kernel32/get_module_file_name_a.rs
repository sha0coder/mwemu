use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::{constants, emu};

pub fn GetModuleFileNameA(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let out_filename = emu.regs().rdx;
    let sz = emu.regs().r8;

    if sz >= 11 {
        emu.maps.write_string(out_filename, constants::EXE_NAME);
        emu.regs_mut().rax = 11;
    } else {
        emu.regs_mut().rax = 0;
    }

    log::info!(
        "{}** {} kernel32!GetModuleFileNameA hndl:{:x} {}",
        emu.colors.light_red,
        emu.pos,
        hndl,
        emu.colors.nc
    );
}