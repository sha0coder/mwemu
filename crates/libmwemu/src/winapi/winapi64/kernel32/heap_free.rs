use crate::utils::likely;
use crate::emu;

pub fn HeapFree(emu: &mut emu::Emu) {
    let heap = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mem = emu.regs().r8;

    let heap_manage = emu.heap_management.as_mut().unwrap();
    if likely(heap_manage.check_fragment_exists(mem)) {
        heap_manage.free(mem);
    }
    heap_manage.free(mem);
    emu.maps.dealloc(mem);
    log_red!(emu, "kernel32!HeapFree mem: 0x{:x}", mem);

    emu.regs_mut().rax = 1;
}
