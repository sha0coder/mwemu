# PE Parser Refactoring Plan: Bug Fixes and Optimization

## Overview

This document outlines a comprehensive refactoring plan to optimize the PE (Portable Executable) loading process in mwemu. The plan is divided into 6 phases, with Phase 0 focusing on critical bug fixes that must be completed before any other work.

---

## Phase 0: Critical Bug Fixes (PREREQUISITE - Must Complete First)

### 0.1 PE32 Bug Fixes

**File**: `crates/libmwemu/src/pe/pe32.rs`

| Line(s) | Bug | Fix |
|---------|-----|-----|
| 416-417 | Wrong export directory offsets | Change `off + 22` to `off + 28` for `address_of_names`, `off + 26` to `off + 36` for `address_of_name_ordinals` |
| 810 | Unnecessary `to_vec()` copy | Change `ImageOptionalHeader::load(&raw.to_vec(), ...)` to `ImageOptionalHeader::load(&raw, ...)` |
| 939-942 | Incomplete `clear()` method | Add clearing of `raw`, `sect_hdr`, `delay_load_dir`, `image_import_descriptor` |
| 1292, 1298, 1309, 1315 | Debug `println!` statements | Remove all debug print statements |
| 984 | Off-by-one bounds check | Change `sz = self.raw.len() - off - 1;` to `sz = self.raw.len() - off;` |

### 0.2 PE64 Bug Fixes

**File**: `crates/libmwemu/src/pe/pe64.rs`

| Line(s) | Bug | Fix |
|---------|-----|-----|
| 260-267 | Duplicate parsing in `load_from_raw()` | Remove first set of `dos`, `nt`, `fh`, `opt` parsing (keep only second set) |
| 172 | Wrong `size_of_heap_commit` offset | Change `off + 94` to `off + 96` |
| 512 | TLS callback pointer masking | Change `(tls.tls_callbacks & 0xffff)` to proper 64-bit handling |
| 277-278 | Unused variables | Remove `importd` and `exportd` variables |

---

## Phase 1: Foundation Layer

### 1.1 Create Common PE Reader Trait

**New file**: `crates/libmwemu/src/pe/reader.rs`

```rust
/// Common interface for PE32 and PE64 parsers
pub trait PEReader {
    fn is_pe(&self) -> bool;
    fn is_dll(&self) -> bool;
    fn num_of_sections(&self) -> u16;
    fn get_section(&self, index: usize) -> Option<&SectionHeader>;
    fn vaddr_to_off(&self, vaddr: u64) -> Option<usize>;
    fn mem_size(&self) -> u64;
}
```

### 1.2 Create Section Data Manager

**New file**: `crates/libmwemu/src/pe/section_data.rs`

```rust
/// Manages lazy loading and caching of section data
pub struct SectionDataManager {
    file_path: String,
    cached_sections: HashMap<String, Vec<u8>>,
    cache_policy: CachePolicy,
}

pub enum CachePolicy {
    LRU(usize),  // Least Recently Used with max size
    Unlimited,
}

impl SectionDataManager {
    pub fn new(file_path: String) -> Self;
    pub fn load_section(&mut self, name: &str) -> Result<&[u8], SectionError>;
    pub fn preload_section(&mut self, name: &str) -> Result<(), SectionError>;
    pub fn is_loaded(&self, name: &str) -> bool;
    pub fn clear_cache(&mut self);
}
```

### 1.3 Create PE Type Enum

**New file**: `crates/libmwemu/src/pe/types.rs`

```rust
/// Represents PE32 vs PE64 variants
pub enum PEType {
    PE32(PE32),
    PE64(PE64),
}

impl PEType {
    pub fn load(filename: &str) -> Result<Self, PEError>;
    pub fn as_reader(&self) -> &dyn PEReader;
}
```

### 1.4 Create Header Parser

**New file**: `crates/libmwemu/src/pe/header_parser.rs`

```rust
/// Custom header-only parser (~4KB memory footprint)
pub struct HeaderParser {
    header_data: Vec<u8>,  // Only first ~4KB of PE file
    dos_header: ImageDosHeader,
    nt_headers: NtHeaders,
    sections: Vec<ImageSectionHeader>,
}

impl HeaderParser {
    pub fn from_file(path: &str) -> Result<Self, HeaderError>;
    pub fn from_bytes(data: &[u8]) -> Result<Self, HeaderError>;
    pub fn parse_headers(&mut self) -> Result<(), HeaderError>;
    
    // Header-only accessors
    pub fn get_dos_header(&self) -> &ImageDosHeader;
    pub fn get_nt_headers(&self) -> &NtHeaders;
    pub fn get_sections(&self) -> &[ImageSectionHeader];
    pub fn get_data_directories(&self) -> &[ImageDataDirectory];
}
```

---

## Phase 2: Header-Only Operations

### 2.1 Lazy Header Parsing

Implement header parsing that reads only what's needed:

```
DOS Header (64 bytes)
    ↓
NT Headers (248 bytes)
    ↓
COFF Header (20 bytes)
    ↓
Optional Header (224/240 bytes)
    ↓
Section Headers (40 bytes × number of sections)
```

Total: ~4KB for typical PE files

### 2.2 Header-Only Method Implementation

Update methods that only need header data to work without full file loading:

| Method | Current | New Implementation |
|--------|---------|-------------------|
| `is_pe64()` | Reads from `raw` | Reads from `HeaderParser` |
| `is_pe32()` | Reads from `raw` | Reads from `HeaderParser` |
| `size()` | Returns `raw.len()` | Use file metadata |
| `mem_size()` | Iterates sections | Use `HeaderParser::get_sections()` |
| `is_dll()` | Checks characteristics | Use `HeaderParser::get_nt_headers()` |
| `num_of_sections()` | Returns `sect_hdr.len()` | Use `HeaderParser::get_sections()` |
| `get_section()` | Returns from `sect_hdr` | Use `HeaderParser::get_sections()` |
| `get_section_vaddr()` | Returns from `sect_hdr` | Use `HeaderParser::get_sections()` |
| `vaddr_to_off()` | Iterates sections | Use `HeaderParser::get_sections()` |
| `get_pe_off()` | Reads from `dos` | Use `HeaderParser::get_dos_header()` |

---

## Phase 3: Lazy Section Loading

### 3.1 On-Demand Section Loading

Modify section access methods to load data only when needed:

```rust
pub fn get_section_ptr(&self, id: usize) -> &[u8] {
    let section = &self.sect_hdr[id];
    self.section_data_manager.load_section(&section.get_name())
        .unwrap_or(&[])
}

pub fn get_section_ptr_by_name(&self, name: &str) -> Option<&[u8]> {
    self.section_data_manager.load_section(name).ok()
}
```

### 3.2 Cache Eviction Policy

Implement LRU cache for section data:

```rust
impl SectionDataManager {
    fn evict_if_needed(&mut self) {
        if let CachePolicy::LRU(max_size) = self.cache_policy {
            while self.total_cached_size() > max_size {
                self.evict_lru();
            }
        }
    }
}
```

### 3.3 Memory Management

- Track total cached section data size
- Implement cache statistics (hits, misses, evictions)
- Provide manual cache control methods

---

## Phase 4: API Compatibility Layer

### 4.1 Feature Flags

**File**: `crates/libmwemu/Cargo.toml`

```toml
[features]
default = []
pe-lazy = []  # Enable lazy loading optimizations
```

### 4.2 Maintain Existing API

Keep all public method signatures unchanged:

```rust
impl PE64 {
    // Existing methods maintain same signatures
    pub fn load(filename: &str) -> Self;
    pub fn load_from_raw(filename: &str, raw: &[u8]) -> Self;
    pub fn get_section_ptr(&self, id: usize) -> &[u8];
    // ... all other existing methods
}
```

### 4.3 Encapsulate Struct Fields

Make fields private and provide getters:

```rust
pub struct PE64 {
    filename: String,
    raw: Vec<u8>,  // Now private
    dos: ImageDosHeader,  // Now private
    nt: ImageNtHeaders,  // Now private
    // ... other fields now private
}

impl PE64 {
    // Provide getters instead of direct field access
    pub fn filename(&self) -> &str;
    pub fn dos_header(&self) -> &ImageDosHeader;
    pub fn nt_headers(&self) -> &ImageNtHeaders;
    // ... other getters
}
```

---

## Phase 5: Testing & Validation

### 5.1 Unit Tests

**New file**: `crates/libmwemu/src/pe/tests/header_parser_tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dos_header_parsing() {
        let parser = HeaderParser::from_file("test_data/pe32.exe").unwrap();
        assert_eq!(parser.get_dos_header().e_magic, 0x5A4D);
    }

    #[test]
    fn test_nt_headers_parsing() {
        let parser = HeaderParser::from_file("test_data/pe64.dll").unwrap();
        assert!(parser.get_nt_headers().signature == 0x4550);
    }

    #[test]
    fn test_section_headers_parsing() {
        let parser = HeaderParser::from_file("test_data/pe32.exe").unwrap();
        let sections = parser.get_sections();
        assert!(!sections.is_empty());
    }
}
```

### 5.2 Integration Tests

**New file**: `crates/libmwemu/src/pe/tests/integration_tests.rs`

```rust
#[test]
fn test_pe32_loading() {
    let pe = PE32::load("test_data/pe32.exe");
    assert!(pe.is_pe());
    assert!(!pe.is_dll());
}

#[test]
fn test_pe64_loading() {
    let pe = PE64::load("test_data/pe64.dll");
    assert!(pe.is_pe());
    assert!(pe.is_dll());
}

#[test]
fn test_lazy_section_loading() {
    let pe = PE64::load("test_data/pe64.dll");
    // Access section data - should load on demand
    let text_section = pe.get_section_ptr(0);
    assert!(!text_section.is_empty());
}
```

### 5.3 Memory Profiling

Create benchmarks to measure memory usage:

```rust
#[bench]
fn bench_header_only_parsing(b: &mut Bencher) {
    b.iter(|| {
        let parser = HeaderParser::from_file("test_data/large.dll").unwrap();
        parser.parse_headers().unwrap();
    });
}

#[bench]
fn bench_lazy_section_loading(b: &mut Bencher) {
    let pe = PE64::load("test_data/large.dll");
    b.iter(|| {
        let _ = pe.get_section_ptr(0);
    });
}
```

---

## Key Files to Modify

### Primary Files

| File | Phase | Changes |
|------|-------|---------|
| `crates/libmwemu/src/pe/PLAN.md` | 0 | Complete rewrite with new phases |
| `crates/libmwemu/src/pe/pe32.rs` | 0 | Fix critical bugs |
| `crates/libmwemu/src/pe/pe64.rs` | 0 | Fix critical bugs |
| `crates/libmwemu/src/pe/reader.rs` | 1 | NEW - Common PE reader trait |
| `crates/libmwemu/src/pe/section_data.rs` | 1 | NEW - Section data manager |
| `crates/libmwemu/src/pe/types.rs` | 1 | NEW - PE type enum |
| `crates/libmwemu/src/pe/header_parser.rs` | 1 | NEW - Header-only parser |

### Supporting Files

| File | Phase | Changes |
|------|-------|---------|
| `crates/libmwemu/src/pe/mod.rs` | 1 | Add module exports |
| `crates/libmwemu/src/pe/pe32.rs` | 2-4 | Implement lazy loading |
| `crates/libmwemu/src/pe/pe64.rs` | 2-4 | Implement lazy loading |
| `crates/libmwemu/src/pe/tests/` | 5 | NEW - Test directory |
| `crates/libmwemu/Cargo.toml` | 4 | Add feature flags |

---

## Expected Benefits

| Metric | Current (After Phase 0) | After Phase 5 |
|--------|-------------------------|---------------|
| Initial memory (DLL only) | ~1MB+ | ~4KB |
| Parse time (large DLL) | ~50ms | ~5ms |
| Code maintainability | Manual parsing | Structured with traits |
| Bug count | 0 (after fixes) | 0 (verified) |
| Test coverage | Minimal | Comprehensive |

---

## Migration Strategy

1. **Phase 0**: Complete all bug fixes (blocking - must finish first)
2. **Phase 1**: Build foundation layer without breaking changes
3. **Phase 2**: Implement header-only parsing behind feature flag
4. **Phase 3**: Add lazy loading behind feature flag
5. **Phase 4**: Enable optimizations by default with feature flag to disable
6. **Phase 5**: Comprehensive testing and validation

---

## Summary

This refactoring plan transforms the PE parser through a systematic approach:
- **Phase 0** fixes critical bugs that are blocking all other work
- **Phase 1** builds a solid foundation with traits and abstractions
- **Phase 2** enables header-only operations for minimal memory footprint
- **Phase 3** implements lazy loading for on-demand section data
- **Phase 4** maintains API compatibility while enabling optimizations
- **Phase 5** ensures correctness through comprehensive testing

The plan prioritizes bug fixes first, then builds incrementally while maintaining backward compatibility.
