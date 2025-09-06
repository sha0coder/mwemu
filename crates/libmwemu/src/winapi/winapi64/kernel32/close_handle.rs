use crate::emu;
use crate::winapi::helper;

pub fn CloseHandle(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;

    log_red!(emu, "kernel32!CloseHandle 0x{:X}", handle);

    if !helper::handler_close(handle) {
        panic!("\tinvalid handle.")
    }

    emu.regs_mut().rax = 1;
}
