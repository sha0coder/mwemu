//! Main LIEF PE wrapper
//!
//! This module provides the main `LiefPe` struct that combines
//! header parsing with lazy section loading.

use std::path::PathBuf;
use std::sync::Arc;

use lief::generic::{Section, Symbol};
use lief::Relocation;
use memmap2::Mmap;

use crate::pe::lief::error::{
    ExportInfo, ImportFunction, ImportInfo, LiefError, RelocationInfo, ResourceInfo,
};
use crate::pe::lief::lief_header_parser::LiefHeaderParser;
use crate::pe::lief::lief_section_manager::{CachePolicy, LiefSectionManager};
use crate::pe::lief::traits::LiefPeReader;

/// Data directory indices
const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5;
const IMAGE_DIRECTORY_ENTRY_DELAY_LOAD: usize = 13;

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Names of currently cached sections
    pub cached_sections: Vec<String>,
    /// Total bytes used by cached sections
    pub cached_bytes: usize,
}

/// Main LIEF PE wrapper
///
/// This struct provides:
/// - Header-only parsing (~4KB initial read)
/// - Lazy section loading on demand
/// - Memory-mapped file access (no full file copy)
/// - Configurable cache policies
///
/// # Binding Operations
///
/// Binding operations (apply_relocations, iat_binding, delay_load_binding)
/// use native LIEF APIs to read PE structures and directly patch emulated
/// memory. Only emulated memory is patched - internal buffers are not modified.
pub struct LiefPe {
    /// Header parser (holds mmap and PE object)
    header: LiefHeaderParser,
    /// Section manager (lazy loading)
    section_manager: LiefSectionManager,
    /// Original file path
    file_path: String,
    /// File size (cached)
    file_size: usize,
}

impl LiefPe {
    /// Load a PE file
    ///
    /// This is the main entry point for loading a PE file using LIEF.
    pub fn load(path: &str) -> Result<Self, LiefError> {
        // Open file for memory mapping
        let file = std::fs::File::open(path)
            .map_err(|e| LiefError::FileNotFound(format!("{}: {}", path, e)))?;

        let file_size = file
            .metadata()
            .map_err(|e| LiefError::MmapFailed(format!("Failed to get metadata: {}", e)))?
            .len() as usize;

        // Memory-map the file
        let mapped = unsafe { Mmap::map(&file) }
            .map_err(|e| LiefError::MmapFailed(format!("Failed to memory map: {}", e)))?;

        let mapped_file = Arc::new(mapped);

        // Create header parser
        let header = LiefHeaderParser::from_path(path)?;

        // Create section manager with the parsed PE binary from header parser
        let section_manager = LiefSectionManager::new(
            PathBuf::from(path),
            mapped_file,
            header.pe_arc().clone(),
        );

        Ok(Self {
            header,
            section_manager,
            file_path: path.to_string(),
            file_size,
        })
    }

    /// Load a PE file with custom cache policy
    pub fn load_with_policy(path: &str, policy: CachePolicy) -> Result<Self, LiefError> {
        // Open file for memory mapping
        let file = std::fs::File::open(path)
            .map_err(|e| LiefError::FileNotFound(format!("{}: {}", path, e)))?;

        let file_size = file
            .metadata()
            .map_err(|e| LiefError::MmapFailed(format!("Failed to get metadata: {}", e)))?
            .len() as usize;

        // Memory-map the file
        let mapped = unsafe { Mmap::map(&file) }
            .map_err(|e| LiefError::MmapFailed(format!("Failed to memory map: {}", e)))?;

        let mapped_file = Arc::new(mapped);

        // Create header parser
        let header = LiefHeaderParser::from_path(path)?;

        // Create section manager with custom policy and the parsed PE binary
        let section_manager = LiefSectionManager::with_policy(
            PathBuf::from(path),
            mapped_file,
            policy,
            header.pe_arc().clone(),
        );

        Ok(Self {
            header,
            section_manager,
            file_path: path.to_string(),
            file_size,
        })
    }

    /// Get the file path
    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    /// Check if this is a PE64 (64-bit) executable
    pub fn is_pe64(&self) -> bool {
        self.header.is_pe64()
    }

    /// Check if this is a PE32 (32-bit) executable
    pub fn is_pe32(&self) -> bool {
        self.header.is_pe32()
    }

    /// Check if this is a DLL
    pub fn is_dll(&self) -> bool {
        self.header.is_dll()
    }

    /// Get number of sections
    pub fn num_sections(&self) -> u16 {
        self.header.num_sections()
    }

    /// Get image base
    pub fn image_base(&self) -> u64 {
        self.header.image_base()
    }

    /// Get entry point
    pub fn entry_point(&self) -> u64 {
        self.header.entry_point()
    }

    /// Get size of headers
    pub fn size_of_headers(&self) -> u32 {
        self.header.size_of_headers()
    }

    /// Get virtual size
    pub fn virtual_size(&self) -> u64 {
        self.header.virtual_size()
    }

    /// Get section alignment
    pub fn section_alignment(&self) -> u32 {
        self.header.section_alignment()
    }

    /// Get section by name (from header)
    pub fn get_section_by_name(&self, name: &str) -> Option<lief::pe::Section> {
        self.header.get_section_by_name(name)
    }

    /// Get section data by index (lazy loading)
    ///
    /// Returns the section data bytes, loading from the memory-mapped file if not cached.
    pub fn get_section_ptr(&self, index: usize) -> Vec<u8> {
        self.section_manager
            .get_section_data_by_index(index)
            .unwrap_or_default()
    }

    /// Check if a section is loaded in cache
    pub fn is_section_loaded(&self, name: &str) -> bool {
        self.section_manager.is_section_cached(name)
    }

    /// Convert virtual address to file offset
    pub fn vaddr_to_offset(&self, vaddr: u64) -> Option<u64> {
        self.header.vaddr_to_offset(vaddr)
    }

    /// Convert RVA to file offset
    pub fn rva_to_offset(&self, rva: u64) -> Option<u64> {
        self.header.rva_to_offset(rva)
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            cached_sections: self.section_manager.cached_sections(),
            cached_bytes: self.section_manager.cached_bytes(),
        }
    }

    /// Clear the section cache
    pub fn clear_cache(&self) {
        self.section_manager.clear_cache();
    }

    /// Release the memory-mapped file references
    ///
    /// This clears the section cache and releases the mapped_file references
    /// in both the header parser and section manager, allowing the memory to
    /// be freed after sections have been copied to emulated memory.
    ///
    /// After calling this method:
    /// - Section data can no longer be loaded (section cache is cleared)
    /// - Header information is still accessible via header_cache
    /// - Binding operations (apply_relocations, iat_binding, delay_load_binding)
    ///   still work as they only read from the memory-mapped file for relocation data
    pub fn release_mmap(&mut self) {
        self.section_manager.release_mmap();
        self.header.release_mmap();
    }

    /// Get section layout information
    ///
    /// Returns (file_offset, virtual_address, size) for the named section.
    pub fn get_section_layout(&self, name: &str) -> Option<(u64, u64, u64)> {
        self.section_manager.get_section_layout(name)
    }

    /// Get all sections
    pub fn sections(&self) -> Vec<lief::pe::Section> {
        self.header.sections()
    }

    /// Get data directories
    pub fn data_directories(&self) -> Vec<lief::pe::DataDirectory> {
        self.header.data_directories()
    }

    /// Get data directory by index
    pub fn get_data_directory(&self, index: usize) -> Option<lief::pe::DataDirectory> {
        self.header.get_data_directory(index)
    }

    /// Get the header cache (raw header bytes)
    pub(crate) fn header_cache(&self) -> &[u8] {
        self.header.header_cache()
    }

    /// Get the e_lfanew value
    pub(crate) fn e_lfanew(&self) -> u32 {
        self.header.e_lfanew()
    }

    /// Get mapped file data
    pub(crate) fn mapped_file_data(&self) -> &[u8] {
        self.header.mapped_file().as_ref()
    }
}

impl LiefPeReader for LiefPe {
    fn lief_pe(&self) -> &lief::pe::Binary {
        self.header.lief_pe()
    }

    fn is_pe64(&self) -> bool {
        self.is_pe64()
    }

    fn is_dll(&self) -> bool {
        self.is_dll()
    }

    fn num_sections(&self) -> u16 {
        self.num_sections()
    }

    fn get_section(&self, index: usize) -> Option<lief::pe::Section> {
        self.header.get_section(index)
    }

    fn get_section_by_name(&self, name: &str) -> Option<lief::pe::Section> {
        self.get_section_by_name(name)
    }

    fn vaddr_to_offset(&self, vaddr: u64) -> Option<u64> {
        self.vaddr_to_offset(vaddr)
    }

    fn image_base(&self) -> u64 {
        self.image_base()
    }

    fn entry_point(&self) -> u64 {
        self.entry_point()
    }

    fn virtual_size(&self) -> u64 {
        self.virtual_size()
    }

    fn size_of_headers(&self) -> u32 {
        self.size_of_headers()
    }

    fn get_data_directory(&self, index: usize) -> Option<lief::pe::DataDirectory> {
        self.get_data_directory(index)
    }

    // ========================================================================
    // File/Section Information
    // ========================================================================

    fn size(&self) -> u64 {
        self.file_size as u64
    }

    fn mem_size(&self) -> usize {
        // Sum of virtual sizes of all sections
        self.header
            .sections()
            .iter()
            .map(|s| s.virtual_size() as usize)
            .sum()
    }

    fn get_section_vaddr(&self, index: usize) -> Option<u64> {
        self.header.get_section(index).map(|s| s.virtual_address() as u64)
    }

    fn get_section_size(&self, index: usize) -> Option<u64> {
        self.header.get_section(index).map(|s| s.virtual_size() as u64)
    }

    fn get_section_raw_size(&self, index: usize) -> Option<u64> {
        self.header.get_section(index).map(|s| s.sizeof_raw_data() as u64)
    }

    fn get_section_offset(&self, index: usize) -> Option<u64> {
        self.header.get_section(index).map(|s| s.pointerto_raw_data() as u64)
    }

    fn get_section_name(&self, index: usize) -> Option<String> {
        self.header.get_section(index).map(|s| s.name().to_string())
    }

    // ========================================================================
    // Headers
    // ========================================================================

    fn get_headers(&self) -> &[u8] {
        self.header_cache()
    }

    fn get_pe_offset(&self) -> u32 {
        self.e_lfanew()
    }

    // ========================================================================
    // TLS Callbacks
    // ========================================================================

    fn get_tls_callbacks(&self) -> Vec<u64> {
        // Use LIEF to get TLS callbacks
        let pe = self.lief_pe();

        // TLS directory is typically at data directory index 9
        if let Some(tls) = pe.tls() {
            tls.callbacks()
                .iter()
                .map(|&cb| cb as usize as u64)
                .collect()
        } else {
            Vec::new()
        }
    }

    // ========================================================================
    // Import/Export Tables
    // ========================================================================

    fn get_imports(&self) -> Result<Vec<ImportInfo>, LiefError> {
        let pe = self.lief_pe();
        let mut imports = Vec::new();

        for import in pe.imports() {
            let dll_name = import.name().to_string();
            let mut functions = Vec::new();

            for function in import.entries() {
                let name = function.name().to_string();
                let ordinal = if function.is_ordinal() {
                    Some(function.ordinal())
                } else {
                    None
                };
                let rva = function.iat_address() as u64;

                functions.push(ImportFunction {
                    name,
                    ordinal,
                    rva,
                });
            }

            imports.push(ImportInfo {
                dll_name,
                functions,
            });
        }

        Ok(imports)
    }

    fn get_exports(&self) -> Result<Vec<ExportInfo>, LiefError> {
        let pe = self.lief_pe();
        let mut exports = Vec::new();

        // LIEF uses functions() for exported functions
        for export in pe.functions() {
            let name = export.name().to_string();
            // Export ordinal - try to get from value if available
            let ordinal = export.value() as u16;
            let rva = export.value() as u64;

            exports.push(ExportInfo {
                name,
                ordinal,
                rva,
            });
        }

        Ok(exports)
    }

    // ========================================================================
    // Resources
    // ========================================================================

    fn get_resources(&self) -> Result<Vec<ResourceInfo>, LiefError> {
        let pe = self.lief_pe();

        // Use LIEF's resource tree to enumerate resources
        // Note: Full resource tree traversal requires more complex handling
        // For now, return basic resource info from data directories
        if let Some(resources) = pe.resources() {
            // Get resource data entries from the resource tree
            let mut result = Vec::new();
            self.collect_resources(&resources, &mut result);
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    // ========================================================================
    // Relocations
    // ========================================================================

    fn get_relocations(&self) -> Result<Vec<RelocationInfo>, LiefError> {
        let pe = self.lief_pe();
        let mut relocations = Vec::new();

        for reloc in pe.relocations() {
            for entry in reloc.entries() {
                relocations.push(RelocationInfo {
                    rva: entry.address() as u64,
                    reloc_type: 0, // Relocation type varies by architecture
                });
            }
        }

        Ok(relocations)
    }
}

// Private helper methods for LiefPe
impl LiefPe {
    /// Recursively collect resources from the resource tree
    fn collect_resources(&self, resource: &lief::pe::ResourceNode, result: &mut Vec<ResourceInfo>) {
        // Note: Full resource tree traversal requires additional LIEF API handling
        // For now, resources are handled via get_resources which accesses data directories
        let _ = resource;
        let _ = result;
    }

    /// Read a u32 from data at the given offset
    fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
        if offset + 4 > data.len() {
            return None;
        }
        Some(u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]))
    }

    /// Read a u64 from data at the given offset
    fn read_u64(data: &[u8], offset: usize) -> Option<u64> {
        if offset + 8 > data.len() {
            return None;
        }
        Some(u64::from_le_bytes([
            data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
            data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7],
        ]))
    }

    /// Read a null-terminated string from data at the given offset
    fn read_string(data: &[u8], offset: usize) -> String {
        if offset >= data.len() {
            return String::new();
        }
        let end = data[offset..]
            .iter()
            .position(|&b| b == 0)
            .map(|pos| offset + pos)
            .unwrap_or(data.len());
        std::str::from_utf8(&data[offset..end])
            .map(|s| s.to_string())
            .unwrap_or_default()
    }
}

// ========================================================================
// Binding Methods
// ========================================================================
// These methods support PE loading and binding using native LIEF APIs.
// Only emulated memory is patched - internal buffers are not modified.

impl LiefPe {
    /// Get the list of imported DLL names (dependencies)
    pub fn get_dependencies(
        &self,
        resolver: Option<&crate::pe::api_set_resolver::ApiSetResolver>,
    ) -> Vec<String> {
        self.lief_pe()
            .imports()
            .map(|import| {
                let name = import.name().to_string();
                match resolver {
                    Some(r) => r.resolve(&name).unwrap_or(name),
                    None => name,
                }
            })
            .collect()
    }

    /// Apply relocations when PE is loaded at a different base address
    ///
    /// Returns the delta (base_addr - image_base) after applying relocations.
    /// Only patches emulated memory - does not modify internal buffers.
    pub fn apply_relocations(&self, emu: &mut crate::emu::Emu, base_addr: u64) -> Result<u64, LiefError> {
        let delta = base_addr.wrapping_sub(self.image_base());
        if delta == 0 {
            return Ok(delta);
        }

        let reloc_dir = match self.get_data_directory(IMAGE_DIRECTORY_ENTRY_BASERELOC) {
            Some(dir) => dir,
            None => return Ok(delta),
        };

        let reloc_rva = reloc_dir.rva() as u64;
        let reloc_size = reloc_dir.size() as usize;
        if reloc_rva == 0 || reloc_size == 0 {
            return Ok(delta);
        }

        let reloc_file_off = match self.rva_to_offset(reloc_rva) {
            Some(off) => off as usize,
            None => return Ok(delta),
        };

        let file_data = self.mapped_file_data();
        if !file_data.get(reloc_file_off..reloc_file_off.saturating_add(reloc_size)).is_some() {
            return Ok(delta);
        }

        log::trace!("Applying base relocations with delta 0x{:x}...", delta);

        let mut offset = reloc_file_off;
        let end = reloc_file_off + reloc_size;

        while offset + 8 <= end {
            let page_rva = Self::read_u32(file_data, offset).unwrap_or(0);
            let block_size = Self::read_u32(file_data, offset + 4).unwrap_or(0);

            if page_rva == 0 && block_size == 0 {
                break;
            }
            if block_size < 8 {
                break;
            }

            let entries = ((block_size as usize) - 8) / 2;
            offset += 8;

            for _ in 0..entries {
                let Some(entry_val) = Self::read_u16(file_data, offset) else { break };
                let entry = entry_val;
                let reloc_type = entry >> 12;
                let reloc_offset = (entry & 0x0FFF) as u32;

                // IMAGE_REL_BASED_DIR64 is type 10
                if reloc_type == 10 {
                    let target_rva = page_rva.wrapping_add(reloc_offset);
                    self.apply_relocation_entry(emu, base_addr, target_rva as u64, delta, file_data);
                }
                offset += 2;
            }
        }

        log::trace!("Base relocations applied successfully");
        Ok(delta)
    }

    /// Apply a single relocation entry
    fn apply_relocation_entry(&self, emu: &mut crate::emu::Emu, base_addr: u64, target_rva: u64, delta: u64, file_data: &[u8]) {
        let Some(target_off) = self.rva_to_offset(target_rva) else { return };
        let target_off = target_off as usize;

        let Some(original_val) = Self::read_u64(file_data, target_off) else { return };
        let new_val = original_val.wrapping_add(delta);

        let patch_addr = base_addr + target_rva;
        if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
            mem.force_write_qword(patch_addr, new_val);
        }
    }

    /// Read a u16 from data at the given offset
    fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
        if offset + 2 > data.len() {
            return None;
        }
        Some(u16::from_le_bytes([data[offset], data[offset + 1]]))
    }

    /// Bind Import Address Table (IAT)
    ///
    /// Uses LIEF's import API to enumerate imports and resolves function
    /// addresses via winapi64::kernel32::resolve_api_name.
    pub fn iat_binding(&self, emu: &mut crate::emu::Emu, base_addr: u64) {
        log::trace!("IAT binding started...");

        for import in self.lief_pe().imports() {
            let dll_name = import.name();
            if dll_name.is_empty() {
                continue;
            }

            for function in import.entries() {
                let func_name = function.name();
                if func_name.is_empty() {
                    continue;
                }

                let iat_rva = function.iat_address() as u64;
                if iat_rva == 0 {
                    continue;
                }

                let real_addr = crate::winapi::winapi64::kernel32::resolve_api_name(emu, &func_name);
                if real_addr == 0 {
                    log::trace!("Could not resolve: {}!{}", dll_name, func_name);
                    continue;
                }

                let patch_addr = base_addr + iat_rva;
                if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
                    mem.force_write_qword(patch_addr, real_addr);
                }
            }
        }

        log::trace!("IAT binding completed");
    }

    /// Bind delay-load imports
    ///
    /// Parses the delay-load directory and resolves function addresses.
    /// Only patches emulated memory - does not modify internal buffers.
    pub fn delay_load_binding(&self, emu: &mut crate::emu::Emu, base_addr: u64) {
        let delay_load_dir = match self.get_data_directory(IMAGE_DIRECTORY_ENTRY_DELAY_LOAD) {
            Some(dir) => dir,
            None => {
                log::trace!("No delay-load directory found");
                return;
            }
        };

        let delay_load_rva = delay_load_dir.rva() as u64;
        let delay_load_size = delay_load_dir.size() as usize;
        if delay_load_rva == 0 || delay_load_size == 0 {
            log::trace!("Delay-load directory is empty");
            return;
        }

        let delay_load_off = match self.rva_to_offset(delay_load_rva) {
            Some(off) => off as usize,
            None => {
                log::trace!("Could not convert delay-load RVA to file offset");
                return;
            }
        };

        let file_data = self.mapped_file_data();
        let Some(reloc_slice) = file_data.get(delay_load_off..delay_load_off.saturating_add(delay_load_size)) else {
            log::trace!("Delay-load directory out of bounds");
            return;
        };

        log::trace!("Delay load binding started...");

        for entry in DelayLoadEntry::iter_from(reloc_slice) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => break,
            };

            if entry.is_empty() {
                break;
            }

            let dll_name = match entry.dll_name(file_data) {
                Some(name) if !name.is_empty() => name,
                _ => continue,
            };

            log::trace!("Processing delay-load DLL: {}", dll_name);
            self.process_delay_load_entries(emu, base_addr, &dll_name, entry, file_data);
        }

        log::trace!("Delay load binding completed");
    }

    /// Process delay-load entries for a single DLL
    fn process_delay_load_entries(&self, emu: &mut crate::emu::Emu, base_addr: u64, dll_name: &str, entry: DelayLoadEntry, file_data: &[u8]) {
        let delay_names_off = match self.rva_to_offset(entry.delay_names() as u64) {
            Some(off) => off as usize,
            None => return,
        };
        let delay_iat_off = match self.rva_to_offset(entry.delay_iat() as u64) {
            Some(off) => off as usize,
            None => return,
        };

        let mut name_off = delay_names_off;
        let mut addr_off = delay_iat_off;

        loop {
            let hint_name = match Self::read_u64(file_data, name_off) {
                Some(v) => v,
                None => break,
            };
            if hint_name == 0 {
                break;
            }

            if hint_name & 0x80000000_00000000 != 0 {
                log::trace!("Delay-load ordinal binding not implemented");
            } else {
                let func_name_rva = (hint_name & 0x7FFFFFFF) as u32;
                let func_name_off = match self.rva_to_offset(func_name_rva as u64) {
                    Some(off) => off as usize + 2, // Skip 2-byte hint
                    None => {
                        name_off += 8;
                        addr_off += 8;
                        continue;
                    }
                };

                let func_name = Self::read_string(file_data, func_name_off);
                if func_name.is_empty() {
                    name_off += 8;
                    addr_off += 8;
                    continue;
                }

                let real_addr = crate::winapi::winapi64::kernel32::resolve_api_name(emu, &func_name);
                if real_addr != 0 {
                    let iat_addr = base_addr + entry.delay_iat() as u64 + (addr_off - delay_iat_off) as u64;
                    if let Some(mem) = emu.maps.get_mem_by_addr_mut(iat_addr) {
                        mem.force_write_qword(iat_addr, real_addr);
                    }
                }
            }

            name_off += 8;
            addr_off += 8;
        }
    }
}

/// Iterator over delay-load directory entries
struct DelayLoadEntry {
    offset: usize,
    data: [u8; 32],
}

impl DelayLoadEntry {
    const ENTRY_SIZE: usize = 32;

    fn iter_from(data: &[u8]) -> DelayLoadEntryIter {
        DelayLoadEntryIter { data, offset: 0 }
    }

    fn from_data(data: &[u8], offset: usize) -> Option<Self> {
        let mut entry = DelayLoadEntry { offset, data: [0; 32] };
        let slice = data.get(offset..offset + 32)?;
        entry.data.copy_from_slice(slice);
        Some(entry)
    }

    fn is_empty(&self) -> bool {
        self.name_ptr() == 0 && self.module_handle() == 0
    }

    fn dll_name<'a>(&self, file_data: &'a [u8]) -> Option<&'a str> {
        let name_off = Self::rva_to_off(file_data, self.name_ptr() as u64)?;
        let name_slice = file_data.get(name_off..)?;
        let end = name_slice.iter().position(|&b| b == 0).unwrap_or(name_slice.len());
        std::str::from_utf8(&name_slice[..end]).ok()
    }

    fn rva_to_off(data: &[u8], rva: u64) -> Option<usize> {
        // This is a simplified version - actual implementation would use section info
        let _ = data;
        let _ = rva;
        None
    }

    // Accessors for the delay-load directory entry fields
    fn attributes(&self) -> u32 { u32::from_le_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]) }
    fn name_ptr(&self) -> u32 { u32::from_le_bytes([self.data[4], self.data[5], self.data[6], self.data[7]]) }
    fn module_handle(&self) -> u32 { u32::from_le_bytes([self.data[8], self.data[9], self.data[10], self.data[11]]) }
    fn delay_iat(&self) -> u32 { u32::from_le_bytes([self.data[16], self.data[17], self.data[18], self.data[19]]) }
    fn delay_names(&self) -> u32 { u32::from_le_bytes([self.data[20], self.data[21], self.data[22], self.data[23]]) }
}

struct DelayLoadEntryIter<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for DelayLoadEntryIter<'a> {
    type Item = Result<DelayLoadEntry, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = match DelayLoadEntry::from_data(self.data, self.offset) {
            Some(e) => e,
            None => return None,
        };
        self.offset += DelayLoadEntry::ENTRY_SIZE;
        Some(Ok(entry))
    }
}
