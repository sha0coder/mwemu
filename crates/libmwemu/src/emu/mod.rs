use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::File,
    sync::{Arc, atomic::AtomicU32},
    time::Instant,
};

use crate::emu::decoded_instruction::DecodedInstruction;
use crate::emu::disassemble::InstructionCache;
use crate::emu::object_handle::HandleManagement;
use crate::maps::heap_allocation::O1Heap;
use crate::{
    api::banzai::Banzai,
    arch::OperatingSystem,
    config::Config,
    debug::breakpoint::Breakpoints,
    debug::definitions::{Definition, StoredContext},
    hooks::Hooks,
    loaders::elf::{elf32::Elf32, elf64::Elf64},
    loaders::macho::macho64::Macho64,
    loaders::pe::{pe32::PE32, pe64::PE64},
    maps::Maps,
    threading::context::ThreadContext,
    threading::global_locks::GlobalLocks,
    utils::colors::Colors,
    windows::structures::MemoryOperation,
    pe::{api_set_resolver::ApiSetResolver, lief::lief_pe::LiefPe, pe32::PE32},
};

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
mod display;
pub mod emu_context;
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
    pub maps: Maps, // virtual memory map (all allocations, stack, heap, code regions)
    pub base: u64,  // base address for code loading
    pub heap_addr: u64, // current heap base address
    pub heap_management: Option<Box<O1Heap>>, // O(1) heap allocator for managed allocations
    pub memory_operations: Vec<MemoryOperation>, // per-step memory read/write log for tracing

    // --- Instruction decoding & disassembly ---
    pub arch_state: ArchState, // architecture-specific decode/cache/formatter state
    pub last_decoded: Option<DecodedInstruction>, // last decoded instruction (arch-neutral)
    pub last_decoded_addr: u64,                   // address where `last_decoded` lived; needed
                                                  // for state dumps because `pc()` already
                                                  // reflects the *next* instruction (post-ret /
                                                  // post-branch / post-advance) and would print
                                                  // the wrong pc next to the last opcode.
    pub last_instruction_size: usize,
    pub pe64: Option<PE64>,
    pub pe32: Option<PE32>,
    pub rep: Option<u64>,
    pub tick: usize,
    pub trace_file: Option<File>,
    pub base: u64,
    pub formatter: IntelFormatter,
    pub heap_addr: u64,
    pub rng: RefCell<rand::rngs::ThreadRng>,

    // --- Platform & loaded binary ---
    pub os: OperatingSystem,      // target OS (set by loader / init)
    pub pe64: Option<PE64>,       // parsed PE64 for runtime import resolution & resources
    pub pe32: Option<PE32>,       // parsed PE32 for runtime import resolution & resources
    pub elf64: Option<Elf64>,     // parsed ELF64 (Linux x86_64 / AArch64)
    pub elf32: Option<Elf32>,     // parsed ELF32 (Linux x86)
    pub macho64: Option<Macho64>, // parsed Mach-O 64 (macOS AArch64), includes addr_to_symbol
    pub tls_callbacks: Vec<u64>,  // PE TLS callback addresses
    pub library_loaded: bool,     // flag for GDB to detect library load events

    // --- Thread management ---
    pub threads: Vec<ThreadContext>,
    pub current_thread_id: usize,  // Index into threads vec
    pub global_locks: GlobalLocks, // Critical section lock tracking
    pub instruction_cache: InstructionCache,
    pub definitions: HashMap<u64, Definition>,
    pub stored_contexts: HashMap<String, StoredContext>,
    pub entropy: f64,
    pub heap_management: Option<Box<O1Heap>>,
    pub last_error: u32,
    pub is_api_run: bool,
    pub is_break_on_api: bool, // this value will only be modify internally for python use case
    pub instruction_count: u64,
    pub fault_count: u32,
    pub handle_management: HandleManagement,
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
            ArchState::X86 {
                instruction_cache, ..
            } => instruction_cache,
            ArchState::AArch64 { .. } => panic!("x86_instruction_cache called on aarch64 emu"),
        }
    }

    /// Get the x86 instruction cache immutably.
    #[inline]
    pub fn x86_instruction_cache_ref(&self) -> &InstructionCache<iced_x86::Instruction> {
        match &self.arch_state {
            ArchState::X86 {
                instruction_cache, ..
            } => instruction_cache,
            ArchState::AArch64 { .. } => panic!("x86_instruction_cache_ref called on aarch64 emu"),
        }
    }

    /// Get the aarch64 instruction cache (panics on x86).
    #[inline]
    pub fn aarch64_instruction_cache(
        &mut self,
    ) -> &mut InstructionCache<yaxpeax_arm::armv8::a64::Instruction> {
        match &mut self.arch_state {
            ArchState::AArch64 {
                instruction_cache, ..
            } => instruction_cache,
            ArchState::X86 { .. } => panic!("aarch64_instruction_cache called on x86 emu"),
        }
    }

    /// Get the aarch64 instruction cache immutably.
    #[inline]
    pub fn aarch64_instruction_cache_ref(
        &self,
    ) -> &InstructionCache<yaxpeax_arm::armv8::a64::Instruction> {
        match &self.arch_state {
            ArchState::AArch64 {
                instruction_cache, ..
            } => instruction_cache,
            ArchState::X86 { .. } => panic!("aarch64_instruction_cache_ref called on x86 emu"),
        }
    }

    /// Get the x86 decoder position (panics on aarch64).
    #[inline]
    pub fn x86_decoder_position(&self) -> usize {
        match &self.arch_state {
            ArchState::X86 {
                decoder_position, ..
            } => *decoder_position,
            ArchState::AArch64 { .. } => panic!("x86_decoder_position called on aarch64 emu"),
        }
    }

    /// Set the x86 decoder position.
    #[inline]
    pub fn set_x86_decoder_position(&mut self, pos: usize) {
        match &mut self.arch_state {
            ArchState::X86 {
                decoder_position, ..
            } => *decoder_position = pos,
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
