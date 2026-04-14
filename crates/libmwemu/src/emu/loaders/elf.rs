use std::collections::HashMap;
use std::path::Path;

use crate::emu::Emu;
use crate::loaders::elf::elf64::Elf64;
use crate::windows::constants;

impl Emu {
    /// Loads an ELF64 parsing sections etc, powered by elf64.rs
    /// This is called from load_code() if the sample is ELF64
    pub fn load_elf64(&mut self, filename: &str) {
        let mut elf64 = Elf64::parse(filename).unwrap();
        let dyn_link = !elf64.get_dynamic().is_empty();

        if dyn_link {
            log::trace!("dynamic elf64 detected.");
        } else {
            log::trace!("static elf64 detected.");
        }

        elf64.load(
            &mut self.maps,
            "elf64",
            false,
            dyn_link,
            self.cfg.code_base_addr,
        );
        if self.cfg.arch.is_aarch64() {
            self.init_linux64_aarch64();
        } else {
            self.init_linux64(dyn_link);
        }

        // --- Dynamic linking: load stub libs and apply relocations ---
        if dyn_link {
            self.load_elf64_dynamic_libs(&mut elf64);
        }

        // Get .text addr and size
        let mut text_addr: u64 = 0;
        let mut text_sz = 0;
        for i in 0..elf64.elf_shdr.len() {
            let sname = elf64.get_section_name(elf64.elf_shdr[i].sh_name as usize);
            if sname == ".text" {
                text_addr = elf64.elf_shdr[i].sh_addr;
                text_sz = elf64.elf_shdr[i].sh_size;
                break;
            }
        }

        if text_addr == 0 {
            panic!(".text not found on this elf64");
        }

        // entry point logic:

        // 1. Configured entry point
        if self.cfg.entry_point != constants::CFG_DEFAULT_BASE {
            log::trace!("forcing entry point to 0x{:x}", self.cfg.entry_point);
            self.set_pc(self.cfg.entry_point);

        // 2. Entry point pointing inside .text
        } else if elf64.elf_hdr.e_entry >= text_addr && elf64.elf_hdr.e_entry < text_addr + text_sz
        {
            log::trace!(
                "Entry point pointing to .text 0x{:x}",
                elf64.elf_hdr.e_entry
            );
            self.set_pc(elf64.elf_hdr.e_entry);

        // 3. Entry point points above .text, relative entry point
        } else if elf64.elf_hdr.e_entry < text_addr {
            self.set_pc(elf64.elf_hdr.e_entry + elf64.base);
            log::trace!(
                "relative entry point: 0x{:x}  fixed: 0x{:x}",
                elf64.elf_hdr.e_entry,
                self.pc()
            );

        // 4. Entry point points below .text, weird case.
        } else {
            panic!(
                "Entry points is pointing below .text 0x{:x}",
                elf64.elf_hdr.e_entry
            );
        }

        // Write the Linux initial stack layout (argc, argv, envp, auxv)
        // so _start can read argc/argv and __libc_start_main gets proper args.
        let phdr_addr = elf64.base + elf64.elf_hdr.e_phoff;
        self.write_linux_stack_layout(
            self.pc(),
            phdr_addr,
            elf64.elf_hdr.e_phentsize,
            elf64.elf_hdr.e_phnum,
        );

        self.elf64 = Some(elf64);
    }

    /// Load dynamic libraries for an ELF64 binary.
    /// Loads real stub ELFs from disk, then applies relocations.
    fn load_elf64_dynamic_libs(&mut self, elf64: &mut Elf64) {
        let mut export_map: HashMap<String, u64> = HashMap::new();

        let libs = elf64.get_dynamic();
        elf64.needed_libs = libs.clone();

        for lib in &libs {
            log::trace!("dynamic library {}", lib);

            let Some(local_path) = self.resolve_linux_stub_path(lib) else {
                log::warn!("elf64: could not locate linux stub library {}", lib);
                continue;
            };

            let mut elflib = match Elf64::parse(&local_path) {
                Ok(lib) => lib,
                Err(err) => {
                    log::warn!("elf64: failed to parse {}: {}", local_path, err);
                    continue;
                }
            };

            let map_name = lib.rsplit('/').next().unwrap_or(lib);
            elflib.load(&mut self.maps, map_name, true, true, constants::CFG_DEFAULT_BASE);

            for (sym, addr) in elflib.exported_symbols() {
                export_map.entry(sym.clone()).or_insert(addr);
                elf64.addr_to_symbol.insert(addr, sym.clone());
                elf64.sym_to_addr.insert(sym, addr);
            }
        }

        if !export_map.is_empty() {
            if self.cfg.arch.is_aarch64() {
                elf64.apply_rela_aarch64(&mut self.maps, &export_map);
            } else {
                let unresolved = elf64.apply_dynamic_relocations(&mut self.maps, &export_map);
                if !unresolved.is_empty() {
                    log::warn!("elf64: unresolved dynamic imports: {:?}", unresolved);
                }
            }
        }
    }

    fn resolve_linux_stub_path(&self, lib_name: &str) -> Option<String> {
        let mut candidates = Vec::new();

        if !self.cfg.maps_folder.is_empty() {
            candidates.push(self.cfg.get_maps_folder(lib_name));
        }

        if self.cfg.arch.is_aarch64() {
            candidates.push(format!("maps/linux/aarch64/{}", lib_name));
            candidates.push(format!("../../maps/linux/aarch64/{}", lib_name));
        } else {
            candidates.push(format!("maps/linux/x86_64/{}", lib_name));
            candidates.push(format!("../../maps/linux/x86_64/{}", lib_name));
        }

        candidates
            .into_iter()
            .find(|candidate| Path::new(candidate).exists())
    }
}
