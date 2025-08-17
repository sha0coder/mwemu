use crate::{emu, structures};

pub fn GetVersionExA(emu: &mut emu::Emu) {
    let version_info_ptr = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!GetVersionExA 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        version_info_ptr,
        emu.colors.nc
    );

    // Check if pointer is valid
    if version_info_ptr == 0 || !emu.maps.is_mapped(version_info_ptr) {
        log::info!("{}** {} GetVersionExA: Invalid pointer {}", 
                  emu.colors.light_red, emu.pos, emu.colors.nc);
        emu.regs_mut().rax = 0;
        return;
    }

    // Read the dwOSVersionInfoSize field (first 4 bytes) to determine structure type
    let struct_size = emu.maps.read_dword(version_info_ptr).expect("Cannot read struct size");
    
    log::info!(
        "{}** {} GetVersionExA: Structure size: {} {}",
        emu.colors.light_red,
        emu.pos,
        struct_size,
        emu.colors.nc
    );

    // Determine which structure type based on size
    const OSVERSIONINFOA_SIZE: u32 = 148;        // Basic structure
    const OSVERSIONINFOEXA_SIZE: u32 = 284;      // Extended structure

    let use_extended = match struct_size {
        OSVERSIONINFOA_SIZE => {
            log::info!("{}** {} Using OSVERSIONINFOA (basic) {}", 
                      emu.colors.light_red, emu.pos, emu.colors.nc);
            false
        },
        OSVERSIONINFOEXA_SIZE => {
            log::info!("{}** {} Using OSVERSIONINFOEXA (extended) {}", 
                      emu.colors.light_red, emu.pos, emu.colors.nc);
            true
        },
        _ => {
            log::info!(
                "{}** {} GetVersionExA: Invalid struct size: {} (expected {} or {}) {}",
                emu.colors.light_red,
                emu.pos,
                struct_size,
                OSVERSIONINFOA_SIZE,
                OSVERSIONINFOEXA_SIZE,
                emu.colors.nc
            );
            emu.regs_mut().rax = 0;
            return;
        }
    };

    if use_extended {
        let os_version_info = structures::OsVersionInfo::new();
        os_version_info.save(version_info_ptr, &mut emu.maps);
    } else {
        panic!("TODO");
    }

    emu.regs_mut().rax = 1;
}
