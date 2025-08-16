use crate::emu;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    let file_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FindFirstFileA cannot read file_ptr") as u64;
    let find_data = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!FindFirstFileA cannot read find_data");

    let file = emu.maps.read_string(file_ptr);

    log::info!(
        "{}** {} kernel32!FindFirstFileA file: {} {}",
        emu.colors.light_red,
        emu.pos,
        file,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}