use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn lstrcatW(emu: &mut emu::Emu) {
    let str1_ptr = emu.regs().rcx;
    let str2_ptr = emu.regs().rdx;

    let mut str1 = emu.maps.read_wide_string(str1_ptr);
    let str2 = emu.maps.read_wide_string(str2_ptr);

    log::info!(
        "{}** {} kernel32!lstrcatW '{}'+'{}' {}",
        emu.colors.light_red,
        emu.pos,
        str1,
        str2,
        emu.colors.nc
    );

    str1.push_str(&str2);
    emu.maps.write_wide_string(str1_ptr, &str1);

    emu.regs_mut().rax = 1;
}