use crate::{constants, emu, winapi::winapi64::kernel32::set_last_error};

pub fn GetModuleFileNameW(emu: &mut emu::Emu) {
    let module_handle = emu.regs().rcx;
    let lp_filename = emu.regs().rdx;
    let n_size = emu.regs().r8 as u32;
    
    log::info!(
        "{}** {} kernel32!GetModuleFileNameW hModule: 0x{:x} lpFilename: 0x{:x} nSize: {} {}",
        emu.colors.light_red,
        emu.pos,
        module_handle,
        lp_filename,
        n_size,
        emu.colors.nc
    );

    // Handle zero size buffer
    if n_size == 0 {
        log::info!(
            "{}** {} GetModuleFileNameW: Zero size buffer {}",
            emu.colors.light_red,
            emu.pos,
            emu.colors.nc
        );
        emu.regs_mut().rax = 0;
        return;
    }

    // Validate buffer pointer
    if lp_filename == 0 || !emu.maps.is_mapped(lp_filename) {
        log::info!(
            "{}** {} GetModuleFileNameW: Invalid buffer pointer {}",
            emu.colors.light_red,
            emu.pos,
            emu.colors.nc
        );
        emu.regs_mut().rax = 0;
        return;
    }

    // Determine which module name to use based on handle
    let module_name = if module_handle == 0 {
        // NULL handle means current process executable
        constants::MODULE_NAME  // or constants::EXE_NAME if you have it
    } else {
        // TODO: Look up actual module by handle
        // For now, just use the default module name
        constants::MODULE_NAME
    };

    let name_chars = module_name.chars().count();
    let required_chars = name_chars + 1; // +1 for null terminator

    if (n_size as usize) < required_chars {
        // Buffer too small - truncate to fit
        let max_chars = (n_size as usize) - 1; // Reserve space for null terminator
        let truncated: String = module_name.chars().take(max_chars).collect();
        
        emu.maps.write_wide_string(lp_filename, &truncated);
        
        log::info!(
            "{}** {} GetModuleFileNameW: Buffer too small, truncated to '{}' {}",
            emu.colors.light_red,
            emu.pos,
            truncated,
            emu.colors.nc
        );
        
        // Set last error for Windows XP+ behavior
        set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = n_size as u64; // Return buffer size when truncated
    } else {
        // Buffer is large enough
        emu.maps.write_wide_string(lp_filename, module_name);
        
        log::info!(
            "{}** {} GetModuleFileNameW: Returning '{}' (length: {}) {}",
            emu.colors.light_red,
            emu.pos,
            module_name,
            name_chars,
            emu.colors.nc
        );
        
        emu.regs_mut().rax = name_chars as u64; // Return actual length (without null terminator)
    }
}
