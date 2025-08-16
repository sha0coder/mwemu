use crate::emu;

pub fn AreFileApisANSI(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!AreFileApisANSI {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 1;
}