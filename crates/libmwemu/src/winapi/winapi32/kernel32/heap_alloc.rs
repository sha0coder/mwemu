use crate::emu;
use crate::maps::mem64::Permission;

pub fn HeapAlloc(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapAlloc cannot read the handle");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapAlloc cannot read the flags");
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!HeapAlloc cannot read the size") as u64;

    emu.regs_mut().rax = emu.maps.alloc(size).unwrap_or_default();

    emu.maps
        .create_map(
            format!("alloc_{:x}", emu.regs().get_eax() as u32).as_str(),
            emu.regs().get_eax(),
            size,
            Permission::READ_WRITE,
        )
        .expect("kernel32!HeapAlloc out of memory");

    log_red!(
        emu,
        "kernel32!HeapAlloc eip: 0x{:x} flags: 0x{:x} size: {} =0x{:x}",
        emu.regs().get_eip(),
        flags,
        size,
        emu.regs().get_eax() as u32
    );

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
