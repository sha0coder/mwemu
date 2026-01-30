use crate::constants;
use crate::emu;

pub fn GetMaximumProcessorGroupCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaximumProcessorGroupCount");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
