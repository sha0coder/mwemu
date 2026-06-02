# LIEF Runtime Integration Change Review and Next Implementation Plan

Date: 2026-05-24
Repository: `D:\Projects\mwemu`
Review scope: current uncommitted LIEF/runtime PE64 integration changes, especially `crates/libmwemu/src/emu/loaders/pe.rs`, `crates/libmwemu/src/loaders/pe/runtime_pe64.rs`, `crates/libmwemu/src/loaders/pe/lief/*`, PE64 serialization, fixture test handling, and the new parity tests.

## Executive Summary

The current change moves in the right direction by adding an explicit PE64 backend policy, preserving legacy as the default, adding a `RuntimePe64` abstraction, converting many fixture-only `#[ignore]` tests to conditional skips, and adding an initial legacy-vs-LIEF parity harness.

However, this is **not complete enough to declare LIEF runtime parity or runtime readiness**. Several implemented fixes are shallow or actively incorrect. The highest-risk issues are:

1. `LiefPe::apply_relocations()` now contains **two relocation parsing loops**. The first loop is the intended strict path; the second old loop still runs afterward and can apply relocations a second time while silently ignoring malformed entries.
2. `load_pe64()` still has an unsafe header copy: `pemap.memcpy(&headers, pe64.size_of_headers() as usize)` can panic if `headers.len() < size_of_headers`.
3. `Auto` backend cannot safely fallback after a relocation error because mapping has already occurred; it currently logs and continues for non-`Lief` mode, which can leave corrupted mapped state.
4. Delay-load VA/RVA handling remains incomplete: DLL-name parsing still ignores descriptor attributes, VA-mode import/name table entries are not normalized, and no synthetic RVA/VA tests exist.
5. `LiefPe::import_addr_to_dll_and_name()` likely compares the wrong address basis. Legacy compares the runtime IAT **contents** after binding; current LIEF compares the IAT **slot RVA**. This will regress API interception diagnostics.
6. The parity harness is too shallow: resources are a stub, imports compare only DLL names, relocations compare only directory RVA/size, no raw-byte hashes, no delay-load descriptors, no mapped bytes, and no `exe64win_enigma.bin` coverage.
7. Serialization is typed but not truly backward-aware: old lowercase string values such as `"legacy"` / `"lief"` may fail to deserialize into the new enum.
8. `LiefHeaderParser::from_bytes()` still has a fragile temp-file lifetime and appears to emit `Can't open ...` messages in tests.
9. `Cargo.lock` and `Cargo.toml` churn is far larger than the LIEF runtime change requires.

Do not claim LIEF-backed runtime readiness until forced-LIEF loader execution and legacy-vs-LIEF parity pass for msgbox and enigma with exact logs.

## Current Behavior to Preserve in the Report

The active PE64 runtime path is now explicit policy-based:

```rust
let mut pe64 = RuntimePe64::load_with_backend(filename, self.cfg.pe64_backend);
```

`Config::default()` sets:

```rust
pe64_backend: Pe64Backend::Legacy
```

This is acceptable as a conservative default, but it means the normal runtime is still behaviorally legacy unless callers opt into `Lief` or `Auto`. The final report must state this plainly.

## Detailed Review Findings

### Critical 1 - Duplicate relocation application loop

File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`

Problem area: `LiefPe::apply_relocations()` around lines 870-952.

The function now does this:

1. Strict relocation loop with error returns.
2. Then a second old relocation loop that logs again and silently ignores malformed entries.

Consequences:

- DIR64 relocations can be applied twice: `original + delta + delta`.
- Strict errors are partially bypassed by the old loop structure.
- The code no longer has a single authoritative relocation parser.

Required change:

- Delete the entire second relocation loop.
- Keep only one strict parser.
- Make every present-but-invalid relocation directory return `Err`.
- Make `apply_relocation_entry()` return `Result<(), LiefError>` instead of silently returning on invalid target RVA, unreadable value, or missing mapped memory.
- Add tests with synthetic malformed relocation blocks.

Acceptance:

- A test fails before the fix because a relocation would be double-applied.
- After the fix, relocation is applied exactly once.
- Malformed present relocation directories return `Err` in LIEF strict mode.

### Critical 2 - Header copy can still panic

File: `crates/libmwemu/src/emu/loaders/pe.rs`

Current code:

```rust
let headers = pe64.get_headers();
pemap.memcpy(&headers, pe64.size_of_headers() as usize);
```

Problem:

If `headers.len() < pe64.size_of_headers()`, `memcpy` can panic. This remains explicitly called out in the original review plan and was not fixed.

Required change:

```rust
let expected = pe64.size_of_headers() as usize;
let copy_len = headers.len().min(expected);
pemap.memcpy(&headers[..copy_len], copy_len);
if copy_len < expected {
    match self.cfg.pe64_backend {
        Pe64Backend::Lief => panic!(...),
        Pe64Backend::Auto => { /* fallback before mapping, or hard error if rollback unavailable */ }
        Pe64Backend::Legacy => log::warn!(...),
    }
}
```

Better design: validate header size before creating maps so `Auto` can fallback without rollback.

Acceptance:

- Add a test using truncated/synthetic PE bytes where header cache is shorter than `SizeOfHeaders`.
- LIEF strict mode returns/panics with a clear error; it must not panic from slice bounds accidentally.

### Critical 3 - Auto fallback semantics are unsafe/incomplete

Files:

- `crates/libmwemu/src/loaders/pe/runtime_pe64.rs`
- `crates/libmwemu/src/emu/loaders/pe.rs`

Problems:

- `RuntimePe64::load_with_backend()` returns `Self`, not `Result`.
- Forced LIEF parse failure panics inside `load_with_backend()` instead of returning an error to the loader.
- Relocation errors occur after sections and headers are already mapped. In `Auto`, `load_pe64()` logs a warning and continues rather than falling back or failing.
- There is no deterministic Auto fallback test.

Required change:

Introduce a typed runtime error:

```rust
#[derive(Debug)]
pub enum RuntimePeError {
    LiefLoad { path: String, source: LiefError },
    Relocation { path: String, backend: Pe64Backend, source: LiefError },
    HeaderTruncated { path: String, expected: usize, actual: usize },
    // etc.
}
```

Change:

```rust
pub fn load_with_backend(path: &str, backend: Pe64Backend) -> Result<Self, RuntimePeError>
```

Implement a two-stage loader strategy:

1. Parse/select backend.
2. Validate headers and relocation metadata before mutating maps when possible.
3. For `Auto`, fallback to legacy before any irreversible map writes. If fallback after mapping is impossible, do not pretend fallback happened; return a clear error.

Acceptance:

- Test forced Legacy proves `RuntimePe64::Legacy` is used.
- Test forced LIEF proves `RuntimePe64::Lief` is used.
- Test Auto parse fallback is deterministic.
- Test Auto relocation validation fallback or hard error is deterministic and logged.

### Critical 4 - Delay-load VA/RVA handling remains incomplete

File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`

Problems:

- `delay_load_binding()` still does:

```rust
let rva_to_offset = |rva: u64| self.rva_to_offset(rva);
let dll_name = entry.dll_name(file_data, rva_to_offset)
```

This ignores descriptor attributes for the DLL name.

- `process_delay_load_entries()` normalizes descriptor `delay_names` and `delay_iat`, but not the hint/name pointer read from the delay INT table.
- `DelayLoadEntry::dll_name()` assumes `name_ptr()` is always an RVA.
- Old `delay_ptr_to_offset()` remains and should be removed or replaced by a descriptor-mode abstraction.
- No tests prove RVA-mode and VA-mode descriptors.

Required change:

Implement the plan's abstraction exactly:

```rust
enum DelayPointerMode { Rva, Va }

impl DelayPointerMode {
    fn from_descriptor_attrs(attrs: u32) -> Self { ... }
}

fn delay_ptr_to_rva(&self, value: u64, mode: DelayPointerMode) -> Option<u64>;
fn delay_ptr_to_offset(&self, value: u64, mode: DelayPointerMode) -> Option<usize>;
fn delay_ptr_to_patch_va(&self, value: u64, mode: DelayPointerMode, loaded_base: u64) -> Option<u64>;
```

Apply it to:

- DLL name pointer
- delay name/INT table pointer
- delay IAT table pointer
- each hint/name pointer read from the INT table
- patch address calculation

Acceptance:

- Add synthetic RVA-mode delay-load descriptor test.
- Add synthetic VA-mode delay-load descriptor test.
- Add at least one fixture parity assertion if a fixture contains delay-load descriptors.

### Critical 5 - `import_addr_to_dll_and_name()` address basis mismatch

Files:

- `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`
- `crates/libmwemu/src/loaders/pe/runtime_pe64.rs`
- `crates/libmwemu/src/emu/instruction_pointer.rs`

Legacy behavior:

`PE64::pe64_import_addr_to_dll_and_name(paddr)` compares `paddr` to the **contents of the IAT entry** after binding:

```rust
let addr = read_u64_le!(self.raw, off_addr);
if addr == paddr { return format!("{}!{}", dll, func); }
```

Current LIEF behavior:

```rust
let iat_rva = self.normalize_iat_to_rva(function.iat_address() as u64);
if iat_rva == paddr { ... }
```

This compares `paddr` to the IAT slot RVA, not to the resolved import address. In `instruction_pointer.rs`, the lookup is used when trying to execute an unmapped address. That address is usually the resolved API target or stale IAT content, not the IAT slot RVA.

Required change options:

Option A - Maintain a LIEF import binding map:

- Add a field to `LiefPe` such as `import_bindings: RwLock<HashMap<u64, String>>` mapping resolved address -> `dll!name`.
- In `iat_binding()` and delay-load binding, after resolving `real_addr`, insert `real_addr -> dll!func`.
- `import_addr_to_name()` and `import_addr_to_dll_and_name()` consult this map first.

Option B - Patch a shadow raw buffer like legacy:

- Store mutable raw bytes or a shadow IAT table in `LiefPe`.
- Patch the shadow buffer during binding so legacy lookup logic can be reproduced.

Acceptance:

- After binding msgbox imports, choose a known imported API, get its resolved address, and assert legacy and LIEF both return the same `dll!function` for that resolved address.
- Cover ordinal imports if a fixture exists or with a synthetic import table.

### High 6 - IAT normalization is only partially applied

File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`

Problems:

- `iat_binding()` normalizes `function.iat_address()`.
- `get_imports()` still stores raw `function.iat_address()`.
- `LiefPeReader::import_addr_to_name()` still compares raw `function.iat_address()`.
- The parity harness does not compare normalized function IAT RVAs against legacy `first_thunk` RVAs.

Required change:

- Centralize IAT normalization in one public/test-visible helper.
- Use it in `iat_binding()`, `get_imports()`, `import_addr_to_name()`, `import_addr_to_dll_and_name()`, and parity tests.

Acceptance:

- Parity test compares each legacy import function/ordinal and normalized IAT RVA to LIEF.

### High 7 - Resource enumeration is still lossy and parity test is a stub

Files:

- `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`
- `crates/libmwemu/src/loaders/pe/lief/error.rs`
- `crates/libmwemu/src/tests/loaders/pe/pe64_parity_tests.rs`

Problems:

- `ResourceInfo` still flattens type/name/language into a weak projection.
- `collect_resources()` loses PE hierarchy and language ID semantics.
- `data.offset()` semantics are not documented or validated.
- `compare_resources()` returns `Vec::new()` unconditionally, so `parity_resources` proves nothing.

Required change:

Add richer descriptor:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceEntryInfo {
    pub type_id: Option<u32>,
    pub type_name: Option<String>,
    pub name_id: Option<u32>,
    pub name: Option<String>,
    pub language_id: Option<u32>,
    pub data_rva: u64,
    pub file_offset: Option<u64>,
    pub size: u64,
}
```

- Preserve compatibility `ResourceInfo` as projection.
- Document `data_rva` and `file_offset` semantics.
- Implement actual resource parity against legacy `PE64::get_resource` where fixture resources exist.

Acceptance:

- Resource enumeration test must fail if resources exist but enumeration returns zero.
- Parity test must compare at least type/name/language/size and `get_resource()` results.

### High 8 - Parity harness is too shallow

File: `crates/libmwemu/src/tests/loaders/pe/pe64_parity_tests.rs`

Current coverage gaps:

- Only `exe64win_msgbox.bin`; no `exe64win_enigma.bin` coverage.
- Headers compare only a subset.
- Sections compare metadata only; no raw-byte hash and no mapped-byte comparison.
- Imports compare only DLL names; no functions, ordinals, IAT RVAs.
- Relocations compare only directory RVA/size; no entries.
- Resources are stubbed.
- Delay-load descriptors are not compared.
- Serialization test checks only image base, entry point, and section count.

Required change:

Create a normalized parity model:

```rust
struct Pe64ParityModel {
    image_base: u64,
    entry_rva: u64,
    size_of_image: u32,
    size_of_headers: u32,
    section_alignment: u32,
    pe_offset: u32,
    sections: Vec<SectionModel>,
    imports: Vec<ImportModel>,
    relocations: RelocationModel,
    delay_loads: Vec<DelayLoadModel>,
    resources: Vec<ResourceEntryInfo>,
}
```

Add comparison functions that print exact mismatches and include fixture name.

Acceptance:

- Run parity for `exe64win_msgbox.bin` and `exe64win_enigma.bin` if present.
- If `exe64win_enigma.bin` is missing, print conditional skip message, not `#[ignore]`.
- Mismatch output must include field, legacy value, LIEF value, and section/import/resource identifier.

### High 9 - Serialization typed enum is not backward-aware enough

File: `crates/libmwemu/src/serialization/pe64.rs`

Problems:

- The new enum likely serializes as `"Legacy"` / `"Lief"` by default, but prior code wrote lowercase strings `"legacy"` / `"lief"`.
- `#[serde(default)] Option<SerializablePe64Backend>` handles missing fields but not unknown/lowercase string fields.
- LIEF raw-byte deserialization may fail because `LiefHeaderParser::from_bytes()` temp file path is deleted before LIEF parses it.

Required change:

Option A:

```rust
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SerializablePe64Backend { Legacy, Lief }
```

Option B: custom deserializer accepting:

- missing -> legacy
- null -> legacy
- `"legacy"`, `"Legacy"` -> legacy
- `"lief"`, `"Lief"` -> lief
- unknown -> legacy with warning

Acceptance:

- Test old JSON/bitcode with missing backend defaults to legacy.
- Test old lowercase string `"legacy"` deserializes.
- Test old lowercase string `"lief"` attempts LIEF and falls back to legacy on parse failure.
- Test LIEF serialized from raw bytes works after original file is deleted or path is invalid.

### High 10 - `LiefHeaderParser::from_bytes()` temp file lifetime is still fragile

File: `crates/libmwemu/src/loaders/pe/lief/lief_header_parser.rs`

Current code:

```rust
let temp_path = temp_file.path().to_str()?.to_string();
let file = temp_file.into_file();
...
let pe = PeBinary::parse(&temp_path)?;
```

Problem:

`NamedTempFile::into_file()` consumes the named temp file. On platforms where the path is removed, LIEF cannot open `temp_path`. Test logs already show messages like `Can't open 'C:\Users\ADMIN\AppData\Local\Temp\.tmp...'`.

Required change:

- Keep `NamedTempFile` alive until after `PeBinary::parse(&temp_path)`.
- Flush/sync writes before parsing.
- Build `mapped_file` from the original `data` bytes or from the still-open file.
- Prefer LIEF parse-from-memory API if available.

Acceptance:

- Add test: load LIEF from raw bytes with filename pointing to a deleted/nonexistent path; assert backend is LIEF if LIEF parse-from-raw succeeds.
- No `Can't open temp` messages in passing tests.

### Medium 11 - LIEF section cache fix lacks tests and conflates empty with out-of-bounds

File: `crates/libmwemu/src/loaders/pe/lief/lief_section_manager.rs`

Problems:

- LRU size update was fixed, but no tests validate partial eviction or access order.
- `raw_size == 0` and `file_offset >= mapped_file.len()` both cache empty. A truncated/out-of-bounds section is not the same as a valid empty section.
- Existing methods return `Option<Vec<u8>>`, making it hard to represent malformed section data.

Required change:

- Add tests for LRU eviction: cache A/B/C under a tiny max and verify oldest eviction.
- Add tests for access-order refresh.
- Add tests distinguishing valid empty section from missing section.
- Consider returning `Result<Option<Vec<u8>>, LiefError>` internally for malformed/out-of-bounds sections.

### Medium 12 - `map_dll_pe64()` remains legacy-only

File: `crates/libmwemu/src/emu/loaders/pe.rs`

Current code:

```rust
pub fn map_dll_pe64(&mut self, filename: &str) -> (u64, PE64) {
    let mut pe64 = PE64::load(&filename.to_lowercase());
```

If the goal is runtime LIEF PE64 coverage beyond main EXEs, DLL mapping also needs a RuntimePe64 path or a documented reason to stay legacy-only.

Required change:

- Either document this explicitly as out of scope, or add `map_dll_pe64_runtime()` using `RuntimePe64` and backend policy.
- Update callers in `emu/initialization.rs` if safe.

### Medium 13 - `crate::pe` compatibility re-exports lack docs

File: `crates/libmwemu/src/pe/mod.rs`

Current code has compatibility re-exports but no docs.

Required change:

Add module docs:

```rust
//! Compatibility re-exports for older `crate::pe::*` users.
//! New loader code should use `crate::loaders::pe::*`.
```

### Medium 14 - Loader comments and duplicate booleans need cleanup

File: `crates/libmwemu/src/emu/loaders/pe.rs`

Problems:

- Useful mapping comments were removed in the diff.
- Conditions such as `self.cfg.emulate_winapi && self.cfg.emulate_winapi` are duplicated and should be replaced with a named helper/boolean.
- Some formatting is too wide and should be rustfmt-clean.

Required change:

- Restore explanatory comments about section virtual size vs raw size and `.didat` COW behavior.
- Replace duplicate boolean expression with a named variable or the intended real condition.
- Run `cargo fmt` if the repository expects it; otherwise keep formatting consistent.

### Medium 15 - Cargo.lock and Cargo.toml churn must be minimized

Files:

- `Cargo.lock`
- `crates/libmwemu/Cargo.toml`

Problem:

The diff updates many unrelated crates and adds large dependency graph churn. It also converts many libmwemu dependency declarations to workspace dependencies. That may be a separate workspace cleanup, but it is not necessary for LIEF parity and makes review risky.

Required change:

- Revert unrelated `Cargo.lock` version bumps unless required by manifest changes.
- Revert unrelated `Cargo.toml` workspace dependency normalization unless intentionally part of a separate commit.
- Keep only required dependencies for LIEF runtime work (`lief`, `memmap2`, `tempfile`, etc.) if they were not already present.
- Verify with `cargo check -p libmwemu --locked` after minimization.

## Implementation Plan for the Next Coding Agent

### Phase 0 - Preserve Current Evidence

Run and save exact output before changing code:

```powershell
git status --short
git diff --stat
git diff --check
cargo check -p libmwemu --locked
cargo test -p libmwemu --locked --no-default-features -- loaders::pe::lief
cargo test -p libmwemu --locked --no-default-features -- parity
cargo test -p libmwemu --locked --no-default-features -- pe64_loader
cargo test -p libmwemu --locked --no-default-features -- exe64win_enigma
cargo test -p libmwemu --locked --no-default-features -- serialization
```

Do not claim tests pass without logs.

### Phase 1 - Fix Immediate Correctness Bugs

1. Remove duplicate relocation loop in `LiefPe::apply_relocations()`.
2. Make `apply_relocation_entry()` return `Result<(), LiefError>`.
3. Bound header copy in `load_pe64()`.
4. Keep `NamedTempFile` alive in `LiefHeaderParser::from_bytes()` until after LIEF parses it.
5. Add regression tests for these four issues.

### Phase 2 - Make Backend Policy Typed and Fallible

1. Add `RuntimePeError`.
2. Change `RuntimePe64::load_with_backend()` to return `Result<Self, RuntimePeError>`.
3. Change relocation errors to preserve `LiefError` rather than `String`.
4. Define exact semantics:
   - `Legacy`: legacy only.
   - `Lief`: fail hard on LIEF parse/reloc/header/binding errors.
   - `Auto`: fallback only before mutating maps; otherwise return an explicit error.
5. Add forced backend tests through the real `Emu::load_pe64()` path.

### Phase 3 - Complete VA/RVA Normalization

1. Implement a single helper for LIEF address normalization:

```rust
fn normalize_lief_va_or_rva_to_rva(&self, value: u64) -> Option<u64>
```

2. Use it in:
   - entrypoint if needed
   - IAT binding
   - `get_imports()`
   - `import_addr_to_name()`
   - `import_addr_to_dll_and_name()`
   - parity harness
3. Add tests comparing legacy `first_thunk` RVAs to LIEF normalized IAT RVAs.

### Phase 4 - Complete Delay-load Support

1. Replace old helper with `DelayPointerMode`.
2. Apply mode to DLL name, INT/name table, IAT table, hint/name pointer values, and patch VA.
3. Add synthetic RVA-mode and VA-mode tests.
4. Add fixture-based delay-load parity if a fixture contains delay-load descriptors.

### Phase 5 - Restore Import Lookup Compatibility

1. Decide whether to patch a LIEF shadow raw buffer or maintain a runtime binding map.
2. Make `import_addr_to_name()` and `import_addr_to_dll_and_name()` match legacy behavior for resolved API addresses after binding.
3. Add tests that bind imports and compare lookup results between legacy and LIEF.

### Phase 6 - Improve Resource Semantics

1. Add `ResourceEntryInfo` preserving type/name/language hierarchy.
2. Keep `ResourceInfo` as compatibility projection.
3. Document offset semantics.
4. Implement real `compare_resources()` in the parity harness.
5. Add tests that fail when a resource-bearing fixture returns zero resources.

### Phase 7 - Expand Parity Harness

1. Build normalized parity models for legacy and LIEF.
2. Cover both `exe64win_msgbox.bin` and `exe64win_enigma.bin` when present.
3. Compare:
   - image base
   - entry RVA
   - size of image
   - size of headers
   - section alignment
   - PE offset
   - section count
   - section name/RVA/virtual size/raw size/characteristics/raw-byte hash
   - imports including DLL/function/ordinal/normalized IAT RVA
   - relocation directory and entries
   - delay-load descriptors
   - resources
   - serialization roundtrip backend and raw-byte availability
4. Mismatch output must be exact and actionable.

### Phase 8 - Serialization Backward Compatibility

1. Add serde lowercase rename or custom backend deserializer.
2. Test missing backend.
3. Test old lowercase backend strings.
4. Test unknown backend string defaults to legacy or errors intentionally with documentation.
5. Test original-file-deleted LIEF raw-byte deserialization.

### Phase 9 - Section Cache Tests

1. Add LRU eviction tests.
2. Add access-order tests.
3. Add empty-vs-missing tests.
4. Treat out-of-bounds/truncated section data as an error, not as empty.

### Phase 10 - Clean Scope and Docs

1. Minimize `Cargo.lock` churn.
2. Revert unrelated `Cargo.toml` workspace dependency migration unless required.
3. Add docs to `crate::pe` compatibility re-exports.
4. Document `map_dll_pe64()` as legacy-only or convert it to RuntimePe64.
5. Update old LIEF docs to point to the current canonical plan.

### Final Verification Required

Run and paste exact output:

```powershell
git status --short
git diff --stat
git diff --check
cargo check -p libmwemu --locked
cargo test -p libmwemu --locked --no-default-features -- loaders::pe::lief
cargo test -p libmwemu --locked --no-default-features -- parity
cargo test -p libmwemu --locked --no-default-features -- pe64_loader
cargo test -p libmwemu --locked --no-default-features -- exe64win_enigma
cargo test -p libmwemu --locked --no-default-features -- serialization
cargo test -p libmwemu --locked --no-default-features
```

## Ready-to-Use Coding Agent Prompt

Implement the follow-up fixes in `docs/lief-runtime-integration-change-review-followup-plan.md`. Be strict and do not claim tests pass without exact logs. First report the current PE64 backend behavior and state clearly that the default policy is still `Pe64Backend::Legacy`, so default runtime execution remains legacy unless forced otherwise. Fix the critical correctness bugs first: remove the duplicate relocation loop in `LiefPe::apply_relocations`, make relocation entry failures return `Result`, bound PE header copying, and keep the LIEF temp file alive during `from_bytes` parsing. Then make backend selection fallible/typed with a `RuntimePeError`, define safe `Legacy`/`Lief`/`Auto` semantics, and add tests that force each backend through the real loader. Complete VA/RVA normalization for imports and delay-loads, implement descriptor-mode delay-load parsing with RVA and VA tests, and fix LIEF `import_addr_to_dll_and_name` so it matches legacy lookup by resolved IAT contents after binding. Replace the resource stub with a hierarchy-preserving `ResourceEntryInfo` and real resource parity tests. Expand the parity harness to compare msgbox and enigma fixtures for headers, section raw-byte hashes, imports/functions/ordinals/IAT RVAs, reloc entries, delay-load descriptors, resources, mapped bytes, and serialization. Make PE64 serialization backward-compatible with old lowercase string backend values and test original-file-deleted LIEF raw-byte deserialization. Add LIEF section cache eviction/access-order/empty-vs-missing tests. Minimize unrelated `Cargo.lock` and `Cargo.toml` churn, document `crate::pe` re-exports, and either document or convert `map_dll_pe64` legacy-only behavior. Run the required final verification commands and paste exact output in the final report.
