use crate::emu;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    let filename_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!GetFileAttributesA cannot read filename_ptr") as u64;
    let filename = emu.maps.read_string(filename_ptr);

    log::info!(
        "{}** {} kernel32!GetFileAttributesA file: {} {}",
        emu.colors.light_red,
        emu.pos,
        filename,
        emu.colors.nc
    );

    emu.regs_mut().rax = 0x123; // file attributes

    emu.stack_pop32(false);
}