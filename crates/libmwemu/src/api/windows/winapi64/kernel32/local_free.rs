use crate::emu;

pub fn LocalFree(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;

    log_red!(emu, "kernel32!LocalFree flags: {:x}", addr);

    let heap_management = emu.heap_management.as_mut().unwrap();
    let base = heap_management.free(addr);
    emu.regs_mut().rax = 0;
}
