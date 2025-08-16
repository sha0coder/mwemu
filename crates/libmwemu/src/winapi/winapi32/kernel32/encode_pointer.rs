use crate::emu;

pub fn EncodePointer(emu: &mut emu::Emu) {
    let ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!EncodePointer cannot read the pointer") as u64;

    log::info!(
        "{}** {} kernel32!EncodePointer ptr: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = ptr;
}