//! Runtime PE64 wrapper that can use either legacy PE64 or LIEF-based PE parsing.
//!
//! This enum provides a unified interface for PE64 operations while allowing
//! the underlying implementation to be either the legacy parser or the LIEF-based
//! parser.

use crate::emu::Emu;
use crate::config::Pe64Backend;
use crate::loaders::pe::pe64::PE64;
use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::traits::LiefPeReader;
use crate::loaders::pe::lief::error::LiefError;
use lief::generic::Section as _;

#[derive(Debug)]
pub enum RuntimePeError {
    LiefLoad { path: String, source: LiefError },
    Relocation { path: String, backend: Pe64Backend, source: LiefError },
    HeaderTruncated { path: String, expected: usize, actual: usize },
}

impl std::fmt::Display for RuntimePeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimePeError::LiefLoad { path, source } => {
                write!(f, "LIEF load failed for {}: {}", path, source)
            }
            RuntimePeError::Relocation { path, backend, source } => {
                write!(f, "Relocation error for {} (backend {:?}): {}", path, backend, source)
            }
            RuntimePeError::HeaderTruncated { path, expected, actual } => {
                write!(f, "Header truncated for {}: expected {} bytes, got {}", path, expected, actual)
            }
        }
    }
}

impl std::error::Error for RuntimePeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RuntimePeError::LiefLoad { source, .. } => Some(source),
            RuntimePeError::Relocation { source, .. } => Some(source),
            RuntimePeError::HeaderTruncated { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSection {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub size_of_raw_data: u32,
    pub characteristics: u32,
}

pub enum RuntimePe64 {
    Legacy(PE64),
    Lief(LiefPe),
}

impl From<PE64> for RuntimePe64 {
    fn from(pe: PE64) -> Self {
        RuntimePe64::Legacy(pe)
    }
}

impl From<LiefPe> for RuntimePe64 {
    fn from(pe: LiefPe) -> Self {
        RuntimePe64::Lief(pe)
    }
}

impl RuntimePe64 {
    /// Load a PE file using the default backend (Legacy).
    ///
    /// For backward compatibility, this uses the legacy parser.
    /// Use `load_with_backend` with `Pe64Backend::Auto` or `Pe64Backend::Lief`
    /// for LIEF-based parsing.
    pub fn load(path: &str) -> Self {
        RuntimePe64::Legacy(PE64::load(path))
    }

    /// Load using LIEF with fallback to legacy (Auto behavior).
    pub fn load_auto(path: &str) -> Self {
        match LiefPe::load(path) {
            Ok(lief_pe) => RuntimePe64::Lief(lief_pe),
            Err(e) => {
                log::warn!("LIEF loading failed for {}, falling back to legacy parser: {}", path, e);
                RuntimePe64::Legacy(PE64::load(path))
            }
        }
    }

    /// Try to load using LIEF, returning error if it fails
    pub fn load_lief(path: &str) -> Result<Self, LiefError> {
        LiefPe::load(path).map(RuntimePe64::Lief)
    }

    /// Load using legacy parser
    pub fn load_legacy(path: &str) -> Self {
        RuntimePe64::Legacy(PE64::load(path))
    }

    /// Load with explicit backend policy.
    ///
    /// - `Legacy`: uses the legacy parser only.
    /// - `Lief`: uses LIEF only; returns `RuntimePeError::LiefLoad` on failure.
    /// - `Auto`: tries LIEF first, falls back to legacy on LIEF parse failure.
    pub fn load_with_backend(path: &str, backend: Pe64Backend) -> Result<Self, RuntimePeError> {
        match backend {
            Pe64Backend::Legacy => {
                log::trace!("PE64 backend: forced legacy for {}", path);
                Ok(RuntimePe64::Legacy(PE64::load(path)))
            }
            Pe64Backend::Lief => {
                match LiefPe::load(path) {
                    Ok(lief_pe) => {
                        log::trace!("PE64 backend: forced LIEF for {}", path);
                        Ok(RuntimePe64::Lief(lief_pe))
                    }
                    Err(e) => {
                        Err(RuntimePeError::LiefLoad {
                            path: path.to_string(),
                            source: e,
                        })
                    }
                }
            }
            Pe64Backend::Auto => {
                match LiefPe::load(path) {
                    Ok(lief_pe) => {
                        log::trace!("PE64 backend: auto-selected LIEF for {}", path);
                        Ok(RuntimePe64::Lief(lief_pe))
                    }
                    Err(e) => {
                        log::warn!("PE64 backend: LIEF failed for {}, falling back to legacy: {}", path, e);
                        Ok(RuntimePe64::Legacy(PE64::load(path)))
                    }
                }
            }
        }
    }

    /// Return which backend variant is active
    pub fn backend_name(&self) -> &'static str {
        match self {
            RuntimePe64::Legacy(_) => "legacy",
            RuntimePe64::Lief(_) => "lief",
        }
    }

    /// Check if the LIEF backend is active
    pub fn is_lief(&self) -> bool {
        matches!(self, RuntimePe64::Lief(_))
    }

    /// Get a reference to the inner LiefPe if LIEF backend is active
    pub fn as_lief(&self) -> Option<&LiefPe> {
        match self {
            RuntimePe64::Lief(pe) => Some(pe),
            _ => None,
        }
    }
}

impl RuntimePe64 {
    /// Check if this is a DLL
    pub fn is_dll(&self) -> bool {
        match self {
            RuntimePe64::Legacy(pe) => pe.is_dll(),
            RuntimePe64::Lief(pe) => pe.is_dll(),
        }
    }

    /// Get the size of the PE file
    pub fn size(&self) -> u64 {
        match self {
            RuntimePe64::Legacy(pe) => pe.size(),
            RuntimePe64::Lief(pe) => pe.size(),
        }
    }

    /// Get the image base address
    pub fn image_base(&self) -> u64 {
        match self {
            RuntimePe64::Legacy(pe) => pe.opt.image_base,
            RuntimePe64::Lief(pe) => pe.image_base(),
        }
    }

    /// Get the entry point RVA
    pub fn entry_point(&self) -> u64 {
        match self {
            RuntimePe64::Legacy(pe) => pe.opt.address_of_entry_point as u64,
            RuntimePe64::Lief(pe) => pe.entry_point(),
        }
    }

    /// Get the section alignment
    pub fn section_alignment(&self) -> u32 {
        match self {
            RuntimePe64::Legacy(pe) => pe.opt.section_alignment,
            RuntimePe64::Lief(pe) => pe.section_alignment(),
        }
    }

    /// Get the size of headers
    pub fn size_of_headers(&self) -> u32 {
        match self {
            RuntimePe64::Legacy(pe) => pe.opt.size_of_headers,
            RuntimePe64::Lief(pe) => pe.size_of_headers(),
        }
    }

    /// Get the PE offset (e_lfanew)
    pub fn get_pe_offset(&self) -> u32 {
        match self {
            RuntimePe64::Legacy(pe) => pe.dos.e_lfanew,
            RuntimePe64::Lief(pe) => pe.e_lfanew(),
        }
    }

    /// Get the number of sections
    pub fn num_of_sections(&self) -> usize {
        match self {
            RuntimePe64::Legacy(pe) => pe.num_of_sections(),
            RuntimePe64::Lief(pe) => pe.num_sections() as usize,
        }
    }

    /// Get section pointer by index
    pub fn get_section_ptr(&self, index: usize) -> Vec<u8> {
        match self {
            RuntimePe64::Legacy(pe) => pe.get_section_ptr(index).to_vec(),
            RuntimePe64::Lief(pe) => pe.get_section_ptr(index),
        }
    }

    /// Get headers as byte slice
    pub fn get_headers(&self) -> Vec<u8> {
        match self {
            RuntimePe64::Legacy(pe) => pe.get_headers().to_vec(),
            RuntimePe64::Lief(pe) => pe.get_headers().to_vec(),
        }
    }

    pub fn apply_relocations(&mut self, emu: &mut Emu, base_addr: u64) -> Result<(), RuntimePeError> {
        match self {
            RuntimePe64::Legacy(pe) => {
                pe.apply_relocations(emu, base_addr);
                Ok(())
            }
            RuntimePe64::Lief(pe) => {
                let path = pe.file_path().to_string();
                pe.apply_relocations(emu, base_addr)
                    .map(|_| ())
                    .map_err(|source| RuntimePeError::Relocation {
                        path,
                        backend: Pe64Backend::Lief,
                        source,
                    })
            }
        }
    }

    pub fn iat_binding(&mut self, emu: &mut Emu, base_addr: u64) {
        match self {
            RuntimePe64::Legacy(pe) => pe.iat_binding(emu, base_addr),
            RuntimePe64::Lief(pe) => pe.iat_binding(emu, base_addr),
        }
    }

    /// Bind delay-load imports
    pub fn delay_load_binding(&mut self, emu: &mut Emu, base_addr: u64) {
        match self {
            RuntimePe64::Legacy(pe) => pe.delay_load_binding(emu, base_addr),
            RuntimePe64::Lief(pe) => pe.delay_load_binding(emu, base_addr),
        }
    }

    /// Get dependencies
    pub fn get_dependencies(&mut self, emu: &mut Emu) -> Vec<String> {
        match self {
            RuntimePe64::Legacy(pe) => pe.get_dependencies(emu),
            RuntimePe64::Lief(pe) => pe.get_dependencies(emu.api_set_resolver.as_ref()),
        }
    }

    /// Look up import address to get function name
    pub fn import_addr_to_name(&self, addr: u64) -> String {
        match self {
            RuntimePe64::Legacy(pe) => pe.import_addr_to_name(addr),
            RuntimePe64::Lief(pe) => pe.import_addr_to_name(addr),
        }
    }

    /// Look up import address to get DLL and function name
    pub fn import_addr_to_dll_and_name(&self, addr: u64) -> String {
        match self {
            RuntimePe64::Legacy(pe) => pe.pe64_import_addr_to_dll_and_name(addr),
            RuntimePe64::Lief(pe) => pe.import_addr_to_dll_and_name(addr),
        }
    }

    /// Get a resource by type/name
    pub fn get_resource(
        &self,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u64, usize)> {
        match self {
            RuntimePe64::Legacy(pe) => pe.get_resource(type_id, name_id, type_name, name),
            RuntimePe64::Lief(pe) => pe.get_resource(type_id, name_id, type_name, name),
        }
    }

    pub fn size_of_image(&self) -> u32 {
        match self {
            RuntimePe64::Legacy(pe) => pe.opt.size_of_image,
            RuntimePe64::Lief(pe) => pe.virtual_size() as u32,
        }
    }

    pub fn get_section(&self, index: usize) -> Option<RuntimeSection> {
        match self {
            RuntimePe64::Legacy(pe) => {
                if index >= pe.sect_hdr.len() {
                    return None;
                }
                let sect = &pe.sect_hdr[index];
                Some(RuntimeSection {
                    name: sect.get_name(),
                    virtual_address: sect.virtual_address,
                    virtual_size: sect.virtual_size,
                    size_of_raw_data: sect.size_of_raw_data,
                    characteristics: sect.characteristics,
                })
            }
            RuntimePe64::Lief(pe) => {
                let section = pe.get_section(index)?;
                Some(RuntimeSection {
                    name: section.name(),
                    virtual_address: section.virtual_address() as u32,
                    virtual_size: section.virtual_size(),
                    size_of_raw_data: section.sizeof_raw_data(),
                    characteristics: section.characteristics().bits() as u32,
                })
            }
        }
    }
}
