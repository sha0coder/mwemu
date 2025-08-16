use crate::emu;

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let out_sysinfo = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetSystemInfo cannot read out_sysinfo") as u64;

    log::info!(
        "{}** {} kernel32!GetSystemInfo sysinfo: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        out_sysinfo,
        emu.colors.nc
    );

    // let mut sysinfo = emu::structures::SystemInfo32::new();
    // sysinfo.save(out_sysinfo, &mut emu.maps);

    emu.stack_pop32(false);
}