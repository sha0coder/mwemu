# mwemu

A multi-platform x86/x64/AArch64 binary emulator written in Rust. Supports PE, ELF, and Mach-O formats across Windows, Linux, and macOS.

## Building & Testing on Apple Silicon

**This is critical.** On Apple Silicon (arm64) hosts, always build and test targeting `x86_64-apple-darwin`:

```bash
cargo check --target x86_64-apple-darwin
cargo test --target x86_64-apple-darwin
cargo build --target x86_64-apple-darwin
```

The test suite produces worse results when the aarch64 host natively compiles and runs the x86_64 emulation code. When we instead tell the OS/CPU to run in x86_64 mode (via Rosetta), tests behave correctly. The root cause is not fully understood yet — it may be related to how the Rust compiler generates code differently on aarch64 vs x86_64 for our emulation logic.

**Never run bare `cargo test` or `cargo check` on an Apple Silicon host** without the `--target x86_64-apple-darwin` flag.

Ensure the target is installed:
```bash
rustup target add x86_64-apple-darwin
```

## Project Structure

```
crates/
  libmwemu/     Core emulation library
  mwemu/        CLI binary
  pymwemu/      Python bindings (PyO3)
  mwemu-test/   Integration tests
```

### libmwemu/src/ module layout

```
arch/           Arch enum, x86/ (regs, flags, FPU, eflags, context), aarch64/ (regs)
loaders/        Binary format parsers: elf/, macho/, pe/
threading/      Thread scheduler, thread context, crit_state, global_locks
exception/      Exception handlers (SEH/VEH/UEF), exception types
windows/        Windows internals: peb/, structures/, kuser_shared, constants
debug/          Console, script interpreter, GDB server, breakpoints, tracing, definitions
utils/          Macros, ANSI colors, helper utilities
api/            Platform API interception: windows/, linux/, macos/, banzai (unimplemented API stubs)
emu/            Core emulator state machine, execution, memory, registers, emu_context
engine/         x86 instruction dispatch (mnemonic -> handler)
maps/           Memory regions, heap allocation, TLB, permissions
syscall/        Syscall dispatch per platform (windows/, linux/, macos/)
serialization/  State serialization, minidump conversion
config.rs       Emulation configuration
err.rs          Custom error type
hooks.rs        Callback hook definitions
```
