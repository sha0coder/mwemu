use crate::emu;
use crate::constants;

pub fn GetVersion(emu: &mut emu::Emu) {
    emu.regs_mut().rax = constants::VERSION;
    log::info!(
        "{}** {} kernel32!GetVersion   =0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().get_eax() as u32,
        emu.colors.nc
    );
}