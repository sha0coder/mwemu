# x86/AArch64 Parity Debt

Remaining x86-only code that is either deferred or architecturally
x86-specific by design.

---

## Deferred

### 1. Serialization (`serialization/emu.rs`) — Low Priority

The entire `SerializableEmu` struct and its `From<&Emu>` impl are x86-only.
Calling `dump_to_file()` or deserializing on aarch64 will panic.

- `emu.x86_instruction()` — panics
- `emu.regs().clone()` — panics
- `emu.pre_op_regs()` / `emu.post_op_regs()` — panics
- `emu.flags()` — panics
- `emu.fpu().clone()` — panics

**Fix:** Add `ArchSerializableState` enum with x86 and aarch64 variants.
Serialize aarch64 registers (x0-x30, sp, pc, NZCV) and current instruction.

### 2. GDB server (`debug/gdb/`) — Deferrable

Entirely x86: `target.rs`, `registers.rs`, `mod.rs` all use `regs()`,
`flags()`, `fpu()` without guards. ~12 unguarded call sites.

**Fix:** Add aarch64 register set support in GDB protocol (x0-x30, sp, pc,
CPSR/NZCV). This is Step 13 in the plan.

---

## Architecturally x86-specific (not bugs)

These are x86-only by design and not reachable during aarch64 emulation.

| File | Reason |
|------|--------|
| `emu/operands.rs` | x86 instruction operand decoding; aarch64 handlers don't use it |
| `emu/display.rs` `featured_regs32/64` | x86 display methods; `featured_regs_aarch64()` exists, callers dispatch |
| `emu/execution.rs` `step_single_threaded` | Deprecated, x86-only by design |
| `emu/stack.rs` `push32/pop32` | 32-bit x86 stack ops; aarch64 uses `push64/pop64` (already fixed) |
| `emu/winapi.rs`, `emu/tls.rs`, `emu/fls.rs` | Windows API emulation, x86/x64-only |
| `emu/call_stack.rs` | Architecture-neutral, no `regs()` calls |
| `emu/memory.rs` `memory_operand_to_address` | x86 operand syntax parser; guarded with early panic on aarch64 |

---

## Previously fixed (this refactor)

| Item | Fix |
|------|-----|
| `emu/stack.rs` push64/pop64 | Now uses `sp()`/`set_sp()`/`pc()` — works on both arches |
| `emu/exception_handlers.rs` | Uses `pc()`, returns early on aarch64 |
| `emu/memory.rs` trace logging | All `regs().rip` replaced with `pc()` |
| `emu/memory.rs` operand parsing | Guarded with arch check |
| `emu/trace.rs` capture_pre_op/post_op | aarch64 register snapshots implemented |
| `emu/trace.rs` write_to_trace_file | aarch64 reg diff via `RegsAarch64::diff()` |
| `debug/console.rs` | Full aarch64 support: registers, flags, stack, radare2 |
| `debug/script.rs` | All commands arch-dispatched |
| `debug/definitions.rs` | Uses `pc()`, aarch64 register lookup |
| `debug/tracing.rs` | `TraceRecord::capture()` dispatches to aarch64 |
| `threading/scheduler.rs` | Threading enabled for aarch64 |
| `pymwemu` | `get_reg`/`set_reg` arch-dispatched, added `get_pc`/`set_pc`/`get_sp`/`set_sp` |
