
use crate::emu;

pub fn VirtualAllocEx(emu: &mut emu::Emu) {
    let proc_hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let size = emu.regs().r8;
    let alloc_type = emu.regs().r9;
    let protect = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!VirtualAllocEx cannot read_qword protect");

    let base = emu
        .maps
        .alloc(size)
        .expect("kernel32!VirtualAllocEx out of memory");

    log_red!(
        emu,
        "kernel32!VirtualAllocEx hproc: 0x{:x} addr: 0x{:x} sz: {} = 0x{:x}",
        proc_hndl,
        addr,
        size,
        base
    );

    emu.maps
        .create_map(format!("alloc_{:x}", base).as_str(), base, size)
        .expect("kernel32!VirtualAllocEx out of memory");

    emu.regs_mut().rax = base;
}