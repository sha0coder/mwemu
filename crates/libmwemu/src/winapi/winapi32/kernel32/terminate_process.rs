use crate::emu;

pub fn TerminateProcess(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!TerminateProcess cannot read the handle");
    let code = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!TerminateProcess cannot read the exit code");

    log::info!(
        "{}** {} kernel32!TerminateProcess hndl: {} code: {} {}",
        emu.colors.light_red,
        emu.pos,
        hndl,
        code,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}