# LIEF Runtime Integration Review and Agent Implementation Plan

Date: 2026-05-24
Repo: mwemu
Scope: current uncommitted LIEF/runtime PE changes

## Current Verification Evidence

Commands already run in this workspace:

- `cargo check -p libmwemu --locked`
  - Result: passes.
- `cargo test -p libmwemu --locked --no-default-features -- loaders::pe::lief`
  - Result: passes, 7 passed.
- `cargo test -p libmwemu --locked --no-default-features -- serialization`
  - Result: passes, 9 passed.
- `cargo test -p libmwemu --locked --no-default-features -- pe64_loader`
  - Result: 2 passed, 1 failed because `test/exe64win_mingw.bin` is missing.
- Full `cargo test -p libmwemu --locked --no-default-features`
  - Result: 234 passed, 11 failed, 3 ignored.
  - Failures are from missing test binaries, not compiler errors.
- `git diff --check`
  - Result: fails on `crates/libmwemu/src/loaders/pe/mod.rs:11: new blank line at EOF.`

## Why the Current Test Failures Occur

The failing tests expect fixtures under the repository top-level `test/` directory. Current `test/` contents do not include all required binaries. Missing fixtures observed from test names and failure logs:

- `test/exe32win_exception_handler.bin`
- `test/elf64lin_cpu_arithmetics1.bin`
- `test/elf64lin_cpu_arithmetics2.bin`
- `test/elf64lin_cpu_arithmetics3.bin`
- `test/elf64lin_cpu_arithmetics4.bin`
- `test/elf64lin_syscall64.bin`
- `test/exe32win_mingw.bin`
- `test/exe64win_mingw.bin`
- `test/sc64win_strgen.bin`

The failing code path often reports `crates/libmwemu/src/loaders/elf/elf32.rs:142` because `load_code()` probes multiple formats and `Elf32::is_elf32()` calls `File::open(filename).expect("file not found")` on a missing sample path. The root cause is still missing fixtures.

## High-Level Assessment

The change is a useful partial integration: LIEF code has been moved under `loaders/pe/lief`, `RuntimePe64` exists, `Emu.pe64` now stores `RuntimePe64`, and serialization compiles. However, the active PE64 load path still constructs `PE64::load(...)` and stores `RuntimePe64::Legacy(pe64)`. Therefore LIEF is not yet the default runtime backend for `load_pe64`; the integration is type-level, not behavior-level.

There are also duplicate canonical PE/LIEF modules still compiled under both `crate::pe::lief` and `crate::loaders::pe::lief`. This violates the stated architecture goal unless `crate::pe` is intentionally converted into compatibility re-exports only.

## Issues Found

### CRITICAL: LIEF runtime is not actually active in `load_pe64`

- File: `crates/libmwemu/src/emu/loaders/pe.rs`
- Evidence:
  - `load_pe64` still does `let mut pe64 = PE64::load(filename);`
  - End of function stores `self.pe64 = Some(RuntimePe64::Legacy(pe64));`
- Impact: The current runtime path always uses the legacy parser for main PE64 samples. `RuntimePe64::load()` exists but is unused. LIEF-specific fixes for RVA/VA conversion, delay-load parsing, and resource lookup are not exercised by normal PE64 loading.
- Required fix: Convert `load_pe64` to load `RuntimePe64` and use backend-neutral accessors/section snapshots.

### CRITICAL: Duplicate canonical LIEF modules remain compiled

- Files:
  - `crates/libmwemu/src/pe/mod.rs`
  - `crates/libmwemu/src/pe/lief/*`
  - `crates/libmwemu/src/loaders/pe/lief/*`
  - `crates/libmwemu/src/lib.rs`
- Evidence:
  - `src/pe/mod.rs` still declares `pub mod lief;`.
  - Tests run for both `loaders::pe::lief::tests::*` and `pe::lief::tests::*`.
  - `lib.rs` exposes `pub mod pe;`.
- Impact: Two LIEF implementations are compiled and tested, increasing maintenance risk and violating the architecture requirement to avoid duplicate canonical PE implementations.
- Required fix: Make `loaders::pe::lief` canonical. Remove old compiled `crate::pe::lief` or convert `crate::pe` to compatibility re-exports only. Do not keep copied code active in both places.

### WARNING: LIEF IAT and delay-load binding resolve imports by function name only

- File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`
- Evidence:
  - `iat_binding()` calls `kernel32::resolve_api_name(emu, &func_name)`.
  - `process_delay_load_entries()` calls `kernel32::resolve_api_name(emu, &func_name)`.
  - Both ignore `dll_name` even though it is available.
- Impact: If multiple loaded modules export the same symbol, LIEF binding can patch the IAT with the wrong module's export address.
- Required fix: Use `kernel32::resolve_api_name_in_module(emu, dll_name, func_name)` for normal imports and delay-load imports, after applying API-set normalization as needed.

### WARNING: LIEF relocation errors are silently discarded

- File: `crates/libmwemu/src/loaders/pe/runtime_pe64.rs`
- Evidence:
  - `RuntimePe64::apply_relocations()` uses `let _ = pe.apply_relocations(emu, base_addr);` for the LIEF variant.
- Impact: Relocation failure is invisible, causing later crashes that are difficult to diagnose.
- Required fix: At minimum log a warning with the error. Prefer changing the wrapper method to return `Result<(), LiefError>` or a project error type and handle it in the loader.

### WARNING: Serialization loses the runtime backend and may read stale disk contents

- File: `crates/libmwemu/src/serialization/pe64.rs`
- Evidence:
  - `From<SerializablePE64> for RuntimePe64` always returns `RuntimePe64::Legacy(...)`.
  - `RuntimePe64::Lief` serialization attempts `std::fs::read(lief_pe.file_path())` before falling back to `mapped_file_data()`.
- Impact: A LIEF-backed emulator state restores as legacy-backed. Reading from disk can serialize different bytes if the original file changed after loading.
- Required fix: Either explicitly document that serialized PE64 always restores to legacy, or add a backend discriminator. Prefer serializing `mapped_file_data()` for LIEF to preserve the loaded bytes.

### WARNING: `get_resources()` remains a stub while `get_resource()` is partially implemented

- File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`
- Evidence:
  - `collect_resources()` ignores its arguments and returns no resources.
  - `get_resources()` therefore returns an empty vector even when resources exist.
- Impact: Trait consumers that use enumeration get incorrect empty results. Tests do not currently catch this.
- Required fix: Implement resource enumeration or remove/mark the trait method unsupported with a clear error. Add parity tests.

### WARNING: LIEF delay-load parsing does not clearly handle VA-vs-RVA descriptor mode

- File: `crates/libmwemu/src/loaders/pe/lief/lief_pe.rs`
- Evidence:
  - `DelayLoadEntry` reads `attributes()` but the binding code does not use it.
  - Pointer fields are treated as RVAs by `rva_to_offset`.
- Impact: Delay-load descriptors whose attributes indicate VA-style pointers may be parsed incorrectly.
- Required fix: Match PE delay-load semantics. If `dlattrRva` is set, treat fields as RVAs; otherwise treat fields as VAs and convert with `vaddr_to_offset` or subtract image base as appropriate.

### WARNING: LIEF tests are shallow and skip silently when fixture is absent

- File: `crates/libmwemu/src/loaders/pe/lief/tests/binding_tests.rs`
- Evidence:
  - Tests call `LiefPe::load("test/exe64win_msgbox.bin")` and `return` if loading fails.
  - Tests mostly assert only that methods do not panic.
- Impact: CI can pass with no actual LIEF fixture coverage. Runtime behavior is not verified.
- Required fix: Use `tests::helpers::test_data_path`, assert fixture existence or mark tests ignored with a message, and add strict assertions for imports, dependencies, RVA conversion, resource lookup, relocations, and delay-load parsing.

### SUGGESTION: `map_dll_pe64` and initialization still use concrete `PE64`

- Files:
  - `crates/libmwemu/src/emu/loaders/pe.rs`
  - `crates/libmwemu/src/emu/initialization.rs`
- Evidence:
  - `map_dll_pe64` returns `(u64, PE64)`.
  - `Lib.pe64` is `crate::loaders::pe::pe64::PE64`.
- Impact: System DLL loading remains legacy-only. This may be intentional for now, but it should be explicit.
- Required fix: Either document that only the main image uses `RuntimePe64`, or convert DLL loading to `RuntimePe64` too.

### SUGGESTION: Direct PE64 field access remains in new code

- File: `crates/libmwemu/src/emu/initialization.rs`
- Evidence:
  - `dll.pe64.dos.e_lfanew` is used instead of `dll.pe64.get_pe_off()`.
- Impact: Minor encapsulation issue. It complicates future backend abstraction.
- Required fix: Use accessor methods where available.

### SUGGESTION: Clean diff hygiene

- File: `crates/libmwemu/src/loaders/pe/mod.rs`
- Evidence:
  - `git diff --check` reports a new blank line at EOF.
- Required fix: Remove the extra blank line so `git diff --check` passes.

## Implementation Plan for Coding Agent

### Phase 0 - Preserve and Verify Baseline

1. Run and save output:
   - `cargo check -p libmwemu --locked`
   - `cargo test -p libmwemu --locked --no-default-features -- loaders::pe::lief`
   - `cargo test -p libmwemu --locked --no-default-features -- serialization`
   - `git diff --check`
2. Confirm the known full-suite failures are fixture-missing failures only.
3. Do not claim any test passes without command output.

### Phase 1 - Remove Duplicate Canonical PE/LIEF Modules

Goal: only one active LIEF implementation under `crate::loaders::pe::lief`.

1. Search for active uses of `crate::pe::lief`, `crate::pe::pe64`, and `crate::pe::pe32`.
2. Move or re-export API-set resolver cleanly:
   - Preferred: move `api_set_resolver.rs` under `crates/libmwemu/src/loaders/pe/api_set_resolver.rs` or another architecture-approved module.
   - Update imports from `crate::pe::api_set_resolver::ApiSetResolver` to the new canonical path.
3. Change `crates/libmwemu/src/pe/mod.rs` into compatibility re-exports only, or remove it entirely if public compatibility is not required.
4. Ensure `src/pe/lief/*` is not compiled. Delete the duplicate files or remove module declarations and document deprecation.
5. Verify only `loaders::pe::lief::tests::*` runs, not duplicate `pe::lief::tests::*`.

### Phase 2 - Make `RuntimePe64` the Active Loader Backend

Goal: `load_pe64` should be able to load LIEF-backed PE64 images by default, with fallback if needed.

1. Add a backend-neutral section representation, for example:

```rust
pub struct RuntimeSection {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub size_of_raw_data: u32,
    pub characteristics: u32,
    pub data: Vec<u8>,
}
```

2. Add methods to `RuntimePe64`:
   - `size_of_image(&self) -> u32`
   - `headers(&self) -> Cow<'_, [u8]>` or `Vec<u8>`
   - `sections(&self) -> Vec<RuntimeSection>`
   - `pe_offset(&self) -> u32` and consistently name it.
3. Convert `load_pe64` from `PE64::load(filename)` to `RuntimePe64::load(filename)`.
4. Replace direct field accesses in `load_pe64`:
   - `pe64.opt.image_base` -> `pe64.image_base()`
   - `pe64.opt.address_of_entry_point` -> `pe64.entry_point()`
   - `pe64.opt.section_alignment` -> `pe64.section_alignment()`
   - `pe64.opt.size_of_headers` -> `pe64.size_of_headers()`
   - `pe64.dos.e_lfanew` -> `pe64.pe_offset()`
5. Update section mapping to use `RuntimeSection`.
6. Store `self.pe64 = Some(pe64)` without forcing `RuntimePe64::Legacy`.
7. Decide whether `map_dll_pe64` remains legacy-only. If yes, document it. If no, convert it to `RuntimePe64` and update `Lib` in `initialization.rs`.

### Phase 3 - Fix LIEF Binding Correctness

1. In `LiefPe::iat_binding`, replace name-only resolution with module-aware resolution:
   - `resolve_api_name_in_module(emu, dll_name, func_name)`.
2. In `process_delay_load_entries`, use the same module-aware resolver with `dll_name`.
3. Handle ordinal imports explicitly:
   - inspect the LIEF API for ordinal import flags/values,
   - either resolve by ordinal if existing resolver supports it or log and skip with an explicit warning.
4. Change relocation error handling:
   - Do not silently discard `LiefError`.
   - Return or log a warning containing path, base, RVA/offset if available.

### Phase 4 - Fix Delay-Load Descriptor Semantics

1. Use `DelayLoadEntry::attributes()` to detect RVA vs VA mode.
2. Implement helper conversion:

```rust
fn delay_ptr_to_offset(&self, value: u32, attrs: u32) -> Option<usize> {
    if attrs & 1 != 0 {
        self.rva_to_offset(value as u64).map(|x| x as usize)
    } else {
        self.vaddr_to_offset(value as u64).map(|x| x as usize)
    }
}
```

3. Apply this to DLL name, INT/name table, and IAT fields.
4. Add tests for both descriptor modes if fixtures exist; otherwise add a synthetic unit test around conversion helpers.

### Phase 5 - Resource Lookup and Enumeration Parity

1. Implement `get_resources()` or return a real unsupported error rather than an empty successful result.
2. Add parity tests comparing legacy `PE64::get_resource` with `LiefPe::get_resource` for the same fixture.
3. Verify type/name ID lookup, string type/name lookup, and language leaf handling.
4. Confirm whether returned tuple is file offset, RVA, or VA; document and enforce parity.

### Phase 6 - Serialization Contract

1. Decide the intended contract:
   - Option A: serialized PE64 always restores to legacy parser; document this clearly.
   - Option B: preserve backend with a `backend` field.
2. Prefer serializing `LiefPe::mapped_file_data()` over reading `file_path()` from disk.
3. If reading from disk remains, log a warning on fallback and add a test for deleted/or moved original files.

### Phase 7 - Tests and Fixtures

1. Add or restore missing fixtures under top-level `test/` if they are allowed in the repository:
   - `exe32win_exception_handler.bin`
   - `elf64lin_cpu_arithmetics1.bin` through `elf64lin_cpu_arithmetics4.bin`
   - `elf64lin_syscall64.bin`
   - `exe32win_mingw.bin`
   - `exe64win_mingw.bin`
   - `sc64win_strgen.bin`
2. If fixtures cannot be committed, mark those tests `#[ignore = "requires external fixture ..."]` or make them assert a clear skip reason through a helper.
3. Update LIEF tests to use `helpers::test_data_path` and stop silently returning on missing files.
4. Add specific tests for:
   - `RuntimePe64::load` choosing LIEF on a valid PE64.
   - Fallback to legacy on LIEF failure.
   - LIEF import binding uses module-specific resolution.
   - LIEF delay-load binding parses descriptor attributes.
   - LIEF resource lookup parity.
   - Runtime serialization roundtrip for `RuntimePe64::Legacy` and `RuntimePe64::Lief`.

### Phase 8 - Final Verification

Run these and paste outputs in the final report:

```powershell
cargo check -p libmwemu --locked
cargo test -p libmwemu --locked --no-default-features -- loaders::pe::lief
cargo test -p libmwemu --locked --no-default-features -- serialization
cargo test -p libmwemu --locked --no-default-features -- pe64_loader
git diff --check
```

If fixtures are restored or tests are marked correctly, also run:

```powershell
cargo test -p libmwemu --locked --no-default-features
```

On Apple Silicon hosts, follow `AGENTS.md` and use `--target x86_64-apple-darwin` for cargo check/test/build.

## Acceptance Criteria

- `RuntimePe64::load()` is used by the active PE64 loader or a clear config gate controls it.
- No duplicate active LIEF implementation remains under both `crate::pe::lief` and `crate::loaders::pe::lief`.
- LIEF import binding resolves by DLL + function name, not function name only.
- LIEF relocation errors are not silently ignored.
- Delay-load RVA/VA attributes are handled.
- Resource lookup has parity tests and enumeration is not a silent stub.
- Serialization behavior is explicit and tested.
- Missing fixture failures are resolved by adding fixtures or marking tests as fixture-dependent.
- `cargo check`, targeted tests, and `git diff --check` pass with logs.

## Ready-to-Use Agent Prompt

Implement the plan in `docs/lief-runtime-integration-action-plan.md`. Be strict: do not claim tests pass without logs. First remove duplicate active PE/LIEF modules so `crate::loaders::pe::lief` is canonical. Then make `RuntimePe64::load()` the active backend in `emu/loaders/pe.rs`, adding backend-neutral section/header accessors as needed. Fix LIEF IAT and delay-load binding to resolve imports by DLL + function name, handle delay-load RVA/VA attributes, and do not silently discard relocation errors. Implement or explicitly error unsupported resource enumeration, add parity tests for resources/imports/serialization, and address missing fixture failures by either adding fixtures or marking tests as fixture-dependent. Run `cargo check -p libmwemu --locked`, targeted LIEF/serialization/pe64_loader tests, full tests when fixtures are available, and `git diff --check`; include exact output in the final report.
