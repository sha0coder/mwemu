use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn DecodePointer(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!DecodePointer ptr: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr,
        emu.colors.nc
    );

    emu.regs_mut().rax = ptr;
}