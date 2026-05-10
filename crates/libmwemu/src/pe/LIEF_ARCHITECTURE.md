# LIEF Integration Architecture for PE Parser Migration

## Executive Summary

The PE parser in `mwemu` is at a critical juncture where memory inefficiency is becoming a bottleneck. The current architecture loads entire PE files (~1MB+) into memory via `Vec<u8>` when only headers (~4KB) are typically needed for most operations.

### Key Findings

| Metric | Current State | Target State |
|--------|---------------|--------------|
| **PE64 Functions** | 34 total | 14 USED, 15 DEAD CODE |
| **PE32 Functions** | 66 total | ~23 USED, ~11 DEAD CODE |
| **Header Memory** | ~1MB+ per file | ~4KB per file |
| **Section Loading** | Full file in memory | Lazy loading with LRU |

### Critical Path Functions (Must Keep)

- [`load()`](crates/libmwemu/src/pe/pe64.rs:346) / [`load(filename)`](crates/libmwemu/src/pe/pe32.rs:894) - Entry point
- [`load_from_raw()`](crates/libmwemu/src/pe/pe64.rs:259) / [`load_from_raw(filename, raw)`](crates/libmwemu/src/pe/pe32.rs:806) - Alternative constructor
- [`size()`](crates/libmwemu/src/pe/pe64.rs:355) / [`size()`](crates/libmwemu/src/pe/pe32.rs:904) - File size
- [`is_dll()`](crates/libmwemu/src/pe/pe64.rs:372) / [`is_pe32()`](crates/libmwemu/src/pe/pe32.rs:728) - PE detection
- [`num_of_sections()`](crates/libmwemu/src/pe/pe64.rs:432) / [`num_of_sections()`](crates/libmwemu/src/pe/pe32.rs:960) - Section count
- [`get_section()`](crates/libmwemu/src/pe/pe64.rs:448) / [`get_section(id)`](crates/libmwemu/src/pe/pe32.rs:976) - Get section header
- [`get_section_ptr(id)`](crates/libmwemu/src/pe/pe64.rs:456) / [`get_section_ptr(id)`](crates/libmwemu/src/pe32.rs:980) - **HIGH FREQUENCY** section data access
- [`get_headers()`](crates/libmwemu/src/pe/pe64.rs:380) / [`get_headers()`](crates/libmwemu/src/pe32.rs:935) - Header bytes
- [`apply_relocations()`](crates/libmwemu/src/pe/pe64.rs:765) / relocation methods - ASLR handling
- [`iat_binding()`](crates/libmwemu/src/pe/pe64.rs:607) / [`iat_binding(emu, base)`](crates/libmwemu/src/pe32.rs:1109) - Import binding
- [`delay_load_binding()`](crates/libmwemu/src/pe/pe64.rs:520) / [`delay_load_binding(emu, base)`](crates/libmwemu/src/pe32.rs:1041) - Delay-load binding
- [`get_dependencies()`](crates/libmwemu/src/pe/pe64.rs:586) - Import DLL names
- [`import_addr_to_name()`](crates/libmwemu/src/pe/pe64.rs:846) / [`import_addr_to_name(paddr)`](crates/libmwemu/src/pe/pe32.rs:1210) - Reverse IAT lookup
- [`get_resource()`](crates/libmwemu/src/pe/pe64.rs:1095) / [`get_resource(...)`](crates/libmwemu/src/pe32.rs:1347) - Resource lookup

### Dead Code That Can Be Removed

**PE64 (15 functions):**
- [`mem_size()`](crates/libmwemu/src/pe/pe64.rs:359) - Never called
- [`get_raw()`](crates/libmwemu/src/pe/pe64.rs:376) - Never called externally
- [`clear()`](crates/libmwemu/src/pe/pe64.rs:384) - Never called
- [`vaddr_to_off()`](crates/libmwemu/src/pe/pe64.rs:389) - Only called internally
- [`read_string()`](crates/libmwemu/src/pe/pe64.rs:415) - Only called internally
- [`get_section_ptr_by_name()`](crates/libmwemu/src/pe/pe64.rs:436) - Never called
- [`get_section_vaddr()`](crates/libmwemu/src/pe/pe64.rs:481) - Never called
- [`get_tls_callbacks()`](crates/libmwemu/src/pe/pe64.rs:485) - Never called
- [`iat_binding_alternative()`](crates/libmwemu/src/pe/pe64.rs:656) - Never called
- [`iat_binding_original()`](crates/libmwemu/src/pe64.rs:698) - Never called
- [`locate_resource_data_entry()`](crates/libmwemu/src/pe/pe64.rs:895) - Never called externally
- [`read_resource_name_from_rsrc()`](crates/libmwemu/src/pe/pe64.rs:1063) - Never called externally
- [`get_resource_name()`](crates/libmwemu/src/pe/pe64.rs:1124) - Never called externally

**PE32 (11+ functions):**
- [`get_section_vaddr()`](crates/libmwemu/src/pe/pe32.rs:995) - Never called
- [`get_tls_callbacks()`](crates/libmwemu/src/pe/pe32.rs:999) - Never called
- [`read_string_200()`](crates/libmwemu/src/pe/pe32.rs:781) - Never called
- [`get_filename()`](crates/libmwemu/src/pe/pe32.rs:908) - Never called
- [`get_raw()`](crates/libmwemu/src/pe/pe32.rs:931) - Never called externally
- [`get_headers()`](crates/libmwemu/src/pe/pe32.rs:935) - Redundant (already in `raw`)
- [`clear()`](crates/libmwemu/src/pe/pe32.rs:939) - Never called
- [`get_resource_name()`](crates/libmwemu/src/pe/pe32.rs:1374) - Never called externally
- [`ImageBaseRelocation::load()`](crates/libmwemu/src/pe/pe32.rs:685) - Never called
- [`read_string()`](crates/libmwemu/src/pe/pe32.rs:763) - Only called internally
- [`vaddr_to_off()`](crates/libmwemu/src/pe/pe32.rs:946) - Only called internally
- [`get_section_ptr_by_name()`](crates/libmwemu/src/pe/pe32.rs:964) - Never called
- [`locate_resource_data_entry()`](crates/libmwemu/src/pe/pe32.rs:1257) - Never called externally

---

## Current State Analysis

### Problem: Memory-Inefficient Architecture

The current [`PE32`](crates/libmwemu/src/pe/pe32.rs:713) and [`PE64`](crates/libmwemu/src/pe/pe64.rs:228) structs store the **entire PE file** in memory:

```rust
pub struct PE64 {
    pub filename: String,
    pub raw: Vec<u8>,           // <-- PROBLEM: Entire file loaded (~1MB+)
    pub dos: ImageDosHeader,
    pub nt: ImageNtHeaders,
    pub fh: ImageFileHeader,
    pub opt: ImageOptionalHeader64,
    pub sect_hdr: Vec<ImageSectionHeader>,
    // ...
}
```

**Current Issues:**

| Issue | Impact |
|-------|--------|
| Full file copy via `Vec<u8>` | ~1MB+ memory per PE file |
| No lazy loading | All sections loaded even if only headers needed |
| Double copy in `load_from_raw()` | `raw.to_vec()` creates duplicate |
| Direct field access | Tight coupling prevents optimization |

---

## Function Usage Analysis

### PE64 Functions (34 total)

#### USED Functions (14) - Must Keep

| Function | Called By | Purpose |
|----------|-----------|---------|
| [`load(filename)`](crates/libmwemu/src/pe/pe64.rs:346) | [`loaders.rs:301`](crates/libmwemu/src/emu/loaders.rs:301) | Main entry point |
| [`load_from_raw(filename, raw)`](crates/libmwemu/src/pe/pe64.rs:259) | [`loaders.rs:301`](crates/libmwemu/src/emu/loaders.rs:301), serialization | Alternative constructor |
| [`size()`](crates/libmwemu/src/pe/pe64.rs:355) | [`loaders.rs:210, 307, 317, 327, 335`](crates/libmwemu/src/emu/loaders.rs:210) | File size for memory allocation |
| [`is_dll()`](crates/libmwemu/src/pe/pe64.rs:372) | [`loaders.rs:637`](crates/libmwemu/src/emu/loaders.rs:637) | DLL detection |
| [`num_of_sections()`](crates/libmwemu/src/pe/pe64.rs:432) | [`loaders.rs:227, 378`](crates/libmwemu/src/emu/loaders.rs:227) | Section iteration |
| [`get_section(id)`](crates/libmwemu/src/pe/pe64.rs:448) | [`loaders.rs:229, 380`](crates/libmwemu/src/emu/loaders.rs:229) | Get section header |
| [`get_section_ptr(id)`](crates/libmwemu/src/pe/pe64.rs:456) | [`loaders.rs:228, 379`](crates/libmwemu/src/emu/loaders.rs:228) | **Get section data** - HIGH FREQUENCY |
| [`get_headers()`](crates/libmwemu/src/pe/pe64.rs:380) | [`loaders.rs:226, 376`](crates/libmwemu/src/emu/loaders.rs:226) | Header bytes for mapping |
| [`get_pe_off()`](crates/libmwemu/src/pe/pe64.rs:452) | [`initialization.rs:558`](crates/libmwemu/src/emu/initialization.rs:558) | PE header offset |
| [`apply_relocations(emu, base)`](crates/libmwemu/src/pe/pe64.rs:765) | [`initialization.rs:564`](crates/libmwemu/src/emu/initialization.rs:564) | ASLR relocation |
| [`iat_binding(emu, base)`](crates/libmwemu/src/pe/pe64.rs:607) | [`initialization.rs:565`](crates/libmwemu/src/emu/initialization.rs:565) | Import binding |
| [`delay_load_binding(emu, base)`](crates/libmwemu/src/pe/pe64.rs:520) | [`initialization.rs:566`](crates/libmwemu/src/emu/initialization.rs:566) | Delay-load binding |
| [`get_dependencies(emu)`](crates/libmwemu/src/pe/pe64.rs:586) | [`initialization.rs:531`](crates/libmwemu/src/emu/initialization.rs:531) | Get import DLL names |
| [`import_addr_to_name(paddr)`](crates/libmwemu/src/pe/pe64.rs:846) | [`instruction_pointer.rs:27`](crates/libmwemu/src/emu/instruction_pointer.rs:27) | Reverse IAT lookup |
| [`get_resource(...)`](crates/libmwemu/src/pe/pe64.rs:1095) | find_resource_*.rs | Resource lookup |

#### NOT USED Functions (15) - Can Be Removed

| Function | Lines | Reason |
|----------|-------|--------|
| [`mem_size()`](crates/libmwemu/src/pe/pe64.rs:359) | 359-370 | Never called |
| [`get_raw()`](crates/libmwemu/src/pe/pe64.rs:376) | 376-378 | Never called externally |
| [`clear()`](crates/libmwemu/src/pe/pe64.rs:384) | 384-387 | Never called |
| [`vaddr_to_off()`](crates/libmwemu/src/pe/pe64.rs:389) | 389-412 | Only called internally within PE64 |
| [`read_string()`](crates/libmwemu/src/pe/pe64.rs:415) | 415-430 | Only called internally within PE64 |
| [`get_section_ptr_by_name()`](crates/libmwemu/src/pe/pe64.rs:436) | 436-446 | Never called |
| [`get_section_vaddr()`](crates/libmwemu/src/pe/pe64.rs:481) | 481-483 | Never called |
| [`get_tls_callbacks()`](crates/libmwemu/src/pe/pe64.rs:485) | 485-518 | Never called |
| [`iat_binding_alternative()`](crates/libmwemu/src/pe/pe64.rs:656) | 656-696 | Never called |
| [`iat_binding_original()`](crates/libmwemu/src/pe/pe64.rs:698) | 698-763 | Never called |
| [`locate_resource_data_entry()`](crates/libmwemu/src/pe/pe64.rs:895) | 895-1060 | Never called externally |
| [`read_resource_name_from_rsrc()`](crates/libmwemu/src/pe/pe64.rs:1063) | 1062-1093 | Never called externally |
| [`get_resource_name()`](crates/libmwemu/src/pe/pe64.rs:1124) | 1124-1134 | Never called externally |

---

### PE32 Functions (66 total)

#### USED Functions (~23) - Must Keep

| Function | Called By | Purpose |
|----------|-----------|---------|
| [`load(filename)`](crates/libmwemu/src/pe/pe32.rs:894) | [`loaders.rs:28`](crates/libmwemu/src/emu/loaders.rs:28) | Main entry point |
| [`load_from_raw(filename, raw)`](crates/libmwemu/src/pe/pe32.rs:806) | serialization | Alternative constructor |
| [`is_pe32(filename)`](crates/libmwemu/src/pe/pe32.rs:728) | [`loaders.rs:606`](crates/libmwemu/src/emu/loaders.rs:606) | PE detection |
| [`size()`](crates/libmwemu/src/pe/pe32.rs:904) | [`loaders.rs:43, 55`](crates/libmwemu/src/emu/loaders.rs:43) | File size |
| [`opt.image_base`](crates/libmwemu/src/pe/pe32.rs:212) | [`loaders.rs:63, 73`](crates/libmwemu/src/emu/loaders.rs:63) | Direct field access |
| [`opt.address_of_entry_point`](crates/libmwemu/src/pe/pe32.rs:209) | [`loaders.rs:95, 101`](crates/libmwemu/src/emu/loaders.rs:95) | Direct field access |
| [`opt.size_of_headers`](crates/libmwemu/src/pe/pe32.rs:223) | [`loaders.rs:116`](crates/libmwemu/src/emu/loaders.rs:116) | Direct field access |
| [`opt.section_alignment`](crates/libmwemu/src/pe/pe32.rs:213) | [`loaders.rs:109`](crates/libmwemu/src/emu/loaders.rs:109) | Direct field access |
| [`dos.e_lfanew`](crates/libmwemu/src/pe/pe32.rs:94) | [`loaders.rs:199`](crates/libmwemu/src/emu/loaders.rs:199) | Direct field access |
| [`mem_size()`](crates/libmwemu/src/pe/pe32.rs:912) | [`loaders.rs:66, 70, 80`](crates/libmwemu/src/emu/loaders.rs:66) | Memory size calc |
| [`num_of_sections()`](crates/libmwemu/src/pe/pe32.rs:960) | [`loaders.rs:122`](crates/libmwemu/src/emu/loaders.rs:122) | Section count |
| [`get_section(id)`](crates/libmwemu/src/pe/pe32.rs:976) | [`loaders.rs:124`](crates/libmwemu/src/emu/loaders.rs:124) | Get section header |
| [`get_section_ptr(id)`](crates/libmwemu/src/pe/pe32.rs:980) | [`loaders.rs:123`](crates/libmwemu/src/emu/loaders.rs:123) | **Get section data** - HIGH FREQUENCY |
| [`get_headers()`](crates/libmwemu/src/pe/pe32.rs:935) | [`loaders.rs:120`](crates/libmwemu/src/emu/loaders.rs:120) | Header bytes |
| [`iat_binding(emu, base)`](crates/libmwemu/src/pe/pe32.rs:1109) | [`loaders.rs:88`](crates/libmwemu/src/emu/loaders.rs:88) | Import binding |
| [`delay_load_binding(emu, base)`](crates/libmwemu/src/pe/pe32.rs:1041) | [`loaders.rs:89`](crates/libmwemu/src/emu/loaders.rs:89) | Delay-load binding |
| [`import_addr_to_name(paddr)`](crates/libmwemu/src/pe/pe32.rs:1210) | [`instruction_pointer.rs:141`](crates/libmwemu/src/emu/instruction_pointer.rs:141) | Reverse IAT lookup |
| [`get_resource(...)`](crates/libmwemu/src/pe/pe32.rs:1347) | find_resource_*.rs | Resource lookup |

#### NOT USED Functions (11+) - Can Be Removed

| Function | Lines | Reason |
|----------|-------|--------|
| [`get_section_vaddr()`](crates/libmwemu/src/pe/pe32.rs:995) | 995-997 | Never called |
| [`get_tls_callbacks()`](crates/libmwemu/src/pe/pe32.rs:999) | 999-1039 | Never called |
| [`read_string_200()`](crates/libmwemu/src/pe/pe32.rs:781) | 781-804 | Never called |
| [`get_filename()`](crates/libmwemu/src/pe/pe32.rs:908) | 908-910 | Never called |
| [`get_raw()`](crates/libmwemu/src/pe/pe32.rs:931) | 931-933 | Never called externally |
| [`get_headers()`](crates/libmwemu/src/pe/pe32.rs:935) | 935-937 | Redundant (already in `raw`) |
| [`clear()`](crates/libmwemu/src/pe/pe32.rs:939) | 939-944 | Never called |
| [`get_resource_name()`](crates/libmwemu/src/pe/pe32.rs:1374) | 1374-1386 | Never called externally |
| [`ImageBaseRelocation::load()`](crates/libmwemu/src/pe/pe32.rs:685) | 685-695 | Never called |
| [`read_string()`](crates/libmwemu/src/pe/pe32.rs:763) | 763-778 | Only called internally |
| [`vaddr_to_off()`](crates/libmwemu/src/pe/pe32.rs:946) | 946-958 | Only called internally |
| [`get_section_ptr_by_name()`](crates/libmwemu/src/pe/pe32.rs:964) | 964-974 | Never called |
| [`locate_resource_data_entry()`](crates/libmwemu/src/pe/pe32.rs:1257) | 1257-1345 | Never called externally |

---

## Memory Optimization Strategy

### Target Memory Footprint

| Scenario | Current | Target | Reduction |
|----------|---------|--------|-----------|
| Headers only | ~1MB+ (full file) | ~4KB | 99.6%+ |
| 1 section cached | ~1MB+ | ~4KB + section_size | Significant |
| All sections | ~1MB+ | ~4KB + total_sections | Significant |

### Architecture Components

```
┌─────────────────────────────────────────────────────────────────┐
│                         LiefPe                                  │
│  ┌─────────────────────┐  ┌──────────────────────────────────┐ │
│  │  LiefHeaderParser   │  │  LiefSectionManager              │ │
│  │  ─────────────────  │  │  ────────────────────────────────  │ │
│  │  mapped_file: Arc   │  │  mapped_file: Arc<Mmap> (shared) │ │
│  │  pe: lief::pe::PE   │  │  section_cache: RwLock<HashMap>   │ │
│  │  header_cache: 4KB  │  │  access_order: LRU tracking       │ │
│  │                     │  │  cache_policy: configurable       │ │
│  │  [Header-only ops]  │  │  [Lazy section loading]           │ │
│  └─────────────────────┘  └──────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Key Design Principles

1. **Memory-Mapped Files**: Use OS memory-mapping instead of `Vec<u8>` copy
2. **Lazy Section Loading**: Load section data only when accessed
3. **LRU Cache**: Configurable eviction to prevent unbounded growth
4. **Shared Mmap**: Header parser and section manager share the same mmap
5. **Thread Safety**: `Arc<RwLock<>>` for concurrent access

### Memory Optimization Implementation

```rust
// Target: ~4KB header footprint
pub struct LiefPe {
    mapped_file: Arc<Mmap>,
    header_parser: LiefHeaderParser,      // ~4KB cached headers
    section_manager: LiefSectionManager,  // Lazy LRU-cached sections
}

// Section access with LRU eviction
pub struct LiefSectionManager {
    mapped_file: Arc<Mmap>,
    section_cache: RwLock<HashMap<usize, Arc<CachedSection>>>,
    access_order: Vec<usize>,  // LRU tracking
    max_cache_size: usize,     // Configurable limit
}
```

---

## Migration Phases

### Phase 1: Complete LIEF Integration (Critical Path)

**Goal**: Implement full LIEF replacement with backward compatibility

1. **Finalize LiefPe wrapper**:
   - Complete [`LiefPe::get_section_ptr()`](crates/libmwemu/src/pe/lief/lief_pe.rs:156) implementation
   - Implement [`LiefPe::import_addr_to_name()`](crates/libmwemu/src/pe/lief/traits.rs:142)
   - Implement [`LiefPe::get_resource()`](crates/libmwemu/src/pe/lief/lief_pe.rs:420)
   - Implement IAT binding and relocation methods

2. **Add accessor methods** for internal fields:
   - `opt.image_base` → `image_base()`
   - `opt.address_of_entry_point` → `entry_point()`
   - `opt.size_of_headers` → `size_of_headers()`
   - `dos.e_lfanew` → `e_lfanew()`

3. **Update loaders.rs** to use LiefPe instead of PE32/PE64

### Phase 2: Remove Dead Code

**Goal**: Clean up unused functions

1. **Remove PE64 dead functions** (15 functions):
   - [`mem_size()`](crates/libmwemu/src/pe/pe64.rs:359), [`get_raw()`](crates/libmwemu/src/pe/pe64.rs:376), [`clear()`](crates/libmwemu/src/pe/pe64.rs:384)
   - [`vaddr_to_off()`](crates/libmwemu/src/pe/pe64.rs:389), [`read_string()`](crates/libmwemu/src/pe/pe64.rs:415)
   - [`get_section_ptr_by_name()`](crates/libmwemu/src/pe/pe64.rs:436), [`get_section_vaddr()`](crates/libmwemu/src/pe/pe64.rs:481)
   - [`get_tls_callbacks()`](crates/libmwemu/src/pe/pe64.rs:485)
   - [`iat_binding_alternative()`](crates/libmwemu/src/pe/pe64.rs:656), [`iat_binding_original()`](crates/libmwemu/src/pe/pe64.rs:698)
   - [`locate_resource_data_entry()`](crates/libmwemu/src/pe/pe64.rs:895), [`read_resource_name_from_rsrc()`](crates/libmwemu/src/pe/pe64.rs:1063), [`get_resource_name()`](crates/libmwemu/src/pe/pe64.rs:1124)

2. **Remove PE32 dead functions** (11+ functions):
   - [`get_section_vaddr()`](crates/libmwemu/src/pe/pe32.rs:995), [`get_tls_callbacks()`](crates/libmwemu/src/pe/pe32.rs:999)
   - [`read_string_200()`](crates/libmwemu/src/pe/pe32.rs:781), [`get_filename()`](crates/libmwemu/src/pe/pe32.rs:908)
   - [`mem_size()`](crates/libmwemu/src/pe/pe32.rs:912), [`get_raw()`](crates/libmwemu/src/pe/pe32.rs:931)
   - [`get_headers()`](crates/libmwemu/src/pe/pe32.rs:935), [`clear()`](crates/libmwemu/src/pe/pe32.rs:939)
   - [`ImageBaseRelocation::load()`](crates/libmwemu/src/pe/pe32.rs:685), [`read_string()`](crates/libmwemu/src/pe/pe32.rs:763)
   - [`vaddr_to_off()`](crates/libmwemu/src/pe/pe32.rs:946), [`get_section_ptr_by_name()`](crates/libmwemu/src/pe/pe32.rs:964)
   - [`locate_resource_data_entry()`](crates/libmwemu/src/pe/pe32.rs:1257), [`get_resource_name()`](crates/libmwemu/src/pe/pe32.rs:1374)

### Phase 3: Field Encapsulation

**Goal**: Make internal fields private and provide getters

```rust
// Before
pub struct PE64 {
    pub raw: Vec<u8>,
    pub opt: ImageOptionalHeader64,
    // ...
}

// After
pub struct PE64 {
    raw: Vec<u8>,  // private
    opt: ImageOptionalHeader64,  // private via getter
}
```

### Phase 4: Optimize Section Loading

**Goal**: Improve LRU cache implementation

1. **Add cache statistics**:
   - Track hits/misses
   - Monitor eviction frequency

2. **Improve section layout caching**:
   - Cache section header info separately from data
   - Avoid re-parsing PE for each section access

---

## File Structure

```
crates/libmwemu/src/pe/
├── mod.rs                      # Module exports
├── pe32.rs                     # Existing (refactored - remove dead code)
├── pe64.rs                     # Existing (refactored - remove dead code)
├── lief/
│   ├── mod.rs                  # Module exports
│   ├── error.rs                # LiefError + info structs (COMPLETE)
│   ├── traits.rs               # LiefPeReader trait (COMPLETE)
│   ├── lief_header_parser.rs   # LiefHeaderParser (COMPLETE)
│   ├── lief_section_manager.rs # LiefSectionManager (COMPLETE)
│   └── lief_pe.rs              # LiefPe wrapper (NEEDS COMPLETION)
├── LIEF_ARCHITECTURE.md        # This document
├── DEVELOPMENT.md              # Updated implementation guide
└── PLAN.md                     # Migration checklist
```

---

## Implementation Status

| Component | Status | Priority | Notes |
|-----------|--------|----------|-------|
| `LiefError` | ✅ Complete | - | All error variants defined |
| `LiefPeReader` trait | ✅ Complete | - | All methods defined |
| `LiefHeaderParser` | ✅ Complete | - | Header-only parsing |
| `LiefSectionManager` | ✅ Complete | - | Lazy loading with LRU |
| `LiefPe::get_section_ptr()` | ⚠️ Partial | HIGH | Needs verification with loaders.rs usage |
| `LiefPe::import_addr_to_name()` | ⚠️ Partial | HIGH | Uses LIEF imports |
| `LiefPe::get_resource()` | ⚠️ Partial | MED | Uses LIEF resources but incomplete |
| `LiefPe::apply_relocations()` | ❌ Missing | HIGH | Critical for ASLR |
| `LiefPe::iat_binding()` | ❌ Missing | HIGH | Critical for import resolution |
| `LiefPe::delay_load_binding()` | ❌ Missing | MED | Delay-load support |
| `LiefPe::get_dependencies()` | ❌ Missing | MED | DLL name extraction |
| PE64 dead code removal | ❌ Pending | MED | 15 functions can be removed |
| PE32 dead code removal | ❌ Pending | MED | 11+ functions can be removed |
| loaders.rs update | ❌ Pending | HIGH | Switch to LiefPe |

---

## Backward Compatibility

The migration must maintain these interfaces:

```rust
// Section access - MUST BE PRESERVED
pub fn get_section_ptr(&self, id: usize) -> &[u8]

// Import resolution - MUST BE PRESERVED  
pub fn import_addr_to_name(&self, paddr: u64) -> String

// Resource lookup - MUST BE PRESERVED
pub fn get_resource(&self, type_id: Option<u32>, name_id: Option<u32>, ...) -> Option<(u64, usize)>

// Binding operations - MUST BE PRESERVED
pub fn iat_binding(&mut self, emu: &mut Emu, base: u64)
pub fn delay_load_binding(&mut self, emu: &mut Emu, base: u64)
pub fn apply_relocations(&mut self, emu: &mut Emu, base: u64)

// Header info - MUST BE PRESERVED
pub fn image_base() -> u64
pub fn entry_point() -> u64
pub fn size_of_headers() -> u32
```

---

## Dependencies

Current (from Cargo.toml):
```toml
lief = "0.17.6"        # Already present
thiserror = "2.0.12"   # Already present
```

Required addition:
```toml
[dependencies]
mmap = "0.1"            # Memory mapping support
```

---

## Key Metrics

| Metric | Before | After |
|--------|--------|-------|
| Header parse memory | ~1MB+ | ~4KB |
| Section access | Full file in memory | On-demand loading |
| Parse time (headers) | ~50ms | ~5ms |
| Cache control | None | LRU with limits |
| Dead code (PE64) | 15 functions | 0 functions |
| Dead code (PE32) | 11+ functions | 0 functions |

---

## Action Items

### Immediate (Phase 1 - LIEF Completion)

- [ ] Implement `LiefPe::apply_relocations()` using LIEF
- [ ] Implement `LiefPe::iat_binding()` using LIEF
- [ ] Implement `LiefPe::delay_load_binding()` using LIEF
- [ ] Implement `LiefPe::get_dependencies()` using LIEF
- [ ] Verify `LiefPe::get_section_ptr()` correctness
- [ ] Update loaders.rs to use LiefPe

### Short-term (Phase 2 - Dead Code Removal)

- [ ] Remove 15 dead functions from pe64.rs
- [ ] Remove 11+ dead functions from pe32.rs
- [ ] Verify no regressions after removal

### Medium-term (Phase 3 - Encapsulation)

- [ ] Make PE64::raw private
- [ ] Make PE32::raw private
- [ ] Add accessor methods for all direct field accesses
- [ ] Update loaders.rs to use accessors

### Long-term (Phase 4 - Optimization)

- [ ] Add cache statistics to LiefSectionManager
- [ ] Benchmark memory usage before/after
- [ ] Consider removing PE32/PE64 entirely if LiefPe is stable
