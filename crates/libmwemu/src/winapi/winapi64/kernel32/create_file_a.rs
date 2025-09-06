use crate::emu;
use crate::winapi::helper;

pub fn CreateFileA(emu: &mut emu::Emu) {
    let lp_file_name = emu.regs().rcx as usize;
    let dw_desired_access = emu.regs().rdx as usize;
    let dw_share_mode = emu.regs().r8 as usize;
    let lp_security_attributes = emu.regs().r9 as usize;
    let dw_creation_disposition = emu.regs().r10 as usize;
    let dw_flags_and_attributes = emu.regs().r11 as usize;
    let h_template_file = emu.regs().r12 as usize;
    log_red!(emu, "** {} kernel32!CreateFileA lp_file_name: 0x{:x} dw_desired_access: 0x{:x} dw_share_mode: 0x{:x} lp_security_attributes: 0x{:x} dw_creation_disposition: 0x{:x} dw_flags_and_attributes: 0x{:x} h_template_file: 0x{:x}", emu.pos, lp_file_name, dw_desired_access, dw_share_mode, lp_security_attributes, dw_creation_disposition, dw_flags_and_attributes, h_template_file);
    let mut name: String = String::new();
    if lp_file_name > 0 {
        name = emu.maps.read_string(lp_file_name as u64);
    }
    log_red!(
        emu,
        "** {} kernel32!CreateFileA name = {name} {}",
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = helper::handler_create(&name);
}
