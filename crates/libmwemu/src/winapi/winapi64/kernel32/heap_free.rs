
use crate::emu;

pub fn HeapFree(emu: &mut emu::Emu) {
    let heap = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mem = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!HeapFree mem: 0x{:x}",
        mem
    );

    emu.regs_mut().rax = 1;
}