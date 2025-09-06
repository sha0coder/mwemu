use crate::emu;

pub fn TerminateProcess(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let code = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!TerminateProcess hndl: {} code: {}",
        hndl,
        code
    );
    emu.regs_mut().rax = 1;
}
