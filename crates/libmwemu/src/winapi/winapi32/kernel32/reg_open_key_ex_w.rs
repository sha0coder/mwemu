use crate::emu;

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let hkey = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!RegOpenKeyExW cannot read hkey");
    let subkey_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!RegOpenKeyExW cannot read subkey") as u64;
    let options = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!RegOpenKeyExW cannot read options");
    let sam = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!RegOpenKeyExW cannot read sam");
    let result = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!RegOpenKeyExW cannot read result");

    let subkey = emu.maps.read_wide_string(subkey_ptr);
    log::info!(
        "{}** {} kernel32!RegOpenKeyExW {} {}",
        emu.colors.light_red,
        emu.pos,
        subkey,
        emu.colors.nc
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}