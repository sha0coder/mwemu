use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetComputerNameA(emu: &mut emu::Emu) {
    let buff_ptr = emu.regs().rcx;
    let size_ptr = emu.regs().rdx;

    emu.maps.write_dword(size_ptr, 6);
    emu.maps.write_string(buff_ptr, constants::HOST_NAME);

    log::info!(
        "{}** {} kernel32!GetComputerNameA '{}' {}",
        emu.colors.light_red,
        emu.pos,
         constants::HOST_NAME,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}