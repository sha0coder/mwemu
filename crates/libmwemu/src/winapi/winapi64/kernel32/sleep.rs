use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn Sleep(emu: &mut emu::Emu) {
    let millis = emu.regs().rcx;
    
    if millis > 0 {
        // Set wake tick for this thread
        let thread_idx = emu.current_thread_id;
        emu.threads[thread_idx].wake_tick = emu.tick + millis as usize;
    }
    
    // Advance global tick
    advance_tick(emu, millis);
    
    log::info!(
        "{}** {} kernel32!Sleep thread: 0x{:x} millis: {} {}",
        emu.colors.light_red,
        emu.pos,
        emu.current_thread().id,
        millis,
        emu.colors.nc
    );
}