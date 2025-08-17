use crate::emu;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    let file_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetFullPathNameA cannot read file_ptr") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetFullPathNameA cannot read size");
    let buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!GetFullPathNameA cannot read buff");
    let path = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!GetFullPathNameA cannot read path");

    let filename = emu.maps.read_string(file_ptr);

    log::info!(
        "{}** {} kernel32!GetFullPathNameA file: {}  {}",
        emu.colors.light_red,
        emu.pos,
        filename,
        emu.colors.nc
    );

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 10;
}