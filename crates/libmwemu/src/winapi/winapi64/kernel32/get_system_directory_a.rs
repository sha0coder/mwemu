use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetSystemDirectoryA(emu: &mut emu::Emu) {
    let out_buff_ptr = emu.regs().rcx;
    let size = emu.regs().rdx;

    let output = "C:\\Windows\\";
    emu.maps.write_string(out_buff_ptr, &output);

    log::info!(
        "{}** {} kernel32!GetSystemDirectoryW  {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = output.len() as u64;
}