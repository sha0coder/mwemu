use crate::constants;
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
    if hndl == constants::CURRENT_PROCESS_HANDLE {
        emu.stop();
        return;
    }
    emu.regs_mut().rax = 1;
}
