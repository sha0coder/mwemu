//! Main LIEF PE wrapper
//!
//! This module provides the main `LiefPe` struct that combines
//! header parsing with lazy section loading.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use lief::Relocation;
use lief::generic::{Section, Symbol};
use lief::pe::resources::{Node, NodeBase};
use memmap2::Mmap;

use crate::loaders::pe::lief::error::{
    ExportInfo, ImportFunction, ImportInfo, LiefError, RelocationInfo, ResourceEntryInfo,
    ResourceInfo,
};
use crate::loaders::pe::lief::lief_header_parser::LiefHeaderParser;
use crate::loaders::pe::lief::lief_section_manager::{CachePolicy, LiefSectionManager};
use crate::loaders::pe::lief::traits::LiefPeReader;

/// Data directory indices
const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5;
const IMAGE_DIRECTORY_ENTRY_DELAY_LOAD: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DelayPointerMode {
    Rva,
    Va,
}

impl DelayPointerMode {
    fn from_descriptor_attrs(attrs: u32) -> Self {
        if attrs & 1 != 0 {
            DelayPointerMode::Rva
        } else {
            DelayPointerMode::Va
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub cached_sections: Vec<String>,
    pub cached_bytes: usize,
}

#[derive(Clone, Copy)]
enum ResourceTreeLevel {
    Type,
    Name,
    Language,
}

#[derive(Clone, Default)]
struct ResourceWalkContext {
    type_id: Option<u32>,
    type_name: Option<String>,
    name_id: Option<u32>,
    name: Option<String>,
    language_id: Option<u32>,
}

#[derive(Debug, Clone)]
struct ParsedRelocation {
    rva: u64,
    reloc_type: u16,
    target_file_off: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelayLoadDescriptor {
    pub dll_name: String,
    pub attributes: u32,
    pub dll_name_rva: u32,
    pub module_handle: u32,
    pub delay_iat: u32,
    pub delay_int: u32,
    pub bound_iat: u32,
    pub unload_table: u32,
    pub timestamp: u32,
    pub import_function_names: Option<Vec<String>>,
}

impl DelayLoadDescriptor {
    const ENTRY_SIZE: usize = 32;

    pub fn from_raw(raw: &[u8], offset: usize, dll_name: String) -> Option<Self> {
        let data = raw.get(offset..offset + 32)?;
        let read_u32 =
            |i: usize| u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);
        let d = DelayLoadDescriptor {
            dll_name,
            attributes: read_u32(0),
            dll_name_rva: read_u32(4),
            module_handle: read_u32(8),
            delay_iat: read_u32(12),
            delay_int: read_u32(16),
            bound_iat: read_u32(20),
            unload_table: read_u32(24),
            timestamp: read_u32(28),
            import_function_names: None,
        };
        Some(d)
    }

    pub fn is_empty(&self) -> bool {
        self.dll_name_rva == 0 && self.module_handle == 0
    }
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
    header: LiefHeaderParser,
    section_manager: LiefSectionManager,
    file_path: String,
    file_size: usize,
    import_bindings: HashMap<u64, String>,
}

impl LiefPe {
    /// Load a PE file
    pub fn load(path: &str) -> Result<Self, LiefError> {
        let file = std::fs::File::open(path)
            .map_err(|e| LiefError::FileNotFound(format!("{}: {}", path, e)))?;

        let file_size = file
            .metadata()
            .map_err(|e| LiefError::MmapFailed(format!("Failed to get metadata: {}", e)))?
            .len() as usize;

        let mapped: Arc<Mmap> = Arc::new(
            unsafe { Mmap::map(&file) }
                .map_err(|e| LiefError::MmapFailed(format!("Failed to memory map: {}", e)))?,
        );

        let header = LiefHeaderParser::from_path(path)?;

        let section_manager =
            LiefSectionManager::new(PathBuf::from(path), mapped, header.pe_arc().clone());

        Ok(Self {
            header,
            section_manager,
            file_path: path.to_string(),
            file_size,
            import_bindings: HashMap::new(),
        })
    }

    pub fn load_from_raw(filename: &str, raw: &[u8]) -> Result<Self, LiefError> {
        let file_size = raw.len();

        let header = LiefHeaderParser::from_bytes(raw)?;

        let mapped_file: Arc<[u8]> = Arc::from(raw.to_vec().into_boxed_slice());

        let section_manager = LiefSectionManager::from_bytes(
            PathBuf::from(filename),
            mapped_file,
            header.pe_arc().clone(),
        );

        Ok(Self {
            header,
            section_manager,
            file_path: filename.to_string(),
            file_size,
            import_bindings: HashMap::new(),
        })
    }

    /// Load a PE file with custom cache policy
    pub fn load_with_policy(path: &str, policy: CachePolicy) -> Result<Self, LiefError> {
        let file = std::fs::File::open(path)
            .map_err(|e| LiefError::FileNotFound(format!("{}: {}", path, e)))?;

        let file_size = file
            .metadata()
            .map_err(|e| LiefError::MmapFailed(format!("Failed to get metadata: {}", e)))?
            .len() as usize;

        let mapped: Arc<Mmap> = Arc::new(
            unsafe { Mmap::map(&file) }
                .map_err(|e| LiefError::MmapFailed(format!("Failed to memory map: {}", e)))?,
        );

        let header = LiefHeaderParser::from_path(path)?;

        let section_manager = LiefSectionManager::with_policy(
            PathBuf::from(path),
            mapped,
            policy,
            header.pe_arc().clone(),
        );

        Ok(Self {
            header,
            section_manager,
            file_path: path.to_string(),
            file_size,
            import_bindings: HashMap::new(),
        })
    }
}

impl LiefPe {
    /// Normalize a LIEF-returned address to an RVA.
    /// LIEF may return either an RVA or a VA depending on context;
    /// this strips the image base if the value looks like a VA.
    pub fn normalize_to_rva(&self, value: u64) -> u64 {
        if value >= self.image_base() {
            value.wrapping_sub(self.image_base())
        } else {
            value
        }
    }

    /// Normalize a LIEF IAT address to an RVA for patch calculations.
    fn normalize_iat_to_rva(&self, iat_address: u64) -> u64 {
        self.normalize_to_rva(iat_address)
    }

    fn delay_ptr_to_rva(&self, value: u64, mode: &DelayPointerMode) -> Option<u64> {
        match mode {
            DelayPointerMode::Rva => Some(value),
            DelayPointerMode::Va => {
                if value >= self.image_base() {
                    Some(value.wrapping_sub(self.image_base()))
                } else {
                    None
                }
            }
        }
    }

    fn delay_ptr_to_offset(&self, value: u64, mode: &DelayPointerMode) -> Option<u64> {
        let rva = self.delay_ptr_to_rva(value, mode)?;
        self.rva_to_offset(rva)
    }

    fn delay_patch_va(&self, value: u64, mode: &DelayPointerMode, loaded_base: u64) -> Option<u64> {
        let rva = self.delay_ptr_to_rva(value, mode)?;
        Some(loaded_base + rva)
    }
}

impl LiefPe {
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

    /// Check if a section is loaded in cache by index
    pub fn is_section_index_loaded(&self, index: usize) -> bool {
        self.section_manager.is_section_cached_by_index(index)
    }

    /// Get section data by index with exact semantics.
    ///
    /// Returns:
    /// - `Ok(None)` if the section index is out of bounds
    /// - `Ok(Some(Vec::new()))` if the section has zero raw data (valid empty)
    /// - `Ok(Some(data))` with the section data on success
    /// - `Err(LiefError)` if nonzero raw_size with invalid file offset
    pub fn get_section_data_exact_by_index(
        &self,
        index: usize,
    ) -> Result<Option<Vec<u8>>, LiefError> {
        self.section_manager.get_section_data_exact_by_index(index)
    }

    /// Convert virtual address to file offset
    pub fn vaddr_to_offset(&self, vaddr: u64) -> Option<u64> {
        self.header.vaddr_to_offset(vaddr)
    }

    /// Convert RVA to file offset
    pub fn rva_to_offset(&self, rva: u64) -> Option<u64> {
        self.header.rva_to_offset(rva)
    }

    /// Look up an import address to get "DLL!function" string.
    /// Returns the same format as legacy PE64::pe64_import_addr_to_dll_and_name.
    pub fn import_addr_to_dll_and_name(&self, paddr: u64) -> String {
        if paddr == 0 {
            return String::new();
        }

        if let Some(name) = self.import_bindings.get(&paddr) {
            return name.clone();
        }

        let pe = self.lief_pe();
        for import in pe.imports() {
            let dll_name = import.name();
            if dll_name.is_empty() {
                continue;
            }

            for function in import.entries() {
                let iat_rva = self.normalize_iat_to_rva(function.iat_address() as u64);

                if iat_rva == paddr {
                    let name = function.name();
                    if name.is_empty() {
                        if function.is_ordinal() {
                            return format!("{}!#{}", dll_name, function.ordinal());
                        }
                        return format!("{}!<ordinal>", dll_name);
                    }
                    return format!("{}!{}", dll_name, name);
                }
            }
        }

        String::new()
    }

    pub fn import_addr_to_name(&self, paddr: u64) -> String {
        if paddr == 0 {
            return String::new();
        }

        if let Some(name) = self.import_bindings.get(&paddr) {
            if let Some(pos) = name.find('!') {
                return name[pos + 1..].to_string();
            }
            return name.clone();
        }

        let pe = self.lief_pe();
        for import in pe.imports() {
            for function in import.entries() {
                let iat_rva = self.normalize_iat_to_rva(function.iat_address() as u64);
                if iat_rva == paddr {
                    let name = function.name();
                    if name.is_empty() {
                        if function.is_ordinal() {
                            return format!("#{}", function.ordinal());
                        }
                        return "<ordinal>".to_string();
                    }
                    return name;
                }
            }
        }

        String::new()
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
    pub fn get_section(&self, index: usize) -> Option<lief::pe::Section> {
        self.header.get_section(index)
    }

    pub fn get_data_directory(&self, index: usize) -> Option<lief::pe::DataDirectory> {
        self.header.get_data_directory(index)
    }

    /// Get hierarchy-preserving resource entries.
    ///
    /// Returns `ResourceEntryInfo` structs that preserve the PE resource
    /// directory tree: type -> name/id -> language.
    pub fn get_resource_entries(&self) -> Vec<ResourceEntryInfo> {
        let pe = self.lief_pe();
        let resources = match pe.resources() {
            Some(r) => r,
            None => return Vec::new(),
        };
        let mut result = Vec::new();
        let ctx = ResourceWalkContext::default();
        self.collect_resource_entries(&resources, ResourceTreeLevel::Type, &ctx, &mut result);
        result
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
        self.header
            .get_section(index)
            .map(|s| s.virtual_address() as u64)
    }

    fn get_section_size(&self, index: usize) -> Option<u64> {
        self.header
            .get_section(index)
            .map(|s| s.virtual_size() as u64)
    }

    fn get_section_raw_size(&self, index: usize) -> Option<u64> {
        self.header
            .get_section(index)
            .map(|s| s.sizeof_raw_data() as u64)
    }

    fn get_section_offset(&self, index: usize) -> Option<u64> {
        self.header
            .get_section(index)
            .map(|s| s.pointerto_raw_data() as u64)
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
                let rva = self.normalize_iat_to_rva(function.iat_address() as u64);

                functions.push(ImportFunction { name, ordinal, rva });
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

            exports.push(ExportInfo { name, ordinal, rva });
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

    fn get_resource(
        &self,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        resource_name: Option<&str>,
    ) -> Option<(u64, usize)> {
        // Find the .rsrc section index
        let section_idx = (0..self.num_sections() as usize)
            .find(|&i| self.get_section_name(i).as_deref() == Some(".rsrc"))?;

        let rsrc = self.get_section_ptr(section_idx);
        if rsrc.is_empty() {
            return None;
        }

        let (offset_to_data, size) = self.locate_resource_data_entry(
            &rsrc,
            0,
            0,
            type_id,
            name_id,
            type_name,
            resource_name,
        )?;

        Some((offset_to_data as u64, size as usize))
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
                    reloc_type: u32::from(entry.get_type()) as u16,
                });
            }
        }

        Ok(relocations)
    }
}

// Private helper methods for LiefPe
impl LiefPe {
    fn collect_resource_entries(
        &self,
        node: &lief::pe::ResourceNode,
        level: ResourceTreeLevel,
        context: &ResourceWalkContext,
        result: &mut Vec<ResourceEntryInfo>,
    ) {
        for child in node.children() {
            let id = child.id();
            let name_str = child.name();

            let ctx = match level {
                ResourceTreeLevel::Type => ResourceWalkContext {
                    type_id: Some(id),
                    type_name: name_str,
                    ..context.clone()
                },
                ResourceTreeLevel::Name => ResourceWalkContext {
                    name_id: Some(id),
                    name: name_str,
                    ..context.clone()
                },
                ResourceTreeLevel::Language => ResourceWalkContext {
                    language_id: Some(id),
                    ..context.clone()
                },
            };

            match &child {
                Node::Directory(dir) => {
                    let next = match level {
                        ResourceTreeLevel::Type => ResourceTreeLevel::Name,
                        ResourceTreeLevel::Name => ResourceTreeLevel::Language,
                        ResourceTreeLevel::Language => ResourceTreeLevel::Language,
                    };
                    self.collect_resource_entries_dir(dir, next, &ctx, result);
                }
                Node::Data(data) => {
                    let data_rva = data.offset() as u64;
                    let file_offset = self.rva_to_offset(data_rva);
                    let size = data.content().len() as u64;
                    result.push(ResourceEntryInfo {
                        type_id: ctx.type_id,
                        type_name: ctx.type_name.clone(),
                        name_id: ctx.name_id,
                        name: ctx.name.clone(),
                        language_id: ctx.language_id,
                        data_rva,
                        file_offset,
                        size,
                    });
                }
            }
        }
    }

    fn collect_resource_entries_dir(
        &self,
        dir: &lief::pe::resources::Directory,
        next_level: ResourceTreeLevel,
        ctx: &ResourceWalkContext,
        result: &mut Vec<ResourceEntryInfo>,
    ) {
        for child in dir.children() {
            let id = child.id();
            let name_str = child.name();

            let child_ctx = match next_level {
                ResourceTreeLevel::Type => ResourceWalkContext {
                    type_id: Some(id),
                    type_name: name_str,
                    ..ctx.clone()
                },
                ResourceTreeLevel::Name => ResourceWalkContext {
                    name_id: Some(id),
                    name: name_str,
                    ..ctx.clone()
                },
                ResourceTreeLevel::Language => ResourceWalkContext {
                    language_id: Some(id),
                    ..ctx.clone()
                },
            };

            match &child {
                Node::Directory(sub_dir) => {
                    let next = match next_level {
                        ResourceTreeLevel::Type => ResourceTreeLevel::Name,
                        ResourceTreeLevel::Name => ResourceTreeLevel::Language,
                        ResourceTreeLevel::Language => ResourceTreeLevel::Language,
                    };
                    self.collect_resource_entries_dir(sub_dir, next, &child_ctx, result);
                }
                Node::Data(data) => {
                    let data_rva = data.offset() as u64;
                    let file_offset = self.rva_to_offset(data_rva);
                    let size = data.content().len() as u64;
                    result.push(ResourceEntryInfo {
                        type_id: child_ctx.type_id,
                        type_name: child_ctx.type_name.clone(),
                        name_id: child_ctx.name_id,
                        name: child_ctx.name.clone(),
                        language_id: child_ctx.language_id,
                        data_rva,
                        file_offset,
                        size,
                    });
                }
            }
        }
    }

    fn collect_resources(&self, resource: &lief::pe::ResourceNode, result: &mut Vec<ResourceInfo>) {
        for child in resource.children() {
            let type_id = child.id();
            let name_str = child.name().unwrap_or_else(|| format!("{}", type_id));

            match &child {
                Node::Directory(dir) => {
                    self.collect_resources_from_dir(dir, type_id, &name_str, result);
                }
                Node::Data(data) => {
                    result.push(ResourceInfo {
                        resource_type: type_id,
                        name: name_str,
                        rva: data.offset() as u64,
                        size: data.content().len() as u64,
                    });
                }
            }
        }
    }

    fn collect_resources_from_dir(
        &self,
        dir: &lief::pe::resources::Directory,
        parent_type: u32,
        parent_name: &str,
        result: &mut Vec<ResourceInfo>,
    ) {
        for child in dir.children() {
            let child_id = child.id();
            let child_name = child.name().unwrap_or_else(|| format!("{}", child_id));

            match &child {
                Node::Directory(sub_dir) => {
                    self.collect_resources_from_dir(sub_dir, parent_type, parent_name, result);
                }
                Node::Data(data) => {
                    result.push(ResourceInfo {
                        resource_type: parent_type,
                        name: format!("{}:{}", parent_name, child_name),
                        rva: data.offset() as u64,
                        size: data.content().len() as u64,
                    });
                }
            }
        }
    }

    /// Read a u32 from data at the given offset
    fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
        if offset + 4 > data.len() {
            return None;
        }
        Some(u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]))
    }

    /// Read a u64 from data at the given offset
    fn read_u64(data: &[u8], offset: usize) -> Option<u64> {
        if offset + 8 > data.len() {
            return None;
        }
        Some(u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
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

    /// Read a UTF-16 LE string from resource data at the given offset
    fn read_resource_name(data: &[u8], offset: usize) -> String {
        if offset + 1 >= data.len() {
            return String::new();
        }
        let length = u16::from_le_bytes([data[offset], data[offset + 1]]) as usize;
        let string_start = offset + 2;
        let required_bytes = string_start + (length * 2);
        if required_bytes > data.len() {
            return String::new();
        }
        let utf16_data: Vec<u16> = (0..length)
            .map(|i| {
                let idx = string_start + i * 2;
                u16::from_le_bytes([data[idx], data[idx + 1]])
            })
            .collect();
        String::from_utf16_lossy(&utf16_data)
    }

    /// Locate a resource data entry by traversing the resource directory tree
    ///
    /// Returns (offset_to_data, size) if found.
    fn locate_resource_data_entry(
        &self,
        rsrc: &[u8],
        offset: usize,
        level: u32,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u32, u32)> {
        if level >= 10 {
            return None;
        }
        if offset + 16 > rsrc.len() {
            return None;
        }

        let characteristics = Self::read_u32(rsrc, offset)?;
        let _time_date_stamp = Self::read_u32(rsrc, offset + 4)?;
        let _major_version = Self::read_u16(rsrc, offset + 8)?;
        let _minor_version = Self::read_u16(rsrc, offset + 10)?;
        let number_of_named_entries = Self::read_u16(rsrc, offset + 12)?;
        let number_of_id_entries = Self::read_u16(rsrc, offset + 14)?;

        let total_entries = (number_of_named_entries as usize) + (number_of_id_entries as usize);

        for i in 0..total_entries {
            let entry_offset = offset + 16 + (i * 8);
            if entry_offset + 8 > rsrc.len() {
                continue;
            }

            let name_or_id = Self::read_u32(rsrc, entry_offset)?;
            let data_or_directory = Self::read_u32(rsrc, entry_offset + 4)?;

            let is_directory = (data_or_directory & 0x80000000) != 0;
            let entry_offset_val = data_or_directory & 0x7FFFFFFF;

            let matched = if (name_or_id & 0x80000000) == 0 {
                let entry_id = name_or_id;
                if level == 0 && type_id.is_some() && type_id.unwrap() == entry_id {
                    true
                } else if level == 1 && name_id.is_some() && name_id.unwrap() == entry_id {
                    true
                } else if level == 2 {
                    true
                } else {
                    false
                }
            } else {
                let name_offset = (name_or_id & 0x7FFFFFFF) as usize;
                let resource_name = Self::read_resource_name(rsrc, name_offset);
                if level == 0 && type_name.is_some() && type_name.unwrap() == resource_name {
                    true
                } else if level == 1 && name.is_some() && name.unwrap() == resource_name {
                    true
                } else {
                    false
                }
            };

            if matched {
                if is_directory {
                    return self.locate_resource_data_entry(
                        rsrc,
                        entry_offset_val as usize,
                        level + 1,
                        type_id,
                        name_id,
                        type_name,
                        name,
                    );
                } else {
                    let data_entry_offset = entry_offset_val as usize;
                    if data_entry_offset + 16 > rsrc.len() {
                        return None;
                    }
                    let offset_to_data = Self::read_u32(rsrc, data_entry_offset)?;
                    let size = Self::read_u32(rsrc, data_entry_offset + 4)?;
                    return Some((offset_to_data, size));
                }
            }
        }
        None
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
    /// Parse relocation directory blocks into typed parsed entries.
    /// Validates 8 readable bytes at each DIR64 target file offset.
    /// Used by both preflight validation and apply. Returns Ok(Vec) or Err on parse failure.
    fn parse_relocation_blocks(&self) -> Result<Vec<ParsedRelocation>, LiefError> {
        let reloc_dir = match self.get_data_directory(IMAGE_DIRECTORY_ENTRY_BASERELOC) {
            Some(dir) => dir,
            None => return Ok(Vec::new()),
        };

        let reloc_rva = reloc_dir.rva() as u64;
        let reloc_size = reloc_dir.size() as usize;
        if reloc_rva == 0 || reloc_size == 0 {
            return Ok(Vec::new());
        }

        let reloc_file_off = match self.rva_to_offset(reloc_rva) {
            Some(off) => off as usize,
            None => {
                return Err(LiefError::RelocationError(format!(
                    "cannot convert reloc RVA 0x{:x} to file offset (file: {})",
                    reloc_rva, self.file_path
                )));
            }
        };

        let file_data = self.mapped_file_data();
        if reloc_file_off.saturating_add(reloc_size) > file_data.len() {
            return Err(LiefError::RelocationError(format!(
                "reloc directory out of bounds: off=0x{:x} size={} file_len={} (file: {})",
                reloc_file_off,
                reloc_size,
                file_data.len(),
                self.file_path
            )));
        }

        let reloc_data = &file_data[reloc_file_off..reloc_file_off + reloc_size];
        let mut offset: usize = 0;
        let mut entries: Vec<ParsedRelocation> = Vec::new();

        while offset + 8 <= reloc_data.len() {
            let page_rva = match Self::read_u32(reloc_data, offset) {
                Some(v) => v as u64,
                None => break,
            };
            let block_size = match Self::read_u32(reloc_data, offset + 4) {
                Some(v) => v,
                None => break,
            };

            if page_rva == 0 && block_size == 0 {
                break;
            }
            if block_size < 8 {
                if block_size > 0 {
                    return Err(LiefError::RelocationError(format!(
                        "invalid reloc block size {} at offset 0x{:x} (file: {})",
                        block_size, offset, self.file_path
                    )));
                }
                break;
            }
            if offset.saturating_add(block_size as usize) > reloc_data.len() {
                return Err(LiefError::RelocationError(format!(
                    "reloc block truncated at offset 0x{:x} size={} (file: {})",
                    offset, block_size, self.file_path
                )));
            }

            let header_size: usize = 8;
            let entry_count = (block_size as usize - header_size) / 2;
            let entry_start = offset + header_size;

            for i in 0..entry_count {
                let entry_off = entry_start + i * 2;
                let entry = match Self::read_u16(reloc_data, entry_off) {
                    Some(v) => v,
                    None => break,
                };
                let reloc_type = entry >> 12;
                let reloc_offset = (entry & 0x0FFF) as u64;

                if reloc_type == 0 {
                    continue;
                }

                if reloc_type != 10 {
                    let target_rva = page_rva + reloc_offset;
                    return Err(LiefError::UnsupportedRelocationType {
                        reloc_type,
                        rva: target_rva,
                    });
                }

                let target_rva = page_rva + reloc_offset;
                let target_file_off = match self.rva_to_offset(target_rva) {
                    Some(off) => off as usize,
                    None => {
                        return Err(LiefError::RelocationError(format!(
                            "reloc target RVA 0x{:x} falls in no section (file: {})",
                            target_rva, self.file_path
                        )));
                    }
                };
                if target_file_off.saturating_add(8) > file_data.len() {
                    return Err(LiefError::RelocationError(format!(
                        "cannot read 8 bytes at file offset 0x{:x} for DIR64 target RVA 0x{:x} (file: {})",
                        target_file_off, target_rva, self.file_path
                    )));
                }

                let image_size = self.lief_pe().optional_header().sizeof_image();
                if target_rva + 8 > image_size as u64 {
                    return Err(LiefError::RelocationError(format!(
                        "DIR64 target RVA 0x{:x} extends beyond image size 0x{:x} (file: {})",
                        target_rva, image_size, self.file_path
                    )));
                }

                entries.push(ParsedRelocation {
                    rva: target_rva,
                    reloc_type,
                    target_file_off,
                });
            }

            offset = offset.saturating_add(block_size as usize);
        }

        Ok(entries)
    }

    pub fn apply_relocations(
        &self,
        emu: &mut crate::emu::Emu,
        base_addr: u64,
    ) -> Result<u64, LiefError> {
        let delta = base_addr.wrapping_sub(self.image_base());
        if delta == 0 {
            return Ok(delta);
        }

        let entries = self.parse_relocation_blocks()?;
        if entries.is_empty() {
            return Ok(delta);
        }

        log::trace!(
            "Applying {} base relocations with delta 0x{:x}...",
            entries.len(),
            delta
        );

        for entry in &entries {
            self.apply_relocation_entry(emu, base_addr, entry, delta)?;
        }

        log::trace!("Base relocations applied successfully");
        Ok(delta)
    }

    /// Validate that the relocation directory can be parsed without actually
    /// applying relocations. Used as a preflight check before map creation.
    pub fn validate_relocation_directory(&self) -> Result<(), LiefError> {
        self.parse_relocation_blocks()?;
        Ok(())
    }

    /// Apply a single relocation entry
    fn apply_relocation_entry(
        &self,
        emu: &mut crate::emu::Emu,
        base_addr: u64,
        entry: &ParsedRelocation,
        delta: u64,
    ) -> Result<(), LiefError> {
        let target_rva = entry.rva;
        let file_data = self.mapped_file_data();
        let target_off = entry.target_file_off;
        let original_val = Self::read_u64(file_data, target_off).ok_or_else(|| {
            LiefError::RelocationError(format!(
                "cannot read 8-byte value at file offset 0x{:x} for reloc RVA 0x{:x} (file: {}, base: 0x{:x})",
                target_off, target_rva, self.file_path, base_addr
            ))
        })?;
        let new_val = original_val.wrapping_add(delta);

        let patch_addr = base_addr + target_rva;
        if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
            mem.force_write_qword(patch_addr, new_val);
        } else {
            return Err(LiefError::RelocationError(format!(
                "no mapped memory at patch VA 0x{:x} for reloc RVA 0x{:x} (file: {}, base: 0x{:x})",
                patch_addr, target_rva, self.file_path, base_addr
            )));
        }

        Ok(())
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
    /// Records resolved addresses in the import_bindings map for later lookup.
    pub fn iat_binding(&mut self, emu: &mut crate::emu::Emu, base_addr: u64) {
        log::trace!("IAT binding started...");

        let mut new_bindings: Vec<(u64, String)> = Vec::new();
        let mut cache: HashMap<String, u64> = HashMap::new();

        {
            let pe = self.lief_pe();
            for import in pe.imports() {
                let dll_name = import.name();
                if dll_name.is_empty() {
                    continue;
                }

                if crate::winapi::winapi64::kernel32::load_library(emu, &dll_name) == 0 {
                    if emu.cfg.verbose >= 1 {
                        log::trace!(
                            "cannot find/import library `{}` (LIEF IAT binding will skip it)",
                            dll_name
                        );
                    }
                    continue;
                }

                for function in import.entries() {
                    let func_name = function.name();
                    let is_ordinal = function.is_ordinal();

                    let iat_rva = self.normalize_iat_to_rva(function.iat_address() as u64);
                    if iat_rva == 0 {
                        continue;
                    }

                    let (cache_key, real_addr) = if !func_name.is_empty() {
                        let key =
                            format!("{}!{}", dll_name.to_lowercase(), func_name.to_lowercase());
                        let addr = if let Some(&cached) = cache.get(&key) {
                            cached
                        } else {
                            let resolved =
                                crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                                    emu, &dll_name, &func_name,
                                );
                            if resolved != 0 {
                                cache.insert(key.clone(), resolved);
                            }
                            resolved
                        };
                        (key, addr)
                    } else if is_ordinal {
                        let ordinal = function.ordinal();
                        let key = format!("{}!#{}", dll_name.to_lowercase(), ordinal);
                        let addr = if let Some(&cached) = cache.get(&key) {
                            cached
                        } else {
                            let resolved =
                                crate::winapi::winapi64::kernel32::resolve_api_ordinal_in_module(
                                    emu, &dll_name, ordinal,
                                );
                            if resolved != 0 {
                                cache.insert(key.clone(), resolved);
                            }
                            resolved
                        };
                        (key, addr)
                    } else {
                        continue;
                    };

                    if real_addr == 0 {
                        continue;
                    }

                    let patch_addr = base_addr + iat_rva;
                    if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
                        mem.force_write_qword(patch_addr, real_addr);
                    }

                    let binding_name = if !func_name.is_empty() {
                        format!("{}!{}", dll_name, func_name)
                    } else {
                        format!("{}!#{}", dll_name, function.ordinal())
                    };
                    new_bindings.push((real_addr, binding_name));
                }
            }
            let _ = pe;
        }

        for (addr, name) in new_bindings {
            self.import_bindings.insert(addr, name);
        }

        log::trace!("IAT binding completed");
    }

    /// Parse the delay-load directory and return normalized descriptors (for parity testing).
    /// Uses the data directory (index 13) RVA and size to locate descriptors on disk.
    pub fn delay_load_descriptors(&self) -> Vec<DelayLoadDescriptor> {
        let delay_dir = match self.get_data_directory(IMAGE_DIRECTORY_ENTRY_DELAY_LOAD) {
            Some(dir) => dir,
            None => return Vec::new(),
        };
        let delay_dir_rva = delay_dir.rva() as u64;
        let delay_dir_size = delay_dir.size() as usize;
        if delay_dir_rva == 0 || delay_dir_size == 0 {
            return Vec::new();
        }
        let file_data = self.mapped_file_data();
        let file_off = match self.rva_to_offset(delay_dir_rva) {
            Some(off) => off as usize,
            None => return Vec::new(),
        };
        if file_off.saturating_add(DelayLoadDescriptor::ENTRY_SIZE) > file_data.len() {
            return Vec::new();
        }
        parse_delay_load_descriptors_raw(&file_data, file_off, delay_dir_size, self)
    }

    pub fn delay_load_binding(&self, emu: &mut crate::emu::Emu, base_addr: u64) {
        let mut cache: HashMap<String, u64> = HashMap::new();
        self.delay_load_binding_inner(emu, base_addr, &mut cache);
    }

    fn delay_load_binding_inner(
        &self,
        emu: &mut crate::emu::Emu,
        base_addr: u64,
        cache: &mut HashMap<String, u64>,
    ) -> Vec<(u64, String)> {
        log::trace!("Delay-load binding started...");

        let file_data = self.mapped_file_data();
        let descriptors = self.delay_load_descriptors();
        if descriptors.is_empty() {
            log::trace!("No delay imports found");
            return Vec::new();
        }

        let mut total_bindings: Vec<(u64, String)> = Vec::new();

        for descriptor in descriptors {
            let mode = DelayPointerMode::from_descriptor_attrs(descriptor.attributes);
            if descriptor.dll_name.is_empty() {
                continue;
            }

            if crate::winapi::winapi64::kernel32::load_library(emu, &descriptor.dll_name) == 0 {
                continue;
            }

            let bindings =
                self.process_delay_load_entries(emu, base_addr, &descriptor, &file_data, cache);
            total_bindings.extend(bindings);
        }

        log::trace!(
            "Delay-load binding completed ({} bindings)",
            total_bindings.len()
        );
        total_bindings
    }

    /// Process delay-load entries for a single descriptor
    fn process_delay_load_entries(
        &self,
        emu: &mut crate::emu::Emu,
        base_addr: u64,
        descriptor: &DelayLoadDescriptor,
        file_data: &[u8],
        cache: &mut HashMap<String, u64>,
    ) -> Vec<(u64, String)> {
        let mut bindings = Vec::new();
        let mode = DelayPointerMode::from_descriptor_attrs(descriptor.attributes);
        let dll_name = &descriptor.dll_name;

        let delay_int_ptr = descriptor.delay_int as u64;
        let delay_iat_ptr = descriptor.delay_iat as u64;

        let delay_names_off = match self.delay_ptr_to_offset(delay_int_ptr, &mode) {
            Some(off) => off as usize,
            None => return bindings,
        };

        let mut name_off = delay_names_off;
        let mut iat_slot_idx: usize = 0;

        loop {
            let hint_name = match Self::read_u64(file_data, name_off) {
                Some(v) => v,
                None => break,
            };
            if hint_name == 0 {
                break;
            }

            if hint_name & 0x80000000_00000000 != 0 {
                let ordinal = (hint_name & 0xFFFF) as u16;
                let cache_key = format!("{}!#{}", dll_name.to_lowercase(), ordinal);
                let real_addr = if let Some(&cached) = cache.get(&cache_key) {
                    cached
                } else {
                    let addr = crate::winapi::winapi64::kernel32::resolve_api_ordinal_in_module(
                        emu, dll_name, ordinal,
                    );
                    if addr != 0 {
                        cache.insert(cache_key.clone(), addr);
                    }
                    addr
                };
                if real_addr != 0 {
                    let slot_ptr = delay_iat_ptr.saturating_add((iat_slot_idx as u64) * 8);
                    let patch_va = match self.delay_patch_va(slot_ptr, &mode, base_addr) {
                        Some(va) => va,
                        None => {
                            name_off += 8;
                            iat_slot_idx += 1;
                            continue;
                        }
                    };
                    if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_va) {
                        mem.force_write_qword(patch_va, real_addr);
                    }
                    bindings.push((real_addr, format!("{}!#{}", dll_name, ordinal)));
                }
            } else {
                let func_name_off = match self.delay_ptr_to_offset(hint_name, &mode) {
                    Some(off) => off as usize + 2,
                    None => {
                        name_off += 8;
                        iat_slot_idx += 1;
                        continue;
                    }
                };

                let func_name = Self::read_string(file_data, func_name_off);
                if func_name.is_empty() {
                    name_off += 8;
                    iat_slot_idx += 1;
                    continue;
                }

                let cache_key = format!("{}!{}", dll_name.to_lowercase(), func_name.to_lowercase());
                let real_addr = if let Some(&cached) = cache.get(&cache_key) {
                    cached
                } else {
                    let addr = crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                        emu, dll_name, &func_name,
                    );
                    if addr != 0 {
                        cache.insert(cache_key.clone(), addr);
                    }
                    addr
                };
                if real_addr != 0 {
                    let slot_ptr = delay_iat_ptr.saturating_add((iat_slot_idx as u64) * 8);
                    let patch_va = match self.delay_patch_va(slot_ptr, &mode, base_addr) {
                        Some(va) => va,
                        None => {
                            name_off += 8;
                            iat_slot_idx += 1;
                            continue;
                        }
                    };
                    if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_va) {
                        mem.force_write_qword(patch_va, real_addr);
                    }
                    bindings.push((real_addr, format!("{}!{}", dll_name, func_name)));
                }
            }

            name_off += 8;
            iat_slot_idx += 1;
        }

        bindings
    }
}

/// Parse delay-load descriptors from raw PE data at the given file offset.
/// Uses `DelayLoadDescriptor::from_raw` for each entry within the directory bounds.
fn parse_delay_load_descriptors_raw(
    data: &[u8],
    start_off: usize,
    dir_size: usize,
    reader: &impl LiefPeReader,
) -> Vec<DelayLoadDescriptor> {
    let mut result = Vec::new();
    let mut off = start_off;
    let end = (start_off + dir_size).min(data.len());
    while off + DelayLoadDescriptor::ENTRY_SIZE <= end {
        let desc = match DelayLoadDescriptor::from_raw(data, off, String::new()) {
            Some(d) => d,
            None => break,
        };
        if desc.is_empty() {
            break;
        }
        let dll_name = if desc.dll_name_rva > 0 {
            let name_file_off = match reader.rva_to_offset(desc.dll_name_rva as u64) {
                Some(o) => o as usize,
                None => {
                    off += DelayLoadDescriptor::ENTRY_SIZE;
                    continue;
                }
            };
            match data.get(name_file_off..) {
                Some(slice) => reader::read_cstr(slice),
                None => String::new(),
            }
        } else {
            String::new()
        };
        let mut desc = desc;
        desc.dll_name = dll_name;
        result.push(desc);
        off += DelayLoadDescriptor::ENTRY_SIZE;
    }
    result
}

mod reader {
    pub(super) fn read_cstr(data: &[u8]) -> String {
        let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
        String::from_utf8_lossy(&data[..end]).to_string()
    }
}

// ========================================================================
// 32-bit Binding Methods (for PE32)
// ========================================================================
// These mirror the 64-bit binding methods but use winapi32 and patch
// 32-bit (4-byte) values in emulated memory. PE32 IAT and delay-import
// entries are 32-bit pointers; using 64-bit writes here would corrupt
// the target's memory layout.

impl LiefPe {
    /// Bind Import Address Table (IAT) for a 32-bit (PE32) image.
    ///
    /// Uses LIEF's import API to enumerate imports and resolves function
    /// addresses via `winapi32::kernel32::resolve_api_name`. Records resolved
    /// addresses in the `import_bindings` map for later lookup.
    /// Only emulated memory is patched (4-byte dwords).
    pub fn iat_binding32(&mut self, emu: &mut crate::emu::Emu, base_addr: u32) {
        log::trace!("IAT (32-bit) binding started...");

        let mut new_bindings: Vec<(u64, String)> = Vec::new();
        let mut cache: HashMap<String, u64> = HashMap::new();

        {
            let pe = self.lief_pe();
            for import in pe.imports() {
                let dll_name = import.name();
                if dll_name.is_empty() {
                    continue;
                }

                if crate::winapi::winapi32::kernel32::load_library(emu, &dll_name) == 0 {
                    if emu.cfg.verbose >= 1 {
                        log::trace!(
                            "cannot find/import library `{}` (LIEF PE32 IAT binding will skip it)",
                            dll_name
                        );
                    }
                    continue;
                }

                for function in import.entries() {
                    let func_name = function.name();
                    let is_ordinal = function.is_ordinal();

                    let iat_rva = self.normalize_iat_to_rva(function.iat_address() as u64);
                    if iat_rva == 0 {
                        continue;
                    }

                    let (cache_key, real_addr) = if !func_name.is_empty() {
                        let key =
                            format!("{}!{}", dll_name.to_lowercase(), func_name.to_lowercase());
                        let addr = if let Some(&cached) = cache.get(&key) {
                            cached
                        } else {
                            let resolved =
                                crate::winapi::winapi32::kernel32::resolve_api_name_in_module(
                                    emu, &dll_name, &func_name,
                                );
                            if resolved != 0 {
                                cache.insert(key.clone(), resolved);
                            }
                            resolved
                        };
                        (key, addr)
                    } else if is_ordinal {
                        let ordinal = function.ordinal();
                        let key = format!("{}!#{}", dll_name.to_lowercase(), ordinal);
                        let addr = if let Some(&cached) = cache.get(&key) {
                            cached
                        } else {
                            let resolved =
                                crate::winapi::winapi32::kernel32::resolve_api_ordinal_in_module(
                                    emu, &dll_name, ordinal,
                                );
                            if resolved != 0 {
                                cache.insert(key.clone(), resolved);
                            }
                            resolved
                        };
                        (key, addr)
                    } else {
                        continue;
                    };

                    if real_addr == 0 {
                        continue;
                    }

                    let patch_addr = base_addr as u64 + iat_rva;
                    let real_addr_u32 = real_addr as u32;
                    if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
                        mem.force_write_dword(patch_addr, real_addr_u32);
                    }

                    let binding_name = if !func_name.is_empty() {
                        format!("{}!{}", dll_name, func_name)
                    } else {
                        format!("{}!#{}", dll_name, function.ordinal())
                    };
                    new_bindings.push((real_addr, binding_name));
                }
            }
            let _ = pe;
        }

        for (addr, name) in new_bindings {
            self.import_bindings.insert(addr, name);
        }

        log::trace!("IAT (32-bit) binding completed");
    }

    /// Bind delay-load imports for a 32-bit (PE32) image.
    ///
    /// 32-bit delay-import entries use 4-byte pointers in the IAT and INT
    /// (Import Name Table). This walks the delay descriptor, reads 4-byte
    /// hint-name pointers from the INT, and patches the 4-byte slots in
    /// the IAT. Function names are resolved via `winapi32::kernel32`.
    ///
    /// Resolved bindings are recorded in `import_bindings` so that
    /// `import_addr_to_name32` can resolve names for delay-load slots
    /// (mirrors the behavior of `iat_binding32`).
    pub fn delay_load_binding32(&mut self, emu: &mut crate::emu::Emu, base_addr: u32) {
        log::trace!("Delay-load (32-bit) binding started...");

        let descriptors = self.delay_load_descriptors();
        if descriptors.is_empty() {
            log::trace!("No delay imports found (PE32)");
            return;
        }

        let file_data = self.mapped_file_data();
        let mut cache: HashMap<String, u64> = HashMap::new();
        let mut new_bindings: Vec<(u64, String)> = Vec::new();

        for descriptor in descriptors {
            if descriptor.dll_name.is_empty() {
                continue;
            }

            if crate::winapi::winapi32::kernel32::load_library(emu, &descriptor.dll_name) == 0 {
                continue;
            }

            let mode = DelayPointerMode::from_descriptor_attrs(descriptor.attributes);
            let dll_name = &descriptor.dll_name;

            // Convert delay_int and delay_iat from RVA to file offset
            let delay_names_off = match self.delay_ptr_to_offset(descriptor.delay_int as u64, &mode)
            {
                Some(off) => off as usize,
                None => continue,
            };

            let delay_iat_rva = match self.delay_ptr_to_rva(descriptor.delay_iat as u64, &mode) {
                Some(rva) => rva,
                None => continue,
            };

            let mut name_off = delay_names_off;
            let mut iat_slot_idx: usize = 0;

            loop {
                // Read 4-byte INT entry
                let hint_name = match Self::read_u32(file_data, name_off) {
                    Some(v) => v as u64,
                    None => break,
                };
                if hint_name == 0 {
                    break;
                }

                if hint_name & 0x80000000 != 0 {
                    // Ordinal import: low 16 bits are the ordinal
                    let ordinal = (hint_name & 0xFFFF) as u16;
                    let cache_key = format!("{}!#{}", dll_name.to_lowercase(), ordinal);
                    let real_addr = if let Some(&cached) = cache.get(&cache_key) {
                        cached
                    } else {
                        let addr = crate::winapi::winapi32::kernel32::resolve_api_ordinal_in_module(
                            emu, dll_name, ordinal,
                        );
                        if addr != 0 {
                            cache.insert(cache_key.clone(), addr);
                        }
                        addr
                    };
                    if real_addr != 0 {
                        // Patch 4-byte IAT slot at base_addr + delay_iat_rva + slot_idx*4
                        let slot_rva = delay_iat_rva + (iat_slot_idx as u64) * 4;
                        let patch_va = base_addr as u64 + slot_rva;
                        if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_va) {
                            mem.force_write_dword(patch_va, real_addr as u32);
                        }
                        new_bindings.push((real_addr, format!("{}!#{}", dll_name, ordinal)));
                    }
                } else {
                    // Name import: hint_name is an RVA to a Hint/Name entry
                    let func_name_off = match self.delay_ptr_to_offset(hint_name, &mode) {
                        Some(off) => off as usize + 2, // skip 2-byte Hint
                        None => {
                            name_off += 4;
                            iat_slot_idx += 1;
                            continue;
                        }
                    };

                    let func_name = Self::read_string(file_data, func_name_off);
                    if func_name.is_empty() {
                        name_off += 4;
                        iat_slot_idx += 1;
                        continue;
                    }

                    let cache_key =
                        format!("{}!{}", dll_name.to_lowercase(), func_name.to_lowercase());
                    let real_addr = if let Some(&cached) = cache.get(&cache_key) {
                        cached
                    } else {
                        let addr = crate::winapi::winapi32::kernel32::resolve_api_name_in_module(
                            emu, dll_name, &func_name,
                        );
                        if addr != 0 {
                            cache.insert(cache_key.clone(), addr);
                        }
                        addr
                    };
                    if real_addr != 0 {
                        let slot_rva = delay_iat_rva + (iat_slot_idx as u64) * 4;
                        let patch_va = base_addr as u64 + slot_rva;
                        if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_va) {
                            mem.force_write_dword(patch_va, real_addr as u32);
                        }
                        new_bindings.push((real_addr, format!("{}!{}", dll_name, func_name)));
                    }
                }

                name_off += 4;
                iat_slot_idx += 1;
            }
        }

        for (addr, name) in new_bindings {
            self.import_bindings.insert(addr, name);
        }

        log::trace!("Delay-load (32-bit) binding completed");
    }

    /// Map a 32-bit import address (RVA) to a function name using LIEF.
    ///
    /// `paddr` is a 32-bit RVA in the import address table.
    pub fn import_addr_to_name32(&self, paddr: u32) -> String {
        self.import_addr_to_name(paddr as u64)
    }
}
