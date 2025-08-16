use crate::emu;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetEnvironmentStrings {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    let ptr = emu.alloc("environment", 1024);
    emu.maps.write_string(ptr, "PATH=c:\\Windows\\System32");
    emu.regs_mut().rax = ptr;
}