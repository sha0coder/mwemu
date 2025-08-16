use crate::emu;

pub fn DisconnectNamedPipe(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!DisconnectNamedPipe cannot read the handle");

    log::info!(
        "{}** {} kernel32!DisconnectNamedPipe hndl: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}