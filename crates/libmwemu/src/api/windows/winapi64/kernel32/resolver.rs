use crate::api::windows::common::kernel32 as kernel32_common;
use crate::emu;
use crate::windows::peb::peb64;

pub fn dump_module_iat(emu: &mut emu::Emu, module: &str) {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(module) && flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                log::trace!(
                    "0x{:x} {}!{}",
                    ordinal.func_va,
                    &flink.mod_name,
                    &ordinal.func_name
                );
            }
        }
        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }
}

pub fn resolve_api_addr_to_name(emu: &mut emu::Emu, addr: u64) -> String {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_va == addr {
                    return ordinal.func_name.to_string();
                }
            }
        }
        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    String::new()
}

fn resolve_in_module_exports_depth(
    emu: &mut emu::Emu,
    module_hint: &str,
    name: &str,
    depth: u32,
) -> u64 {
    if depth > 8 {
        return 0;
    }
    let want = module_hint.trim().to_lowercase();
    let name_lc = name.to_lowercase();

    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();
    loop {
        if flink.export_table_rva > 0 && kernel32_common::module_name_matches(&flink.mod_name, &want) {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let func_name_rva = emu
                    .maps
                    .read_dword(flink.func_name_tbl + i * 4)
                    .unwrap_or(0) as u64;
                let export_name = emu.maps.read_string(func_name_rva + flink.mod_base);
                if export_name.to_lowercase() != name_lc {
                    continue;
                }

                let ordinal = flink.get_function_ordinal_depth(emu, i, depth);
                return ordinal.func_va;
            }
        }
        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

/// Resolve `name` preferring the module named in the PE import table (`KERNEL32.DLL`, etc.).
/// API-set / `ext-ms-*` names are mapped to the backing DLLs used in `maps/windows/` (see `get_dependencies`).
pub fn resolve_api_name_in_module(emu: &mut emu::Emu, module: &str, name: &str) -> u64 {
    let module_lc = module.trim().to_lowercase();

    if kernel32_common::is_api_set_contract(&module_lc) {
        let addr = resolve_in_module_exports_depth(emu, "kernelbase.dll", name, 0);
        if addr != 0 {
            return addr;
        }
        let addr = resolve_in_module_exports_depth(emu, "kernel32.dll", name, 0);
        if addr != 0 {
            return addr;
        }
        return 0;
    }

    let addr = resolve_in_module_exports_depth(emu, &module_lc, name, 0);
    if addr != 0 {
        return addr;
    }
    resolve_api_name(emu, name)
}

/// Resolve a PE export forwarder string (`KERNELBASE.QueryPerformanceCounter`).
pub fn resolve_forwarded_export_string(emu: &mut emu::Emu, forwarder: &str) -> u64 {
    resolve_forwarded_export_string_depth(emu, forwarder, 0)
}

pub(crate) fn resolve_forwarded_export_string_depth(
    emu: &mut emu::Emu,
    forwarder: &str,
    inner_depth: u32,
) -> u64 {
    if inner_depth > 8 {
        return 0;
    }
    let forwarder = forwarder.trim();
    let Some(dot) = forwarder.find('.') else {
        return 0;
    };
    let dll_part = forwarder[..dot].trim();
    let sym_part = forwarder[dot + 1..].trim();
    if dll_part.is_empty() || sym_part.is_empty() {
        return 0;
    }
    let Some(dll) = kernel32_common::normalize_library_name(dll_part, false) else {
        return 0;
    };
    // Avoid re-entering the PE loader during export resolution (can recurse from delay-load binding).
    // Forwarders should resolve against already-linked modules in the current LDR state.
    let mapped_dll = if kernel32_common::is_api_set_contract(&dll) {
        "kernelbase.dll".to_string()
    } else {
        dll
    };
    if peb64::get_module_base(&mapped_dll, emu).is_none() {
        return 0;
    }
    resolve_in_module_exports_depth(emu, &mapped_dll, sym_part, inner_depth)
}

pub fn resolve_api_name(emu: &mut emu::Emu, name: &str) -> u64 {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();
    let name_lc = name.to_lowercase();
    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name.to_lowercase() == name_lc {
                    return ordinal.func_va;
                }
            }
        }

        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

pub fn search_api_name(emu: &mut emu::Emu, name: &str) -> (u64, String, String) {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name.contains(name) {
                    return (
                        ordinal.func_va,
                        flink.mod_name.clone(),
                        ordinal.func_name.clone(),
                    );
                }
            }
        }
        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    (0, String::new(), String::new())
}

pub fn guess_api_name(emu: &mut emu::Emu, addr: u64) -> String {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);

                if ordinal.func_va == addr {
                    let lib = flink
                        .mod_name
                        .rsplit_once('.')
                        .map(|(name, _)| name)
                        .unwrap_or(&flink.mod_name);

                    return format!("{}!{}", lib, ordinal.func_name);
                }
            }
        }

        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    String::new()
}
