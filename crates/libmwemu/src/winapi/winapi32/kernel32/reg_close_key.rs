use crate::emu;
use crate::constants;

pub fn RegCloseKey(emu: &mut emu::Emu) {
    let hKey = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!RegCloseKey: error reading param") as u64;

    log::info!(
        "{}** {} kernel32!RegCloseKey hkey: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        hKey,
        emu.colors.nc
    );
    emu.stack_pop32(false);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}