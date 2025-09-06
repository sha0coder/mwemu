use crate::emu;
use crate::winapi::helper;

pub fn CreateFileMappingA(emu: &mut emu::Emu) {
    let hFile = emu.regs().rcx;
    let attr = emu.regs().rdx;
    let protect = emu.regs().r8;
    let max_sz_high = emu.regs().r9;
    let max_sz_low = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!CreateFileMappingW cannot read max size low");
    let name_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("kernel32!CreateFileMappingW cannot read name pointer");

    let mut name: String = String::new();
    if name_ptr > 0 {
        name = emu.maps.read_string(name_ptr);
    }

    emu.regs_mut().rax = helper::handler_create(&name);
    log_red!(
        emu,
        "kernel32!CreateFileMappingA '{}' ={}",
        name,
        emu.regs().get_eax()
    );
}
