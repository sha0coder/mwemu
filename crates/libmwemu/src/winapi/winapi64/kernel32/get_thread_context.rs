use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::context::context64;
use crate::emu;

pub fn GetThreadContext(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let ctx_ptr = emu.regs().rdx;

    let ctx = context64::Context64::new(&emu.regs());
    ctx.save(ctx_ptr, &mut emu.maps);

    log::info!(
        "{}** {} kernel32!GetThreadContext  {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}