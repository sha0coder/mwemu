use log::error;
use crate::emu;
use crate::winapi::helper;
use crate::emu::object_handle;
use crate::emu::object_handle::{windows_to_emulate_path, HANDLE_MANGEMENT, FileHandle};
use crate::emu::object_handle::file_handle::INVALID_HANDLE_VALUE;

pub fn CreateFileW(emu: &mut emu::Emu) {
    let lp_file_name_wide = emu.regs().rcx as u64;
    let dw_desired_access = emu.regs().rdx as u32;
    let dw_share_mode = emu.regs().r8 as u32;
    let lp_security_attributes = emu.regs().r9 as u64; // LPSECURITY_ATTRIBUTES
    let dw_creation_disposition = emu.regs().r10 as u32;
    let dw_flags_and_attributes = emu.regs().r11 as u32;

    log_red!(emu, "** {} kernel32!CreateFileW lp_file_name: 0x{:x} dw_desired_access: 0x{:x} dw_share_mode: 0x{:x} lp_security_attributes: 0x{:x} dw_creation_disposition: 0x{:x} dw_flags_and_attributes: 0x{:x}",
             emu.pos, lp_file_name_wide, dw_desired_access, dw_share_mode, lp_security_attributes, dw_creation_disposition, dw_flags_and_attributes);

    let name_utf8 =
        if lp_file_name_wide > 0 {
            emu.maps.read_wide_string(lp_file_name_wide)
        } else {
            String::new()
        };

    log_red!(emu, "** {} kernel32!CreateFileW name = {} {}", emu.pos, name_utf8, emu.colors.nc);

    // Map the Windows path to the emulator's path
    let emu_path = windows_to_emulate_path(&name_utf8);
    let emu_path_str = emu_path.to_string_lossy().to_string();

    // Attempt to create or open the file using the FileHandle struct
    match FileHandle::new(
        emu_path_str, // Original name
        dw_desired_access,
        dw_creation_disposition,
        dw_flags_and_attributes,
        dw_share_mode, // Pass the parsed struct
    ) {
        Ok(file_handle) => {
            let mut handle_mgmt = crate::emu::object_handle::HANDLE_MANGEMENT.lock().unwrap();
            let handle_key = handle_mgmt.insert_file_handle(file_handle);
            emu.regs_mut().rax = handle_key as u64;
            log_red!(emu, "** {} kernel32!CreateFileW SUCCESS, handle: 0x{:x}", emu.pos, handle_key);
        },
        Err(e) => {
            error!("CreateFileW failed for '{}': {}", name_utf8, e);
            emu.regs_mut().rax = INVALID_HANDLE_VALUE as u64;
            log_red!(emu, "** {} kernel32!CreateFileW FAILED, returning INVALID_HANDLE_VALUE", emu.pos);
        }
    }
}