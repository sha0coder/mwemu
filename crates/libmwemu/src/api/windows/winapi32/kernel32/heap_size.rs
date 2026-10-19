use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn HeapSize(emu: &mut emu::Emu) {
    let _hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapSize cannot read handle");
    let _flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapSize cannot read flags");
    let mem = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!HeapSize cannot read mem") as u64;

    let size = heap_engine::heap_allocation_size(emu, mem).unwrap_or(0);

    log_red!(emu, "kernel32!HeapSize mem: 0x{:x} = {}", mem, size);

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = size as u64;
}
