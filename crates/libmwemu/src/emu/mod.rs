use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    sync::{atomic::AtomicU32, Arc},
    time::Instant,
};

use crate::emu::decoded_instruction::DecodedInstruction;
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

/// Architecture-specific instruction decoding and disassembly state.
/// Discriminated by target architecture so each variant carries only
/// the decode state relevant to its ISA.
pub enum ArchState {
    X86 {
        instruction: Option<iced_x86::Instruction>,
        formatter: iced_x86::IntelFormatter,
        instruction_cache: InstructionCache<iced_x86::Instruction>,
        decoder_position: usize,
    },
    AArch64 {
        instruction: Option<yaxpeax_arm::armv8::a64::Instruction>,
        instruction_cache: InstructionCache<yaxpeax_arm::armv8::a64::Instruction>,
    },
}

mod banzai;
mod call_stack;
mod config;
mod console;
pub mod decoded_instruction;
pub mod disassemble;
pub mod emu_context;
mod display;
mod exception_handlers;
mod execution;
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
    pub arch_state: ArchState,                         // architecture-specific decode/cache/formatter state
    pub last_decoded: Option<DecodedInstruction>,      // last decoded instruction (arch-neutral)
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

// --- ArchState accessors ---
impl Emu {
    /// Get the current x86 instruction (panics on aarch64).
    #[inline]
    pub fn x86_instruction(&self) -> Option<iced_x86::Instruction> {
        match &self.arch_state {
            ArchState::X86 { instruction, .. } => *instruction,
            ArchState::AArch64 { .. } => panic!("x86_instruction called on aarch64 emu"),
        }
    }

    /// Set the current x86 instruction.
    #[inline]
    pub fn set_x86_instruction(&mut self, ins: Option<iced_x86::Instruction>) {
        match &mut self.arch_state {
            ArchState::X86 { instruction, .. } => *instruction = ins,
            ArchState::AArch64 { .. } => panic!("set_x86_instruction called on aarch64 emu"),
        }
    }

    /// Get the x86 formatter (panics on aarch64).
    #[inline]
    pub fn x86_formatter(&mut self) -> &mut iced_x86::IntelFormatter {
        match &mut self.arch_state {
            ArchState::X86 { formatter, .. } => formatter,
            ArchState::AArch64 { .. } => panic!("x86_formatter called on aarch64 emu"),
        }
    }

    /// Get the x86 instruction cache (panics on aarch64).
    #[inline]
    pub fn x86_instruction_cache(&mut self) -> &mut InstructionCache<iced_x86::Instruction> {
        match &mut self.arch_state {
            ArchState::X86 { instruction_cache, .. } => instruction_cache,
            ArchState::AArch64 { .. } => panic!("x86_instruction_cache called on aarch64 emu"),
        }
    }

    /// Get the x86 instruction cache immutably.
    #[inline]
    pub fn x86_instruction_cache_ref(&self) -> &InstructionCache<iced_x86::Instruction> {
        match &self.arch_state {
            ArchState::X86 { instruction_cache, .. } => instruction_cache,
            ArchState::AArch64 { .. } => panic!("x86_instruction_cache_ref called on aarch64 emu"),
        }
    }

    /// Get the aarch64 instruction cache (panics on x86).
    #[inline]
    pub fn aarch64_instruction_cache(&mut self) -> &mut InstructionCache<yaxpeax_arm::armv8::a64::Instruction> {
        match &mut self.arch_state {
            ArchState::AArch64 { instruction_cache, .. } => instruction_cache,
            ArchState::X86 { .. } => panic!("aarch64_instruction_cache called on x86 emu"),
        }
    }

    /// Get the aarch64 instruction cache immutably.
    #[inline]
    pub fn aarch64_instruction_cache_ref(&self) -> &InstructionCache<yaxpeax_arm::armv8::a64::Instruction> {
        match &self.arch_state {
            ArchState::AArch64 { instruction_cache, .. } => instruction_cache,
            ArchState::X86 { .. } => panic!("aarch64_instruction_cache_ref called on x86 emu"),
        }
    }

    /// Get the x86 decoder position (panics on aarch64).
    #[inline]
    pub fn x86_decoder_position(&self) -> usize {
        match &self.arch_state {
            ArchState::X86 { decoder_position, .. } => *decoder_position,
            ArchState::AArch64 { .. } => panic!("x86_decoder_position called on aarch64 emu"),
        }
    }

    /// Set the x86 decoder position.
    #[inline]
    pub fn set_x86_decoder_position(&mut self, pos: usize) {
        match &mut self.arch_state {
            ArchState::X86 { decoder_position, .. } => *decoder_position = pos,
            ArchState::AArch64 { .. } => panic!("set_x86_decoder_position called on aarch64 emu"),
        }
    }

    /// Format an x86 instruction to a string using the Intel formatter.
    #[inline]
    pub fn x86_format_instruction(&mut self, ins: &iced_x86::Instruction) -> String {
        let mut output = String::new();
        match &mut self.arch_state {
            ArchState::X86 { formatter, .. } => {
                use iced_x86::Formatter as _;
                formatter.format(ins, &mut output);
            }
            ArchState::AArch64 { .. } => panic!("x86_format_instruction called on aarch64 emu"),
        }
        output
    }

    /// Format a `DecodedInstruction` to a human-readable string.
    ///
    /// Dispatches to `IntelFormatter` for x86 or `Display` for aarch64.
    #[inline]
    pub fn format_instruction(&mut self, ins: &DecodedInstruction) -> String {
        match ins {
            DecodedInstruction::X86(x86_ins) => self.x86_format_instruction(x86_ins),
            DecodedInstruction::AArch64(aarch64_ins) => format!("{}", aarch64_ins),
        }
    }
}
