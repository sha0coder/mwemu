
use crate::{constants, emu};

pub fn GetVersion(emu: &mut emu::Emu) {
    emu.regs_mut().rax = constants::VERSION;
    log::info!(
        "{}** {} kernel32!GetVersion   =0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rax,
        emu.colors.nc
    );
}