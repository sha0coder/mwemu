use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn Thread32First(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let entry = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!Thread32First {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
    //emu.regs_mut().rax = constants::ERROR_NO_MORE_FILES;
}