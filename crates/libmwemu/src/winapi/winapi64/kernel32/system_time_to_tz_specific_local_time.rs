use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn SystemTimeToTzSpecificLocalTime(emu: &mut emu::Emu) {
    let tz_ptr = emu.regs().rcx;
    let ut_ptr = emu.regs().rcx;
    let lt_ptr = emu.regs().r8;

    log::info!(
        "{}** {} kernel32!SystemTimeToTzSpecificLocalTime {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}