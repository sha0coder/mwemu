#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(clippy::assertions_on_constants)]

pub mod banzai;
pub mod breakpoint;
pub mod colors;
pub mod config;
pub mod console;
pub mod constants;
pub mod context;
pub mod definitions;
pub mod eflags;
pub mod elf;
pub mod emu;
pub mod emu_context;
pub mod engine;
pub mod err;
pub mod exception;
pub mod flags;
pub mod fpu;
pub mod hooks;
#[macro_use]
pub mod macros;
pub mod crit_state;
pub mod exception_type;
pub mod global_locks;
pub mod kuser_shared;
pub mod maps;
pub mod ntapi;
pub mod pe;
pub mod peb;
pub mod regs64;
pub mod script;
pub mod serialization;
pub mod structures;
pub mod syscall;
pub mod thread_context;
pub mod threading;
pub mod tracing;
pub mod winapi;

// re-export the helper so the macro can reach it
pub use utils::color_enabled;

#[cfg(test)]
mod tests;
mod utils;
use config::Config;
use emu::Emu;

pub fn emu64() -> Emu {
    let mut emu = Emu::new();
    let mut cfg = Config::new();
    cfg.is_64bits = true;
    emu.set_config(cfg);
    emu.disable_ctrlc();
    //tracing::init_tracing("/tmp/mwemu-tracing.bin");
    emu
}

pub fn emu32() -> Emu {
    let mut emu = Emu::new();
    let mut cfg = Config::new();
    cfg.is_64bits = false;
    emu.set_config(cfg);
    emu.disable_ctrlc();
    //tracing::init_tracing("/tmp/mwemu-tracing.bin");
    emu
}
