use crate::emu;

pub fn GetSystemTimeAsFileTime(emu: &mut emu::Emu) {
    let sys_time_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetSystemTimeAsFileTime cannot read sys_time_ptr");

    log::info!(
        "{}** {} kernel32!GetSystemTimeAsFileTime {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}