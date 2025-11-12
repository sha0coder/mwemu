use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    sync::{atomic::AtomicU32, Arc},
    time::Instant,
};

use iced_x86::{Instruction, IntelFormatter};

use crate::emu::disassemble::InstructionCache;
use crate::{
    banzai::Banzai,
    breakpoint::Breakpoints,
    colors::Colors,
    config::Config,
    definitions::{Definition, StoredContext},
    global_locks::GlobalLocks,
    hooks::Hooks,
    maps::Maps,
    pe::{pe32::PE32, pe64::PE64},
    structures::MemoryOperation,
    thread_context::ThreadContext,
};
use crate::maps::heap_allocation::O1Heap;

mod banzai;
mod call_stack;
mod config;
mod console;
pub mod disassemble;
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

pub struct Emu {
    // Global/shared state
    pub maps: Maps,
    pub hooks: Hooks,
    pub exp: u64,
    pub break_on_alert: bool,
    pub bp: Breakpoints,
    pub cfg: Config,
    pub colors: Colors,
    pub pos: u64,
    pub max_pos: Option<u64>,
    pub force_break: bool,
    pub force_reload: bool,
    pub tls_callbacks: Vec<u64>,
    pub instruction: Option<Instruction>,
    pub decoder_position: usize,
    pub memory_operations: Vec<MemoryOperation>,
    pub main_thread_cont: u64,
    pub gateway_return: u64,
    pub is_running: Arc<AtomicU32>,
    pub break_on_next_cmp: bool,
    pub break_on_next_return: bool,
    pub filename: String,
    pub enabled_ctrlc: bool,
    pub run_until_ret: bool,
    pub running_script: bool,
    pub banzai: Banzai,
    pub mnemonic: String,
    pub linux: bool,
    pub now: Instant,
    pub skip_apicall: bool,
    pub its_apicall: Option<u64>,
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
    // Thread management
    pub threads: Vec<ThreadContext>,
    pub current_thread_id: usize,  // Index into threads vec
    pub global_locks: GlobalLocks, // Critical section lock tracking
    pub instruction_cache: InstructionCache,
    pub definitions: HashMap<u64, Definition>,
    pub stored_contexts: HashMap<String, StoredContext>,
    pub entropy: f64,
    pub heap_management: Option<Box<O1Heap>>,
}
