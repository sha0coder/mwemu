
use crate::emu;

pub fn LocalAlloc(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let bytes = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!LocalAlloc flags: {:x} sz: {}",
        flags,
        bytes
    );

    let base = emu
        .maps
        .alloc(bytes)
        .expect("kernel32!LocalAlloc out of memory");
    emu.maps
        .create_map(format!("alloc_{:x}", base).as_str(), base, bytes)
        .expect("kernel32!LocalAlloc out of memory");

    emu.regs_mut().rax = base;
}