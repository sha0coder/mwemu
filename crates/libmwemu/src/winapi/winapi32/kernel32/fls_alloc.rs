use crate::emu;
use crate::winapi::helper;
use crate::constants;
use crate::structures;

pub fn FlsAlloc(emu: &mut emu::Emu) {
    let callback = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FlsAlloc cannot read callback");

    log::info!(
        "{}** {} kernel32!FlsAlloc callback: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        callback,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}