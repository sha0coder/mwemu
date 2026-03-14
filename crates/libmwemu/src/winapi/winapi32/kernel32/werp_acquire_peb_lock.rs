use crate::constants;
use crate::emu;

pub fn WerpAcquirePebLock(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!WerpAcquirePebLock");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
