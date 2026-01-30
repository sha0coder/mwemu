use crate::constants;
use crate::emu;

pub fn NtWow64CsrBasepNlsUpdateCacheCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!NtWow64CsrBasepNlsUpdateCacheCount");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
