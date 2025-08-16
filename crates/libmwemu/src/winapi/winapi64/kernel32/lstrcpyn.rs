use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn lstrcpyn(emu: &mut emu::Emu) {
    let out_str1 = emu.regs().rcx;
    let in_str2 = emu.regs().rdx;
    let len = emu.regs().r8 as usize;

    let mut s = emu.maps.read_string(in_str2);
    if s.len() - 1 > len {
        s = s.chars().take(len).collect();
    }
    emu.maps.memset(out_str1, 0, len);
    emu.maps.write_string(out_str1, &s);

    log::info!(
        "{}** {} kernel32!lstrcpyn {} {}",
        emu.colors.light_red,
        emu.pos,
        s,
        emu.colors.nc
    );

    emu.regs_mut().rax = out_str1;
}