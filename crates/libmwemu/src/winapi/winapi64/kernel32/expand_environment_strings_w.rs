
use crate::emu;

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let size = emu.regs().r8;

    let src = emu.maps.read_wide_string(src_ptr);

    log::info!(
        "{}** {} kernel32!ExpandEnvironmentStringsW `{}` {}",
        emu.colors.light_red,
        emu.pos,
        src,
        emu.colors.nc
    );
    // TODO: expand typical environment varsl.
    emu.regs_mut().rax = 1;
}