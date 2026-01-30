use crate::constants;
use crate::emu;

pub fn GetConsoleInputWaitHandle(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleInputWaitHandle");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
