use crate::emu;
use crate::winapi::helper;

pub fn GetCurrentProcess(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentProcess");
    emu.regs_mut().rax = helper::handler_create("current process");
}
