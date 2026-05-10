# PE Parser Migration to LIEF - Migration Plan

## Executive Summary

This document outlines the step-by-step migration plan to replace the custom PE32/PE64 parsers in [`pe32.rs`](crates/libmwemu/src/pe/pe32.rs) and [`pe64.rs`](crates/libmwemu/src/pe/pe64.rs) with the LIEF library via [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs). The migration eliminates ~26 dead code functions, reduces memory footprint from full file copies to ~4KB headers, and centralizes PE parsing logic.

**Key Benefits:**
- Eliminate 26+ dead code functions (15 from PE64, 11+ from PE32)
- Memory optimization: ~4KB header vs full file copy
- Single, well-maintained parsing implementation
- Better error handling via LIEF's error types

---

## Current State Analysis

### Function Inventory

| Parser | Total Functions | Used | Dead Code |
|--------|-----------------|------|-----------|
| [`pe64.rs`](crates/libmwemu/src/pe/pe64.rs) | 34 | 14 | 15 |
| [`pe32.rs`](crates/libmwemu/src/pe/pe32.rs) | 66 | ~23 | ~11 |

### Dead Code by File

**[`pe64.rs`](crates/libmwemu/src/pe/pe64.rs)** - 15 unused functions:
- `pe64_print()`, `pe64_free()`, `pe64_rva_to_offset()`, `pe64_get_sections()`
- `pe64_get_section_by_name()`, `pe64_get_section_count()`, `pe64_get_imports()`
- `pe64_get_exports()`, `pe64_get_relocations()`, `pe64_get_tls()`
- `pe64_get_resources()`, `pe64_apply_relocations()`, `pe64_load_resources()`
- `pe64_get_data_directory()` (if unused)

**[`pe32.rs`](crates/libmwemu/src/pe/pe32.rs)** - ~11 unused functions:
- Similar pattern to PE64 dead code
- Print/debug functions, unused accessors

### Memory Usage Comparison

| Approach | Memory | Implementation |
|----------|--------|----------------|
| Current | Full file copy in buffer | [`pe64_read_file()`](crates/libmwemu/src/pe/pe64.rs:1) / [`pe32_read_file()`](crates/libmwemu/src/pe/pe32.rs:1) |
| Target | ~4KB header only | [`LiefPe::parse`](crates/libmwemu/src/pe/lief/lief_pe.rs:1) |

### Entry Points Requiring Migration

The following loaders reference PE64/PE32 directly:
- [`loaders.rs`](crates/libmwemu/src/emu/loaders.rs) - PE loading logic
- [`initialization.rs`](crates/libmwemu/src/emu/initialization.rs) - DLL loading

---

## Migration Phases

### Phase 1: Fix Critical Issues (Day 1)

**Objective:** Get LIEF integration compiling and verify [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs) works

#### Step 1.1: Add mmap Dependency

```toml
# crates/libmwemu/Cargo.toml
[dependencies]
mmap = "0.1"
```

#### Step 1.2: Verify LiefPe Module Compiles

```bash
cargo build -p libmwemu --features pe 2>&1 | head -50
```

#### Step 1.3: Fix Any Compilation Errors

Common issues to watch for:
- Missing imports in [`lief_pe.rs`](crates/libmwemu/src/pe/lief/lief_pe.rs)
- Type mismatches with LIEF API
- Missing trait implementations

#### Step 1.4: Verify Basic Parsing Works

Add a simple test in [`lief_pe.rs`](crates/libmwemu/src/pe/lief/lief_pe.rs):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_valid_pe() {
        // Use a test PE file
    }
}
```

**Exit Criteria:** `cargo build` succeeds with LIEF module

---

### Phase 2: Complete LiefPe Implementation (Days 2-3)

**Objective:** Implement all accessor methods needed by the emulators

#### Step 2.1: Complete `get_section_data()`

In [`lief_pe.rs`](crates/libmwemu/src/pe/lief/lief_pe.rs), implement section data retrieval:

```rust
pub fn get_section_data(&self, section_name: &str) -> Option<&[u8]> {
    self.pe.section_by_name(section_name)
        .map(|s| s.content())
}
```

#### Step 2.2: Add Missing Accessor Methods

Add these methods to [`LiefPe`](crates/libmwemu/src/pe/lief/lief_pe.rs):

| Method | LIEF API | Notes |
|--------|----------|-------|
| `image_base()` | `binary.image_base()` | Virtual address base |
| `entry_point()` | `binary.entrypoint()` | Starting RVA |
| `size_of_headers()` | `binary.dos_header().addressof_new_exeheader()` | Header size |
| `section_count()` | `binary.sections().len()` | Number of sections |

#### Step 2.3: Implement `vaddr_to_offset()`

```rust
pub fn vaddr_to_offset(&self, vaddr: u64) -> Option<u64> {
    self.pe.rva_to_offset(vaddr as u32)
        .map(|o| o as u64)
}
```

#### Step 2.4: Implement `get_dependencies()`

```rust
pub fn get_dependencies(&self) -> Vec<String> {
    self.pe.imports()
        .iter()
        .map(|i| i.name().to_string())
        .collect()
}
```

#### Step 2.5: Implement Section Accessors

```rust
pub fn get_section(&self, name: &str) -> Option<&LiefSection> {
    self.pe.section_by_name(name)
}

pub fn sections(&self) -> impl Iterator<Item = &LiefSection> {
    self.pe.sections().iter()
}
```

**Exit Criteria:** All accessor methods implemented and tested

---

### Phase 3: Implement Binding Methods (Days 4-5)

**Objective:** Implement relocation and binding methods using LIEF

#### Step 3.1: Implement `apply_relocations()`

In [`lief_pe.rs`](crates/libmwemu/src/pe/lief/lief_pe.rs):

```rust
pub fn apply_relocations(&self, delta: i64) -> Result<(), LiefError> {
    for reloc in self.pe.relocations() {
        // LIEF provides relocation data
        for entry in reloc.entries() {
            // Process each relocation entry
        }
    }
    Ok(())
}
```

#### Step 3.2: Implement `iat_binding()`

```rust
pub fn bind_iat(&self, imports: &ImportResolver) -> Result<(), LiefError> {
    // May need custom IAT patching logic
    // LIEF can read imports but writing requires careful handling
    todo!("IAT binding implementation")
}
```

#### Step 3.3: Implement `delay_load_binding()`

```rust
pub fn bind_delay_load(&self, delay_handler: DelayLoadHandler) -> Result<(), LiefError> {
    // Delay-load specific handling
    // Look for DLLOption structures
    todo!("Delay-load binding implementation")
}
```

#### Step 3.4: Implement `get_tls_callbacks()`

```rust
pub fn get_tls_callbacks(&self) -> Vec<u64> {
    self.pe.tls()
        .map(|tls| {
            tls.callbacks()
                .iter()
                .map(|&cb| cb as u64)
                .collect()
        })
        .unwrap_or_default()
}
```

**Exit Criteria:** Binding methods compile and handle basic cases

---

### Phase 4: Wire Up Loaders (Days 6-7)

**Objective:** Replace PE64/PE32 usage with LiefPe in production code

#### Step 4.1: Update [`loaders.rs`](crates/libmwemu/src/emu/loaders.rs)

Replace PE64 field accesses with LiefPe methods:

```rust
// BEFORE
let base = pe64.opt.image_base;
let entry = pe64.opt.address_of_entry_point;

// AFTER  
let base = pe64.image_base();
let entry = pe64.entry_point();
```

Key changes needed:
- `pe64.opt.image_base` → `pe64.image_base()`
- `pe64.opt.address_of_entry_point` → `pe64.entry_point()`
- `pe64.size_of_headers` → `pe64.size_of_headers()`
- Section iteration via `pe64.sections()` iterator

#### Step 4.2: Update [`initialization.rs`](crates/libmwemu/src/emu/initialization.rs)

Replace DLL loading code:
- Find where PE32/PE64 is instantiated for DLL loading
- Replace with `LiefPe::parse(path)`

#### Step 4.3: Replace Import Access

```rust
// BEFORE
for imp in &pe64.imports {

// AFTER
for imp in pe64.get_dependencies() {
```

#### Step 4.4: Replace Section Access

```rust
// BEFORE
for section in &pe64.sections {

// AFTER
for section in pe64.sections() {
```

#### Step 4.5: Test PE Loading

Run existing tests:
```bash
cargo test -p libmwemu -- pe --nocapture
```

**Exit Criteria:** All PE loading functionality works via LiefPe

---

### Phase 5: Remove Dead Code (Day 8)

**Objective:** Clean up unused code from PE32/PE64

#### Step 5.1: Remove Dead Code from [`pe64.rs`](crates/libmwemu/src/pe/pe64.rs)

Remove these 15 functions (verify each is truly unused first):

```rust
// REMOVE:
pe64_print()           // Debug only
pe64_free()            // Memory cleanup, no longer needed
pe64_rva_to_offset()  // Replaced by LiefPe::vaddr_to_offset
pe64_get_sections()   // Replaced by LiefPe::sections()
pe64_get_section_by_name()  // Replaced by LiefPe::get_section()
pe64_get_section_count()    // Replaced by LiefPe::section_count()
pe64_get_imports()     // Replaced by LiefPe::get_dependencies()
pe64_get_exports()     // If unused
pe64_get_relocations() // Replaced by LiefPe::apply_relocations()
pe64_get_tls()         // Replaced by LiefPe::get_tls_callbacks()
pe64_get_resources()   // If unused
pe64_apply_relocations()  // Replaced
pe64_load_resources()    // If unused
pe64_get_data_directory() // If unused
pe64_verify()            // If unused
```

#### Step 5.2: Remove Dead Code from [`pe32.rs`](crates/libmwemu/src/pe/pe32.rs)

Apply same analysis to PE32 functions.

#### Step 5.3: Remove Print/Debug Methods

```rust
// If these are only used for debugging:
pe64_print()    // DELETE
pe32_print()    // DELETE
```

#### Step 5.4: Verify Compilation

```bash
cargo build -p libmwemu 2>&1 | grep -E "(error|warning: unused)"
```

#### Step 5.5: Commit Intermediate Result

```bash
git add -A
git commit -m "chore(pe): remove dead code after LIEF migration"
```

**Exit Criteria:** No unused functions remain, project compiles cleanly

---

### Phase 6: Testing & Validation (Days 9-10)

**Objective:** Ensure migration is correct and performant

#### Step 6.1: Unit Tests for Header Parsing

Create [`tests/lief_pe_tests.rs`](crates/libmwemu/src/pe/lief/tests/lief_pe_tests.rs):

```rust
#[test]
fn test_image_base_matches() {
    let test_files = ["msgbox.exe", "minecraft.exe"];
    for file in test_files {
        let lief_pe = LiefPe::parse(file);
        let legacy_pe = pe64_read_file(file);
        
        assert_eq!(lief_pe.image_base(), legacy_pe.opt.image_base);
    }
}
```

#### Step 6.2: Integration Tests

Compare outputs between LiefPe and original PE64/PE32:

```rust
#[test]
fn test_section_parsing_consistency() {
    // Verify section names, sizes, characteristics match
}
```

#### Step 6.3: Memory Profiling

```bash
# Measure memory before
cargo build --release
valgrind --tool=massif ./target/release/mwemu test.lua

# Compare heap usage
ms_print massif.out.XXX | grep -A 20 "Peak"
```

#### Step 6.4: Test with Real PE Files

| Test File | What to Verify |
|-----------|----------------|
| `msgbox.exe` | Basic loading, entry point |
| `minecraft.exe` | Large PE, many sections |
| `ntdll.dll` | DLL loading, imports |
| `kernel32.dll` | Delay-load scenarios |

#### Step 6.5: Performance Benchmark

```rust
#[test]
fn test_parse_performance() {
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = LiefPe::parse("test.exe");
    }
    let elapsed = start.elapsed();
    println!("Average parse time: {:?}", elapsed / 1000);
}
```

**Exit Criteria:** All tests pass, memory usage reduced, performance acceptable

---

## Rollback Plan

If issues occur at any phase:

### Quick Rollback (Same Day)

```bash
git checkout HEAD~1 -- crates/libmwemu/src/pe/
git checkout HEAD~1 -- crates/libmwemu/src/emu/loaders.rs
git checkout HEAD~1 -- crates/libmwemu/src/emu/initialization.rs
cargo build -p libmwemu
```

### Gradual Rollback (After Phase 3)

If binding methods fail:

1. Keep PE64/PE32 alive for binding operations
2. Use LiefPe only for header parsing
3. Mark binding methods as `#[deprecated]`

```rust
#[deprecated(since = "0.2.0", note = "Use LiefPe methods instead")]
pub fn pe64_apply_relocations(...) { ... }
```

### Full Rollback

```bash
git revert <migration-commit-hash>
```

---

## Success Criteria

### Must Have (Release Blocker)

- [ ] `cargo build -p libmwemu` succeeds with no errors
- [ ] All existing PE loading tests pass
- [ ] No regression in msgbox.exe execution
- [ ] Memory usage for header parsing < 10KB per PE

### Should Have (Post-Migration)

- [ ] 0 dead code functions in pe64.rs/pe32.rs (or documented why needed)
- [ ] Performance: LiefPe parse time < legacy parse time
- [ ] Integration tests compare LiefPe vs legacy output

### Nice to Have (Polished)

- [ ] Benchmarks showing memory improvement
- [ ] Documentation updated in README
- [ ] LiefPe methods have rustdoc comments

---

## Estimated Timeline

| Phase | Days | Personnel | Dependencies |
|-------|------|-----------|--------------|
| Phase 1: Fix Critical Issues | 1 | 1 | None |
| Phase 2: Complete LiefPe | 2-3 | 1 | Phase 1 |
| Phase 3: Binding Methods | 4-5 | 1 | Phase 2 |
| Phase 4: Wire Up Loaders | 6-7 | 1 | Phase 3 |
| Phase 5: Remove Dead Code | 8 | 1 | Phase 4 |
| Phase 6: Testing & Validation | 9-10 | 1 | Phase 5 |

**Total:** 10 days, 1 developer

### Risk Factors

| Risk | Likelihood | Impact | Mitigation |
|------|-------------|--------|------------|
| LIEF API missing features | Medium | Medium | Implement custom fallbacks |
| Binding methods complex | High | High | Phase 3 buffer, skip if needed |
| Performance regression | Low | Medium | Benchmark before/after |

---

## File Reference

### Key Files

| File | Purpose |
|------|---------|
| [`crates/libmwemu/src/pe/lief/lief_pe.rs`](crates/libmwemu/src/pe/lief/lief_pe.rs) | Main LIEF wrapper |
| [`crates/libmwemu/src/pe/pe64.rs`](crates/libmwemu/src/pe/pe64.rs) | Legacy PE64 (source to migrate from) |
| [`crates/libmwemu/src/pe/pe32.rs`](crates/libmwemu/src/pe/pe32.rs) | Legacy PE32 (source to migrate from) |
| [`crates/libmwemu/src/emu/loaders.rs`](crates/libmwemu/src/emu/loaders.rs) | PE loading code to update |
| [`crates/libmwemu/src/emu/initialization.rs`](crates/libmwemu/src/emu/initialization.rs) | DLL initialization to update |
| [`crates/libmwemu/Cargo.toml`](crates/libmwemu/Cargo.toml) | Dependencies |

### Related Documentation

| Document | Purpose |
|----------|---------|
| [`DEVELOPMENT.md`](crates/libmwemu/src/pe/DEVELOPMENT.md) | Current state analysis |
| [`LIEF_ARCHITECTURE.md`](crates/libmwemu/src/pe/LIEF_ARCHITECTURE.md) | LIEF integration design |
| [`PLAN.md`](crates/libmwemu/src/pe/PLAN.md) | Implementation plan |

---

## Quick Reference: Method Mapping

| Old Method (PE64) | New Method (LiefPe) |
|-------------------|---------------------|
| `pe64.opt.image_base` | `pe64.image_base()` |
| `pe64.opt.address_of_entry_point` | `pe64.entry_point()` |
| `pe64.size_of_headers` | `pe64.size_of_headers()` |
| `pe64.sections` | `pe64.sections()` |
| `pe64_get_section_by_name(name)` | `pe64.get_section(name)` |
| `pe64_get_imports()` | `pe64.get_dependencies()` |
| `pe64_rva_to_offset(rva)` | `pe64.vaddr_to_offset(rva)` |
| `pe64_get_tls()` | `pe64.get_tls_callbacks()` |
| `pe64_apply_relocations()` | `pe64.apply_relocations()` |
| `pe64_read_file(path)` | `LiefPe::parse(path)` |
