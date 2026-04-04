use crate::constants;
use crate::emu;

pub fn GetJapaneseEraItems(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetJapaneseEraItems");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
