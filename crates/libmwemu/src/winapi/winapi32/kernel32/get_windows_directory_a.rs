use crate::emu;

pub fn GetWindowsDirectoryA(emu: &mut emu::Emu) {
    let ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetWindowsDirectoryA: error reading param") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetWindowsDirectoryA: error reading param") as u64;

    log::info!(
        "{}** {} kernel32!GetWindowsDirectoryA {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.maps.write_string(ptr, "C:\\Windows\\");
    emu.regs_mut().rax = size;

    emu.stack_pop32(false);
    emu.stack_pop32(false);
}