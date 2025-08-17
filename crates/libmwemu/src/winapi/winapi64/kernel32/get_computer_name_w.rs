
use crate::{constants, emu};

pub fn GetComputerNameW(emu: &mut emu::Emu) {
    let buff_ptr = emu.regs().rcx;
    let size_ptr = emu.regs().rdx;

    emu.maps.write_dword(size_ptr, constants::HOST_NAME.len() as u32);
    emu.maps.write_wide_string(buff_ptr, constants::HOST_NAME);

    log::info!(
        "{}** {} kernel32!GetComputerNameW '{}' {}",
        emu.colors.light_red,
        emu.pos,
        constants::HOST_NAME,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}