use crate::emu;
use crate::winapi::helper;

pub fn HeapDestroy(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapDestroy cannot read handle") as u64;

    log::info!(
        "{}** {} kernel32!HeapDestroy {:x}  {}",
        emu.colors.light_red,
        emu.pos,
        hndl,
        emu.colors.nc
    );

    helper::handler_close(hndl);

    emu.regs_mut().rax = hndl;
    emu.stack_pop32(false);
}