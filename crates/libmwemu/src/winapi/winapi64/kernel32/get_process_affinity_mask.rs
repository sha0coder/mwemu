use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetProcessAffinityMask(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let proc_affinity_mask_ptr = emu.regs().rdx;
    let sys_affinity_mask_ptr = emu.regs().r8;

    emu.maps.write_dword(proc_affinity_mask_ptr, 0x1337);
    emu.maps.write_dword(sys_affinity_mask_ptr, 0x1337);

    log::info!(
        "{}** {} kernel32!GetProcessAffinityMask {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}