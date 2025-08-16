use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::{constants, emu};

pub fn GetUserDefaultLCID(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetUserDefaultLCID {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = constants::LOCALE_USER_DEFAULT;
}