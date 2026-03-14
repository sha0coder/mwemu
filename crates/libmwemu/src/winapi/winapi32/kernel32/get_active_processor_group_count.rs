use crate::constants;
use crate::emu;

pub fn GetActiveProcessorGroupCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetActiveProcessorGroupCount");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
