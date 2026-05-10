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
/// This manager handles on-demand loading of PE sections with:
/// - Zero-copy access via memory mapping
/// - Configurable cache policies (LRU or Unlimited)
/// - Thread-safe caching
pub struct LiefSectionManager {
    /// Path to the PE file
    file_path: PathBuf,
    /// Memory-mapped file for section data access (or bytes for from_bytes)
    mapped_file: Arc<[u8]>,
    /// Section data cache
    section_cache: RwLock<HashMap<String, Vec<u8>>>,
    /// Cache eviction policy
    cache_policy: CachePolicy,
    /// LRU access order tracking
    access_order: RwLock<Vec<String>>,
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
                max_bytes: 100 * 1024 * 1024, // 100MB default
            },
            access_order: RwLock::new(Vec::new()),
            pe_binary: Some(pe_binary),
        }
    }

    /// Create a new section manager with custom cache policy
    pub fn with_policy(file_path: PathBuf, mapped_file: Arc<Mmap>, policy: CachePolicy, pe_binary: Arc<PeBinary>) -> Self {
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

    /// Get section data by name (lazy loading)
    ///
    /// Returns the section data if found, None otherwise.
    /// Returns owned data (Vec<u8>) since the cache is internal.
    pub fn get_section_data(&self, name: &str) -> Option<Vec<u8>> {
        // Check if already cached
        {
            let cache = self.section_cache.read().unwrap();
            if let Some(data) = cache.get(name) {
                self.update_access_order(name);
                return Some(data.clone());
            }
        }

        // Load the section
        self.load_section(name)?;

        // Return from cache
        let cache = self.section_cache.read().unwrap();
        self.update_access_order(name);
        cache.get(name).map(|v| v.clone())
    }

    /// Get section data by index (lazy loading)
    ///
    /// Uses the pe_binary to get the section name by index, then loads the section.
    pub fn get_section_data_by_index(&self, index: usize) -> Option<Vec<u8>> {
        // Use cached pe_binary to get section name
        let pe = self.pe_binary.as_ref()?;

        // Find the section by index
        let section = pe.sections().nth(index)?;

        let name = section.name();
        self.get_section_data(&name)
    }

    /// Load a section from the memory-mapped file
    ///
    /// This is the internal method that reads section data from the mmap.
    /// Uses the cached `PeBinary` object to avoid re-parsing the PE file.
    fn load_section(&self, name: &str) -> Option<Vec<u8>> {
        // Use cached pe_binary instead of re-parsing
        let pe = self.pe_binary.as_ref()?;

        // Find the section
        let section = pe.sections().find(|s| s.name() == name)?;

        let file_offset = section.pointerto_raw_data() as usize;
        let virtual_size = section.virtual_size() as usize;
        let raw_size = section.sizeof_raw_data() as usize;
        let size = if virtual_size > 0 { virtual_size } else { raw_size };

        if file_offset >= self.mapped_file.as_ref().len() || file_offset + size > self.mapped_file.as_ref().len() {
            return None;
        }

        let data = self.mapped_file.as_ref()[file_offset..file_offset + size].to_vec();

        // Cache the section
        self.cache_section(name, data.clone());

        // Return the loaded data
        Some(data)
    }

    /// Get section layout information
    ///
    /// Returns (file_offset, virtual_address, size) if found.
    pub fn get_section_layout(&self, name: &str) -> Option<(u64, u64, u64)> {
        // Use cached pe_binary instead of re-parsing
        let pe = self.pe_binary.as_ref()?;

        let section = pe.sections().find(|s| s.name() == name)?;

        Some((
            section.pointerto_raw_data() as u64,
            section.virtual_address() as u64,
            section.sizeof_raw_data() as u64,
        ))
    }

    /// Cache a section's data
    pub fn cache_section(&self, name: &str, data: Vec<u8>) {
        let new_size = data.len();

        // Apply cache policy
        {
            let mut cache = self.section_cache.write().unwrap();
            self.evict_if_needed(&mut cache, &new_size);
            cache.insert(name.to_string(), data);
        }

        self.update_access_order(name);
    }

    /// Evict entries if needed based on cache policy
    pub fn evict_if_needed(&self, cache: &mut HashMap<String, Vec<u8>>, new_size: &usize) {
        match &self.cache_policy {
            CachePolicy::LRU { max_bytes } => {
                let current_size: usize = cache.values().map(|v| v.len()).sum();
                
                if current_size + new_size > *max_bytes {
                    // Need to evict
                    let mut access_order = self.access_order.write().unwrap();
                    
                    while current_size + new_size > *max_bytes && !access_order.is_empty() {
                        if let Some(oldest) = access_order.pop() {
                            if let Some(evicted) = cache.remove(&oldest) {
                                let _ = evicted; // Drop the data
                            }
                        }
                    }
                }
            }
            CachePolicy::Unlimited => {
                // No eviction needed
            }
        }
    }

    /// Update access order for LRU tracking
    pub fn update_access_order(&self, name: &str) {
        let mut access_order = self.access_order.write().unwrap();
        
        // Remove from current position
        access_order.retain(|n| n != name);
        
        // Add to front (most recently used)
        access_order.insert(0, name.to_string());
    }

    /// Clear all cached sections
    pub fn clear_cache(&self) {
        let mut cache = self.section_cache.write().unwrap();
        cache.clear();
        
        let mut access_order = self.access_order.write().unwrap();
        access_order.clear();
    }

    /// Get list of cached section names
    pub fn cached_sections(&self) -> Vec<String> {
        let cache = self.section_cache.read().unwrap();
        cache.keys().cloned().collect()
    }

    /// Get total bytes cached
    pub fn cached_bytes(&self) -> usize {
        let cache = self.section_cache.read().unwrap();
        cache.values().map(|v| v.len()).sum()
    }

    /// Check if a section is currently loaded in cache
    pub fn is_section_cached(&self, name: &str) -> bool {
        let cache = self.section_cache.read().unwrap();
        cache.contains_key(name)
    }

    /// Release the memory-mapped file reference
    ///
    /// This clears the section cache and releases the mapped_file and pe_binary
    /// references, allowing the memory to be freed after sections have been
    /// copied to emulated memory.
    pub fn release_mmap(&mut self) {
        // Clear the section cache
        {
            let mut cache = self.section_cache.write().unwrap();
            cache.clear();
        }
        {
            let mut access_order = self.access_order.write().unwrap();
            access_order.clear();
        }

        // Release the mapped file reference by replacing with empty Arc
        let empty: Arc<[u8]> = Arc::new([]);
        mem::replace(&mut self.mapped_file, empty);

        // Release pe_binary
        self.pe_binary = None;
    }
}
