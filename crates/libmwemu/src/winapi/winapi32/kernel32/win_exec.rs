use crate::emu;

pub fn WinExec(emu: &mut emu::Emu) {
    let cmdline_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the cmdline parameter of WinExec") as u64;
    let cmdline = emu.maps.read_string(cmdline_ptr);

    //emu.spawn_console();

    log::info!(
        "{}** {} WinExec  '{}'  {}",
        emu.colors.light_red,
        emu.pos,
        cmdline,
        emu.colors.nc
    );

    emu.regs_mut().rax = 0;
    emu.stack_pop32(false);
}