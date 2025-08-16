use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileW(emu: &mut emu::Emu) {
    let file_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FindFirstFileW cannot read file_ptr") as u64;
    let find_data = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!FindFirstFileW cannot read find_data");

    let file = emu.maps.read_wide_string(file_ptr);

    log::info!(
        "{}** {} kernel32!FindFirstFileW file: {} {}",
        emu.colors.light_red,
        emu.pos,
        file,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = helper::handler_create(&file);
}