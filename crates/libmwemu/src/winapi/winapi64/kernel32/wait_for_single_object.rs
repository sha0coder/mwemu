use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let millis = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!WaitForSingleObject  hndl: {} millis: {} {}",
        emu.colors.light_red,
        emu.pos,
        hndl,
        millis,
        emu.colors.nc
    );

    emu.regs_mut().rax = constants::WAIT_TIMEOUT;
}