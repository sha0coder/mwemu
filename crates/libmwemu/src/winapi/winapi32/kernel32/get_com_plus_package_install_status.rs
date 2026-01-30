use crate::constants;
use crate::emu;

pub fn GetComPlusPackageInstallStatus(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetComPlusPackageInstallStatus");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
