use crate::emu;

pub fn LoadResource(emu: &mut emu::Emu) {
    let hModule = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let hResInfo = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    log_red!(
        emu,
        "** {} kernel32!LoadResource {:x} {:x}",
        emu.pos,
        hModule,
        hResInfo
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = hResInfo;
}
