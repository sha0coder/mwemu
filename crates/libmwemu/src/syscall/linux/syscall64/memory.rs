use crate::emu;
use crate::windows::constants;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_BRK => handle_syscall64_brk(emu),
        constants::NR64_NANOSLEEP => handle_syscall64_nanosleep(emu),
        constants::NR64_MREMAP => handle_syscall64_mremap(emu),
        _ => return false,
    }

    true
}

pub(super) fn handle_syscall64_brk(emu: &mut emu::Emu) {
    let heap_base = 0x4b5b00;
    let heap_size = 0x4d8000 - 0x4b5000;

    /*
    heap = emu
        .maps
        .create_map("heap", heap_base, heap_size)
        .expect("cannot create heap map from brk syscall");
    */

    if emu.regs_mut().rdi == 0 {
        emu.regs_mut().r11 = 0x346;
        emu.regs_mut().rcx = 0x4679f7;
        emu.regs_mut().rax = emu.heap_addr;
    } else {
        // let bottom = emu.regs().rdi;
        // let new_sz = bottom - heap_base;
        // heap.set_size(new_sz);
        emu.regs_mut().rax = emu.regs().rdi;
        emu.regs_mut().rcx = 0x4679f7;
        emu.regs_mut().rdx = 0x2f;
        emu.regs_mut().r11 = 0x302;
    }

    //emu.fs.insert(0xffffffffffffffc8, 0x4b6c50);

    log::trace!(
        "{}** {} syscall brk({:x}) ={:x} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rdi,
        emu.regs().rax,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_nanosleep(emu: &mut emu::Emu) {
    let req = emu.regs().rdi;
    let tv_sec = emu.maps.read_qword(req).unwrap_or(0);
    let tv_nsec = emu.maps.read_qword(req + 8).unwrap_or(0);

    log::info!(
        "{}** {} syscall nanosleep(tv_sec: 0x{:x}, tv_nsec: 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        tv_sec,
        tv_nsec,
        emu.colors.nc
    );

    // TODO: implement actual sleep
    emu.regs_mut().rax = 0;
}

pub(super) fn handle_syscall64_mremap(emu: &mut emu::Emu) {
    let old_addr = emu.regs().rdi;
    let old_sz = emu.regs().rsi;
    let new_sz = emu.regs().rdx;
    let flags = emu.regs().r10;
    let new_addr = emu.regs().r8;

    log::info!(
        "{}** {} syscall mremap(old_addr: 0x{:x}, old_sz: 0x{:x}, new_sz: 0x{:x}, flags: 0x{:x}, new_addr: 0x{:x}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        old_addr,
        old_sz,
        new_sz,
        flags,
        new_addr,
        emu.regs().rax,
        emu.colors.nc
    );

    // TODO: implement actual mremap
    emu.regs_mut().rax = 0;
}
