use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn VirtualProtect(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let new_prot = emu.regs().r8;
    let old_prot_ptr = emu.regs().r9;

    emu.maps.write_qword(old_prot_ptr, new_prot);

    log::info!(
        "{}** {} kernel32!VirtualProtect addr: 0x{:x} sz: {} prot: {} {}",
        emu.colors.light_red,
        emu.pos,
        addr,
        size,
        new_prot,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}