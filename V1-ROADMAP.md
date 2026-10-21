# libmwemu v1.0 Roadmap

What needs to happen before cutting a stable 1.0 release.

Current state: 766+ tests, multi-platform (PE/ELF/Mach-O), x86/x64/AArch64,
kernel-mode driver emulation, syscall mode, and Windows system simulation
(PEB/TEB/LDR, heap, TLS, IAT binding, SEH/VEH). CI already runs `fmt`,
`clippy`, and a `test` matrix; `examples/` (01–04) exists.

> Counts re-audited on 2026-10-21 against `crates/libmwemu/src` (v0.26.0).
> The previous audit (2026-09-28) quoted 11 `unimplemented!()` — the real
> count was ~98 (most with message strings that the grep missed). All 98
> have now been eliminated.

---

## P0 — Must-have (stability contract)

### 1. No panics on guest code

The emulator must never abort on any guest binary, no matter how malformed.
The real crash surface is not the handful of `unimplemented!()` macros — it
is the mass of `unwrap()`/`expect()` on paths reachable from guest bytes.

Current audit (v0.26.0):

| Signal                | Total | Guest-reachable hot spots            |
|-----------------------|-------|--------------------------------------|
| `unimplemented!()`    | ~~98~~ **0** | All converted to log + graceful stop (v0.26.0) |
| `panic!(...)`         | 14    | 4 are arch-assertion guards (not guest-reachable), 10 in tests |
| `.unwrap()`           | ~858  | emu 57, engine 24→4*, maps 54→~45    |
| `.expect(...)`        | ~3096 | emu 63→~50, engine 8→0, maps 32→~25  |

\* engine remaining 4 are in commented-out debug code or infallible `try_into`.

**Work:**
- [x] Replace all `unimplemented!()` with log + graceful stop. DLL gateways
      use `log::error`, ISA handlers use `log::warn + return false`, API
      stubs log and return defaults. Done in v0.26.0 (98 → 0).
- [x] **engine/instructions/**: all `rep.unwrap()` → `unwrap_or(0)`, all
      `stack_pop*.unwrap()` → match+return false, all memory read `.expect()`
      → match+return false (pop, fadd, ret, pshufd, movs*, cmps*, stos*, scas*).
- [x] **maps/**: `mem64.rs` slice/file I/O unwraps → `unwrap_or`/log+return,
      `mod.rs` map lookups, free/dealloc, alloc → log+return 0.
- [x] **emu/operands.rs**: banzai map `.expect()` → match+return false.
- [x] **emu/instruction_pointer.rs**: `get_addr_name().expect()` → match+return false.
- [x] **emu/object_handle/hive_parser.rs**: `RegType` conversion → `?` propagation.
- [ ] **Headline gate:** remaining ~2700 `.expect()` in `api/` winapi stubs
      (stack reads). These are lower risk (stack is set up by gateway) but
      should eventually use a fallible helper. Hold the line with a CI grep
      gate on `engine/`, `maps/`, `emu/` core (see P2 #9).
- [ ] Unimplemented x86 instructions should raise `#UD` (exception), not
      `return false` + silent stop.

### 2. Public API stability

The `libmwemu` crate API is what downstream users (pymwemu, cmwemu, custom
tools) depend on. A v1 promises it won't break without a major bump.

**Work:**
- [ ] Review `pub` visibility — internals that leaked as `pub` should be
      `pub(crate)`.
- [ ] Stable entry points: `Emu::new`, `load_code`, `run`, `run_to`,
      `set_hook`, config fields. Document with `///` doc comments.
- [ ] Stabilize the `Hooks` callback signatures.
- [ ] `cfg` fields that control behavior should have builder methods or
      a `Config` struct with `Default`.

### 3. Error handling

The public signatures are already `Result`-based (`run`, `run_to`,
`run_until_ret`, and the per-arch variants all return
`Result<u64, MwemuError>`). The open work is making the internals honor
that contract instead of panicking — which is the same sweep as P0 #1.

- [ ] `run()` must propagate every internal failure as `Err`, never abort.
- [ ] Malformed PE/ELF/Mach-O input: return `Err` from `load_code`, not
      panic on short reads or missing sections.
- [ ] OOM in `maps.alloc()` / `create_map()`: propagate, don't `expect`.

### 4. Fuzz the input surface (new)

P0 is "don't crash on malformed input"; the only way to *prove* it is to
feed random bytes at the entry points. This also operationalizes the
Enigma-canary philosophy the rest of the suite already follows.

- [ ] `cargo-fuzz` (or `afl`) harness over `load_code` (PE/ELF/Mach-O) and
      over `run` on random code buffers.
- [ ] Run a short fuzz smoke pass in CI; keep a corpus of crashers as
      regression seeds.

---

## P1 — Should-have (completeness)

### 5. Core Windows API coverage

Triage the top unimplemented APIs hit by real samples and implement stubs.
Priority list (based on Enigma, Themida, and common malware):

- [x] **ole32**: `CoInitialize`, `CoCreateInstance`, `CoTaskMemAlloc/Free`
- [x] **advapi32**: `RegOpenKeyExA/W`, `RegQueryValueExA/W`, `RegCloseKey`
- [ ] **crypt32**: `CryptDecodeObjectEx`, `CertOpenStore` (stub)
- [x] **user32**: `wsprintfA/W`, `LoadStringA/W`, `CharNextA`
- [x] **kernel32**: `CreateFileMappingA/W`, `MapViewOfFile` (basic)
- [x] **ntdll**: `RtlGetVersion`, `NtQueryInformationProcess` (anti-debug)

### 6. 32-bit / 64-bit parity

- [ ] ntdll cross-module call interception (done for 64-bit, needs 32-bit)
- [ ] `_HEAP` structure emulation for 32-bit
- [ ] VirtualAllocEx for 32-bit (same commit-without-reserve fix)
- [ ] Test coverage: every 64-bit winapi test should have a 32-bit mirror

### 7. Syscall mode (`--syscall-mode`) robustness

- [ ] `_HEAP` struct: populate `BlocksIndex` function pointers or intercept
      CFG dispatch so ntdll heap code doesn't crash (see `docs/ANTI_EMU_HEAP.md`)
- [ ] TEB fields: `+0x1480` (FlsData), `+0x1858` (heap-related) used by
      ntdll internals
- [ ] More syscall stubs: `NtQueryVirtualMemory`, `NtProtectVirtualMemory`,
      `NtAllocateVirtualMemory`

---

## P2 — Nice-to-have (polish)

### 8. Documentation

- [ ] `///` doc comments on all public API types and methods
- [x] `examples/` directory (01_shellcode, 02_memory, 03_hooks,
      04_load_binary already exist)
- [ ] Add examples for the Python (pymwemu) and C (cmwemu) bindings
- [ ] `CHANGELOG.md`

### 9. CI improvements

CI already has `fmt`, `clippy`, and a `test` matrix. What's missing:

- [x] Make clippy **fail on warnings** (`-D warnings`) — done in CI and
      `make clippy`. MSRV was bumped 1.88 -> 1.95 so the auto-fixes (which use
      `is_multiple_of()` etc.) are valid. ~613 -> the residual backlog is
      grandfathered via crate-level `#![allow(...)]` blocks marked
      "clippy v1 burn-down backlog"; new warnings outside that set now fail.
- [ ] **Burn down** the grandfathered clippy allow list (search the codebase
      for "clippy v1 burn-down backlog") and remove entries as fixed. Keep
      `neg_cmp_op_on_partial_ord` — it guards intended NaN semantics in the
      float-compare handlers.
- [ ] Add the integration tests (test_linux, test_windows, test_syscall) to
      the CI matrix.
- [ ] Add the P0 grep gate: fail if `unwrap`/`expect`/`panic!` appears on
      guest-reachable paths (`emu/`, `engine/`, `maps/`), and a panic-count
      regression check so the number can only go down.

### 10. Performance

- [ ] Profile hot paths (instruction decode + dispatch, memory read/write)
- [ ] `get_addr_name()` has ~57 call sites and runs on RIP changes — confirm
      it's still on the hot path, then cache the lookups if so
- [ ] Benchmark: instructions-per-second on reference samples

---

## Non-goals for v1

- Full Windows kernel emulation (kernel-mode is experimental, driver-focused)
- GUI / debugger frontend (the console debugger `-c` is enough)
- Network I/O emulation (ws2_32 stubs are sufficient)
- Multi-threaded guest execution (single-threaded scheduler is fine)

---

## How to track progress

Each item above can be a GitHub issue. The TEST.md integration tests are the
gate: nothing ships if any step fails. The Enigma binary
(`test/exe64win_enigma.bin`) is the canary — it exercises heap, TLS, API
resolution, and anti-emulation checks harder than anything else in the test
suite.
