
use crate::emu;

pub fn GetCurrentDirectoryA(emu: &mut emu::Emu) {
    let buff_len = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;

    emu.maps.write_string(buff_ptr, "c:\\\x00");
    log::info!(
        "{}** {} kernel32!GetCurrentDirectoryA {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 3;
}