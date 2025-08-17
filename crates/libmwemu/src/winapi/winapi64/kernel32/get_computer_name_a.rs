
use crate::{constants, emu, winapi::winapi64::kernel32::set_last_error};

pub fn GetComputerNameA(emu: &mut emu::Emu) {
    let buff_ptr = emu.regs().rcx;   // LPSTR lpBuffer
    let size_ptr = emu.regs().rdx;   // LPDWORD nSize

    log::info!(
        "{}** {} kernel32!GetComputerNameA lpBuffer: 0x{:x} nSize: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        buff_ptr,
        size_ptr,
        emu.colors.nc
    );

    // Check if size pointer is valid
    if size_ptr == 0 || !emu.maps.is_mapped(size_ptr) {
        log::info!("{}** {} GetComputerNameA: Invalid nSize pointer {}", 
                  emu.colors.light_red, emu.pos, emu.colors.nc);
        set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read current buffer size (in bytes)
    let buffer_size = emu.maps.read_dword(size_ptr).expect("Cannot read buffer size") as usize;
    
    // Calculate required size in bytes
    let computer_name_bytes = constants::HOST_NAME.len();
    let required_size_with_null = computer_name_bytes + 1; // +1 for null terminator

    // Check if output buffer is valid (only if buffer_size > 0)
    if buffer_size > 0 && (buff_ptr == 0 || !emu.maps.is_mapped(buff_ptr)) {
        log::info!("{}** {} GetComputerNameA: Invalid lpBuffer pointer {}", 
                  emu.colors.light_red, emu.pos, emu.colors.nc);
        set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Check if buffer is large enough
    if buffer_size < required_size_with_null {
        log::info!(
            "{}** {} GetComputerNameA: Buffer too small. Required: {}, Provided: {} {}",
            emu.colors.light_red,
            emu.pos,
            required_size_with_null,
            buffer_size,
            emu.colors.nc
        );
        // Set size to required size (including null terminator)
        emu.maps.write_dword(size_ptr, required_size_with_null as u32);
        set_last_error(constants::ERROR_BUFFER_OVERFLOW);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Buffer is large enough, write the computer name
    emu.maps.write_string(buff_ptr, constants::HOST_NAME);
    
    // On success, write the number of bytes copied (NOT including null terminator)
    emu.maps.write_dword(size_ptr, computer_name_bytes as u32);

    log::info!(
        "{}** {} kernel32!GetComputerNameA returning: '{}' (bytes: {}) {}",
        emu.colors.light_red,
        emu.pos,
        constants::HOST_NAME,
        computer_name_bytes,
        emu.colors.nc
    );

    emu.regs_mut().rax = constants::TRUE;
}
