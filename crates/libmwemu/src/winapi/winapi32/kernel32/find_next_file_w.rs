use crate::emu;
use crate::constants;

pub fn FindNextFileW(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FindNextFileW cannot read the handle");
    let find_data = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!FindNextFileW cannot read the find_data");

    log::info!(
        "{}** {} kernel32!FindNextFileW {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = constants::ERROR_NO_MORE_FILES;
}