use crate::emu;
use crate::structures;

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let sysinfo_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!GetNativeSystemInfo cannot read sysinfo_ptr") as u64;

    let mut sysinfo = structures::SystemInfo32::new();
    sysinfo.save(sysinfo_ptr, &mut emu.maps);

    log::info!(
        "{}** {} kernel32!GetNativeSystemInfo {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.stack_pop32(false);
}