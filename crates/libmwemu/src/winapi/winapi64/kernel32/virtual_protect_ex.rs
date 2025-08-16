use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn VirtualProtectEx(emu: &mut emu::Emu) {
    let hproc = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let size = emu.regs().r8;
    let new_prot = emu.regs().r9;
    let oldld_prot_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!VirtualProtectEx cannot read old_prot");

    log::info!(
        "{}** {} kernel32!VirtualProtectEx hproc: {} addr: 0x{:x} sz: {} prot: {} {}",
        emu.colors.light_red,
        emu.pos,
        hproc,
        addr,
        size,
        new_prot,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}