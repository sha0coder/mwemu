use crate::emu;
use crate::winapi::helper;

pub fn BeginUpdateResourceA(emu: &mut emu::Emu) {
    let pFileName = emu.regs().rcx;
    let bDeleteExistingResources = emu.regs().rdx;

    let filename = emu.maps.read_string(pFileName);

    log::info!(
        "{}** {} kernel32!BeginUpdateResourceA `{}` {} {}",
        emu.colors.light_red,
        emu.pos,
        filename,
        bDeleteExistingResources,
        emu.colors.nc
    );

    emu.regs_mut().rax = helper::handler_create(&filename);
}