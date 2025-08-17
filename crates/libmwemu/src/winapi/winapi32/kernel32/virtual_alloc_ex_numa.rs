use crate::emu;

pub fn VirtualAllocExNuma(emu: &mut emu::Emu) {
    let proc_hndl =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!VirtualAllocExNuma cannot read the proc handle") as u64;
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!VirtualAllocExNuma cannot read the address") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!VirtualAllocExNuma cannot read the size") as u64;
    let alloc_type = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!VirtualAllocExNuma cannot read the type");
    let protect = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!VirtualAllocExNuma cannot read the protect");
    let nnd = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
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
        .expect("kernel32!VirtualAllocExNuma out of memory");

    emu.regs_mut().rax = base;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}