use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn TerminateProcess(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let code = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!TerminateProcess hndl: {} code: {} {}",
        emu.colors.light_red,
        emu.pos,
        hndl,
        code,
        emu.colors.nc
    );
    emu.regs_mut().rax = 1;
}