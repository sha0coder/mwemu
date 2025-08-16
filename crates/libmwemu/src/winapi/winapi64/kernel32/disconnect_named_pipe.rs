use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn DisconnectNamedPipe(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!DisconnectNamedPipe hndl: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}