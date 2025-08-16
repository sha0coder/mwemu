use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn CopyFileA(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let do_fail = emu.regs().r8;

    let src = emu.maps.read_string(src_ptr);
    let dst = emu.maps.read_string(dst_ptr);

    log::info!(
        "{}** {} kernel32!CopyFileA `{}` to `{}` {}",
        emu.colors.light_red,
        emu.pos,
        src,
        dst,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}