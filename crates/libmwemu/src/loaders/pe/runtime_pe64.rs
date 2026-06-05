//! Runtime PE64 wrapper — LIEF-only.
//!
//! All PE64 loading, binding, and resource access goes through LIEF.
//! The legacy hand-rolled `PE64` parser modules live behind the
//! non-default `legacy-pe-parity` cargo feature for diagnostic parity
//! tests. They are not used for runtime loading or serialization
//! restore; serialized PE64 bytes are re-parsed by LIEF and rejected
//! when LIEF cannot parse them.

use crate::emu::Emu;
use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::error::LiefError;
use crate::loaders::pe::lief::traits::LiefPeReader;
use lief::generic::Section as _;

#[derive(Debug)]
pub enum RuntimePeError {
    LiefLoad {
        path: String,
        source: LiefError,
    },
    Relocation {
        path: String,
        source: LiefError,
    },
    HeaderTruncated {
        path: String,
        expected: usize,
        actual: usize,
    },
}

impl std::fmt::Display for RuntimePeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimePeError::LiefLoad { path, source } => {
                write!(f, "LIEF load failed for {}: {}", path, source)
            }
            RuntimePeError::Relocation { path, source } => {
                write!(f, "Relocation error for {}: {}", path, source)
            }
            RuntimePeError::HeaderTruncated {
                path,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Header truncated for {}: expected {} bytes, got {}",
                    path, expected, actual
                )
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

/// LIEF-backed PE64 runtime. Loader code interacts with this type directly
/// without needing to match on a backend variant.
pub struct RuntimePe64 {
    inner: LiefPe,
}

impl RuntimePe64 {
    /// Load a PE64 file using LIEF. Returns an error if the file is not a
    /// valid PE64 binary (e.g. it parses as PE32 or is otherwise not a 64-bit
    /// PE) or if LIEF parsing fails.
    pub fn load(path: &str) -> Result<Self, LiefError> {
        let lief_pe = LiefPe::load(path)?;
        if !lief_pe.is_pe64() {
            return Err(LiefError::ParseFailed(format!(
                "File is not PE64: {}",
                path
            )));
        }
        Ok(RuntimePe64 { inner: lief_pe })
    }

    /// Construct directly from an already-loaded `LiefPe`. Used by the
    /// serialization layer to rehydrate dumps.
    pub fn from_inner(lief_pe: LiefPe) -> Self {
        RuntimePe64 { inner: lief_pe }
    }

    /// Consume and return the inner LiefPe. Useful for tests that want to
    /// drive the underlying parser directly.
    pub fn into_inner(self) -> LiefPe {
        self.inner
    }

    /// Return which backend variant is active
    pub fn backend_name(&self) -> &'static str {
        "lief"
    }

    /// Check if the LIEF backend is active — always true now.
    pub fn is_lief(&self) -> bool {
        true
    }

    /// Get a reference to the inner LiefPe.
    pub fn as_lief(&self) -> &LiefPe {
        &self.inner
    }

    /// Get a mutable reference to the inner LiefPe.
    pub fn as_lief_mut(&mut self) -> Option<&mut LiefPe> {
        Some(&mut self.inner)
    }

    /// Release the memory-mapped file references after sections have been
    /// copied to emulated memory. After calling this, raw section data is
    /// no longer available; callers must ensure all raw-data-dependent
    /// binding/relocation/resource parsing is complete before invoking it.
    pub fn release_mmap(&mut self) {
        self.inner.release_mmap();
    }
}

impl RuntimePe64 {
    /// Check if this is a DLL
    pub fn is_dll(&self) -> bool {
        self.inner.is_dll()
    }

    /// Get the size of the PE file
    pub fn size(&self) -> u64 {
        self.inner.size()
    }

    /// Get the image base address
    pub fn image_base(&self) -> u64 {
        self.inner.image_base()
    }

    /// Get the entry point RVA
    pub fn entry_point(&self) -> u64 {
        self.inner.entry_point()
    }

    /// Get the section alignment
    pub fn section_alignment(&self) -> u32 {
        self.inner.section_alignment()
    }

    /// Get the size of headers
    pub fn size_of_headers(&self) -> u32 {
        self.inner.size_of_headers()
    }

    /// Get the PE offset (e_lfanew)
    pub fn get_pe_offset(&self) -> u32 {
        self.inner.e_lfanew()
    }

    /// Get the number of sections
    pub fn num_of_sections(&self) -> usize {
        self.inner.num_sections() as usize
    }

    /// Get section pointer by index
    pub fn get_section_ptr(&self, index: usize) -> Vec<u8> {
        self.inner.get_section_ptr(index)
    }

    /// Get headers as byte slice
    pub fn get_headers(&self) -> Vec<u8> {
        self.inner.get_headers().to_vec()
    }

    pub fn apply_relocations(
        &mut self,
        emu: &mut Emu,
        base_addr: u64,
    ) -> Result<(), RuntimePeError> {
        let path = self.inner.file_path().to_string();
        self.inner
            .apply_relocations(emu, base_addr)
            .map(|_| ())
            .map_err(|source| RuntimePeError::Relocation { path, source })
    }

    pub fn iat_binding(&mut self, emu: &mut Emu, base_addr: u64) {
        self.inner.iat_binding(emu, base_addr);
    }

    /// Bind delay-load imports
    pub fn delay_load_binding(&mut self, emu: &mut Emu, base_addr: u64) {
        self.inner.delay_load_binding(emu, base_addr);
    }

    /// Get dependencies
    pub fn get_dependencies(&mut self, emu: &mut Emu) -> Vec<String> {
        self.inner.get_dependencies(emu.api_set_resolver.as_ref())
    }

    /// Look up import address to get function name
    pub fn import_addr_to_name(&self, addr: u64) -> String {
        self.inner.import_addr_to_name(addr)
    }

    /// Look up import address to get DLL and function name
    pub fn import_addr_to_dll_and_name(&self, addr: u64) -> String {
        self.inner.import_addr_to_dll_and_name(addr)
    }

    /// Get a resource by type/name
    pub fn get_resource(
        &self,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u64, usize)> {
        self.inner.get_resource(type_id, name_id, type_name, name)
    }

    pub fn size_of_image(&self) -> u32 {
        self.inner.virtual_size() as u32
    }

    pub fn get_section(&self, index: usize) -> Option<RuntimeSection> {
        let section = self.inner.get_section(index)?;
        Some(RuntimeSection {
            name: section.name(),
            virtual_address: section.virtual_address() as u32,
            virtual_size: section.virtual_size(),
            size_of_raw_data: section.sizeof_raw_data(),
            characteristics: section.characteristics().bits() as u32,
        })
    }
}
