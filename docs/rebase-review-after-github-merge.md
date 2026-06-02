# Rebase Review Report

Date: 2026-05-23
Repository: D:\Projects\mwemu
Branch: main
Reviewed range: sha0coder/main (315eac94) .. HEAD (d185c311)
Pre-rebase stack reference from reflog: d90bffb5..f98567fe

## Executive summary

The rebase is not safe as-is. The rebased branch does not currently compile, and several failures point directly to conflict-resolution/regression mistakes introduced while rebasing the local LIEF/VirtualAlloc work on top of the upstream source-tree refactor.

Most serious problems:

1. `crates/libmwemu/src/emu/mod.rs` contains a broken merge of the `Emu` struct: duplicated fields, removed/omitted fields, stale non-generic `InstructionCache`, and a field type (`IntelFormatter`) that is not imported and should not exist as a standalone `Emu` field in the current architecture.
2. `crates/libmwemu/Cargo.toml` reverted many dependency entries away from workspace dependencies and dropped required crates (`yaxpeax-arm`, `goblin`, `gdbstub`, `gdbstub_arch`, `flate2`). This breaks already-existing upstream modules.
3. LIEF PE support was added under a new top-level `crate::pe` module but the existing post-refactor PE loader still lives under `crate::loaders::pe`. The rebase duplicated PE loader implementations into `crates/libmwemu/src/emu/loaders.rs` instead of integrating into the existing split modules under `crates/libmwemu/src/emu/loaders/`. This also loses upstream loader fixes from `emu/loaders/pe.rs`.
4. `api_set_resolver` is used as `self.api_set_resolver` in initialization and serialization but there is no `api_set_resolver` field in `Emu`.
5. The new `LiefPeReader` trait does not provide all methods required by existing call sites (`import_addr_to_dll_and_name`), and `get_resource` is still a stub that always returns `None`, breaking `FindResourceA/W` behavior for PE64.
6. `git diff --check` reports trailing whitespace in new docs and Rust files.

## Evidence / verification performed

All checks were read-only except writing this report.

### Branch/rebase state

- `git status --short --branch` reported:
  - `## main...origin/main [ahead 103, behind 3]`
- `git reflog --date=iso --max-count=30` showed the rebase sequence:
  - `rebase (start): checkout sha0coder/main` at `315eac94`
  - `rebase (continue): Adding lief parsing to minimize the allocation` at `2e6aa88a`
  - `rebase (continue): Fixing VirtualAlloc stop early and remove Bound log` at `d185c311`
- `git log --oneline --left-right --cherry-pick --boundary sha0coder/main...HEAD` shows only two commits on top of `sha0coder/main`:
  - `2e6aa88a Adding lief parsing to minimize the allocation`
  - `d185c311 Fixing VirtualAlloc stop early and remove Bound log`

### Build verification

Command run:

```bash
cargo check -p libmwemu --locked --target-dir "C:\Users\ADMIN\AppData\Local\Temp\kilo\mwemu-check-target"
```

Result: failed. Relevant errors from `C:\Users\ADMIN\.local\share\kilo\tool-output\tool_e55bffbef001UG8CibUiySqMAX`:

- Missing dependencies caused by `crates/libmwemu/Cargo.toml` regression:
  - `error[E0433]: failed to resolve: use of unresolved module or unlinked crate gdbstub`
  - `error[E0433]: failed to resolve: use of unresolved module or unlinked crate gdbstub_arch`
  - `error[E0433]: failed to resolve: use of unresolved module or unlinked crate yaxpeax_arm`
  - `error[E0433]: failed to resolve: use of unresolved module or unlinked crate goblin`
  - `error[E0433]: failed to resolve: use of unresolved module or unlinked crate flate2`
- Broken duplicated `Emu` struct fields in `crates/libmwemu/src/emu/mod.rs`:
  - `error[E0124]: field base is already declared` at line 104, first declared at line 85
  - `error[E0124]: field heap_addr is already declared` at line 106, first declared at line 86
  - `error[E0124]: field pe64 is already declared` at line 111, first declared at line 99
  - `error[E0124]: field pe32 is already declared` at line 112, first declared at line 100
  - `error[E0124]: field heap_management is already declared` at line 127, first declared at line 87
  - `error[E0107]: missing generics for struct InstructionCache` at line 123
  - `error[E0412]: cannot find type IntelFormatter in this scope` at line 105
- Duplicated loader implementation missing macro:
  - `error: cannot find macro align_up in this scope` at `crates/libmwemu/src/emu/loaders.rs` lines 122, 162, 224, 267, 374, 419

### Whitespace verification

Command run:

```bash
git diff --check 315eac94..HEAD
```

Result: failed with trailing whitespace in:

- `crates/libmwemu/src/pe/DEVELOPMENT.md`
- `crates/libmwemu/src/pe/LIEF_ARCHITECTURE.md`
- `crates/libmwemu/src/pe/MIGRATION.md`
- `crates/libmwemu/src/pe/PLAN.md`
- `crates/libmwemu/src/pe/lief/lief_header_parser.rs`
- `crates/libmwemu/src/pe/lief/lief_section_manager.rs`
- `crates/libmwemu/src/pe/pe64.rs`

## Detailed issues and required modifications

### 1. Fix `crates/libmwemu/Cargo.toml` dependency regression

Problem:

The rebase changed `crates/libmwemu/Cargo.toml` from workspace dependencies back to hard-coded dependencies and removed several dependencies required by upstream code. Build errors confirm missing crates: `yaxpeax-arm`, `goblin`, `gdbstub`, `gdbstub_arch`, and `flate2`.

What to modify:

- Restore the upstream workspace dependency style for existing dependencies.
- Ensure these dependencies exist in `crates/libmwemu/Cargo.toml`:
  - `iced-x86 = { workspace = true, features = ["serde"] }`
  - `yaxpeax-arm.workspace = true`
  - `yaxpeax-arch.workspace = true`
  - `goblin.workspace = true`
  - `gdbstub.workspace = true`
  - `gdbstub_arch.workspace = true`
  - `flate2 = "1"`
  - all other pre-existing workspace dependencies that the rebase replaced with literals.
- Keep new LIEF-related dependencies if they are truly needed:
  - `lief`
  - `memmap2`
  - `nt-apiset`
  - `pelite`
  - `thiserror` if used by the new LIEF error layer

What to remove:

- Remove the hard-coded replacement of existing workspace dependencies that conflicts with the post-refactor workspace setup.

Why:

This is one of the first build blockers and directly resulted from applying an older pre-refactor manifest over the new workspace dependency layout.

### 2. Repair `Emu` struct merge in `crates/libmwemu/src/emu/mod.rs`

Problem:

`Emu` contains duplicate fields and lost fields. The current file has duplicate `base`, `heap_addr`, `pe64`, `pe32`, and `heap_management` fields. It also has stale `formatter: IntelFormatter` and `instruction_cache: InstructionCache` fields, while upstream uses `ArchState` for formatter/cache. Many fields still initialized in `Emu::new` are absent from the struct or are inconsistent with the struct.

What to modify:

- Start from the upstream `Emu` struct layout from `315eac94` and reapply only intentional LIEF additions.
- Keep all upstream fields:
  - `pos`, `max_pos`, `is_running`, `now`, `force_break`, `process_terminated`, `call_depth`, `ldr_init_done`, `force_reload`, `run_until_ret`
  - `main_thread_cont`, `gateway_return`
  - `hooks`, `skip_apicall`, `its_apicall`, `banzai`
  - `bp`, `break_on_alert`, `break_on_next_cmp`, `break_on_next_return`, `enabled_ctrlc`, `running_script`, `exp`
  - `section_handles`, `file_handles`, `known_dll_dir_handles`, `ssdt_pad_stack`
- Remove duplicate/stale fields introduced by the bad merge:
  - duplicate `base`
  - duplicate `heap_addr`
  - duplicate `pe64`
  - duplicate `pe32`
  - duplicate `heap_management`
  - standalone `formatter: IntelFormatter`
  - standalone `instruction_cache: InstructionCache`
- Add exactly one new field if API-set resolver is intended:
  - `pub api_set_resolver: Option<ApiSetResolver>`
- Import only the needed LIEF/API-set types.
- Decide the intended type of `pe64` and `pe32` before editing dependent code:
  - safest short-term path: leave `Emu.pe64` as the existing `loaders::pe::pe64::PE64` and `Emu.pe32` as existing `loaders::pe::pe32::PE32`, then use LIEF only in sidecar code until the whole loader is migrated.
  - larger migration path: change `pe64` to `LiefPe` only after implementing all compatibility methods and updating serialization/call sites.

What to add:

- Initialization for `api_set_resolver: None` in `Emu::new` and `From<SerializableEmu> for Emu` if the field is kept.

Why:

The current struct cannot compile, and preserving upstream fields is critical for GDB, SSDT, thread scheduling, serialization, and syscall code.

### 3. Do not duplicate PE loader methods in `crates/libmwemu/src/emu/loaders.rs`

Problem:

The upstream refactor split loader implementations into submodules under `crates/libmwemu/src/emu/loaders/`. The rebase added old-style `load_pe32`, `map_dll_pe64`, `load_pe64`, and `load_elf64` methods directly into `crates/libmwemu/src/emu/loaders.rs`. This conflicts with the existing modular implementation and loses upstream fixes.

Examples of upstream fixes in `crates/libmwemu/src/emu/loaders/pe.rs` that the duplicated implementation misses:

- local `align_up!` macro
- `pick_pe64_dll_base`
- `.didat` write-permission handling
- use of virtual size for PE section map size and bounded raw copy length
- relocation application after mapping for all PE64 load paths
- protection against DLL loads clobbering `self.base` and entry PC
- SSDT/LdrInitializeThunk-specific eager-IAT and LDR-entry handling
- updated map path detection using `windows/x86/`, `windows/x86_64/`, and `windows/aarch64/`

What to modify:

- Remove the duplicated PE/ELF loader methods from `crates/libmwemu/src/emu/loaders.rs`.
- Integrate any LIEF-specific PE work into `crates/libmwemu/src/emu/loaders/pe.rs` instead.
- Keep ELF code in `crates/libmwemu/src/emu/loaders/elf.rs`, not duplicated in `loaders.rs`.
- If LIEF should replace only some parsing paths, add a carefully-scoped helper in `emu/loaders/pe.rs` rather than an entire parallel implementation.

What to remove:

- Remove unused imports from `emu/loaders.rs` introduced only for duplicated loader code, such as `lief::generic::Section`, `LiefPe`, old PE imports, and `Permission` if no longer used there.

Why:

The current duplicate methods cause `align_up!` errors and risk silently bypassing the upstream loader fixes even after compilation is restored.

### 4. Reconcile top-level `crate::pe` with existing `crate::loaders::pe`

Problem:

The rebase added a new `crates/libmwemu/src/pe/` module with its own `pe32.rs` and `pe64.rs`, while the current architecture already has PE loaders under `crates/libmwemu/src/loaders/pe/`. This creates two PE implementations and confusing imports:

- existing: `crate::loaders::pe::{pe32::PE32, pe64::PE64}`
- new: `crate::pe::{pe32::PE32, pe64::PE64, lief::*}`

What to modify:

Choose one of these approaches:

#### Recommended short-term approach

- Keep the existing PE parser/loader under `crate::loaders::pe` as the authoritative PE implementation.
- Move/keep only LIEF-specific code under a clearer module name, for example `crate::loaders::pe::lief` or `crate::pe_lief`, instead of adding duplicate `crate::pe::pe32` and `crate::pe::pe64`.
- Update new imports to refer to the existing PE types or to the LIEF sidecar module explicitly.
- Remove duplicate `crates/libmwemu/src/pe/pe32.rs` and `crates/libmwemu/src/pe/pe64.rs` if they are only old copies.

#### Larger migration approach

- If `crate::pe` is intended to become the new canonical PE subsystem, then migrate all existing `crate::loaders::pe` call sites, tests, resource handling, binding, serialization, and docs in one coherent change.
- Do not leave both systems half-active.

Why:

The current half-migration is a rebase smell and makes the codebase ambiguous, brittle, and hard to compile.

### 5. Implement or remove `api_set_resolver` integration

Problem:

`initialization.rs` and `serialization/emu.rs` write `self.api_set_resolver`, but `Emu` has no such field. Also, API-set handling already exists in `api/windows/common/kernel32.rs` and `api/windows/winapi64/kernel32/loader.rs` for some cases, so there is risk of duplicate or inconsistent resolution logic.

What to modify:

- If keeping `ApiSetResolver`:
  - add `pub api_set_resolver: Option<ApiSetResolver>` to `Emu` exactly once.
  - initialize it to `None` in `Emu::new` and deserialization.
  - ensure `ApiSetResolver` is loaded only for Windows x64 paths where `apisetschema.dll` exists.
  - wire it into the existing `api/windows/winapi64/kernel32/loader.rs` / resolver code rather than the old `winapi/winapi64/kernel32/mod.rs` copy.
- If not keeping it yet:
  - remove the `self.api_set_resolver` assignments and calls.
  - keep the existing `kernel32_common::is_api_set_contract` fallback behavior.

What to add:

- A test or at minimum a smoke path for resolving one known API-set contract to the expected host DLL using `maps/windows/x86_64/apisetschema.dll`.

Why:

Current code cannot compile because the field is missing. If added incorrectly, it can conflict with existing API-set fallback routing.

### 6. Do not add old monolithic `crates/libmwemu/src/winapi/winapi64/kernel32/mod.rs`

Problem:

The rebase added `crates/libmwemu/src/winapi/winapi64/kernel32/mod.rs`, a large old-style monolithic kernel32 module. The current architecture uses `crates/libmwemu/src/api/windows/winapi64/kernel32/` with split modules (`loader.rs`, `resolver.rs`, `memory/`, etc.), and `lib.rs` re-exports `api::windows` as `winapi`. The added file is not the correct integration point and contains stale imports (`crate::constants`, `crate::peb`) and old behavior.

What to modify:

- Remove `crates/libmwemu/src/winapi/winapi64/kernel32/mod.rs` unless there is an explicit module path that still requires it.
- Port any unique, intentional changes into the split modules under `crates/libmwemu/src/api/windows/winapi64/kernel32/`.
- In particular, if API-set handling is needed, add it to existing `loader.rs` and/or `resolver.rs`, not to the old monolith.

Why:

Keeping the old file encourages divergence and future accidental imports. It is also a clear sign the rebase applied pre-refactor code to post-refactor layout.

### 7. Restore PE64 resource behavior or do not switch `FindResourceA/W` to LIEF yet

Problem:

`FindResourceA/W` now import `LiefPeReader` and call `get_resource` through the trait. But the trait default implementation returns `None`, and `LiefPe` does not override it. That means if `Emu.pe64` becomes a `LiefPe`, PE64 resources will stop working.

What to modify:

Choose one:

- Keep `Emu.pe64` as the existing `loaders::pe::pe64::PE64` for now, so existing `PE64::get_resource` continues to work.
- Or implement real resource lookup in `LiefPe::get_resource`, porting behavior from existing `PE64::locate_resource_data_entry` / `PE64::get_resource`.

What to add:

- A regression test for PE64 `FindResourceA/W` or direct resource lookup covering numeric type/name and string type/name cases.

Why:

A parser migration must preserve existing Windows API behavior, especially resource lookup.

### 8. Add missing LIEF compatibility methods before changing `Emu.pe64` type

Problem:

Existing code calls methods such as `import_addr_to_dll_and_name`, while the new `LiefPeReader` only provides `import_addr_to_name`. `emu/instruction_pointer.rs` still calls `self.pe64.as_ref().unwrap().import_addr_to_dll_and_name(addr)`.

What to modify:

- If `Emu.pe64` remains `loaders::pe::pe64::PE64`, no immediate trait work is required.
- If migrating to `LiefPe`, implement at least:
  - `import_addr_to_dll_and_name`
  - correct `import_addr_to_name` behavior for bound and unbound IAT entries
  - resource lookup
  - delay-load table handling
  - any parser fields currently accessed through `PE64` in serialization, loader, resource, and API code

Why:

Without compatibility methods, the migration will either not compile or silently lose API interception behavior.

### 9. Fix LIEF RVA/VA conversion and delay-load parsing

Problem:

`LiefHeaderParser::vaddr_to_offset` treats values below image base as raw file offsets. The new code frequently passes RVAs to this method. For RVA-to-file-offset, values below image base should be interpreted as RVAs, not returned unchanged.

Also, `DelayLoadEntry::rva_to_off` is hardcoded to return `None`, so `LiefPe::delay_load_binding` cannot read DLL names and effectively does not process delay-load descriptors correctly.

What to modify:

- Implement correct `rva_to_offset` by section table:
  - if the input is an RVA, find section where `virtual_address <= rva < virtual_address + max(virtual_size, sizeof_raw_data)` and return `pointerto_raw_data + (rva - virtual_address)`.
  - handle header RVAs separately when `rva < size_of_headers`.
- Make `vaddr_to_offset` explicitly subtract image base only for true VAs.
- Replace `DelayLoadEntry::rva_to_off` with a conversion using `self.rva_to_offset` or pass a converter closure into `dll_name`.
- Add bounds checks based on raw file size and section raw data size.

Why:

Incorrect RVA conversion breaks relocations, imports, delay-load, and resources.

### 10. Preserve upstream PE mapping fixes when integrating LIEF

Problem:

The duplicated loader maps sections using `max(virtual_size, size_of_raw_data)` and copies the entire raw pointer if non-empty. Upstream intentionally changed this to map by virtual size and copy only the fitting raw bytes. The duplicated code can create oversized maps and section overlap bugs.

What to modify:

- Preserve upstream logic from `crates/libmwemu/src/emu/loaders/pe.rs`:
  - use `map_sz = virtual_size` when non-zero, else raw size
  - `copy_len = min(size_of_raw_data, map_sz, ptr.len())`
  - force `.didat` writable
  - apply relocations after the image is mapped

Why:

The rebased code risks regressing the exact loader bugs fixed in upstream commits.

### 11. Resolve `Cargo.lock` consistently after dependency fixes

Problem:

Running `cargo check` after the current manifest can try to update `Cargo.lock`. That is a symptom of manifest/lock mismatch.

What to modify:

- After fixing `crates/libmwemu/Cargo.toml`, run:
  - `cargo check -p libmwemu --locked`
- If it fails only because the lockfile legitimately needs entries for new dependencies (`lief`, `nt-apiset`, `pelite`, etc.), update `Cargo.lock` once with normal cargo resolution and include the resulting lockfile changes.
- Do not leave `Cargo.lock` modified by failed/incorrect manifest states.

Why:

The repo should build reproducibly with `--locked` after the dependency set is finalized.

### 12. Clean trailing whitespace

Problem:

`git diff --check 315eac94..HEAD` reports trailing whitespace in new docs and Rust code.

What to modify:

- Remove trailing whitespace from all reported files:
  - `crates/libmwemu/src/pe/DEVELOPMENT.md`
  - `crates/libmwemu/src/pe/LIEF_ARCHITECTURE.md`
  - `crates/libmwemu/src/pe/MIGRATION.md`
  - `crates/libmwemu/src/pe/PLAN.md`
  - `crates/libmwemu/src/pe/lief/lief_header_parser.rs`
  - `crates/libmwemu/src/pe/lief/lief_section_manager.rs`
  - `crates/libmwemu/src/pe/pe64.rs`

Why:

This is low-risk cleanup and keeps diffs hook-friendly.

## Suggested implementation order

1. Fix `crates/libmwemu/Cargo.toml` to restore required dependencies and workspace dependency style.
2. Fix `crates/libmwemu/src/emu/mod.rs` by restoring upstream `Emu` struct layout and adding only `api_set_resolver` if needed.
3. Remove duplicate old loader methods from `crates/libmwemu/src/emu/loaders.rs`; integrate LIEF work into `emu/loaders/pe.rs` only.
4. Remove or quarantine the old monolithic `crates/libmwemu/src/winapi/winapi64/kernel32/mod.rs` and port any needed logic into `api/windows/winapi64/kernel32/`.
5. Decide and implement the PE migration boundary:
   - short-term: existing `loaders::pe::PE32/PE64` remain authoritative; LIEF is sidecar/header-only.
   - long-term: implement missing compatibility in `LiefPe` before changing `Emu.pe64`/`pe32` types.
6. Fix `api_set_resolver` field and initialization or remove the integration until it is complete.
7. Fix LIEF RVA conversion and delay-load parsing.
8. Restore PE64 resource lookup behavior if `LiefPe` is used by runtime code.
9. Clean trailing whitespace.
10. Run verification commands listed below.

## Required verification after implementation

Run these commands and inspect the logs; do not claim success without logs:

```bash
git status --short
cargo check -p libmwemu --locked
cargo test -p libmwemu --locked --no-default-features
cargo check --workspace --locked
git diff --check
```

If `Cargo.lock` legitimately needs an update after dependency fixes, run the non-locked cargo command once to update it, then rerun all locked checks.

On Apple Silicon hosts, follow AGENTS.md and use `--target x86_64-apple-darwin` for cargo check/test/build.

## Prompt for coding agent

Use this prompt for an implementation agent:

```text
You are working in D:\Projects\mwemu after a problematic rebase. Do not take shortcuts. Fix the rebase regressions described below and prove the result with command logs.

Context:
- Branch main is rebased on sha0coder/main at 315eac94, with local commits 2e6aa88a and d185c311.
- Current `cargo check -p libmwemu --locked` fails.
- The most important project rule is to preserve the post-refactor architecture: PE loaders live under `crates/libmwemu/src/emu/loaders/` and `crates/libmwemu/src/loaders/pe/`; Windows APIs live under `crates/libmwemu/src/api/windows/...`. Do not reintroduce old monolithic modules.

Tasks:

1. Restore `crates/libmwemu/Cargo.toml` to the workspace dependency style used by upstream. Ensure required upstream dependencies are present: `yaxpeax-arm.workspace = true`, `goblin.workspace = true`, `gdbstub.workspace = true`, `gdbstub_arch.workspace = true`, `flate2 = "1"`, plus the existing workspace dependencies. Keep new LIEF-related dependencies only if the code uses them (`lief`, `memmap2`, `nt-apiset`, `pelite`, `thiserror`).

2. Repair `crates/libmwemu/src/emu/mod.rs`:
   - Start from the upstream `Emu` struct layout.
   - Remove duplicate/stale fields: duplicate `base`, duplicate `heap_addr`, duplicate `pe64`, duplicate `pe32`, duplicate `heap_management`, standalone `formatter`, and standalone non-generic `instruction_cache`.
   - Preserve upstream fields such as `pos`, `max_pos`, `is_running`, `now`, `force_break`, `process_terminated`, `call_depth`, `ldr_init_done`, `force_reload`, `run_until_ret`, `main_thread_cont`, `gateway_return`, `hooks`, `skip_apicall`, `its_apicall`, `banzai`, `bp`, `break_on_alert`, `break_on_next_cmp`, `break_on_next_return`, `enabled_ctrlc`, `running_script`, `exp`, `section_handles`, `file_handles`, `known_dll_dir_handles`, and `ssdt_pad_stack`.
   - Add `api_set_resolver: Option<ApiSetResolver>` only if you keep the API-set resolver integration, and initialize it everywhere `Emu` is constructed/deserialized.

3. Remove the duplicated old PE/ELF loader methods from `crates/libmwemu/src/emu/loaders.rs`. Integrate any LIEF changes into `crates/libmwemu/src/emu/loaders/pe.rs` while preserving upstream fixes: `pick_pe64_dll_base`, `.didat` writable mapping, virtual-size section mapping, bounded raw copy length, relocation after mapping, and not clobbering `self.base` for DLL loads.

4. Do not keep the old monolithic `crates/libmwemu/src/winapi/winapi64/kernel32/mod.rs`. Remove it if it is not used, and port any intentional logic into the split modules under `crates/libmwemu/src/api/windows/winapi64/kernel32/` (`loader.rs`, `resolver.rs`, etc.).

5. Reconcile the new `crates/libmwemu/src/pe/` LIEF code with the existing `crates/libmwemu/src/loaders/pe/` code. Prefer the safe short-term approach: keep existing `loaders::pe::PE32/PE64` as the runtime `Emu.pe32/pe64` types and treat LIEF as sidecar/header-only until compatibility is complete. If you change `Emu.pe64` to `LiefPe`, implement all missing compatibility first, including `import_addr_to_dll_and_name`, real `get_resource`, correct imports/delay-load binding, and serialization behavior.

6. Fix or remove `api_set_resolver` integration. If kept, add the field to `Emu`, initialize it to `None` in `Emu::new` and deserialization, load it only when `apisetschema.dll` exists, and wire it through the existing split kernel32 loader/resolver logic.

7. Fix LIEF RVA/VA conversion and delay-load parsing if LIEF remains active:
   - `rva_to_offset` must treat input as RVA and convert via section table and headers.
   - `vaddr_to_offset` must only subtract image base for true VAs.
   - `DelayLoadEntry::rva_to_off` must not always return `None`; use real section conversion.

8. Preserve PE64 resource behavior. Do not route `FindResourceA/W` through a trait method that always returns `None`. Either keep existing `PE64::get_resource` or implement real `LiefPe::get_resource` parity.

9. Remove trailing whitespace reported by `git diff --check`.

Verification required before finishing:
- `git status --short`
- `cargo check -p libmwemu --locked`
- `cargo test -p libmwemu --locked --no-default-features`
- `cargo check --workspace --locked`
- `git diff --check`

If `Cargo.lock` legitimately needs to change after dependency fixes, update it once with cargo, then rerun the locked commands. Include exact command output in the final response. If any command fails, do not claim success; report the exact failure and fix it.
```
