use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn MapViewOfFile(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let access = emu.regs().rdx;
    let off_high = emu.regs().r8;
    let off_low = emu.regs().r9;
    let mut size = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!MapViewOfFile cannot read the size");

    let off: u64 = (off_high << 32) + off_low;

    if size > 1024 * 4 {
        size = 1024
    }

    let addr = emu
        .maps
        .alloc(size)
        .expect("kernel32!MapViewOfFile cannot allocate");
    let mem = emu
        .maps
        .create_map("file_map", addr, size)
        .expect("kernel32!MapViewOfFile cannot create map");
    let loaded = mem.load_chunk(&emu.filename, off, size as usize);

    log::info!(
        "{}** {} kernel32!MapViewOfFile hndl: {} off: {} sz: {} ={} {}",
        emu.colors.light_red,
        emu.pos,
        hndl,
        off,
        size,
        addr,
        emu.colors.nc
    );

    if off > 0 {
        log::info!("the non-zero offset is not implemented for now");
    }

    emu.regs_mut().rax = addr;
}