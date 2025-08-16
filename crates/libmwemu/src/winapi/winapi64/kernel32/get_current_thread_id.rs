use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetCurrentThreadId(emu: &mut emu::Emu) {
    let thread_id = emu.current_thread().id;
    
    log_red!(emu, "kernel32!GetCurrentThreadId = 0x{:x}", thread_id);
    
    emu.regs_mut().rax = thread_id;
}