use crate::emu;
use crate::winapi::helper;

pub fn CreateFileW(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let share = emu.regs().r8;
    let security = emu.regs().r9;
    
    let filename = emu.maps.read_wide_string(filename_ptr);

    log_red!(
        emu,
        "kernel32!CreateFileW `{}` access:0x{:x} share:0x{:x} sec:0x{:x}",
        filename,
        access,
        share,
        security
    );

    emu.regs_mut().rax = helper::handler_create(&filename);
}

