
use crate::emu;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let find_data = emu.regs().rdx;

    let file = emu.maps.read_string(file_ptr);
    log::info!(
        "{}** {} kernel32!FindFirstFileA file: {} {}",
        emu.colors.light_red,
        emu.pos,
        file,
        emu.colors.nc
    );
    emu.regs_mut().rax = 1;
}