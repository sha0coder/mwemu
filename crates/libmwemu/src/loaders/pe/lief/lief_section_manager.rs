//! Lazy section loading manager with LRU cache
//!
//! This module provides section loading on-demand with configurable
//! cache policies to prevent unbounded memory growth.

use std::collections::HashMap;
use std::mem;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use lief::generic::Section;
use lief::pe::Binary as PeBinary;
use memmap2::Mmap;

use crate::loaders::pe::lief::error::LiefError;

/// Cache policy for section data
#[derive(Debug, Clone)]
pub enum CachePolicy {
    /// LRU eviction with maximum byte limit
    LRU { max_bytes: usize },
    /// No limit on cached data
    Unlimited,
}

/// Lazy section loading manager
///
/// Cache is internally canonical by section index to avoid duplicate entries
/// for the same section loaded by name and by index, and to handle sections
/// with duplicate or empty names.
pub struct LiefSectionManager {
    /// Path to the PE file
    file_path: PathBuf,
    /// Memory-mapped file for section data access (or bytes for from_bytes)
    mapped_file: Arc<[u8]>,
    /// Section data cache keyed by index
    section_cache: RwLock<HashMap<usize, Vec<u8>>>,
    /// Cache eviction policy
    cache_policy: CachePolicy,
    /// LRU access order tracking (by index)
    access_order: RwLock<Vec<usize>>,
    /// Cached LIEF PE binary (via Arc) to avoid re-parsing on every section access
    pe_binary: Option<Arc<PeBinary>>,
}

impl LiefSectionManager {
    /// Create a new section manager
    pub fn new(file_path: PathBuf, mapped_file: Arc<Mmap>, pe_binary: Arc<PeBinary>) -> Self {
        let mapped: Arc<[u8]> = Arc::from(&mapped_file[..]);
        Self {
            file_path,
            mapped_file: mapped,
            section_cache: RwLock::new(HashMap::new()),
            cache_policy: CachePolicy::LRU {
                max_bytes: 16 * 1024 * 1024, // 16 MB
            },
            access_order: RwLock::new(Vec::new()),
            pe_binary: Some(pe_binary),
        }
    }

    pub fn from_bytes(file_path: PathBuf, raw_bytes: Arc<[u8]>, pe_binary: Arc<PeBinary>) -> Self {
        Self {
            file_path,
            mapped_file: raw_bytes,
            section_cache: RwLock::new(HashMap::new()),
            cache_policy: CachePolicy::LRU {
                max_bytes: 16 * 1024 * 1024, // 16 MB
            },
            access_order: RwLock::new(Vec::new()),
            pe_binary: Some(pe_binary),
        }
    }

    /// Create a new section manager with custom cache policy
    pub fn with_policy(
        file_path: PathBuf,
        mapped_file: Arc<Mmap>,
        policy: CachePolicy,
        pe_binary: Arc<PeBinary>,
    ) -> Self {
        let mapped: Arc<[u8]> = Arc::from(&mapped_file[..]);
        Self {
            file_path,
            mapped_file: mapped,
            section_cache: RwLock::new(HashMap::new()),
            cache_policy: policy,
            access_order: RwLock::new(Vec::new()),
            pe_binary: Some(pe_binary),
        }
    }

    /// Translate a section name to its first matching index.
    fn name_to_index(&self, name: &str) -> Option<usize> {
        let pe = self.pe_binary.as_ref()?;
        pe.sections().position(|s| s.name() == name)
    }

    /// Get section data by name (lazy loading).
    ///
    /// Translates name to index internally and uses index-based caching.
    /// If duplicate section names exist, returns the first match.
    pub fn get_section_data(&self, name: &str) -> Option<Vec<u8>> {
        let index = self.name_to_index(name)?;
        self.get_section_data_by_index(index)
    }

    /// Get section data by index (lazy loading).
    ///
    /// Uses canonical index-based caching.
    pub fn get_section_data_by_index(&self, index: usize) -> Option<Vec<u8>> {
        {
            let cache = self.section_cache.read().unwrap();
            if let Some(data) = cache.get(&index) {
                self.update_access_order(index);
                return Some(data.clone());
            }
        }

        let pe = self.pe_binary.as_ref()?;
        let section = pe.sections().nth(index)?;

        let file_offset = section.pointerto_raw_data() as usize;
        let raw_size = section.sizeof_raw_data() as usize;

        let data = if raw_size == 0 || file_offset >= self.mapped_file.as_ref().len() {
            Vec::new()
        } else {
            let mapped = self.mapped_file.as_ref();
            let read_len = raw_size.min(mapped.len().saturating_sub(file_offset));
            mapped[file_offset..file_offset + read_len].to_vec()
        };

        self.cache_section(index, data.clone());
        Some(data)
    }

    /// Get section data by index with exact semantics.
    ///
    /// Returns:
    /// - `Ok(None)` if the section index is out of bounds
    /// - `Ok(Some(Vec::new()))` if the section has zero raw data (valid empty)
    /// - `Ok(Some(data))` with the section data on success
    /// - `Err(LiefError::InvalidOffset(...))` if section has nonzero raw_size
    ///   but file_offset >= file length
    /// - `Err(LiefError::SectionNotFound(...))` if PE binary is not available
    ///
    /// This method always validates section bounds against the file regardless
    /// of whether the section data is already cached.
    pub fn get_section_data_exact_by_index(
        &self,
        index: usize,
    ) -> Result<Option<Vec<u8>>, LiefError> {
        let pe = match self.pe_binary.as_ref() {
            Some(pe) => pe,
            None => {
                return Err(LiefError::SectionNotFound(
                    "PE binary not available".to_string(),
                ));
            }
        };
        let section = match pe.sections().nth(index) {
            Some(s) => s,
            None => return Ok(None),
        };

        let file_offset = section.pointerto_raw_data() as usize;
        let raw_size = section.sizeof_raw_data() as usize;

        if raw_size == 0 {
            self.cache_section(index, Vec::new());
            return Ok(Some(Vec::new()));
        }

        let mapped = self.mapped_file.as_ref();
        if file_offset >= mapped.len() {
            return Err(LiefError::InvalidOffset(format!(
                "section {} raw_size={} but file_offset=0x{:x} >= file_len={}",
                index,
                raw_size,
                file_offset,
                mapped.len()
            )));
        }

        let available = mapped.len().saturating_sub(file_offset);
        if raw_size > available {
            return Err(LiefError::InvalidOffset(format!(
                "section {} raw_size={} exceeds file: file_offset=0x{:x} available={} file_len={}",
                index,
                raw_size,
                file_offset,
                available,
                mapped.len()
            )));
        }

        let data = mapped[file_offset..file_offset + raw_size].to_vec();

        self.cache_section(index, data.clone());

        Ok(Some(data))
    }

    /// Get section layout information
    ///
    /// Returns (file_offset, virtual_address, size) if found.
    pub fn get_section_layout(&self, name: &str) -> Option<(u64, u64, u64)> {
        let pe = self.pe_binary.as_ref()?;
        let section = pe.sections().find(|s| s.name() == name)?;

        Some((
            section.pointerto_raw_data() as u64,
            section.virtual_address() as u64,
            section.sizeof_raw_data() as u64,
        ))
    }

    /// Cache a section's data by index
    pub fn cache_section(&self, index: usize, data: Vec<u8>) {
        let new_size = data.len();

        {
            let mut cache = self.section_cache.write().unwrap();
            self.evict_if_needed(&mut cache, &new_size);
            cache.insert(index, data);
        }

        self.update_access_order(index);
    }

    /// Evict entries if needed based on cache policy
    pub fn evict_if_needed(&self, cache: &mut HashMap<usize, Vec<u8>>, new_size: &usize) {
        match &self.cache_policy {
            CachePolicy::LRU { max_bytes } => {
                let mut current_size: usize = cache.values().map(|v| v.len()).sum();

                if current_size + *new_size > *max_bytes {
                    let mut access_order = self.access_order.write().unwrap();

                    while current_size + *new_size > *max_bytes && !access_order.is_empty() {
                        if let Some(oldest_idx) = access_order.pop() {
                            if let Some(evicted) = cache.remove(&oldest_idx) {
                                current_size = current_size.saturating_sub(evicted.len());
                            }
                        }
                    }
                }
            }
            CachePolicy::Unlimited => {}
        }
    }

    /// Update access order for LRU tracking (by index)
    pub fn update_access_order(&self, index: usize) {
        let mut access_order = self.access_order.write().unwrap();

        access_order.retain(|&n| n != index);
        access_order.insert(0, index);
    }

    /// Clear all cached sections
    pub fn clear_cache(&self) {
        let mut cache = self.section_cache.write().unwrap();
        cache.clear();

        let mut access_order = self.access_order.write().unwrap();
        access_order.clear();
    }

    /// Get list of cached section names (derived from cached indices)
    pub fn cached_sections(&self) -> Vec<String> {
        let cache = self.section_cache.read().unwrap();
        if let Some(pe) = self.pe_binary.as_ref() {
            cache
                .keys()
                .filter_map(|&idx| pe.sections().nth(idx).map(|s| s.name().to_string()))
                .collect()
        } else {
            cache.keys().map(|idx| format!("section_{}", idx)).collect()
        }
    }

    /// Get total bytes cached
    pub fn cached_bytes(&self) -> usize {
        let cache = self.section_cache.read().unwrap();
        cache.values().map(|v| v.len()).sum()
    }

    /// Check if a section is currently loaded in cache (by name)
    pub fn is_section_cached(&self, name: &str) -> bool {
        match self.name_to_index(name) {
            Some(idx) => self.is_section_cached_by_index(idx),
            None => false,
        }
    }

    /// Check if a section is loaded in cache by index.
    pub fn is_section_cached_by_index(&self, index: usize) -> bool {
        let cache = self.section_cache.read().unwrap();
        cache.contains_key(&index)
    }

    /// Release the memory-mapped file reference
    ///
    /// This clears the section cache and releases the mapped_file and pe_binary
    /// references, allowing the memory to be freed after sections have been
    /// copied to emulated memory.
    pub fn release_mmap(&mut self) {
        {
            let mut cache = self.section_cache.write().unwrap();
            cache.clear();
        }
        {
            let mut access_order = self.access_order.write().unwrap();
            access_order.clear();
        }

        let empty: Arc<[u8]> = Arc::new([]);
        mem::replace(&mut self.mapped_file, empty);

        self.pe_binary = None;
    }
}
