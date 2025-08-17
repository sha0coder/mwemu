
use crate::emu;

pub fn VirtualAllocExNuma(emu: &mut emu::Emu) {
    let proc_hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let size = emu.regs().r8;
    let alloc_type = emu.regs().r9;
    let protect = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!VirtualAllocExNuma cannot read the protect");
    let nnd = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("kernel32!VirtualAllocExNuma cannot read the nndPreferred");

    log_red!(
        emu,
        "kernel32!VirtualAllocExNuma hproc: 0x{:x} addr: 0x{:x}",
        proc_hndl,
        addr
    );

    let base = emu
        .maps
        .alloc(size)
        .expect("kernel32!VirtualAllocExNuma out of memory");
    emu.maps
        .create_map(format!("alloc_{:x}", base).as_str(), base, size)
        .expect("kernel32!VirtualAllocExNuma cannot create map");

    emu.regs_mut().rax = base;
}