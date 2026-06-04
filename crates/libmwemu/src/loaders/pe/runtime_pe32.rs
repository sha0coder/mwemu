//! Runtime PE32 wrapper that can use either legacy PE32 or LIEF-based PE parsing.
//!
//! This enum provides a unified interface for PE32 operations while allowing
//! the underlying implementation to be either the legacy parser or the LIEF-based
//! parser.

use crate::config::Pe32Backend;
use crate::emu::Emu;
use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::error::LiefError;
use crate::loaders::pe::lief::traits::LiefPeReader;
use crate::loaders::pe::pe32::PE32;
use lief::generic::Section as _;

#[derive(Debug)]
pub enum RuntimePe32Error {
    LiefLoad { path: String, source: LiefError },
}

impl std::fmt::Display for RuntimePe32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimePe32Error::LiefLoad { path, source } => {
                write!(f, "LIEF load failed for {}: {}", path, source)
            }
        }
    }
}

impl std::error::Error for RuntimePe32Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RuntimePe32Error::LiefLoad { source, .. } => Some(source),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePe32Section {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub size_of_raw_data: u32,
    pub characteristics: u32,
}

pub enum RuntimePe32 {
    Legacy(PE32),
    Lief(LiefPe),
}

impl From<PE32> for RuntimePe32 {
    fn from(pe: PE32) -> Self {
        RuntimePe32::Legacy(pe)
    }
}

impl From<LiefPe> for RuntimePe32 {
    fn from(pe: LiefPe) -> Self {
        RuntimePe32::Lief(pe)
    }
}

impl RuntimePe32 {
    /// Load a PE32 file using the default backend (Legacy).
    ///
    /// For backward compatibility, this uses the legacy parser.
    pub fn load(path: &str) -> Self {
        RuntimePe32::Legacy(PE32::load(path))
    }

    /// Load using LIEF with fallback to legacy (Auto behavior).
    pub fn load_auto(path: &str) -> Self {
        match LiefPe::load(path) {
            Ok(lief_pe) => {
                if !lief_pe.is_pe32() {
                    log::warn!(
                        "LIEF auto-selected file is not PE32 for {}, falling back to legacy parser",
                        path
                    );
                    RuntimePe32::Legacy(PE32::load(path))
                } else {
                    RuntimePe32::Lief(lief_pe)
                }
            }
            Err(e) => {
                log::warn!(
                    "LIEF loading failed for {}, falling back to legacy parser: {}",
                    path,
                    e
                );
                RuntimePe32::Legacy(PE32::load(path))
            }
        }
    }

    /// Try to load using LIEF, returning error if it fails or file is not PE32.
    pub fn load_lief(path: &str) -> Result<Self, LiefError> {
        let lief_pe = LiefPe::load(path)?;
        if !lief_pe.is_pe32() {
            return Err(LiefError::ParseFailed(format!(
                "File is not PE32: {}",
                path
            )));
        }
        Ok(RuntimePe32::Lief(lief_pe))
    }

    /// Load using legacy parser
    pub fn load_legacy(path: &str) -> Self {
        RuntimePe32::Legacy(PE32::load(path))
    }

    /// Load with explicit backend policy.
    ///
    /// - `Legacy`: uses the legacy parser only.
    /// - `Lief`: uses LIEF only; returns `RuntimePe32Error::LiefLoad` on failure
    ///           or if file is not PE32.
    /// - `Auto`: tries LIEF first, falls back to legacy on LIEF parse failure
    ///           or non-PE32 detection.
    pub fn load_with_backend(path: &str, backend: Pe32Backend) -> Result<Self, RuntimePe32Error> {
        match backend {
            Pe32Backend::Legacy => {
                log::trace!("PE32 backend: forced legacy for {}", path);
                Ok(RuntimePe32::Legacy(PE32::load(path)))
            }
            Pe32Backend::Lief => match LiefPe::load(path) {
                Ok(lief_pe) => {
                    if !lief_pe.is_pe32() {
                        return Err(RuntimePe32Error::LiefLoad {
                            path: path.to_string(),
                            source: LiefError::ParseFailed("File is not PE32".to_string()),
                        });
                    }
                    log::trace!("PE32 backend: forced LIEF for {}", path);
                    Ok(RuntimePe32::Lief(lief_pe))
                }
                Err(e) => Err(RuntimePe32Error::LiefLoad {
                    path: path.to_string(),
                    source: e,
                }),
            },
            Pe32Backend::Auto => match LiefPe::load(path) {
                Ok(lief_pe) => {
                    if !lief_pe.is_pe32() {
                        log::warn!(
                            "PE32 backend: LIEF loaded file is not PE32 for {}, falling back to legacy",
                            path
                        );
                        Ok(RuntimePe32::Legacy(PE32::load(path)))
                    } else {
                        log::trace!("PE32 backend: auto-selected LIEF for {}", path);
                        Ok(RuntimePe32::Lief(lief_pe))
                    }
                }
                Err(e) => {
                    log::warn!(
                        "PE32 backend: LIEF failed for {}, falling back to legacy: {}",
                        path,
                        e
                    );
                    Ok(RuntimePe32::Legacy(PE32::load(path)))
                }
            },
        }
    }

    /// Return which backend variant is active
    pub fn backend_name(&self) -> &'static str {
        match self {
            RuntimePe32::Legacy(_) => "legacy",
            RuntimePe32::Lief(_) => "lief",
        }
    }

    /// Check if the LIEF backend is active
    pub fn is_lief(&self) -> bool {
        matches!(self, RuntimePe32::Lief(_))
    }

    /// Get a reference to the inner LiefPe if LIEF backend is active
    pub fn as_lief(&self) -> Option<&LiefPe> {
        match self {
            RuntimePe32::Lief(pe) => Some(pe),
            _ => None,
        }
    }

    /// Get a mutable reference to the inner LiefPe if LIEF backend is active
    pub fn as_lief_mut(&mut self) -> Option<&mut LiefPe> {
        match self {
            RuntimePe32::Lief(pe) => Some(pe),
            _ => None,
        }
    }
}

impl RuntimePe32 {
    /// Check if this is a DLL
    pub fn is_dll(&self) -> bool {
        match self {
            RuntimePe32::Legacy(pe) => pe.is_dll(),
            RuntimePe32::Lief(pe) => pe.is_dll(),
        }
    }

    /// Get the size of the PE file
    pub fn size(&self) -> u64 {
        match self {
            RuntimePe32::Legacy(pe) => pe.size() as u64,
            RuntimePe32::Lief(pe) => pe.size(),
        }
    }

    /// Get the in-memory size of the loaded image
    pub fn mem_size(&self) -> u64 {
        match self {
            RuntimePe32::Legacy(pe) => pe.mem_size() as u64,
            RuntimePe32::Lief(pe) => pe.mem_size() as u64,
        }
    }

    /// Get the image base address (32-bit)
    pub fn image_base(&self) -> u32 {
        match self {
            RuntimePe32::Legacy(pe) => pe.opt.image_base,
            RuntimePe32::Lief(pe) => pe.image_base() as u32,
        }
    }

    /// Get the entry point RVA (32-bit)
    pub fn entry_point(&self) -> u32 {
        match self {
            RuntimePe32::Legacy(pe) => pe.opt.address_of_entry_point,
            RuntimePe32::Lief(pe) => pe.entry_point() as u32,
        }
    }

    /// Get the section alignment
    pub fn section_alignment(&self) -> u32 {
        match self {
            RuntimePe32::Legacy(pe) => pe.opt.section_alignment,
            RuntimePe32::Lief(pe) => pe.section_alignment(),
        }
    }

    /// Get the size of headers
    pub fn size_of_headers(&self) -> u32 {
        match self {
            RuntimePe32::Legacy(pe) => pe.opt.size_of_headers,
            RuntimePe32::Lief(pe) => pe.size_of_headers(),
        }
    }

    /// Get the PE offset (e_lfanew)
    pub fn get_pe_offset(&self) -> u32 {
        match self {
            RuntimePe32::Legacy(pe) => pe.dos.e_lfanew,
            RuntimePe32::Lief(pe) => pe.e_lfanew(),
        }
    }

    /// Get the number of sections
    pub fn num_of_sections(&self) -> usize {
        match self {
            RuntimePe32::Legacy(pe) => pe.num_of_sections(),
            RuntimePe32::Lief(pe) => pe.num_sections() as usize,
        }
    }

    /// Get section pointer (raw bytes) by index
    pub fn get_section_ptr(&self, index: usize) -> Vec<u8> {
        match self {
            RuntimePe32::Legacy(pe) => pe.get_section_ptr(index).to_vec(),
            RuntimePe32::Lief(pe) => pe.get_section_ptr(index),
        }
    }

    /// Get headers as bytes
    pub fn get_headers(&self) -> Vec<u8> {
        match self {
            RuntimePe32::Legacy(pe) => pe.get_headers().to_vec(),
            RuntimePe32::Lief(pe) => pe.get_headers().to_vec(),
        }
    }

    /// Get section by index
    pub fn get_section(&self, index: usize) -> Option<RuntimePe32Section> {
        match self {
            RuntimePe32::Legacy(pe) => {
                if index >= pe.sect_hdr.len() {
                    return None;
                }
                let sect = &pe.sect_hdr[index];
                Some(RuntimePe32Section {
                    name: sect.get_name(),
                    virtual_address: sect.virtual_address,
                    virtual_size: sect.virtual_size,
                    size_of_raw_data: sect.size_of_raw_data,
                    characteristics: sect.characteristics,
                })
            }
            RuntimePe32::Lief(pe) => {
                let section = pe.get_section(index)?;
                Some(RuntimePe32Section {
                    name: section.name().to_string(),
                    virtual_address: section.virtual_address() as u32,
                    virtual_size: section.virtual_size(),
                    size_of_raw_data: section.sizeof_raw_data(),
                    characteristics: section.characteristics().bits() as u32,
                })
            }
        }
    }

    /// Bind the Import Address Table (IAT) for a 32-bit image.
    pub fn iat_binding(&mut self, emu: &mut Emu, base_addr: u32) {
        match self {
            RuntimePe32::Legacy(pe) => pe.iat_binding(emu, base_addr),
            RuntimePe32::Lief(pe) => pe.iat_binding32(emu, base_addr),
        }
    }

    /// Bind delay-load imports for a 32-bit image.
    pub fn delay_load_binding(&mut self, emu: &mut Emu, base_addr: u32) {
        match self {
            RuntimePe32::Legacy(pe) => pe.delay_load_binding(emu, base_addr),
            RuntimePe32::Lief(pe) => pe.delay_load_binding32(emu, base_addr),
        }
    }

    /// Look up an import address to get function name (32-bit RVA/VA).
    pub fn import_addr_to_name(&self, addr: u32) -> String {
        match self {
            RuntimePe32::Legacy(pe) => pe.import_addr_to_name(addr),
            RuntimePe32::Lief(pe) => pe.import_addr_to_name32(addr),
        }
    }

    /// Get a resource by type/name.
    pub fn get_resource(
        &self,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u64, usize)> {
        match self {
            RuntimePe32::Legacy(pe) => pe.get_resource(type_id, name_id, type_name, name),
            RuntimePe32::Lief(pe) => pe.get_resource(type_id, name_id, type_name, name),
        }
    }
}
