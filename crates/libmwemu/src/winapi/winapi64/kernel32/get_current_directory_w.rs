
use crate::emu;

pub fn GetCurrentDirectoryW(emu: &mut emu::Emu) {
    let buff_len = emu.regs().rcx as u32;  // nBufferLength (DWORD) - FIRST parameter
    let buff_ptr = emu.regs().rdx;         // lpBuffer (LPWSTR) - SECOND parameter

    log::info!(
        "{}** {} kernel32!GetCurrentDirectoryW nBufferLength: {} lpBuffer: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        buff_len,
        buff_ptr,
        emu.colors.nc
    );

    // Current directory as a proper wide string
    let current_dir = "C:\\";  // Simple current directory
    let required_length = current_dir.len() + 1; // +1 for null terminator

    if buff_ptr == 0 {
        // If buffer is NULL, return required size
        emu.regs_mut().rax = required_length as u64;
        return;
    }

    if !emu.maps.is_mapped(buff_ptr) {
        log::error!("GetCurrentDirectoryW: lpBuffer 0x{:x} is not mapped", buff_ptr);
        emu.regs_mut().rax = 0; // Failure
        return;
    }

    if buff_len == 0 {
        // If buffer length is 0, return required size
        emu.regs_mut().rax = required_length as u64;
        return;
    }

    if (buff_len as usize) < required_length {
        // Buffer too small, return required size (including null terminator)
        emu.regs_mut().rax = required_length as u64;
        return;
    }

    // Buffer is large enough, write the directory
    emu.maps.write_wide_string(buff_ptr, current_dir);

    log::info!(
        "{}** {} GetCurrentDirectoryW returning: '{}' (length: {}) {}",
        emu.colors.light_red,
        emu.pos,
        current_dir,
        current_dir.len(),
        emu.colors.nc
    );

    // Return number of characters written (NOT including null terminator)
    emu.regs_mut().rax = current_dir.len() as u64;
}