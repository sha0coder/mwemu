use crate::api::windows::common::kernel32 as kernel32_common;
use crate::emu;
use crate::windows::peb::peb64;

pub fn is_library_loaded(emu: &mut emu::Emu, libname: &str) -> bool {
    let Some(dll) = kernel32_common::normalize_library_name(libname, false) else {
        return false;
    };
    peb64::get_module_base(&dll, emu).is_some()
}

pub fn load_library(emu: &mut emu::Emu, libname: &str) -> u64 {
    let Some(mut dll) = kernel32_common::normalize_library_name(libname, false) else {
        emu.regs_mut().rax = 0;
        return 0;
    };

    // API set DLL names are virtual contracts that map to host DLLs.
    if kernel32_common::is_api_set_contract(&dll) {
        if peb64::get_module_base("kernelbase.dll", emu).is_some() {
            return peb64::get_module_base("kernelbase.dll", emu).unwrap_or(0);
        }
        if peb64::get_module_base("kernel32.dll", emu).is_some() {
            return peb64::get_module_base("kernel32.dll", emu).unwrap_or(0);
        }
        dll = "kernelbase.dll".to_string();
    }

    let mut dll_path = emu.cfg.maps_folder.clone();
    dll_path.push_str(&dll);

    match peb64::get_module_base(&dll, emu) {
        Some(base) => {
            if emu.cfg.verbose > 0 {
                log::trace!("dll {} already linked.", dll);
            }
            base
        }
        None => {
            let path = std::path::Path::new(&dll_path);
            if path.try_exists().unwrap() {
                let (base, pe_off) = emu.load_pe64(&dll_path, false, 0);
                peb64::dynamic_link_module(base, pe_off, &dll, emu);
                emu.library_loaded = true; // Signal to GDB that library list changed
                base
            } else {
                log::trace!("dll {} not found.", dll_path);
                0
            }
        }
    }
}

pub fn get_library_handle(emu: &mut emu::Emu, libname: &str) -> u64 {
    let Some(dll) = kernel32_common::normalize_library_name(libname, false) else {
        emu.regs_mut().rax = 0;
        return 0;
    };

    peb64::get_module_base(&dll, emu).unwrap_or(0)
}
