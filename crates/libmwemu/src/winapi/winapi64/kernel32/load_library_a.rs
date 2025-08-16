use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn LoadLibraryA(emu: &mut emu::Emu) {
    let dllptr = emu.regs().rcx;
    let dll = emu.maps.read_string(dllptr);

    emu.regs_mut().rax = load_library(emu, &dll);

    log_red!(emu, "** {} kernel32!LoadLibraryA  '{}' =0x{:x} rip: 0x{:x}", emu.pos, &dll, emu.regs().get_eax() as u32, emu.regs().rip);
}