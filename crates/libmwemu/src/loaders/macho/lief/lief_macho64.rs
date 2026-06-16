use std::fs;
use std::path::Path;

use lief::generic::Binary as GenericBinary;
use lief::generic::Symbol as GenericSymbol;
use lief::macho;
use lief::macho::binding_info::AsGeneric as BindingAsGeneric;

use crate::arch::Arch;
use crate::loaders::macho::macho64::{
    ChainedBind, ChainedImport, Macho64, Macho64Segment, ParserBackendKind,
};

use super::error::LiefMachoError;

pub struct LiefMacho64 {
    file_path: String,
    raw: Vec<u8>,
    selected_arch: Arch,
    _temp_file: Option<tempfile::NamedTempFile>,
}

fn validate_file_range(file_len: usize, offset: u64, size: u64) -> Result<(), LiefMachoError> {
    let end = offset.checked_add(size).ok_or_else(|| {
        LiefMachoError::ParseFailed(format!(
            "offset 0x{:x} + size 0x{:x} overflows u64",
            offset, size
        ))
    })?;
    if end > file_len as u64 {
        return Err(LiefMachoError::ParseFailed(format!(
            "range 0x{:x}..0x{:x} exceeds file_len 0x{:x}",
            offset, end, file_len
        )));
    }
    Ok(())
}

impl LiefMacho64 {
    pub fn load(path: &str, preferred_arch: Option<Arch>) -> Result<Self, LiefMachoError> {
        if !Path::new(path).exists() {
            return Err(LiefMachoError::FileNotFound(path.to_string()));
        }

        let raw =
            fs::read(path).map_err(|e| LiefMachoError::ReadFailed(format!("{}: {}", path, e)))?;

        let fat = macho::FatBinary::parse(path).ok_or_else(|| {
            LiefMachoError::ParseFailed(format!("{}: LIEF parse returned None", path))
        })?;

        Self::from_fat(path.to_string(), raw, fat, preferred_arch, None)
    }

    pub fn load_from_raw(
        filename: &str,
        raw: &[u8],
        preferred_arch: Option<Arch>,
    ) -> Result<Self, LiefMachoError> {
        let mut tmp = tempfile::NamedTempFile::new()
            .map_err(|e| LiefMachoError::TempFileFailed(format!("{}", e)))?;
        std::io::Write::write_all(&mut tmp, raw)
            .map_err(|e| LiefMachoError::TempFileFailed(format!("write: {}", e)))?;

        let fat = macho::FatBinary::parse(tmp.path()).ok_or_else(|| {
            LiefMachoError::ParseFailed(format!("{}: LIEF parse returned None", filename))
        })?;

        Self::from_fat(
            filename.to_string(),
            raw.to_vec(),
            fat,
            preferred_arch,
            Some(tmp),
        )
    }

    fn from_fat(
        filename: String,
        raw: Vec<u8>,
        fat: macho::FatBinary,
        preferred_arch: Option<Arch>,
        _temp_file: Option<tempfile::NamedTempFile>,
    ) -> Result<Self, LiefMachoError> {
        let valid_slices: Vec<_> = fat
            .iter()
            .filter(|b| {
                let h = b.header();
                h.is_64bit()
                    && matches!(
                        h.cpu_type(),
                        macho::header::CpuType::ARM64 | macho::header::CpuType::X86_64
                    )
            })
            .collect();

        if valid_slices.is_empty() {
            return Err(LiefMachoError::ParseFailed(
                "no valid 64-bit slices found".to_string(),
            ));
        }

        let arch = if let Some(pref) = preferred_arch {
            let target = match pref {
                Arch::Aarch64 => macho::header::CpuType::ARM64,
                Arch::X86_64 => macho::header::CpuType::X86_64,
                _ => macho::header::CpuType::ARM64,
            };
            let found = valid_slices
                .iter()
                .find(|b| b.header().cpu_type() == target);
            if found.is_none() {
                return Err(LiefMachoError::NoMatchingSlice {
                    wanted: format!("{:?}", pref),
                });
            }
            pref
        } else if valid_slices.len() == 1 {
            match valid_slices[0].header().cpu_type() {
                macho::header::CpuType::ARM64 => Arch::Aarch64,
                macho::header::CpuType::X86_64 => Arch::X86_64,
                ref cpu => return Err(LiefMachoError::UnsupportedCpu(format!("{:?}", cpu))),
            }
        } else {
            let arm64 = valid_slices
                .iter()
                .find(|b| b.header().cpu_type() == macho::header::CpuType::ARM64);
            if arm64.is_some() {
                Arch::Aarch64
            } else {
                Arch::X86_64
            }
        };

        Ok(LiefMacho64 {
            file_path: filename,
            raw,
            selected_arch: arch,
            _temp_file,
        })
    }

    pub fn detect_arches(path: &str) -> Result<Vec<Arch>, LiefMachoError> {
        if !Path::new(path).exists() {
            return Err(LiefMachoError::FileNotFound(path.to_string()));
        }

        let raw =
            fs::read(path).map_err(|e| LiefMachoError::ReadFailed(format!("{}: {}", path, e)))?;

        let mut tmp = tempfile::NamedTempFile::new()
            .map_err(|e| LiefMachoError::TempFileFailed(format!("{}", e)))?;
        std::io::Write::write_all(&mut tmp, &raw)
            .map_err(|e| LiefMachoError::TempFileFailed(format!("write: {}", e)))?;

        let fat = macho::FatBinary::parse(tmp.path()).ok_or_else(|| {
            LiefMachoError::ParseFailed(format!("{}: LIEF parse returned None", path))
        })?;

        let mut arches = Vec::new();
        for b in fat.iter() {
            if !b.header().is_64bit() {
                continue;
            }
            match b.header().cpu_type() {
                macho::header::CpuType::ARM64 => arches.push(Arch::Aarch64),
                macho::header::CpuType::X86_64 => arches.push(Arch::X86_64),
                _ => {}
            }
        }
        Ok(arches)
    }

    pub fn to_macho64(&self) -> Result<Macho64, LiefMachoError> {
        let target_cpu = match self.selected_arch {
            Arch::Aarch64 => macho::header::CpuType::ARM64,
            Arch::X86_64 => macho::header::CpuType::X86_64,
            _ => {
                return Err(LiefMachoError::UnsupportedCpu(format!(
                    "{:?}",
                    self.selected_arch
                )));
            }
        };

        let binary = if let Some(ref _tf) = self._temp_file {
            macho::FatBinary::parse(_tf.path()).ok_or_else(|| {
                LiefMachoError::ParseFailed("re-parse from temp failed".to_string())
            })?
        } else if Path::new(&self.file_path).exists() {
            macho::FatBinary::parse(&self.file_path).ok_or_else(|| {
                LiefMachoError::ParseFailed("re-parse from path failed".to_string())
            })?
        } else {
            let mut tmp = tempfile::NamedTempFile::new()
                .map_err(|e| LiefMachoError::TempFileFailed(format!("{}", e)))?;
            std::io::Write::write_all(&mut tmp, &self.raw)
                .map_err(|e| LiefMachoError::TempFileFailed(format!("write: {}", e)))?;
            macho::FatBinary::parse(tmp.path()).ok_or_else(|| {
                LiefMachoError::ParseFailed("re-parse from temp failed".to_string())
            })?
        };

        let binary_ref = binary
            .iter()
            .find(|b| b.header().cpu_type() == target_cpu && b.header().is_64bit())
            .ok_or_else(|| LiefMachoError::NoMatchingSlice {
                wanted: format!("{:?}", self.selected_arch),
            })?;

        let header = binary_ref.header();
        if !header.is_64bit() {
            return Err(LiefMachoError::Not64Bit);
        }

        let mut entry = binary_ref.entrypoint();
        if entry == 0 {
            if let Some(main_cmd) = binary_ref.main_command() {
                let text_base = binary_ref
                    .segments()
                    .find(|s| s.name() == "__TEXT")
                    .map(|s| s.virtual_address())
                    .unwrap_or(0);
                entry = text_base + main_cmd.entrypoint();
            }
        }
        if entry == 0 {
            if let Some(thread_cmd) = binary_ref.thread_command() {
                entry = thread_cmd.pc();
            }
        }
        if entry == 0 {
            return Err(LiefMachoError::MissingEntrypoint);
        }

        let mut segments = Vec::new();
        for seg in binary_ref.segments() {
            let name = seg.name().to_string();
            if name == "__PAGEZERO" {
                continue;
            }

            let vmaddr = seg.virtual_address();
            let vmsize = seg.virtual_size();
            let data = seg.content().to_vec();

            let fileoff = seg.file_offset();
            let filesize = seg.file_size();
            if filesize > 0 {
                validate_file_range(self.raw.len(), fileoff, filesize).map_err(|_| {
                    LiefMachoError::InvalidSegmentBounds {
                        name: name.clone(),
                        file_offset: fileoff,
                        file_size: filesize,
                        file_len: self.raw.len(),
                    }
                })?;
            }

            segments.push(Macho64Segment {
                name,
                vmaddr,
                vmsize,
                data,
                initprot: seg.init_protection() as u32,
            });
        }

        let libs_cache: Vec<String> = binary_ref
            .libraries()
            .map(|l| l.name().to_string())
            .filter(|n| n != "self")
            .collect();

        let mut exports_cache: Vec<(String, u64)> = Vec::new();
        for sym in binary_ref.symbols() {
            let name = sym.name();
            if name.is_empty() || sym.value() == 0 {
                continue;
            }
            if matches!(sym.origin(), macho::symbol::Origin::DYLD_EXPORT) {
                exports_cache.push((name, sym.value()));
            }
        }

        let fixups_cache = Self::parse_bindings(&binary_ref);

        Ok(Macho64 {
            bin: self.raw.clone(),
            entry,
            segments,
            addr_to_symbol: std::collections::HashMap::new(),
            slice_offset: 0,
            libs_cache: Some(libs_cache),
            exports_cache: Some(exports_cache),
            fixups_cache: Some(fixups_cache),
            selected_arch: Some(self.selected_arch),
            backend: ParserBackendKind::Lief,
        })
    }

    fn parse_bindings(binary: &macho::Binary) -> (Vec<ChainedImport>, Vec<ChainedBind>) {
        let mut imports = Vec::new();
        let mut binds = Vec::new();
        let mut name_to_ordinal: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();
        let mut seen_binds: std::collections::HashSet<(u64, String)> =
            std::collections::HashSet::new();

        let all_bindings: Vec<_> = binary.bindings().collect();

        for binding in &all_bindings {
            let address = binding.address();
            let sym = binding.symbol();

            let name = match sym {
                Some(s) => s.name(),
                None => continue,
            };
            if name.is_empty() {
                continue;
            }

            let lib_ordinal = binding.library_ordinal() as i8;
            let weak = matches!(lib_ordinal, -2);

            let ordinal = if let Some(&ord) = name_to_ordinal.get(&name) {
                ord
            } else {
                let ord = imports.len() as u32;
                imports.push(ChainedImport {
                    name: name.clone(),
                    lib_ordinal,
                    weak,
                });
                name_to_ordinal.insert(name.clone(), ord);
                ord
            };

            if seen_binds.insert((address, name)) {
                binds.push(ChainedBind {
                    got_vmaddr: address,
                    import_ordinal: ordinal,
                });
            }
        }

        if let Some(chained) = binary.dyld_chained_fixups() {
            for chained_binding in chained.bindings() {
                let address = chained_binding.address();
                let sym = chained_binding.symbol();

                let name = match sym {
                    Some(s) => s.name(),
                    None => continue,
                };
                if name.is_empty() {
                    continue;
                }

                let ordinal = if let Some(&ord) = name_to_ordinal.get(&name) {
                    ord
                } else {
                    let ord = imports.len() as u32;
                    imports.push(ChainedImport {
                        name: name.clone(),
                        lib_ordinal: 0,
                        weak: false,
                    });
                    name_to_ordinal.insert(name.clone(), ord);
                    ord
                };

                if seen_binds.insert((address, name)) {
                    binds.push(ChainedBind {
                        got_vmaddr: address,
                        import_ordinal: ordinal,
                    });
                }
            }
        }

        (imports, binds)
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn selected_arch(&self) -> Arch {
        self.selected_arch
    }
}
