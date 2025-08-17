use crate::emu;

pub fn ResumeThread(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!ResumeThread cannot read the handle");

    log::info!(
        "{}** {} kernel32!ResumeThread hndl: {} {}",
        emu.colors.light_red,
        emu.pos,
        hndl,
        emu.colors.nc
    );

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1; // previous suspend count
}