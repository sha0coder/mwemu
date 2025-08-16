
use crate::emu;

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let buff = emu.regs().r8;
    let path = emu.regs().r9;

    let filename = emu.maps.read_wide_string(file_ptr);
    log::info!(
        "{}** {} kernel32!GetFullPathNameW file: {}  {}",
        emu.colors.light_red,
        emu.pos,
        filename,
        emu.colors.nc
    );
    // TODO: save the path to buff.
    emu.regs_mut().rax = 10;
}