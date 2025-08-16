
use crate::{emu, structures};

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let out_sysinfo = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!GetSystemInfo sysinfo: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        out_sysinfo,
        emu.colors.nc
    );

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(out_sysinfo, &mut emu.maps);
}