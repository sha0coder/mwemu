use crate::constants;
use crate::emu;

pub fn BaseWriteErrorElevationRequiredEvent(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BaseWriteErrorElevationRequiredEvent");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
