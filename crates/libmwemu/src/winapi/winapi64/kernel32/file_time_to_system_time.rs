
use crate::emu;

pub fn FileTimeToSystemTime(emu: &mut emu::Emu) {
    let file_time = emu.regs().rcx;
    let sys_time_ptr = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!FileTimeToSystemTime {} ",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 1;
}