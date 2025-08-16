use crate::emu;

pub fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let callback =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!SetUnhandledExceptionFilter cannot read the callback") as u64;

    log::info!(
        "{}** {} kernel32!SetUnhandledExceptionFilter  callback: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        callback,
        emu.colors.nc
    );

    emu.regs_mut().rax = emu.seh();
    emu.set_seh(callback);

    emu.stack_pop32(false);
}