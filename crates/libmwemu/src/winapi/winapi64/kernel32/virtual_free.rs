
use crate::emu;

pub fn VirtualFree(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let sz = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!VirtualFree {} bytes at 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        sz,
        addr,
        emu.colors.nc
    );
    // TODO: do something (dump+free or not free)
    emu.regs_mut().rax = 1;
}