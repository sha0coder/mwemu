//! Header-only PE parser using LIEF and memory mapping
//!
//! This module provides a lightweight PE parser that only reads headers
//! (~4KB) without loading section data, enabling efficient PE inspection.

use std::fs::File;
use std::mem;
use std::sync::Arc;

use lief::generic::{Binary, Section};
use lief::pe::Binary as PeBinary;
use lief::pe::headers::MachineType;
use memmap2::Mmap;

use crate::pe::lief::error::{
    ExportInfo, ImportInfo, LiefError, RelocationInfo, ResourceInfo,
};
use crate::pe::lief::traits::LiefPeReader;

/// Header-only PE parser using LIEF and memory mapping
///
/// This parser memory-maps the PE file and uses LIEF to parse headers
/// without loading section data into memory. This enables:
/// - Fast parsing (~4KB read vs full file)
/// - Zero-copy access via memory mapping
/// - Lazy section loading on demand
pub struct LiefHeaderParser {
    /// Memory-mapped file for zero-copy access (or bytes for from_bytes)
    mapped_file: Arc<[u8]>,
    /// LIEF PE object for header access (shared via Arc)
    pe: Arc<PeBinary>,
    /// Cached header bytes (~4KB)
    header_cache: Vec<u8>,
}

impl LiefHeaderParser {
    /// Create a new parser from a file path
    ///
    /// Memory-maps the file and parses headers only.
    pub fn from_path(path: &str) -> Result<Self, LiefError> {
        // Open the file
        let file = File::open(path).map_err(|e| LiefError::FileNotFound(format!("{}: {}", path, e)))?;

        // Get file size for memory mapping
        let file_size = file
            .metadata()
            .map_err(|e| LiefError::MmapFailed(format!("Failed to get metadata: {}", e)))?
            .len() as usize;

        if file_size == 0 {
            return Err(LiefError::ParseFailed("File is empty".to_string()));
        }

        // Memory-map the file
        let mapped = unsafe { Mmap::map(&file) }
            .map_err(|e| LiefError::MmapFailed(format!("Failed to memory map file: {}", e)))?;

        let mapped_file: Arc<[u8]> = Arc::from(&mapped[..]);

        // Parse headers using LIEF - use path directly
        let pe = PeBinary::parse(path)
            .ok_or_else(|| LiefError::ParseFailed("Failed to parse PE".to_string()))?;

        // Cache header bytes (up to size_of_headers or file size)
        let header_size = std::cmp::min(pe.sizeof_headers() as usize, file_size);
        let header_cache = mapped_file.as_ref()[..header_size].to_vec();

        Ok(Self {
            mapped_file,
            pe: Arc::new(pe),
            header_cache,
        })
    }

    /// Create a parser from an already-parsed PE binary (wrapped in Arc)
    ///
    /// This allows sharing the parsed binary between multiple components
    /// to avoid re-parsing the file.
    pub fn from_binary(mapped_file: Arc<[u8]>, pe: Arc<PeBinary>) -> Self {
        let header_size = std::cmp::min(pe.sizeof_headers() as usize, mapped_file.len());
        let header_cache = mapped_file.as_ref()[..header_size].to_vec();

        Self {
            mapped_file,
            pe,
            header_cache,
        }
    }

    /// Create a new parser from byte slice
    ///
    /// Copies the data to a temporary file and parses headers.
    pub fn from_bytes(data: &[u8]) -> Result<Self, LiefError> {
        if data.is_empty() {
            return Err(LiefError::ParseFailed("Data is empty".to_string()));
        }

        // Create a temporary file
        let mut temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| LiefError::ParseFailed(format!("Failed to create temp file: {}", e)))?;
        
        use std::io::Write;
        temp_file.write_all(data)
            .map_err(|e| LiefError::ParseFailed(format!("Failed to write temp file: {}", e)))?;

        // Get the path to the temp file BEFORE consuming temp_file
        let temp_path = temp_file.path().to_str()
            .ok_or_else(|| LiefError::ParseFailed("Invalid temp path".to_string()))?
            .to_string();

        // Memory-map the temp file for our own access
        let file = temp_file.into_file();
        let mapped = unsafe { Mmap::map(&file) }
            .map_err(|e| LiefError::MmapFailed(format!("Failed to memory map temp file: {}", e)))?;
        let mapped_file: Arc<[u8]> = Arc::from(&mapped[..]);

        // Parse headers using LIEF
        let pe = PeBinary::parse(&temp_path)
            .ok_or_else(|| LiefError::ParseFailed("Failed to parse PE".to_string()))?;

        // Cache header bytes (up to size_of_headers or data size)
        let header_size = std::cmp::min(pe.sizeof_headers() as usize, data.len());
        let header_cache = data[..header_size].to_vec();

        Ok(Self {
            mapped_file,
            pe: Arc::new(pe),
            header_cache,
        })
    }

    /// Get a reference to the memory-mapped file
    pub fn mapped_file(&self) -> &Arc<[u8]> {
        &self.mapped_file
    }

    /// Get the Arc-wrapped PE binary
    ///
    /// This allows sharing the binary with other components like LiefSectionManager.
    pub fn pe_arc(&self) -> &Arc<PeBinary> {
        &self.pe
    }

    /// Get the header cache (raw header bytes)
    pub fn header_cache(&self) -> &[u8] {
        &self.header_cache
    }

    /// Get DOS magic bytes
    pub fn dos_magic(&self) -> u16 {
        self.pe.dos_header().magic()
    }

    /// Get the e_lfanew value (offset to PE header)
    pub fn e_lfanew(&self) -> u32 {
        self.pe.dos_header().addressof_new_exeheader()
    }

    /// Get NT signature
    pub fn nt_signature(&self) -> u32 {
        // Read from the mapped file at e_lfanew + 4 (signature offset)
        let e_lfanew = self.e_lfanew() as usize;
        if e_lfanew + 8 <= self.mapped_file.len() {
            u32::from_le_bytes([
                self.mapped_file[e_lfanew + 4],
                self.mapped_file[e_lfanew + 5],
                self.mapped_file[e_lfanew + 6],
                self.mapped_file[e_lfanew + 7],
            ])
        } else {
            0
        }
    }

    /// Get machine type
    pub fn machine(&self) -> MachineType {
        self.pe.header().machine()
    }

    /// Get number of sections
    pub fn num_sections(&self) -> u16 {
        self.pe.sections().count() as u16
    }

    /// Get size of optional header
    pub fn size_of_optional_header(&self) -> u16 {
        self.pe.header().sizeof_optional_header()
    }

    /// Get image base
    pub fn image_base(&self) -> u64 {
        self.pe.imagebase()
    }

    /// Get entry point RVA
    pub fn entry_point(&self) -> u64 {
        self.pe.entrypoint()
    }

    /// Get section alignment
    pub fn section_alignment(&self) -> u32 {
        self.pe.optional_header().section_alignment()
    }

    /// Get sections
    pub fn sections(&self) -> Vec<lief::pe::Section> {
        self.pe.sections().collect()
    }

    /// Get data directories
    pub fn data_directories(&self) -> Vec<lief::pe::DataDirectory> {
        self.pe.data_directories().collect()
    }

    /// Get section by index
    pub fn get_section(&self, index: usize) -> Option<lief::pe::Section> {
        self.pe.sections().nth(index)
    }

    /// Get section by name
    pub fn get_section_by_name(&self, name: &str) -> Option<lief::pe::Section> {
        self.pe.sections().find(|s| s.name() == name)
    }

    /// Convert virtual address to file offset
    ///
    /// Uses the section information to perform the conversion.
    pub fn vaddr_to_offset(&self, vaddr: u64) -> Option<u64> {
        let image_base = self.pe.imagebase();

        // If vaddr is below image base, it might be a file offset already
        if vaddr < image_base {
            return Some(vaddr);
        }

        // Calculate RVA (relative to image base)
        let rva = vaddr.checked_sub(image_base)?;

        // Find the section containing this RVA
        for section in self.pe.sections() {
            let section_rva = section.virtual_address() as u64;
            let section_size = section.virtual_size() as u64;

            if rva >= section_rva && rva < section_rva + section_size {
                // Calculate offset within section
                let offset_in_section = rva - section_rva;
                // File offset = section pointer to raw data + offset in section
                let file_offset = section.pointerto_raw_data() as u64 + offset_in_section;
                return Some(file_offset);
            }
        }

        None
    }

    /// Release the memory-mapped file reference
    ///
    /// This releases the mapped_file reference, allowing the memory to be freed.
    /// The header_cache is retained since it's small (~4KB).
    pub fn release_mmap(&mut self) {
        // Release the mapped file reference by replacing with empty Arc
        let empty: Arc<[u8]> = Arc::new([]);
        mem::replace(&mut self.mapped_file, empty);

        // Note: We keep pe (LIEF binary) since it's relatively small
        // and may still be needed for some operations
    }
}

impl LiefPeReader for LiefHeaderParser {
    fn lief_pe(&self) -> &lief::pe::Binary {
        &self.pe
    }

    fn vaddr_to_offset(&self, vaddr: u64) -> Option<u64> {
        LiefHeaderParser::vaddr_to_offset(self, vaddr)
    }

    // For header parser, these methods return None/empty since it's header-only
    // The full LiefPe implementation provides actual functionality

    fn size(&self) -> u64 {
        self.mapped_file.as_ref().len() as u64
    }

    fn mem_size(&self) -> usize {
        self.pe.sections().map(|s| s.virtual_size() as usize).sum()
    }

    fn get_section_vaddr(&self, index: usize) -> Option<u64> {
        self.get_section(index).map(|s| s.virtual_address() as u64)
    }

    fn get_section_size(&self, index: usize) -> Option<u64> {
        self.get_section(index).map(|s| s.virtual_size() as u64)
    }

    fn get_section_raw_size(&self, index: usize) -> Option<u64> {
        self.get_section(index).map(|s| s.sizeof_raw_data() as u64)
    }

    fn get_section_offset(&self, index: usize) -> Option<u64> {
        self.get_section(index).map(|s| s.pointerto_raw_data() as u64)
    }

    fn get_section_name(&self, index: usize) -> Option<String> {
        self.get_section(index).map(|s| s.name().to_string())
    }

    fn get_headers(&self) -> &[u8] {
        &self.header_cache
    }

    fn get_pe_offset(&self) -> u32 {
        self.e_lfanew()
    }

    fn get_tls_callbacks(&self) -> Vec<u64> {
        // Header parser doesn't parse TLS callbacks
        Vec::new()
    }

    fn get_imports(&self) -> Result<Vec<ImportInfo>, LiefError> {
        // Header parser doesn't parse imports
        Ok(Vec::new())
    }

    fn get_exports(&self) -> Result<Vec<ExportInfo>, LiefError> {
        // LIEF 0.17.6 API changed - returning empty for now
        Ok(Vec::new())
    }

    fn get_resources(&self) -> Result<Vec<ResourceInfo>, LiefError> {
        // Header parser doesn't fully parse resources
        Ok(Vec::new())
    }

    fn get_relocations(&self) -> Result<Vec<RelocationInfo>, LiefError> {
        // LIEF 0.17.6 API changed - returning empty for now
        Ok(Vec::new())
    }
}
