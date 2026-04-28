# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
# Build (release)
make all                             # handles macOS ARM cross-compilation automatically
cargo build --release                # or directly
# On macOS ARM, add: --target x86_64-apple-darwin

# Tests — downloads test fixtures automatically on first run
make tests
cargo test --package libmwemu        # debug build tests
cargo test --release --package libmwemu  # release build tests (faster, some tests require this)

# Run a single test
cargo test -p libmwemu <test_name> --release -- --nocapture
# For #[ignore] tests:
cargo test -p libmwemu <test_name> --release -- --ignored --nocapture

# Run the CLI
cargo run --release -- -f test/exe64win_msgbox.bin -6          # 64-bit PE
cargo run --release -- -f test/exe64win_msgbox.bin -6 --ssdt   # SSDT (real ntdll) mode
cargo run --release -- -f test/exe64win_msgbox.bin -6 --ssdt -c 1 -V 200000  # verbose + pos limit

# Python bindings (Linux x86 only, or via pip on Mac)
make pytests
```

## Workspace Structure

- `crates/libmwemu` — core emulator library (the main codebase)
- `crates/mwemu` — CLI binary wrapping libmwemu
- `crates/pymwemu` — Python bindings via pyo3
- `crates/mwemu-test` — CPU instruction regression tests (parallel, reads `.test` files)
- `maps/windows/x86_64/` — real Windows DLL files used for emulation (not shipped in repo, downloaded by `make tests`)
- `test/` — binary samples used by integration tests (downloaded by `make tests`)

## Core Architecture

### The `Emu` struct (`crates/libmwemu/src/emu/mod.rs`)

Everything lives in `Emu`. It holds all emulator state: registers (`arch_state`), virtual memory (`maps`), loaded PE structures (`pe64`/`pe32`), configuration (`cfg`), debugging state, and thread management. Key fields:

- `maps: Maps` — the virtual address space; all memory read/write goes through this
- `arch_state: ArchState` — either `X86` (iced-x86 decoder + formatter) or `AArch64` (yaxpeax)
- `ldr_init_done: bool` — after LdrInitializeThunk completes in SSDT mode, API calls dispatch to Rust stubs instead of real DLL machine code
- `pos: u64` — monotonically-incrementing instruction counter (used by `-V` position limit)

### Execution Loop (`crates/libmwemu/src/emu/execution/`)

`decode_and_execute()` fetches code from `maps`, decodes with iced-x86 (x86) or yaxpeax (aarch64), then calls `engine::emulate_instruction()`. The `engine/instructions/` directory has one file per x86 mnemonic.

### Windows API Dispatch (`crates/libmwemu/src/emu/instruction_pointer.rs`)

When RIP lands in a known DLL range, the emulator intercepts it. With `ldr_init_done = false`, it tries to guess the API from the DLL export table and calls the Rust stub. With `ldr_init_done = true` (SSDT mode post-init), it pops the return address, calls `winapi64::gateway()`, and sets `force_break` to return control.

### Windows API Stubs (`crates/libmwemu/src/api/windows/winapi64/` and `winapi32/`)

Each DLL has a subdirectory (e.g. `kernel32/`, `ntdll/`) with per-function files. The `gateway()` function in `mod.rs` dispatches by address.

### NT Syscalls (`crates/libmwemu/src/syscall/windows/syscall64/mod.rs`)

`syscall64::gateway()` matches `rax` against `WIN64_NT*` constants from `windows/constants.rs` and dispatches to handlers in `memory.rs`, `process.rs`, `registry.rs`, `sync.rs`, `system.rs`. Unhandled syscalls return `STATUS_NOT_IMPLEMENTED`.

### PEB/LDR Management (`crates/libmwemu/src/windows/peb/peb64/`)

- `bootstrap.rs` — `init_peb_teb_empty()` for SSDT mode, `init_peb()` for normal mode
- `ldr.rs` — `Flink` iterator, `get_module_base()`, `dynamic_link_module()`, `rebuild_ldr_lists()`

The LDR lists use sentinel-based termination (`ldr_addr + 0x10/0x20/0x30` are sentinels, not circular). `dynamic_link_module` appends entries at the tail (before the sentinel). `rebuild_ldr_lists` is called after `iat_binding` in the SSDT path so that all `.pe` maps exist before the list is reconstructed.

### SSDT Mode (`--ssdt`)

Enabled by `cfg.emulate_winapi = true`. Flow:
1. `init_win32_mem64` loads only ntdll (returns early without loading kernelbase/kernel32)
2. Target PE is loaded without IAT binding
3. `call64(LdrInitializeThunk)` runs real ntdll machine code via syscalls
4. After completion: `ldr_init_done = true` → `iat_binding` (loads missing DLLs, detects invalid PE bases from `NtMapViewOfSection` stubs) → `rebuild_ldr_lists`

The ntdll CFG patch in `initialization.rs` replaces `RtlFailFast2`'s `mov edx,0xc0000409` with `jmp +0x0c` (jump over the `call NtTerminateProcess`, landing on `add rsp,0x88; ret`).

### Memory Maps (`crates/libmwemu/src/maps/`)

`Maps` wraps a slab allocator. Each region has a name (e.g. `"ntdll.pe"`, `"kernelbase.text"`, `"stack"`, `"peb"`). `name_map: AHashMap<String, usize>` is the primary lookup. PE sections are named `"{dll_name}{section_name}"`. The circular-import guard in `load_library` checks whether `"{name}.pe"` already exists to avoid re-entrant loading.

## Key Conventions

- All code comments must be in English.
- `WIN64_NT*` constants in `windows/constants.rs` must match the actual syscall numbers in the real `maps/windows/x86_64/ntdll.dll`. Verify against binary if adding new ones.
- `log::trace!()` for verbose debug output; `log_orange!(emu, ...)` for syscall/API tracing (respects `cfg.verbose`).
- Test fixtures live in `test/` and are checked by path from `CARGO_MANIFEST_DIR` in `tests/helpers.rs`. Slow integration tests are `#[ignore]`; run them with `--release -- --ignored`.
