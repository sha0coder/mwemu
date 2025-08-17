
use crate::{constants, emu};

pub fn GetCurrentDirectoryW(emu: &mut emu::Emu) {
    let buff_len = emu.regs().rcx as u32;
    let buff_ptr = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!GetCurrentDirectoryW nBufferLength: {} lpBuffer: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        buff_len,
        buff_ptr,
        emu.colors.nc
    );

    let current_dir = constants::CWD_PATH;
    let dir_char_count = current_dir.chars().count(); // Use chars().count() for proper Unicode handling
    
    // When buffer length is 0 or buffer is null, return required size INCLUDING null terminator
    if buff_len == 0 || buff_ptr == 0 {
        emu.regs_mut().rax = (dir_char_count + 1) as u64; // +1 for null terminator
        return;
    }

    if !emu.maps.is_mapped(buff_ptr) {
        log::error!("GetCurrentDirectoryW: lpBuffer 0x{:x} is not mapped", buff_ptr);
        emu.regs_mut().rax = 0;
        return;
    }

    // Check if buffer is large enough (need space for string + null terminator)
    if (buff_len as usize) < (dir_char_count + 1) {
        // Return required size INCLUDING null terminator
        emu.regs_mut().rax = (dir_char_count + 1) as u64;
        return;
    }

    // Buffer is large enough, write the directory
    emu.maps.write_wide_string(buff_ptr, current_dir);

    log::info!(
        "{}** {} GetCurrentDirectoryW returning: '{}' (length: {}) {}",
        emu.colors.light_red,
        emu.pos,
        current_dir,
        dir_char_count,
        emu.colors.nc
    );

    // Return number of characters written (NOT including null terminator)
    emu.regs_mut().rax = dir_char_count as u64;
}
