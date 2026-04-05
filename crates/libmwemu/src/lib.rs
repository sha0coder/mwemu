#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(clippy::assertions_on_constants)]

#[macro_use]
pub mod utils;

// Grouped modules
pub mod arch;
pub mod loaders;
pub mod threading;
pub mod exception;
pub mod windows;
pub mod debug;
pub mod api;

// Core modules
pub mod emu;
pub mod engine;
pub mod maps;
pub mod syscall;
pub mod serialization;

// Standalone modules
pub mod config;
pub mod err;
pub mod hooks;

// Backwards-compatible re-exports (arch)
pub use arch::aarch64::regs as regs_aarch64;
pub use arch::x86::context;
pub use arch::x86::eflags;
pub use arch::x86::flags;
pub use arch::x86::fpu;
pub use arch::x86::regs as regs64;

// Backwards-compatible re-exports (api/syscall)
pub use api::linux as linuxapi;
pub use api::macos as macosapi;
pub use api::windows as winapi;
pub use syscall::windows::ntapi;

// Re-exports for external crates
pub use debug::console;
pub use debug::gdb;
pub use debug::script;
pub use emu::emu_context;
pub use utils::color_enabled;

#[cfg(test)]
mod tests;
use arch::Arch;
use config::Config;
use emu::Emu;

pub fn emu64() -> Emu {
    let mut emu = Emu::new();
    let mut cfg = Config::new();
    cfg.arch = Arch::X86_64;
    emu.set_config(cfg);
    emu.disable_ctrlc();
    emu
}

pub fn emu32() -> Emu {
    let mut emu = Emu::new();
    let mut cfg = Config::new();
    cfg.arch = Arch::X86;
    emu.set_config(cfg);
    emu.disable_ctrlc();
    emu
}

pub fn emu_aarch64() -> Emu {
    let mut emu = Emu::new();
    let mut cfg = Config::new();
    cfg.arch = Arch::Aarch64;
    emu.set_config(cfg);
    emu.disable_ctrlc();
    emu
}
