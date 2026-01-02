use crate::constants;
use crate::emu;

pub fn RegCloseKey(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;

    log_red!(emu, "kernel32!RegCloseKey hkey: 0x{:x}", h_key);

    // Stub implementation, always successful
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
