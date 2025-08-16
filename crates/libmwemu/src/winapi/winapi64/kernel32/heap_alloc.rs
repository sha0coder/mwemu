use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn HeapAlloc(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let size = emu.regs().r8;

    emu.regs_mut().rax = emu.maps.alloc(size).unwrap_or_default();

    emu.maps
        .create_map(
            format!("alloc_{:x}", emu.regs().rax).as_str(),
            emu.regs().rax,
            size,
        )
        .expect("kernel32!HeapAlloc out of memory");

    log::info!(
        "{}** {} kernel32!HeapAlloc rip: 0x{:x} flags: 0x{:x} size: {} =0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rip,
        flags,
        size,
        emu.regs().rax,
        emu.colors.nc
    );
}