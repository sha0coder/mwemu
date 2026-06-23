use std::collections::HashMap;

use crate::emu;
use crate::loaders::pe::readers::{
    read_u64_le as read_u64_le_shared, write_u64_le as write_u64_le_shared,
};
use crate::winapi::winapi64;

use crate::loaders::pe::pe32::HintNameItem;
use super::PE64;

macro_rules! read_u64_le {
    ($raw:expr, $off:expr) => {
        read_u64_le_shared(($raw).as_ref(), $off)
    };
}

macro_rules! write_u64_le {
    ($raw:expr, $off:expr, $val:expr) => {
        write_u64_le_shared(($raw).as_mut(), $off, $val)
    };
}

impl PE64 {
    pub(crate) fn pe64_delay_load_binding(&mut self, emu: &mut emu::Emu, base_addr: u64) {
        let mut resolved_cache: HashMap<String, u64> = HashMap::new();

        for i in 0..self.delay_load_dir.len() {
            let dld = &self.delay_load_dir[i];
            if dld.name.is_empty() {
                continue;
            }

            let mut off_name = PE64::vaddr_to_off(&self.sect_hdr, dld.name_table) as usize;
            let mut off_addr = PE64::vaddr_to_off(&self.sect_hdr, dld.address_table) as usize;
            // RVA of the current Delay Import Address Table slot. This is the
            // location to patch in process memory; it advances in lock-step with
            // `off_addr`. The previous code patched `base_addr + <slot contents>`
            // instead of `base_addr + <slot RVA>` — for pre-bound delay-load
            // tables (advapi32, rpcrt4) the slot contents pointed back into the
            // PE header page, so the resolved address overwrote the MZ magic and
            // ntdll rejected the freshly-mapped image with STATUS_INVALID_IMAGE_FORMAT.
            let mut slot_rva = dld.address_table;

            loop {
                if self.raw.len() <= off_name + 4 || self.raw.len() <= off_addr + 4 {
                    break;
                }

                let hint = HintNameItem::load(&self.raw, off_name);
                let off2 = PE64::vaddr_to_off(&self.sect_hdr, hint.func_name_addr) as usize;
                if off2 == 0 {
                    off_name += HintNameItem::size();
                    off_addr += 8;
                    slot_rva += 8;
                    continue;
                }

                let func_name = PE64::read_string(&self.raw, off2 + 2);
                let cache_key =
                    format!("{}!{}", dld.name.to_lowercase(), func_name.to_lowercase());
                let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
                    *cached
                } else {
                    let resolved =
                        winapi64::kernel32::resolve_api_name_in_module(emu, &dld.name, &func_name);
                    resolved_cache.insert(cache_key, resolved);
                    resolved
                };
                if real_addr == 0 {
                    break;
                }

                write_u64_le!(self.raw, off_addr, real_addr);
                let patch_addr = base_addr + slot_rva as u64;
                if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
                    mem.force_write_qword(patch_addr, real_addr);
                }

                off_name += HintNameItem::size();
                off_addr += 8;
                slot_rva += 8;
            }
        }
    }

    pub(crate) fn pe64_get_dependencies(&mut self, _emu: &mut emu::Emu) -> Vec<String> {
        let mut dependencies: Vec<String> = Vec::new();

        for i in 0..self.image_import_descriptor.len() {
            let iim = &self.image_import_descriptor[i];
            if iim.name.is_empty() {
                continue;
            }

            let mut libname = iim.name.clone();
            if iim.name.starts_with("api-ms-win-") {
                libname = "kernelbase".to_string();
            }

            dependencies.push(libname);
        }

        dependencies
    }

    pub(crate) fn pe64_iat_binding(&mut self, emu: &mut emu::Emu, base_addr: u64) {
        if emu.cfg.verbose >= 1 {
            log::trace!(
                "IAT binding started image_import_descriptor.len() = {} ...",
                self.image_import_descriptor.len()
            );
        }

        let mut resolved_cache: HashMap<String, u64> = HashMap::new();

        for i in 0..self.image_import_descriptor.len() {
            let iim = &self.image_import_descriptor[i];

            if iim.name.is_empty() {
                continue;
            }

            let import_dll = iim.name.clone();
            let original_first_thunk = iim.original_first_thunk;
            let first_thunk = iim.first_thunk;

            if winapi64::kernel32::load_library(emu, &import_dll) == 0 {
                // API-set contract DLLs (`api-ms-win-*`, `ext-ms-*`) are virtual
                // names, not real files: a given build only ships some versioned
                // stubs (e.g. `...libraryloader-l1-1-0.dll`) yet binaries import
                // other versions (`...-l1-2-0.dll`). When the stub file is absent
                // we must NOT skip the descriptor — its functions still resolve to
                // the backing DLLs (kernelbase/kernel32/…) via the per-function
                // resolver below. Skipping would leave the whole import group
                // (GetModuleHandleW, GetProcAddress, …) unbound and crash on use.
                if !crate::api::windows::common::kernel32::is_api_set_contract(&import_dll) {
                    if emu.cfg.verbose >= 1 {
                        log::trace!(
                            "cannot find/import library `{}` (IAT binding will skip it)",
                            &import_dll
                        );
                    }
                    continue;
                }
            }

            if original_first_thunk == 0 {
                self.pe64_iat_binding_alternative(
                    emu,
                    base_addr,
                    first_thunk,
                    &import_dll,
                    &mut resolved_cache,
                );
            } else {
                self.pe64_iat_binding_original(
                    emu,
                    base_addr,
                    original_first_thunk,
                    first_thunk,
                    &import_dll,
                    &mut resolved_cache,
                );
            }
        }
    }

    pub(crate) fn pe64_iat_binding_alternative(
        &mut self,
        emu: &mut emu::Emu,
        base_addr: u64,
        first_thunk: u32,
        import_dll: &str,
        resolved_cache: &mut HashMap<String, u64>,
    ) {
        let mut rva = first_thunk;
        let mut unresolved = 0u32;

        loop {
            let off = PE64::vaddr_to_off(&self.sect_hdr, rva) as usize;
            if self.raw.len() <= off + 8 {
                break;
            }

            let func_name_addr_or_ordinal = read_u64_le!(self.raw, off);
            if func_name_addr_or_ordinal == 0 {
                break;
            }

            let is_ordinal = (func_name_addr_or_ordinal & 0x80000000_00000000) != 0;
            if is_ordinal {
                let ordinal = (func_name_addr_or_ordinal & 0xFFFF) as u16;
                println!("---- ordinal: {}", ordinal);
                unimplemented!("third variation of iat binding not implemented");
            } else {
                let func_name_addr =
                    (func_name_addr_or_ordinal & 0x7fff_ffff_ffff_ffff) as u32;
                let off_name = PE64::vaddr_to_off(&self.sect_hdr, func_name_addr) as usize;
                let api_name = PE64::read_string(&self.raw, off_name + 2);

                let cache_key =
                    format!("{}!{}", import_dll.to_lowercase(), api_name.to_lowercase());
                let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
                    *cached
                } else {
                    let resolved =
                        winapi64::kernel32::resolve_api_name_in_module(emu, import_dll, &api_name);
                    resolved_cache.insert(cache_key, resolved);
                    resolved
                };

                if real_addr > 0 {
                    write_u64_le!(self.raw, off, real_addr);
                    let patch_addr = base_addr + rva as u64;
                    if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
                        mem.force_write_qword(patch_addr, real_addr);
                    }
                } else {
                    unresolved += 1;
                    // Per-symbol detail is noisy (apiset CRT forwarders alone are
                    // hundreds of names), so keep it at -vv; -v gets one summary
                    // line per DLL below.
                    if emu.cfg.verbose >= 2 {
                        log::trace!(
                            "unresolved import {}!{} (IAT rva 0x{:x})",
                            import_dll,
                            api_name,
                            rva
                        );
                    }
                }
            }

            rva += 8;
        }

        // apiset contract DLLs ("phantom" forwarders like `api-ms-win-*`) routinely
        // carry unresolved imports — that's expected noise, not a real problem — so
        // only summarize those at -vv. Genuine DLLs get the summary at -v.
        let summary_threshold =
            if crate::api::windows::common::kernel32::is_api_set_contract(import_dll) {
                2
            } else {
                1
            };
        if unresolved > 0 && emu.cfg.verbose >= summary_threshold {
            log::trace!(
                "{} unresolved imports from {} (use -vv to list them)",
                unresolved,
                import_dll
            );
        }
    }

    pub(crate) fn pe64_iat_binding_original(
        &mut self,
        emu: &mut emu::Emu,
        base_addr: u64,
        original_first_thunk: u32,
        first_thunk: u32,
        import_dll: &str,
        resolved_cache: &mut HashMap<String, u64>,
    ) {
        let mut off_name = PE64::vaddr_to_off(&self.sect_hdr, original_first_thunk) as usize;
        let mut off_addr = PE64::vaddr_to_off(&self.sect_hdr, first_thunk) as usize;
        let mut rva = first_thunk;
        let mut unresolved = 0u32;

        loop {
            if self.raw.len() <= off_name + 8 || self.raw.len() <= off_addr + 8 {
                break;
            }

            let thunk_data = read_u64_le!(self.raw, off_name);
            if thunk_data == 0 {
                break;
            }

            let is_ordinal = (thunk_data & 0x80000000_00000000) != 0;
            if is_ordinal {
                off_name += 8;
                off_addr += 8;
                rva += 8;
                continue;
            }

            let func_name_addr = (thunk_data & 0x7fff_ffff_ffff_ffff) as u32;
            let off2 = PE64::vaddr_to_off(&self.sect_hdr, func_name_addr) as usize;
            if off2 == 0 {
                off_name += 8;
                off_addr += 8;
                rva += 8;
                continue;
            }

            let func_name = PE64::read_string(&self.raw, off2 + 2);
            let cache_key =
                format!("{}!{}", import_dll.to_lowercase(), func_name.to_lowercase());
            let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
                *cached
            } else {
                let resolved =
                    winapi64::kernel32::resolve_api_name_in_module(emu, import_dll, &func_name);
                resolved_cache.insert(cache_key, resolved);
                resolved
            };

            if real_addr != 0 {
                write_u64_le!(self.raw, off_addr, real_addr);
                let patch_addr = base_addr + rva as u64;
                if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
                    mem.force_write_qword(patch_addr, real_addr);
                }
            } else {
                unresolved += 1;
                if emu.cfg.verbose >= 2 {
                    log::trace!(
                        "unresolved import {}!{} (IAT rva 0x{:x})",
                        import_dll,
                        func_name,
                        rva
                    );
                }
            }

            off_name += 8;
            off_addr += 8;
            rva += 8;
        }

        // apiset contract DLLs ("phantom" forwarders like `api-ms-win-*`) routinely
        // carry unresolved imports — that's expected noise, not a real problem — so
        // only summarize those at -vv. Genuine DLLs get the summary at -v.
        let summary_threshold =
            if crate::api::windows::common::kernel32::is_api_set_contract(import_dll) {
                2
            } else {
                1
            };
        if unresolved > 0 && emu.cfg.verbose >= summary_threshold {
            log::trace!(
                "{} unresolved imports from {} (use -vv to list them)",
                unresolved,
                import_dll
            );
        }
    }

    pub(crate) fn pe64_import_addr_to_name(&self, paddr: u64) -> String {
        if paddr == 0 {
            return String::new();
        }

        for i in 0..self.image_import_descriptor.len() {
            let iim = &self.image_import_descriptor[i];

            if iim.name.is_empty() {
                continue;
            }

            let thunk_names_rva = if iim.original_first_thunk != 0 {
                iim.original_first_thunk
            } else {
                iim.first_thunk
            };
            let mut off_name = PE64::vaddr_to_off(&self.sect_hdr, thunk_names_rva) as usize;
            let mut off_addr = PE64::vaddr_to_off(&self.sect_hdr, iim.first_thunk) as usize;

            loop {
                if self.raw.len() <= off_name + 8 || self.raw.len() <= off_addr + 8 {
                    break;
                }

                let thunk_data = read_u64_le!(self.raw, off_name);
                if thunk_data == 0 {
                    break;
                }

                let addr = read_u64_le!(self.raw, off_addr);
                let is_ordinal = (thunk_data & 0x80000000_00000000) != 0;
                if !is_ordinal {
                    let func_name_addr = (thunk_data & 0x7fff_ffff_ffff_ffff) as u32;
                    let off2 = PE64::vaddr_to_off(&self.sect_hdr, func_name_addr) as usize;
                    if off2 != 0 && addr == paddr {
                        let func_name = PE64::read_string(&self.raw, off2 + 2);
                        return func_name;
                    }
                }

                off_name += 8;
                off_addr += 8;
            }
        }

        String::new()
    }

    pub(crate) fn pe64_import_addr_to_dll_and_name(&self, paddr: u64) -> String {
        if paddr == 0 {
            return String::new();
        }

        for iim in &self.image_import_descriptor {
            if iim.name.is_empty() {
                continue;
            }

            let thunk_names_rva = if iim.original_first_thunk != 0 {
                iim.original_first_thunk
            } else {
                iim.first_thunk
            };
            let mut off_name = PE64::vaddr_to_off(&self.sect_hdr, thunk_names_rva) as usize;
            let mut off_addr = PE64::vaddr_to_off(&self.sect_hdr, iim.first_thunk) as usize;

            loop {
                if self.raw.len() <= off_name + 8 || self.raw.len() <= off_addr + 8 {
                    break;
                }
                let thunk_data = read_u64_le!(self.raw, off_name);
                if thunk_data == 0 {
                    break;
                }
                let addr = read_u64_le!(self.raw, off_addr);

                let is_ordinal = (thunk_data & 0x80000000_00000000) != 0;
                if !is_ordinal {
                    let func_name_addr = (thunk_data & 0x7fff_ffff_ffff_ffff) as u32;
                    let off2 = PE64::vaddr_to_off(&self.sect_hdr, func_name_addr) as usize;
                    if off2 != 0 && addr == paddr {
                        let func_name = PE64::read_string(&self.raw, off2 + 2);
                        return format!("{}!{}", iim.name, func_name);
                    }
                }

                off_name += 8;
                off_addr += 8;
            }
        }

        String::new()
    }
}
