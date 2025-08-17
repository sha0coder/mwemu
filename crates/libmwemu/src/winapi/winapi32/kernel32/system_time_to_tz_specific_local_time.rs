use crate::emu;

pub fn SystemTimeToTzSpecificLocalTime(emu: &mut emu::Emu) {
    let tz_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!SystemTimeToTzSpecificLocalTime cannot read tz_ptr");
    let ut_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!SystemTimeToTzSpecificLocalTime cannot read ut_ptr");
    let lt_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!SystemTimeToTzSpecificLocalTime cannot read lt_ptr");

    log::info!(
        "{}** {} kernel32!SystemTimeToTzSpecificLocalTime {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}