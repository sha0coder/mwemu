//! Runtime PE32 wrapper — LIEF-only.
//!
//! All PE32 loading, binding, and resource access goes through LIEF.
//! The legacy hand-rolled `PE32` parser modules live behind the
//! non-default `legacy-pe-parity` cargo feature for diagnostic parity
//! tests. They are not used for runtime loading or serialization
//! restore; serialized PE32 bytes are re-parsed by LIEF and rejected
//! when LIEF cannot parse them.

use crate::emu::Emu;
use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::error::LiefError;
use crate::loaders::pe::lief::traits::LiefPeReader;
use lief::generic::Section as _;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePe32Section {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub size_of_raw_data: u32,
    pub characteristics: u32,
}

/// LIEF-backed PE32 runtime.
///
/// Note: the type is named `RuntimePe32` (a struct, not an enum) and wraps
/// `LiefPe` directly. Loader code can call methods on it without matching
/// on a variant, which keeps call sites compact. The type lives in
/// `runtime_pe32` for backwards-compat with callers that imported it.
pub struct RuntimePe32 {
    inner: LiefPe,
}

impl RuntimePe32 {
    /// Load a PE32 file using LIEF. Returns an error if the file is not a
    /// valid PE32 binary or LIEF parsing fails.
    pub fn load(path: &str) -> Result<Self, LiefError> {
        let lief_pe = LiefPe::load(path)?;
        if !lief_pe.is_pe32() {
            return Err(LiefError::ParseFailed(format!(
                "File is not PE32: {}",
                path
            )));
        }
        Ok(RuntimePe32 { inner: lief_pe })
    }

    /// Construct directly from an already-loaded `LiefPe`. Used by the
    /// serialization layer to rehydrate dumps.
    pub fn from_inner(lief_pe: LiefPe) -> Self {
        RuntimePe32 { inner: lief_pe }
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

impl RuntimePe32 {
    /// Check if this is a DLL
    pub fn is_dll(&self) -> bool {
        self.inner.is_dll()
    }

    /// Get the size of the PE file
    pub fn size(&self) -> u64 {
        self.inner.size()
    }

    /// Get the in-memory size of the loaded image
    pub fn mem_size(&self) -> u64 {
        self.inner.mem_size() as u64
    }

    /// Get the image base address (32-bit)
    pub fn image_base(&self) -> u32 {
        self.inner.image_base() as u32
    }

    /// Get the entry point RVA (32-bit)
    pub fn entry_point(&self) -> u32 {
        self.inner.entry_point() as u32
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

    /// Get section pointer (raw bytes) by index
    pub fn get_section_ptr(&self, index: usize) -> Vec<u8> {
        self.inner.get_section_ptr(index)
    }

    /// Get headers as bytes
    pub fn get_headers(&self) -> Vec<u8> {
        self.inner.get_headers().to_vec()
    }

    /// Get section by index
    pub fn get_section(&self, index: usize) -> Option<RuntimePe32Section> {
        let section = self.inner.get_section(index)?;
        Some(RuntimePe32Section {
            name: section.name().to_string(),
            virtual_address: section.virtual_address() as u32,
            virtual_size: section.virtual_size(),
            size_of_raw_data: section.sizeof_raw_data(),
            characteristics: section.characteristics().bits() as u32,
        })
    }

    /// Bind the Import Address Table (IAT) for a 32-bit image.
    pub fn iat_binding(&mut self, emu: &mut Emu, base_addr: u32) {
        self.inner.iat_binding32(emu, base_addr);
    }

    /// Bind delay-load imports for a 32-bit image.
    pub fn delay_load_binding(&mut self, emu: &mut Emu, base_addr: u32) {
        self.inner.delay_load_binding32(emu, base_addr);
    }

    /// Look up an import address to get function name (32-bit RVA/VA).
    pub fn import_addr_to_name(&self, addr: u32) -> String {
        self.inner.import_addr_to_name32(addr)
    }

    /// Get a resource by type/name.
    pub fn get_resource(
        &self,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u64, usize)> {
        self.inner.get_resource(type_id, name_id, type_name, name)
    }
}
