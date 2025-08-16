
use crate::emu;

pub fn EncodePointer(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!EncodePointer ptr: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr,
        emu.colors.nc
    );

    emu.regs_mut().rax = ptr;
}