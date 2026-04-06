use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    sync::{atomic::AtomicU32, Arc},
    time::Instant,
};

use crate::emu::disassemble::InstructionCache;
use crate::maps::heap_allocation::O1Heap;
use crate::{
    api::banzai::Banzai,
    arch::OperatingSystem,
    debug::breakpoint::Breakpoints,
    utils::colors::Colors,
    config::Config,
    debug::definitions::{Definition, StoredContext},
    threading::global_locks::GlobalLocks,
    hooks::Hooks,
    maps::Maps,
    loaders::elf::{elf32::Elf32, elf64::Elf64},
    loaders::macho::macho64::Macho64,
    loaders::pe::{pe32::PE32, pe64::PE64},
    windows::structures::MemoryOperation,
    threading::context::ThreadContext,
};
use crate::emu::object_handle::HandleManagement;

mod banzai;
mod call_stack;
mod config;
mod console;
pub mod disassemble;
pub mod emu_context;
mod display;
mod exception_handlers;
mod execution;
mod execution_aarch64;
mod flags;
mod fls;
mod fpu;
mod fs;
mod initialization;
mod instruction_pointer;
mod loaders;
mod maps;
mod memory;
mod operands;
mod registers;
mod stack;
mod thread_context;
mod threading;
mod tls;
mod trace;
mod winapi;

pub mod object_handle;

pub struct Emu {
    // --- Configuration & display ---
    pub cfg: Config,
    pub colors: Colors,
    pub filename: String,

    // --- Memory & address space ---
    pub maps: Maps,            // virtual memory map (all allocations, stack, heap, code regions)
    pub base: u64,             // base address for code loading
    pub heap_addr: u64,        // current heap base address
    pub heap_management: Option<Box<O1Heap>>, // O(1) heap allocator for managed allocations
    pub memory_operations: Vec<MemoryOperation>, // per-step memory read/write log for tracing

    // --- Instruction decoding & disassembly ---
    // NOTE: x86 and aarch64 are bolted on separately with no shared abstraction.
    // step()/run() fork at the top via cfg.arch.is_aarch64().
    pub instruction: Option<iced_x86::Instruction>,  // current x86/x64 decoded instruction
    pub formatter: iced_x86::IntelFormatter,          // x86-only Intel syntax formatter
    pub instruction_cache: InstructionCache,           // decoded instruction cache (x86 only)
    pub decoder_position: usize,                       // slot index in instruction_cache
    pub last_instruction_size: usize,
    pub rep: Option<u64>,                              // REP prefix counter for string operations

    // --- Core execution state ---
    pub pos: u64,              // current instruction position counter (incremented each step)
    pub max_pos: Option<u64>,  // optional execution position limit
    pub tick: usize,           // global tick counter, used for thread scheduling
    pub is_running: Arc<AtomicU32>, // thread-safe flag for emulation running state
    pub now: Instant,          // timestamp of emulation start (wall-clock timing)
    pub force_break: bool,     // set by breakpoints, memory violations, etc. to stop execution
    pub force_reload: bool,    // trigger instruction re-decode
    pub run_until_ret: bool,   // step-over mode: run until next RET
    pub rng: RefCell<rand::rngs::ThreadRng>,

    // --- Platform & loaded binary ---
    pub os: OperatingSystem,       // target OS (set by loader / init)
    pub pe64: Option<PE64>,        // parsed PE64 for runtime import resolution & resources
    pub pe32: Option<PE32>,        // parsed PE32 for runtime import resolution & resources
    pub elf64: Option<Elf64>,      // parsed ELF64 (Linux x86_64 / AArch64)
    pub elf32: Option<Elf32>,      // parsed ELF32 (Linux x86)
    pub macho64: Option<Macho64>,  // parsed Mach-O 64 (macOS AArch64), includes addr_to_symbol
    pub tls_callbacks: Vec<u64>,   // PE TLS callback addresses
    pub library_loaded: bool,      // flag for GDB to detect library load events

    // --- Thread management ---
    pub threads: Vec<ThreadContext>,
    pub current_thread_id: usize,  // index into threads vec
    pub main_thread_cont: u64,     // main thread continuation/return address
    pub gateway_return: u64,       // return address from API gateway trampoline
    pub global_locks: GlobalLocks, // critical section/mutex tracking

    // --- API call interception ---
    pub hooks: Hooks,              // registered pre/post-instruction callback hooks
    pub skip_apicall: bool,        // stub/skip current API call
    pub its_apicall: Option<u64>,  // address of API call currently being dispatched
    pub is_api_run: bool,          // true while inside a Windows/system API handler
    pub is_break_on_api: bool,     // break on API calls (internal, for python interface)
    pub banzai: Banzai,            // auto-recovery: skip unimplemented APIs and continue

    // --- Debugging & breakpoints ---
    pub bp: Breakpoints,          // address, instruction, and memory breakpoints
    pub break_on_alert: bool,
    pub break_on_next_cmp: bool,  // pause before next CMP instruction
    pub break_on_next_return: bool, // pause before next RET instruction
    pub enabled_ctrlc: bool,
    pub running_script: bool,     // true while executing a debugger script
    pub exp: u64,                 // instruction-count breakpoint: spawn console when pos == exp
    pub definitions: HashMap<u64, Definition>,       // address annotations (duplicated from Config for serialization)
    pub stored_contexts: HashMap<String, StoredContext>, // named snapshots for breakpoint analysis

    // --- Tracing & statistics ---
    pub trace_file: Option<File>,  // optional file handle for instruction trace output
    pub instruction_count: u64,    // total instructions executed
    pub fault_count: u32,          // page faults / exceptions encountered
    pub entropy: f64,              // entropy measurement for polymorphic code detection
    pub last_error: u32,           // Win32 GetLastError value

    // --- Win32 resource management ---
    pub handle_management: HandleManagement, // file and object handle table
}
