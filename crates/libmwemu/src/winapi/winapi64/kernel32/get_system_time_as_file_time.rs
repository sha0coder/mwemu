
use crate::emu;

pub fn GetSystemTimeAsFileTime(emu: &mut emu::Emu) {
    let sys_time_ptr = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!GetSystemTimeAsFileTime {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    // TODO: implement

    emu.regs_mut().rax = 1;
}