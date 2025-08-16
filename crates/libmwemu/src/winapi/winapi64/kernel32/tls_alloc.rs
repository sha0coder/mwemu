use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn TlsAlloc(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!TlsAlloc {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.tls64_mut().push(0);
    emu.regs_mut().rax = (emu.tls64().len() - 1) as u64;  // Return index of newly allocated slot
}