//! Core trait for LIEF PE reading
//!
//! This module defines the `LiefPeReader` trait that provides a unified
//! interface for reading PE file information using LIEF.

use lief::generic::{Binary, Section, Symbol};
use lief::pe::headers::{MachineType, Characteristics};
use crate::pe::lief::error::{ExportInfo, ImportInfo, LiefError, RelocationInfo, ResourceInfo};

/// Core trait for reading PE file information via LIEF
pub trait LiefPeReader {
    /// Get a reference to the underlying LIEF PE object
    fn lief_pe(&self) -> &lief::pe::Binary;

    // ========================================================================
    // Basic PE Information
    // ========================================================================

    /// Check if this is a PE32+ (64-bit) executable
    fn is_pe64(&self) -> bool {
        self.lief_pe().header().machine() == MachineType::AMD64
    }

    /// Check if this is a PE32 (32-bit) executable
    fn is_pe32(&self) -> bool {
        !self.is_pe64()
    }

    /// Check if this is a DLL
    fn is_dll(&self) -> bool {
        self.lief_pe().header().characteristics().contains(Characteristics::DLL)
    }

    /// Get the number of sections in the PE file
    fn num_sections(&self) -> u16 {
        self.lief_pe().sections().count() as u16
    }

    /// Get a section by index
    fn get_section(&self, index: usize) -> Option<lief::pe::Section> {
        self.lief_pe().sections().nth(index)
    }

    /// Get a section by name
    fn get_section_by_name(&self, name: &str) -> Option<lief::pe::Section> {
        self.lief_pe()
            .sections()
            .find(|s| s.name() == name)
    }

    /// Convert a virtual address to a file offset
    fn vaddr_to_offset(&self, vaddr: u64) -> Option<u64>;

    /// Get the image base address
    fn image_base(&self) -> u64 {
        self.lief_pe().imagebase()
    }

    /// Get the entry point RVA
    fn entry_point(&self) -> u64 {
        self.lief_pe().entrypoint()
    }

    /// Get the section alignment
    fn section_alignment(&self) -> u32 {
        self.lief_pe()
            .optional_header()
            .section_alignment()
    }

    /// Get the virtual size of the image
    fn virtual_size(&self) -> u64 {
        self.lief_pe().virtual_size()
    }

    /// Get the size of headers
    fn size_of_headers(&self) -> u32 {
        self.lief_pe().sizeof_headers() as u32
    }

    /// Get a data directory by index
    fn get_data_directory(&self, index: usize) -> Option<lief::pe::DataDirectory> {
        self.lief_pe().data_directories().nth(index)
    }

    /// Convert an RVA to a file offset
    fn rva_to_offset(&self, rva: u64) -> Option<u64> {
        self.vaddr_to_offset(rva)
    }

    // ========================================================================
    // File/Section Information
    // ========================================================================

    /// Get the size of the PE file on disk
    fn size(&self) -> u64;

    /// Get the memory size of the PE image (sum of virtual sizes)
    fn mem_size(&self) -> usize;

    /// Get a section's virtual address by index
    fn get_section_vaddr(&self, index: usize) -> Option<u64>;

    /// Get a section's virtual size by index
    fn get_section_size(&self, index: usize) -> Option<u64>;

    /// Get a section's raw (on-disk) size by index
    fn get_section_raw_size(&self, index: usize) -> Option<u64>;

    /// Get a section's file offset by index
    fn get_section_offset(&self, index: usize) -> Option<u64>;

    /// Get a section's name by index
    fn get_section_name(&self, index: usize) -> Option<String>;

    // ========================================================================
    // Headers
    // ========================================================================

    /// Get the PE headers as a byte slice
    fn get_headers(&self) -> &[u8];

    /// Get the offset to the PE header (e_lfanew)
    fn get_pe_offset(&self) -> u32;

    // ========================================================================
    // TLS Callbacks
    // ========================================================================

    /// Get TLS callback addresses
    fn get_tls_callbacks(&self) -> Vec<u64>;

    // ========================================================================
    // Import/Export Tables
    // ========================================================================

    /// Get all imports from the PE file
    fn get_imports(&self) -> Result<Vec<ImportInfo>, LiefError>;

    /// Look up an import function name by its IAT address
    ///
    /// Given an import address (IAT entry), finds and returns the name of
    /// the imported function. Returns an empty string if not found.
    fn import_addr_to_name(&self, paddr: u64) -> String {
        if paddr == 0 {
            return String::new();
        }

        let pe = self.lief_pe();
        for import in pe.imports() {
            for function in import.entries() {
                if function.iat_address() as u64 == paddr {
                    let name = function.name();
                    if name.is_empty() {
                        return "<ordinal>".to_string();
                    }
                    return name;
                }
            }
        }
        String::new()
    }

    /// Get all exports from the PE file
    fn get_exports(&self) -> Result<Vec<ExportInfo>, LiefError>;

    // ========================================================================
    // Resources
    // ========================================================================

    /// Get all resources from the PE file
    fn get_resources(&self) -> Result<Vec<ResourceInfo>, LiefError>;

    // ========================================================================
    // Relocations
    // ========================================================================

    /// Get all relocations from the PE file
    fn get_relocations(&self) -> Result<Vec<RelocationInfo>, LiefError>;

    /// Get a specific resource by type and name
    ///
    /// Returns (address, size) if found, None otherwise
    fn get_resource(
        &self,
        _type_id: Option<u32>,
        _name_id: Option<u32>,
        _type_name: Option<&str>,
        _resource_name: Option<&str>,
    ) -> Option<(u64, usize)> {
        // LIEF-based resource lookup not yet implemented
        None
    }
}
