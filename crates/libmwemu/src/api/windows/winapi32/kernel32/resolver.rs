use crate::api::windows::common::kernel32 as kernel32_common;
use crate::emu;
use crate::windows::peb::peb32;

pub fn dump_module_iat(emu: &mut emu::Emu, module: &str) {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(module) && flink.export_table_rva > 0 {
            for i in 0..flink.num_of_names {
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
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }
}

pub fn resolve_api_name_in_module(emu: &mut emu::Emu, module: &str, name: &str) -> u64 {
    // API set DLL names (api-ms-win-*, ext-ms-*) are virtual contracts.
    // Resolve by function name globally like 64-bit path does.
    let module_lc = module.to_lowercase();
    if kernel32_common::is_api_set_contract(&module_lc) {
        return resolve_api_name(emu, name);
    }

    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(&module_lc) {
            if flink.export_table_rva > 0 {
                for i in 0..flink.num_of_names {
                    if flink.pe_hdr == 0 {
                        continue;
                    }

                    let ordinal = flink.get_function_ordinal(emu, i);
                    if ordinal.func_name == name {
                        return ordinal.func_va;
                    }
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

pub fn resolve_api_addr_to_name(emu: &mut emu::Emu, addr: u64) -> String {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_names {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_va == addr {
                    return ordinal.func_name.to_string();
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    String::new()
}

pub fn resolve_api_ordinal_in_module(emu: &mut emu::Emu, module: &str, ordinal: u16) -> u64 {
    let module_lc = module.to_lowercase();
    if kernel32_common::is_api_set_contract(&module_lc) {
        // API-set contracts are virtual DLLs whose exports are typically
        // provided by kernelbase.dll (and as fallback kernel32.dll).
        // Try the most likely host modules by ordinal, mirroring the
        // 64-bit resolver behavior.
        let addr = resolve_ordinal_in_module_impl(emu, "kernelbase.dll", ordinal);
        if addr != 0 {
            return addr;
        }
        return resolve_ordinal_in_module_impl(emu, "kernel32.dll", ordinal);
    }

    resolve_ordinal_in_module_impl(emu, &module_lc, ordinal)
}

fn resolve_ordinal_in_module_impl(emu: &mut emu::Emu, module_hint: &str, ordinal: u16) -> u64 {
    // NOTE: The 32-bit resolver does not (yet) implement PE export
    // forwarder string resolution the way the 64-bit resolver does.
    // Most PE32 binaries in the test corpus do not exercise forwarded
    // exports via this path, so this is a known limitation that can
    // be addressed later. Forwarder API-set / ordinal parity with x64
    // is intentionally not yet wired in here.
    let want = module_hint.trim().to_lowercase();
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(&want) && flink.export_table_rva > 0 {
            if flink.pe_hdr == 0 {
                flink.next(emu);
                if flink.get_ptr() == first_ptr {
                    break;
                }
                continue;
            }

            // Read export directory fields
            let ordinal_base = emu.maps.read_dword(flink.export_table + 0x10).unwrap_or(0) as u64;
            let num_of_funcs = flink.num_of_funcs;
            let func_addr_tbl_rva =
                emu.maps.read_dword(flink.export_table + 0x1c).unwrap_or(0) as u64;
            let func_addr_tbl = func_addr_tbl_rva + flink.mod_base;

            let idx = ordinal as u64;
            if idx >= ordinal_base && idx < ordinal_base + num_of_funcs {
                let func_idx = idx - ordinal_base;
                if func_idx < num_of_funcs {
                    let func_rva = emu
                        .maps
                        .read_dword(func_addr_tbl + 4 * func_idx)
                        .unwrap_or(0) as u64;
                    if func_rva != 0 {
                        let func_va = func_rva + flink.mod_base;
                        if func_va != flink.mod_base {
                            return func_va;
                        }
                    }
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

pub fn resolve_api_name(emu: &mut emu::Emu, name: &str) -> u64 {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_names {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name == name {
                    return ordinal.func_va;
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

pub fn search_api_name(emu: &mut emu::Emu, name: &str) -> (u64, String, String) {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_names {
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
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    (0, String::new(), String::new())
}

pub fn guess_api_name(emu: &mut emu::Emu, addr: u32) -> String {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_names {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_va == addr as u64 {
                    let lib = flink
                        .mod_name
                        .rsplit_once('.')
                        .map(|(name, _)| name)
                        .unwrap_or(&flink.mod_name);
                    return format!("{}!{}", lib, ordinal.func_name);
                }
            }
        }

        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    String::new()
}
