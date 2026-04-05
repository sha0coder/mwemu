#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(clippy::assertions_on_constants)]

pub mod arch;
pub mod banzai;
pub mod breakpoint;
pub mod colors;
pub mod config;
pub mod console;
pub mod constants;
pub mod definitions;
pub mod elf;
pub mod emu;
pub mod emu_context;
pub mod engine;
pub mod gdb;
pub mod err;
pub mod exception;
pub mod hooks;
#[macro_use]
pub mod macros;
pub mod crit_state;
pub mod exception_type;
pub mod global_locks;
pub mod kuser_shared;
pub mod macho;
pub mod maps;
pub mod pe;
pub mod peb;
pub mod script;
pub mod serialization;
pub mod structures;
pub mod syscall;
pub mod thread_context;
pub mod threading;
pub mod tracing;

// Architecture-specific modules
pub mod aarch64;
pub mod x86;

// Platform API interception modules
pub mod api;

// Backwards-compatible re-exports
pub use aarch64::regs as regs_aarch64;
pub use api::linux as linuxapi;
pub use api::macos as macosapi;
pub use api::windows as winapi;
pub use syscall::windows::ntapi;
pub use x86::context;
pub use x86::eflags;
pub use x86::flags;
pub use x86::fpu;
pub use x86::regs as regs64;

// re-export the helper so the macro can reach it
pub use utils::{color_enabled, disable_color};

#[cfg(test)]
mod tests;
mod utils;
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
