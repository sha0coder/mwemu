use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetUserDefaultLangId(emu: &mut emu::Emu) {
    emu.regs_mut().rax = 0x000000000000ffff;
    log::info!(
        "{}** {} kernel32!GetUserDefaultLangID =0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rax as u16,
        emu.colors.nc
    );
}