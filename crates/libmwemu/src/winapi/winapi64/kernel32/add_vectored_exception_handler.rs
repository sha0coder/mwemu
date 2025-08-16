use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let p1 = emu.regs().rcx as usize;
    let fptr = emu.regs().rdx as usize;

    log::info!(
        "{}** {} kernel32!AddVectoredExceptionHandler  {} callback: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        p1,
        fptr,
        emu.colors.nc
    );

    emu.set_veh(fptr as u64);

    emu.regs_mut().rax = 0x2c2878;
}