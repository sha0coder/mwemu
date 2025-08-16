use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn LStrCmpI(emu: &mut emu::Emu) {
    let sptr1 = emu.regs().rcx;
    let sptr2 = emu.regs().rdx;

    let s1 = emu.maps.read_string(sptr1);
    let s2 = emu.maps.read_string(sptr2);

    if s1 == s2 {
        log::info!(
            "{}** {} kernel32!lstrcmpi `{}` == `{}` {}",
            emu.colors.light_red,
            emu.pos,
            s1,
            s2,
            emu.colors.nc
        );
        emu.regs_mut().rax = 0;
    } else {
        log::info!(
            "{}** {} kernel32!lstrcmpi `{}` != `{}` {}",
            emu.colors.light_red,
            emu.pos,
            s1,
            s2,
            emu.colors.nc
        );
        emu.regs_mut().rax = 1;
    }
}