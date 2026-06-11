# LIEF Runtime Integration Current Review and Follow-up Implementation Plan

Date: 2026-05-24
Repository: D:\Projects\mwemu
Scope reviewed: current uncommitted LIEF/runtime PE changes after the latest integration pass.

## Executive Summary

The current change improves the earlier state but is not a completed LIEF runtime integration. The duplicate active `crate::pe::{pe32, pe64, lief}` implementation has been removed from compilation and `crate::loaders::pe::lief` is now the canonical LIEF implementation. `Emu.pe64` now stores `RuntimePe64`, serialization has a backend discriminator, LIEF import binding attempts DLL-aware resolution, delay-load parsing has an initial RVA/VA helper, and missing fixture failures have been converted to ignored tests.

The central problem remains: the active PE64 loader is still legacy. In `crates/libmwemu/src/emu/loaders/pe.rs`, `load_pe64` currently uses:

```rust
let mut pe64 = RuntimePe64::load_legacy(filename);
```

This means the main PE64 runtime path is behaviorally legacy-only even though `RuntimePe64::load()` is LIEF-first. This was likely done because enabling `RuntimePe64::load()` caused enigma fixture regressions. That fallback is understandable, but it leaves the main acceptance criterion unmet. The next pass must either fix LIEF parity and switch the active path to `RuntimePe64::load()`, or add an explicit backend policy (`Legacy`, `Lief`, `Auto`) and document why legacy remains the default.

Do not claim the runtime is LIEF-backed until the active loader uses LIEF and the enigma/msgbox parity tests pass with exact logs.

## Current Verification Evidence

Latest reported verification from the implementation pass:

```text
cargo check -p libmwemu --locked
Result: passes.

cargo test -p libmwemu --locked --no-default-features -- loaders::pe::lief
Result: passes, 11 passed.

cargo test -p libmwemu --locked --no-default-features -- serialization
Result: passes, 10 passed.

cargo test -p libmwemu --locked --no-default-features -- pe64_loader
Result: passes, 2 passed, 1 ignored.

cargo test -p libmwemu --locked --no-default-features
Result after marking fixtures ignored: passes, 231 passed, 0 failed, 14 ignored.

git diff --check
Result: no whitespace errors reported; git emitted CRLF conversion warnings.
```

Important caveat: 14 ignored tests means fixture coverage is disabled. A passing full suite with ignored fixture tests is not proof that fixture-dependent PE, ELF, shellcode, and SSDT behavior is correct.

## High-Priority Findings

### 1. Active `load_pe64` bypasses LIEF-first loading

File: `crates/libmwemu/src/emu/loaders/pe.rs`

Current behavior:

```rust
let mut pe64 = RuntimePe64::load_legacy(filename);
```

Problem:

- The action plan required `RuntimePe64::load()` to become active.
- `RuntimePe64::load()` is LIEF-first with legacy fallback, but it is not called.
- Runtime execution of main PE64 samples is therefore still legacy-backed.
- Tests pass partly because the active runtime path avoids LIEF.

Required improvement:

Either fix LIEF parity and switch to:

```rust
let mut pe64 = RuntimePe64::load(filename);
```

or add an explicit backend policy:

```rust
pub enum Pe64Backend {
    Legacy,
    Lief,
    Auto,
}
```

Then load with:

```rust
let mut pe64 = RuntimePe64::load_with_backend(filename, self.cfg.pe64_backend)?;
```

Acceptance:

- A test can force legacy and prove `RuntimePe64::Legacy` is used.
- A test can force LIEF and prove `RuntimePe64::Lief` is used.
- A test can force auto and prove fallback behavior is logged and deterministic.
- Do not leave backend choice implicit.

### 2. LIEF runtime regression is hidden rather than fixed

Evidence from the previous pass:

- Switching `load_pe64` to LIEF-first caused `exe64win_enigma` and `benchmark64with_enigma` failures.
- The code was changed back to `load_legacy()`.

Problem:

The root cause is unknown. Possible causes include section byte mismatches, import IAT address basis mistakes, relocation differences, or VA/RVA confusion in LIEF APIs.

Required improvement:

Create a legacy-vs-LIEF parity harness that compares the same fixture loaded both ways before execution. Compare:

- image base;
- entry RVA;
- size of image;
- size of headers;
- section alignment;
- PE offset;
- section count;
- per-section name, RVA, virtual size, raw size, characteristics, and raw-byte hash;
- import DLL names, functions, ordinals, and normalized IAT RVA;
- relocation directory RVA/size and relocation entries;
- delay-load descriptors;
- resources when present.

Fixtures to cover:

- `exe64win_msgbox.bin`;
- `exe64win_enigma.bin` when available.

Acceptance:

- The parity harness prints exact mismatches.
- Enigma is not fixed by disabling LIEF silently.
- LIEF backend is not declared runtime-ready until parity passes.

### 3. LIEF relocation errors are warning-only

File: `crates/libmwemu/src/loaders/pe/runtime_pe64.rs`

Current behavior logs relocation errors but continues:

```rust
if let Err(e) = pe.apply_relocations(emu, base_addr) {
    log::warn!("LIEF relocation error for {}: {}", pe.file_path(), e);
}
```

Problem:

A relocation failure can corrupt execution while the loader reports success. This violates the requirement not to discard relocation failures.

Required improvement:

Return a result from relocation application:

```rust
pub fn apply_relocations(&mut self, emu: &mut Emu, base_addr: u64) -> Result<(), RuntimePeError>
```

Legacy can return `Ok(())` initially. LIEF must propagate errors. `load_pe64` must either hard-fail in strict LIEF mode or fallback in `Auto` mode with a warning that includes filename, image base, load base, relocation RVA/size, and the failing offset/RVA.

### 4. LIEF relocation parser treats malformed present relocations as success

File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`

Current patterns:

```rust
None => return Ok(delta),
```

and:

```rust
if block_size < 8 { break; }
```

Problem:

If a relocation directory exists but cannot be translated or parsed, this is not the same as no relocations. It should be an error in strict LIEF mode.

Required improvement:

Add explicit `LiefError` variants such as:

```rust
RelocationError(String),
UnsupportedRelocationType { reloc_type: u16, rva: u64 },
```

Only return success when relocations are truly absent or delta is zero. Return errors for present-but-invalid directories, out-of-bounds blocks, non-forward progress, truncated entries, and invalid block sizes.

### 5. Delay-load RVA/VA handling is incomplete

File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`

Current state:

- `process_delay_load_entries` uses `delay_ptr_to_offset()` for `delay_names` and `delay_iat`.
- `delay_load_binding` still calls `entry.dll_name(file_data, rva_to_offset)` and ignores descriptor attributes for the DLL name.
- Patch address calculation uses `base_addr + entry.delay_iat() + offset`, which is wrong for VA-mode descriptors.

Required improvement:

Introduce a descriptor-mode abstraction:

```rust
enum DelayPointerMode {
    Rva,
    Va,
}
```

Add helpers:

```rust
fn delay_ptr_to_rva(&self, value: u64, mode: DelayPointerMode) -> Option<u64>;
fn delay_ptr_to_offset(&self, value: u64, mode: DelayPointerMode) -> Option<usize>;
fn delay_ptr_to_patch_va(&self, value: u64, mode: DelayPointerMode, loaded_base: u64) -> Option<u64>;
```

Rules:

- RVA mode: file offset = `rva_to_offset(value)`, patch VA = `loaded_base + value`.
- VA mode: file offset = `vaddr_to_offset(value)`, patch VA = `loaded_base + (value - image_base)`.

Apply the mode to DLL name, delay name table, delay IAT, and patch address. Add synthetic tests for both modes.

### 6. LIEF IAT binding assumes `iat_address()` is an RVA

File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`

Current code:

```rust
let iat_rva = function.iat_address() as u64;
let patch_addr = base_addr + iat_rva;
```

Problem:

The code assumes LIEF returns an RVA. The earlier entrypoint bug showed LIEF APIs may return VA where this code expects RVA. This must be normalized and tested.

Required improvement:

Add:

```rust
fn normalize_lief_iat_address_to_rva(&self, value: u64) -> Option<u64> {
    if value >= self.image_base() {
        value.checked_sub(self.image_base())
    } else {
        Some(value)
    }
}
```

Use it before patching. Add parity tests comparing legacy `first_thunk` RVAs to normalized LIEF IAT values.

### 7. LIEF resource enumeration is lossy

File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`

Current behavior flattens resources and builds strings like `type:name`.

Problems:

- PE resource hierarchy is type -> name/id -> language.
- Numeric IDs and string names are conflated.
- It is not proven whether `data.offset()` is RVA, file offset, or resource-section offset.
- `ResourceInfo.rva` may be semantically wrong.

Required improvement:

Use a richer internal descriptor:

```rust
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

If public API changes are too invasive, keep the old `ResourceInfo` as a compatibility projection and use the richer type internally. Add parity tests against legacy `PE64::get_resource`.

### 8. `RuntimePe64::import_addr_to_dll_and_name` is incomplete for LIEF

File: `crates/libmwemu/src/loaders/pe/runtime_pe64.rs`

Current LIEF behavior returns only the function name:

```rust
pe.import_addr_to_name(addr)
```

Problem:

Legacy returns `dll!function`. LIEF runtime would regress diagnostics/API interception behavior.

Required improvement:

Implement:

```rust
pub fn LiefPe::import_addr_to_dll_and_name(&self, paddr: u64) -> String
```

Return the same string shape as legacy. Cover named imports and ordinal imports. Normalize IAT address basis.

### 9. Serialization backend should be typed and backward-aware

File: `crates/libmwemu/src/serialization/pe64.rs`

Current code:

```rust
pub backend: String,
```

Problems:

- Invalid strings are possible.
- Old serialized states may lack `backend`.
- `load_from_raw` for LIEF depends on temp-file parsing.

Required improvement:

Use a typed enum or optional enum:

```rust
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum SerializablePe64Backend {
    Legacy,
    Lief,
}

pub backend: Option<SerializablePe64Backend>
```

Default missing/unknown backend to legacy. Add tests for legacy, LIEF, fallback, and original-file-deleted serialization.

### 10. `LiefHeaderParser::from_bytes()` temp file parsing is fragile

File: `crates/libmwemu/src/loaders/pe/lief/lief_header_parser.rs`

Current behavior writes a `NamedTempFile`, records its path, consumes it with `into_file()`, and calls `PeBinary::parse(&temp_path)`.

Risk:

The temporary path lifecycle is subtle and platform-dependent. If LIEF has a parse-from-memory API, use it. If path parsing is required, keep the named tempfile alive through parse and copy raw bytes into `Arc<[u8]>` for later section access.

Acceptance:

Add a test that deserializes LIEF from raw bytes after the original path is deleted or intentionally invalid.

### 11. `LiefSectionManager` LRU eviction is incorrect

File: `crates/libmwemu/src/loaders/pe/lief/lief_section_manager.rs`

Current code computes `current_size` once and never updates it inside the eviction loop.

Required fix:

```rust
let mut current_size = cache.values().map(|v| v.len()).sum::<usize>();
while current_size + *new_size > *max_bytes {
    let Some(oldest) = access_order.pop() else { break; };
    if let Some(evicted) = cache.remove(&oldest) {
        current_size = current_size.saturating_sub(evicted.len());
    }
}
```

Add tests for partial eviction and access-order consistency.

### 12. Empty existing sections are not cached distinctly from missing sections

File: `crates/libmwemu/src/loaders/pe/lief/lief_section_manager.rs`

Current code returns `Some(Vec::new())` without caching when raw size is zero. The caller then reads from cache and can get `None`, which is later collapsed into an empty vec.

Required improvement:

Cache empty sections and distinguish missing section from empty section in tests.

### 13. Header copy can panic on malformed/truncated inputs

File: `crates/libmwemu/src/emu/loaders/pe.rs`

Current code:

```rust
let headers = pe64.get_headers();
pemap.memcpy(&headers, pe64.size_of_headers() as usize);
```

If `headers.len() < size_of_headers`, `memcpy` panics.

Required improvement:

Bound copy length and treat truncation as an error in strict mode:

```rust
let expected = pe64.size_of_headers() as usize;
let copy_len = headers.len().min(expected);
pemap.memcpy(&headers[..copy_len], copy_len);
if copy_len < expected { ... }
```

### 14. Permanent `#[ignore]` hides fixture tests even when fixtures return

Files include:

- `tests/isa/x86/exception_handler32.rs`
- `tests/loaders/elf64/elf64lin_cpu_arithmetics.rs`
- `tests/loaders/elf64/elf64lin_syscall64.rs`
- `tests/loaders/pe/mingw_tests.rs`
- `tests/loaders/pe/pe64_loader_tests.rs`
- `tests/os/ssdt_win64_tests.rs`
- `tests/shellcode/sc64win_strgen.rs`

Problem:

`#[ignore]` means these tests will not run automatically after fixtures are restored.

Required improvement:

Add helper:

```rust
pub fn optional_test_data_path(name: &str) -> Option<String> {
    let path = test_data_path(name);
    if std::path::Path::new(&path).is_file() {
        Some(path)
    } else {
        eprintln!("skipping fixture-dependent test: missing {}", path);
        None
    }
}
```

Replace fixture ignores with conditional early return. Keep `#[ignore]` only for slow tests.

### 15. LIEF tests are still shallow

File: `crates/libmwemu/src/loaders/pe/lief/tests/binding_tests.rs`

Problems:

- Resource enumeration test can pass with zero resources.
- Serialization test only checks basic fields.
- Import binding test does not verify patched IAT values.
- Runtime load test does not assert whether the backend is LIEF or legacy.

Required improvement:

Add exact assertions for known fixture contents. If the fixture lacks required features, use another fixture or mark with a clear conditional skip.

### 16. `RuntimeSection` should derive common test traits

File: `crates/libmwemu/src/loaders/pe/runtime_pe64.rs`

Required improvement:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSection { ... }
```

Add helpers for map size, copy size, and permission calculation if layering permits.

### 17. `RuntimePe64::get_headers()` and `get_section_ptr()` allocate unnecessarily

Current behavior returns `Vec<u8>` for both legacy and LIEF.

Required improvement:

Use `Cow<'_, [u8]>`:

```rust
pub fn headers(&self) -> Cow<'_, [u8]>;
pub fn section_data(&self, index: usize) -> Cow<'_, [u8]>;
```

Legacy returns borrowed data; LIEF returns owned data when needed.

### 18. `crate::pe` compatibility re-exports need documentation or removal

File: `crates/libmwemu/src/pe/mod.rs`

Current code re-exports `loaders::pe` modules. This is acceptable for compatibility, but new code should use canonical `crate::loaders::pe` paths.

Required improvement:

Add module docs saying `crate::loaders::pe` is canonical. Consider moving `api_set_resolver` under a canonical module and leaving a compatibility re-export.

### 19. `Cargo.lock` churn is large

`Cargo.lock` has large unrelated-looking churn. Minimize it to dependencies actually required by the manifest changes. Verify with `cargo check -p libmwemu --locked` after minimization.

## Detailed Implementation Plan

### Phase 0 - Preserve evidence

Run and save exact output:

```powershell
git status --short
git diff --stat
git diff --check
cargo check -p libmwemu --locked
cargo test -p libmwemu --locked --no-default-features -- loaders::pe::lief
cargo test -p libmwemu --locked --no-default-features -- pe64_loader
cargo test -p libmwemu --locked --no-default-features -- exe64win_enigma
```

Do not claim success without logs.

### Phase 1 - Add explicit backend policy

Add `Pe64Backend { Legacy, Lief, Auto }` to config or loader config. Default to `Legacy` until LIEF parity is proven. Add `RuntimePe64::load_with_backend`. Update `load_pe64` to use this policy rather than hard-coding `load_legacy()`.

### Phase 2 - Add legacy-vs-LIEF parity tests

Create parity tests comparing headers, sections, imports, IAT RVAs, relocations, resources, and mapped bytes for `exe64win_msgbox.bin` and `exe64win_enigma.bin` when fixtures are present. The test must print exact mismatches.

### Phase 3 - Fix VA/RVA normalization

Normalize LIEF entrypoint, IAT addresses, delay-load pointers, and patch addresses. Add tests proving values are RVAs where runtime mapping expects RVAs.

### Phase 4 - Make relocation errors strict

Refactor relocation APIs to return `Result`. Add detailed `LiefError` variants. Fail strict LIEF mode on malformed present relocations. Fallback only in `Auto` mode with explicit logs.

### Phase 5 - Complete delay-load support

Apply descriptor attributes to DLL name, INT/name table, IAT table, and patch address. Add RVA-mode and VA-mode synthetic tests.

### Phase 6 - Complete import compatibility

Implement `LiefPe::import_addr_to_dll_and_name`. Test exact parity with legacy.

### Phase 7 - Improve resources

Preserve resource hierarchy and semantics. Add resource lookup and enumeration parity tests. Document whether returned addresses are file offsets, RVAs, or VAs.

### Phase 8 - Improve serialization

Replace string backend with a typed optional enum. Add backward/fallback tests and raw-byte LIEF deserialization tests that do not depend on the original file path.

### Phase 9 - Replace fixture ignores

Add `optional_test_data_path` helper and replace missing-fixture `#[ignore]` with conditional skips. Keep ignores only for slow tests.

### Phase 10 - Fix section cache

Fix LRU size accounting, cache empty sections, and add cache behavior tests.

### Phase 11 - Reduce allocations and clean loader code

Use `Cow` for headers and section data. Restore useful mapping comments. Bound header copy length.

### Phase 12 - Minimize lockfile and clean docs

Reduce `Cargo.lock` churn. Mark older LIEF plan docs superseded or merge them. Run final verification.

## Final Verification Required

Run and paste exact output:

```powershell
git status --short
git diff --stat
git diff --check
cargo check -p libmwemu --locked
cargo test -p libmwemu --locked --no-default-features -- loaders::pe::lief
cargo test -p libmwemu --locked --no-default-features -- pe64_loader
cargo test -p libmwemu --locked --no-default-features -- exe64win_enigma
cargo test -p libmwemu --locked --no-default-features -- serialization
cargo test -p libmwemu --locked --no-default-features
```

## Ready-to-Use Coding Agent Prompt

Implement the follow-up plan in `docs/lief-runtime-integration-review-plan.md`. Be strict: do not claim tests pass without logs. First verify and report the current backend behavior in `emu/loaders/pe.rs`; do not hide that `load_pe64` currently uses `RuntimePe64::load_legacy`. Add an explicit PE64 backend policy (`Legacy`, `Lief`, `Auto`) or fully fix LIEF parity and switch the active path to `RuntimePe64::load()` with tests. Build a legacy-vs-LIEF parity harness for headers, sections, imports/IAT addresses, relocations, resources, and serialization. Fix LIEF VA/RVA normalization for entrypoint, IAT, delay-load descriptors, and patch addresses. Make LIEF relocation errors return/propagate instead of warning-only continuation. Implement `LiefPe::import_addr_to_dll_and_name`, improve resource enumeration hierarchy/semantics, and replace permanent missing-fixture `#[ignore]` attributes with conditional fixture skip helpers. Fix `LiefSectionManager` LRU/empty-section caching, make PE64 serialization use a typed backend discriminator, and minimize unrelated `Cargo.lock` churn. Run `cargo check -p libmwemu --locked`, targeted LIEF/serialization/pe64_loader/enigma tests, full tests, and `git diff --check`; paste exact output in the final report.