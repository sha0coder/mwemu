use crate::emu;
use crate::winapi::helper;
use crate::constants;
use crate::structures;

pub fn DecodePointer(emu: &mut emu::Emu) {
    let ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!DecodePointer cannot read the pointer") as u64;

    log::info!(
        "{}** {} kernel32!DecodePointer ptr: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = ptr;
}