# PE Parsing Developer Guide - LIEF Migration

This document provides a comprehensive implementation guide for migrating the PE parsing code from custom PE32/PE64 parsers to the LIEF-based [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs) wrapper.

**For detailed architecture analysis, see [LIEF_ARCHITECTURE.md](LIEF_ARCHITECTURE.md).**

---

## Table of Contents

1. [Migration Overview](#migration-overview)
2. [Critical Issues](#critical-issues)
3. [Implementation Phases](#implementation-phases)
4. [Phase 1: Complete LIEF Integration](#phase-1-complete-lief-integration)
5. [Phase 2: Remove Dead Code](#phase-2-remove-dead-code)
6. [Phase 3: Update Loaders](#phase-3-update-loaders)
7. [Phase 4: Testing & Validation](#phase-4-testing--validation)
8. [Backward Compatibility](#backward-compatibility)
9. [Usage Examples](#usage-examples)
10. [Testing Strategy](#testing-strategy)

---

## Migration Overview

### Goal

Migrate from custom PE32/PE64 parsers (with `raw: Vec<u8>` full file copy) to LIEF-based parsing with:

- **Header-only parsing**: ~4KB initial footprint
- **Lazy section loading**: Sections loaded on-demand via [`LiefSectionManager`](crates/libmwemu/src/pe/lief/lief_section_manager.rs)
- **Memory-mapped files**: Zero-copy file access via `Arc<Mmap>`
- **LRU caching**: Configurable memory limits with automatic eviction

### Current vs Target Architecture

```
Current Architecture (Problematic):
┌────────────────────────────────────────┐
│ PE64 {                                 │
│   raw: Vec<u8> = FULL FILE (~1MB+)    │  ← Full file in memory
│   opt: ImageOptionalHeader64           │
│   sect_hdr: Vec<ImageSectionHeader>    │
│ }                                      │
└────────────────────────────────────────┘

Target Architecture (Optimized):
┌────────────────────────────────────────┐
│ LiefPe {                               │
│   header: LiefHeaderParser (~4KB)     │  ← Header-only parsing
│   section_manager: LiefSectionManager  │  ← Lazy loading + LRU
│     └── mapped_file: Arc<Mmap>         │  ← Shared, memory-mapped
│     └── section_cache: HashMap         │  ← LRU evicted
│ }                                      │
└────────────────────────────────────────┘
```

### Memory Improvement

| Scenario | Current | Target | Reduction |
|----------|---------|--------|-----------|
| Headers only | ~1MB+ (full file) | ~4KB | 99.6%+ |
| 1 section cached | ~1MB+ | ~4KB + section_size | Significant |
| All sections | ~1MB+ | ~4KB + total_sections | Significant |

---

## Critical Issues

### 🔴 Missing `mmap` Dependency

The [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs:9) implementation imports `mmap` but it is **NOT declared** in [`Cargo.toml`](crates/libmwemu/Cargo.toml).

**File**: `crates/libmwemu/Cargo.toml`

```toml
[dependencies]
# Add this dependency - REQUIRED
mmap = "0.1"
```

Without this dependency, the project will not compile.

---

## Implementation Phases

### Phase 1: Complete LIEF Integration ⚠️ CRITICAL

**Objective**: Complete the [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs) wrapper to match PE32/PE64 functionality.

#### 1.1 Add Missing Dependency

**File**: [`crates/libmwemu/Cargo.toml`](crates/libmwemu/Cargo.toml:41)

```toml
[dependencies]
# Add mmap for memory-mapped file access
mmap = "0.1"
```

#### 1.2 Verify [`LiefPe::get_section_data()`](crates/libmwemu/src/pe/lief/lief_pe.rs:325) Implementation

The method is implemented but needs verification:

```rust
fn get_section_data(&self, index: usize) -> Option<&[u8]> {
    // First get the section to find its name
    let section = self.header.get_section(index)?;
    self.section_manager.get_section_data(section.name())
}
```

This delegates to [`LiefSectionManager::get_section_data()`](crates/libmwemu/src/pe/lief/lief_section_manager.rs) for lazy loading.

#### 1.3 Verify [`import_addr_to_name()`](crates/libmwemu/src/pe/lief/traits.rs:142) Implementation

The default implementation is provided in the trait:

```rust
fn import_addr_to_name(&self, paddr: u64) -> String {
    if paddr == 0 {
        return String::new();
    }

    let pe = self.lief_pe();
    for import in pe.imports() {
        for function in import.entries() {
            if function.iat_address() as u64 == paddr {
                return function.name().unwrap_or("<ordinal>").to_string();
            }
        }
    }
    String::new()
}
```

#### 1.4 Implement IAT Binding Methods (HIGH PRIORITY)

The current loaders use binding methods that modify both the `raw` buffer AND emulated memory:

| Method | Purpose | Location |
|--------|---------|----------|
| [`iat_binding(emu, base)`](crates/libmwemu/src/pe/pe64.rs:607) | Writes resolved addresses to IAT | PE64 |
| [`delay_load_binding(emu, base)`](crates/libmwemu/src/pe/pe64.rs:520) | Handles delay-load DLLs | PE64 |
| [`apply_relocations(emu, base)`](crates/libmwemu/src/pe/pe64.rs:765) | Applies ASLR relocations | PE64 |

**Required for LiefPe**:

```rust
// In lief_pe.rs - needs implementation
impl LiefPe {
    /// Apply relocations to the loaded PE
    pub fn apply_relocations(&mut self, emu: &mut Emu, base: u64) -> Result<(), LiefError> {
        // Use LIEF's relocation parsing
        // Write patches to emu memory at correct RVA offsets
    }

    /// Perform IAT binding
    pub fn iat_binding(&mut self, emu: &mut Emu, base: u64) -> Result<(), LiefError> {
        // Use LIEF's import parsing
        // For each import DLL/function, write resolved address to IAT
    }

    /// Perform delay-load binding
    pub fn delay_load_binding(&mut self, emu: &mut Emu, base: u64) -> Result<(), LiefError> {
        // Use LIEF's delay-load directory parsing
    }
}
```

#### 1.5 Add Accessor Methods for Internal Fields

Currently, [`loaders.rs`](crates/libmwemu/src/emu/loaders.rs) accesses internal fields directly:

```rust
// Current pattern in loaders.rs
base = pe64.opt.image_base;                          // Line 208, 325, 330
entry = pe64.opt.address_of_entry_point as u64;       // Line 350
headers = pe64.opt.size_of_headers;                    // Line 368
section_alignment = pe64.opt.section_alignment;       // Line 213, 363
e_lfanew = pe64.dos.e_lfanew;                         // Line 457
```

**Required getter methods** (already present in [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs)):

| Existing Method | Purpose |
|-----------------|---------|
| [`image_base()`](crates/libmwemu/src/pe/lief/lief_pe.rs:136) | Returns image base |
| [`entry_point()`](crates/libmwemu/src/pe/lief/lief_pe.rs:141) | Returns entry point RVA |
| [`size_of_headers()`](crates/libmwemu/src/pe/lief/lief_pe.rs:146) | Returns size of headers |
| [`e_lfanew()`](crates/libmwemu/src/pe/lief/lief_pe.rs:229) | Returns PE header offset |

**Missing getters to add**:

```rust
// In lief_pe.rs
impl LiefPe {
    /// Get section alignment
    pub fn section_alignment(&self) -> u32 {
        self.header.section_alignment()
    }

    /// Get file alignment
    pub fn file_alignment(&self) -> u32 {
        self.header.file_alignment()
    }

    /// Get characteristics
    pub fn characteristics(&self) -> u16 {
        self.header.characteristics()
    }
}
```

---

### Phase 2: Remove Dead Code 🧹

**Objective**: Remove unused functions from PE32/PE64 identified in [LIEF_ARCHITECTURE.md](LIEF_ARCHITECTURE.md#dead-code-that-can-be-removed).

#### PE64 Functions to Remove (15 functions)

**File**: [`crates/libmwemu/src/pe/pe64.rs`](crates/libmwemu/src/pe/pe64.rs)

| Function | Lines | Reason |
|----------|-------|--------|
| [`mem_size()`](crates/libmwemu/src/pe/pe64.rs:359) | 359-370 | Never called |
| [`get_raw()`](crates/libmwemu/src/pe/pe64.rs:376) | 376-378 | Never called |
| [`clear()`](crates/libmwemu/src/pe/pe64.rs:384) | 384-387 | Never called |
| [`vaddr_to_off()`](crates/libmwemu/src/pe/pe64.rs:389) | 389-412 | Static, only internal use |
| [`read_string()`](crates/libmwemu/src/pe/pe64.rs:415) | 415-430 | Static, only internal use |
| [`get_section_ptr_by_name()`](crates/libmwemu/src/pe/pe64.rs:436) | 436-446 | Never called |
| [`get_section_vaddr()`](crates/libmwemu/src/pe/pe64.rs:481) | 481-483 | Never called |
| [`get_tls_callbacks()`](crates/libmwemu/src/pe/pe64.rs:485) | 485-518 | Never called |
| [`iat_binding_alternative()`](crates/libmwemu/src/pe/pe64.rs:656) | 656-696 | Never called |
| [`iat_binding_original()`](crates/libmwemu/src/pe/pe64.rs:698) | 698-763 | Never called |
| [`import_addr_to_name()`](crates/libmwemu/src/pe/pe64.rs:846) | 846-893 | Replaced by LiefPe |
| [`locate_resource_data_entry()`](crates/libmwemu/src/pe/pe64.rs:895) | 895-1060 | Never called externally |
| [`read_resource_name_from_rsrc()`](crates/libmwemu/src/pe/pe64.rs:1062) | 1062-1093 | Never called externally |
| [`get_resource()`](crates/libmwemu/src/pe/pe64.rs:1095) | 1095-1122 | Replaced by LiefPe |
| [`get_resource_name()`](crates/libmwemu/src/pe/pe64.rs:1124) | 1124-1134 | Never called |

#### PE32 Functions to Remove (11+ functions)

**File**: [`crates/libmwemu/src/pe/pe32.rs`](crates/libmwemu/src/pe/pe32.rs)

| Function | Lines | Reason |
|----------|-------|--------|
| [`get_section_vaddr()`](crates/libmwemu/src/pe/pe32.rs:995) | 995-997 | Never called |
| [`get_tls_callbacks()`](crates/libmwemu/src/pe/pe32.rs:999) | 999-1039 | Never called |
| [`read_string_200()`](crates/libmwemu/src/pe/pe32.rs:781) | 781-804 | Never called |
| [`get_filename()`](crates/libmwemu/src/pe/pe32.rs:908) | 908-910 | Never called |
| [`size()`](crates/libmwemu/src/pe/pe32.rs:904) | 904-906 | Redundant |
| [`mem_size()`](crates/libmwemu/src/pe/pe32.rs:912) | 912-929 | Never called |
| [`get_raw()`](crates/libmwemu/src/pe/pe32.rs:931) | 931-933 | Never called externally |
| [`get_headers()`](crates/libmwemu/src/pe/pe32.rs:935) | 935-937 | Redundant |
| [`clear()`](crates/libmwemu/src/pe/pe32.rs:939) | 939-944 | Never called |
| [`get_resource_name()`](crates/libmwemu/src/pe/pe32.rs:1374) | 1374-1386 | Never called |
| [`ImageBaseRelocation::load()`](crates/libmwemu/src/pe/pe32.rs:685) | 685-695 | Never called |

---

### Phase 3: Update Loaders

**Objective**: Replace PE32/PE64 with [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs) in [`loaders.rs`](crates/libmwemu/src/emu/loaders.rs).

#### Current Loader Pattern (PE64)

```rust
// loaders.rs:204-290 - map_dll_pe64()
let pe64 = PE64::load(&filename.to_lowercase());

let mut base = pe64.opt.image_base;                    // → pe64.image_base()
if base < constants::LIBS64_MIN {
    base = self.maps.lib64_alloc(pe64.size()).expect("out of memory");
}

let sec_allign = pe64.opt.section_alignment;            // → pe64.section_alignment()

let pemap = self.maps.create_map(
    &format!("{}.pe", map_name),
    base,
    align_up!(pe64.opt.size_of_headers, sec_allign) as u64,  // → pe64.size_of_headers()
    Permission::READ_WRITE,
)?;
pemap.memcpy(pe64.get_headers(), pe64.opt.size_of_headers as usize);

for i in 0..pe64.num_of_sections() {
    let ptr = pe64.get_section_ptr(i);                   // → pe64.get_section_data(i)
    let sect = pe64.get_section(i);                      // → pe64.get_section(i)
    // ... section mapping logic
}
```

#### Target Loader Pattern (LiefPe)

```rust
// Updated loaders.rs
use crate::pe::lief::LiefPe;

let pe = LiefPe::load(filename)?;

let mut base = pe.image_base();
if base < constants::LIBS64_MIN {
    base = self.maps.lib64_alloc(pe.size())?;
}

let sec_allign = pe.section_alignment();

let pemap = self.maps.create_map(
    &format!("{}.pe", map_name),
    base,
    align_up!(pe.size_of_headers(), sec_allign) as u64,
    Permission::READ_WRITE,
)?;
pemap.memcpy(pe.get_headers(), pe.size_of_headers() as usize);

for i in 0..pe.num_sections() {
    let ptr = pe.get_section_data(i).unwrap_or(&[]);     // Lazy loaded!
    let sect = pe.get_section(i);
    // ... section mapping logic
}
```

#### Key Differences

| Aspect | Old (PE64) | New (LiefPe) |
|--------|------------|--------------|
| Section data | Pre-loaded in `raw: Vec<u8>` | Lazy loaded on-demand |
| Memory footprint | Full file (~1MB+) | ~4KB initial |
| Cache control | None | LRU with configurable limits |
| Section access | `get_section_ptr(i)` | `get_section_data(i)` |

---

### Phase 4: Testing & Validation

See [Testing Strategy](#testing-strategy) below.

---

## Backward Compatibility

### Critical Methods to Preserve

These methods MUST maintain their signatures for loader compatibility:

```rust
// Section access - MUST BE PRESERVED
fn get_section_ptr(&self, index: usize) -> Option<&[u8]>
fn get_section_by_name(&self, name: &str) -> Option<&[u8]>

// Header info - MUST BE PRESERVED
fn image_base(&self) -> u64
fn entry_point(&self) -> u64
fn size_of_headers(&self) -> u32
fn num_sections(&self) -> u16
fn section_alignment(&self) -> u32

// Section metadata - MUST BE PRESERVED
fn get_section(&self, index: usize) -> Option<&lief::pe::Section>
fn get_section_vaddr(&self, index: usize) -> Option<u64>
fn get_section_size(&self, index: usize) -> Option<u64>
fn get_section_offset(&self, index: usize) -> Option<u64>

// Import resolution - MUST BE PRESERVED
fn import_addr_to_name(&self, paddr: u64) -> String
fn get_imports(&self) -> Result<Vec<ImportInfo>, LiefError>

// Binding operations - MUST BE IMPLEMENTED
fn iat_binding(&mut self, emu: &mut Emu, base: u64)
fn delay_load_binding(&mut self, emu: &mut Emu, base: u64)
fn apply_relocations(&mut self, emu: &mut Emu, base: u64)
```

### Field Access Patterns in Loaders

Current code uses direct field access:

```rust
// Pattern 1: Direct struct field access
let base = pe64.opt.image_base;
let entry = pe64.opt.address_of_entry_point;

// Pattern 2: Method call
let headers = pe64.get_headers();
let section = pe64.get_section_ptr(i);

// Pattern 3: DOS header access
let pe_off = pe64.dos.e_lfanew;
```

LIEF migration requires:
1. Adding getter methods for all accessed fields
2. Converting direct DOS header access to method calls
3. Using LIEF's section objects instead of custom headers

---

## Usage Examples

### Loading a PE File

```rust
use libmwemu::pe::lief::LiefPe;

// Simple loading
let pe = LiefPe::load("ntdll.dll")?;

println!("Image base: {:#x}", pe.image_base());
println!("Entry point: {:#x}", pe.entry_point());
println!("Sections: {}", pe.num_sections());
println!("Is DLL: {}", pe.is_dll());
```

### Loading with Cache Policy

```rust
use libmwemu::pe::lief::lief_section_manager::CachePolicy;

let pe = LiefPe::load_with_policy("kernel32.dll", CachePolicy::LRU { max_bytes: 50_000_000 })?;
```

### Accessing Sections (Lazy Loaded)

```rust
// Get section by index (lazy loaded on first access)
if let Some(text_data) = pe.get_section_data(0) {
    println!("Section 0 size: {} bytes", text_data.len());
}

// Get section by name
if let Some(data) = pe.get_section_data_by_name(".text") {
    println!("Found .text section");
}

// Check if section is already cached
if pe.is_section_loaded(".text") {
    println!(".text is in cache");
}
```

### Cache Management

```rust
// Get cache statistics
let stats = pe.cache_stats();
println!("Cached sections: {:?}", stats.cached_sections);
println!("Total cached: {} bytes", stats.cached_bytes);

// Clear cache if needed
pe.clear_cache();
```

### Header Access

```rust
// Get raw headers
let headers = pe.get_headers();

// Get specific header values
println!("Size of headers: {}", pe.size_of_headers());
println!("Section alignment: {}", pe.section_alignment());
println!("File alignment: {}", pe.file_alignment());

// Convert addresses
if let Some(offset) = pe.vaddr_to_offset(0x1000) {
    println!("VA 0x1000 -> File offset {:#x}", offset);
}
```

### Import/Export Information

```rust
// Get imports
let imports = pe.get_imports()?;
for import in imports {
    println!("DLL: {}", import.dll_name);
    for func in import.functions {
        println!("  - {} (ordinal: {})", func.name, func.ordinal);
    }
}

// Reverse IAT lookup
let func_name = pe.import_addr_to_name(0x180010000);
println!("IAT entry points to: {}", func_name);
```

---

## Testing Strategy

### 1. Unit Tests

Test each [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs) component:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header_only_parsing() {
        let pe = LiefPe::load("test_data/kernel32.dll").unwrap();
        assert_eq!(pe.num_sections(), 6);
        assert_eq!(pe.image_base(), 0x180000000);
    }
    
    #[test]
    fn test_lazy_section_loading() {
        let pe = LiefPe::load("test_data/kernel32.dll").unwrap();
        
        // Initially, no sections cached
        let stats = pe.cache_stats();
        assert!(stats.cached_sections.is_empty());
        
        // Access section
        let _ = pe.get_section_data(0);
        
        // Now should be cached
        let stats = pe.cache_stats();
        assert!(!stats.cached_sections.is_empty());
    }
    
    #[test]
    fn test_section_data_consistency() {
        // Compare LiefPe section data with raw bytes
        let pe = LiefPe::load("test_data/kernel32.dll").unwrap();
        let pe64 = PE64::load("test_data/kernel32.dll");
        
        for i in 0..pe.num_sections() {
            let lief_data = pe.get_section_data(i);
            let old_data = pe64.get_section_ptr(i);
            assert_eq!(lief_data, old_data);
        }
    }
}
```

### 2. Integration Tests

Compare [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs) output with original PE32/PE64:

```rust
#[test]
fn test_section_data_matches_pe64() {
    let pe_old = PE64::load("test_data/kernel32.dll");
    let pe_new = LiefPe::load("test_data/kernel32.dll").unwrap();
    
    // Header values should match
    assert_eq!(pe_old.opt.image_base, pe_new.image_base());
    assert_eq!(pe_old.opt.address_of_entry_point, pe_new.entry_point() as u32);
    assert_eq!(pe_old.opt.size_of_headers, pe_new.size_of_headers());
    assert_eq!(pe_old.num_of_sections(), pe_new.num_sections());
    
    // Section data should match
    for i in 0..pe_old.num_of_sections() {
        let old_data = pe_old.get_section_ptr(i);
        let new_data = pe_new.get_section_data(i).unwrap();
        assert_eq!(old_data, new_data);
    }
}
```

### 3. Memory Profiling Tests

```rust
#[test]
fn test_memory_footprint() {
    // Before LIEF: ~1MB+ for headers
    // After LIEF: ~4KB for headers
    
    let pe = LiefPe::load("test_data/large.dll").unwrap();
    let stats = pe.cache_stats();
    
    // Should be minimal after header parse
    assert!(stats.cached_bytes < 10_000, "Header cache too large");
}

#[test]
fn test_lazy_loading_efficiency() {
    let pe = LiefPe::load("test_data/large.dll").unwrap();
    
    // Initially no sections cached
    let initial_stats = pe.cache_stats();
    assert_eq!(initial_stats.cached_sections.len(), 0);
    
    // Access only .text section
    let _ = pe.get_section_data_by_name(".text");
    
    // Only .text should be cached
    let final_stats = pe.cache_stats();
    assert_eq!(final_stats.cached_sections.len(), 1);
    assert!(final_stats.cached_sections.contains(".text"));
}
```

### 4. Migration Checklist

- [ ] Add `mmap` dependency to Cargo.toml
- [ ] Verify [`LiefPe::get_section_data()`](crates/libmwemu/src/pe/lief/lief_pe.rs:325) returns correct data
- [ ] Implement `apply_relocations()` using LIEF
- [ ] Implement `iat_binding()` using LIEF
- [ ] Implement `delay_load_binding()` using LIEF
- [ ] Add missing accessor methods (section_alignment, file_alignment, characteristics)
- [ ] Remove 15 dead functions from [`pe64.rs`](crates/libmwemu/src/pe/pe64.rs)
- [ ] Remove 11+ dead functions from [`pe32.rs`](crates/libmwemu/src/pe/pe32.rs)
- [ ] Update [`loaders.rs`](crates/libmwemu/src/emu/loaders.rs) to use LiefPe
- [ ] Add unit tests for LiefPe
- [ ] Add integration tests comparing old vs new
- [ ] Benchmark memory usage before/after
- [ ] Verify all existing functionality works

---

## Related Documentation

- [LIEF_ARCHITECTURE.md](LIEF_ARCHITECTURE.md) - Detailed architecture analysis including dead code analysis
- [PLAN.md](PLAN.md) - Original refactoring plan with bug fixes
- [Microsoft PE Format Spec](https://docs.microsoft.com/en-us/windows/win32/debug/pe-format)
- [LIEF Library](https://lief-project.github.io/)

---

## Implementation Status

| Component | Status | Priority | Notes |
|-----------|--------|----------|-------|
| `LiefError` | ✅ Complete | - | All error variants defined |
| `LiefPeReader` trait | ✅ Complete | - | All methods defined |
| `LiefHeaderParser` | ✅ Complete | - | Header-only parsing |
| `LiefSectionManager` | ✅ Complete | - | Lazy loading with LRU |
| `LiefPe::get_section_data()` | ✅ Complete | - | Verified implementation |
| `LiefPe::import_addr_to_name()` | ✅ Complete | - | Default in trait |
| `mmap` dependency | ❌ Missing | CRITICAL | Project won't compile |
| `LiefPe::apply_relocations()` | ❌ Missing | HIGH | Needed for loaders |
| `LiefPe::iat_binding()` | ❌ Missing | HIGH | Needed for loaders |
| `LiefPe::delay_load_binding()` | ❌ Missing | MED | Delay-load support |
| Missing accessor methods | ❌ Missing | HIGH | section_alignment, etc. |
| PE64 dead code removal | ❌ Pending | MED | 15 functions can be removed |
| PE32 dead code removal | ❌ Pending | MED | 11+ functions can be removed |
| loaders.rs update | ❌ Pending | HIGH | Switch to LiefPe |
