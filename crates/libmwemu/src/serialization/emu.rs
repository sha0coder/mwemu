use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::sync::atomic;
use std::sync::Arc;

use iced_x86::Instruction;
use serde::{Deserialize, Serialize};

use crate::api::banzai::Banzai;
use crate::debug::breakpoint::Breakpoints;
use crate::utils::colors::Colors;
use crate::config::Config;
use crate::eflags::Eflags;
use crate::emu::Emu;
use crate::flags::Flags;
use crate::threading::global_locks::GlobalLocks;
use crate::hooks::Hooks;
use crate::regs64::Regs64;
use crate::serialization::fpu::SerializableFPU;
use crate::serialization::instant::SerializableInstant;
use crate::serialization::maps::SerializableMaps;
use crate::serialization::pe32::SerializablePE32;
use crate::serialization::pe64::SerializablePE64;
use crate::serialization::thread_context::SerializableThreadContext;
use crate::windows::structures::MemoryOperation;

use crate::emu::disassemble::InstructionCache;
use crate::emu::object_handle::HandleManagement;

#[derive(Serialize, Deserialize)]
pub struct SerializableEmu {
    // --- Configuration & display ---
    pub cfg: Config,
    pub colors: Colors,
    pub filename: String,

    // --- Memory & address space ---
    pub maps: SerializableMaps,
    pub base: u64,
    pub heap_addr: u64,
    // NOTE: heap_management is not serialized (runtime only)
    pub memory_operations: Vec<MemoryOperation>,

    // --- Instruction decoding & disassembly ---
    // NOTE: formatter, instruction_cache are recreated on deserialize
    pub instruction: Option<Instruction>,
    pub decoder_position: usize,
    pub last_instruction_size: usize,
    pub rep: Option<u64>,

    // --- Core execution state ---
    pub pos: u64,
    pub max_pos: Option<u64>,
    pub tick: usize,
    pub is_running: u32, // serialized as plain u32 (not atomic)
    pub now: SerializableInstant,
    pub force_break: bool,
    pub force_reload: bool,
    pub run_until_ret: bool,
    // NOTE: rng is not serialized (recreated on deserialize)

    // --- Platform & loaded binary ---
    pub os: crate::arch::OperatingSystem,
    pub pe64: Option<SerializablePE64>,
    pub pe32: Option<SerializablePE32>,
    // NOTE: elf32, elf64, macho64 not yet serialized
    pub tls_callbacks: Vec<u64>,

    // --- Thread management ---
    pub threads: Vec<SerializableThreadContext>,
    pub current_thread_id: usize,
    pub main_thread_cont: u64,
    pub gateway_return: u64,
    // NOTE: global_locks reset on deserialize

    // --- Flattened thread context (from current thread at serialize time) ---
    pub regs: Regs64,
    pub pre_op_regs: Regs64,
    pub post_op_regs: Regs64,
    pub flags: Flags,
    pub pre_op_flags: Flags,
    pub post_op_flags: Flags,
    pub eflags: Eflags,
    pub fpu: SerializableFPU,
    pub seh: u64,
    pub veh: u64,
    pub uef: u64,
    pub eh_ctx: u32,
    pub tls32: Vec<u32>,
    pub tls64: Vec<u64>,
    pub fls: Vec<u32>,
    pub fs: BTreeMap<u64, u64>,
    pub call_stack: Vec<(u64, u64)>,

    // --- API call interception ---
    // NOTE: hooks cannot be serialized
    pub banzai: Banzai,
    pub skip_apicall: bool,
    pub its_apicall: Option<u64>,
    // NOTE: is_api_run, is_break_on_api are runtime-only

    // --- Debugging & breakpoints ---
    pub bp: Breakpoints,
    pub break_on_alert: bool,
    pub break_on_next_cmp: bool,
    pub break_on_next_return: bool,
    pub enabled_ctrlc: bool,
    pub running_script: bool,
    pub exp: u64,
    // NOTE: definitions, stored_contexts reset on deserialize

    // --- Tracing & statistics ---
    // NOTE: trace_file reconstructed from cfg.trace_filename
    pub entropy: f64,
    pub last_error: u32,
    // NOTE: instruction_count, fault_count not serialized
    // NOTE: handle_management not yet serialized
}

impl<'a> From<&'a Emu> for SerializableEmu {
    fn from(emu: &'a Emu) -> Self {
        SerializableEmu {
            // Configuration & display
            cfg: emu.cfg.clone(),
            colors: emu.colors.clone(),
            filename: emu.filename.clone(),
            // Memory & address space
            maps: emu.maps.clone().into(),
            base: emu.base,
            heap_addr: emu.heap_addr,
            memory_operations: emu.memory_operations.clone(),
            // Instruction decoding
            instruction: emu.instruction,
            decoder_position: emu.decoder_position,
            last_instruction_size: emu.last_instruction_size,
            rep: emu.rep,
            // Core execution state
            pos: emu.pos,
            max_pos: emu.max_pos,
            tick: emu.tick,
            is_running: emu.is_running.load(std::sync::atomic::Ordering::Relaxed),
            now: SerializableInstant::from(emu.now),
            force_break: emu.force_break,
            force_reload: emu.force_reload,
            run_until_ret: emu.run_until_ret,
            // Platform & loaded binary
            os: emu.os,
            pe64: emu.pe64.as_ref().map(|x| x.into()),
            pe32: emu.pe32.as_ref().map(|x| x.into()),
            tls_callbacks: emu.tls_callbacks.clone(),
            // Thread management
            threads: emu.threads.iter().map(|t| t.into()).collect(),
            current_thread_id: emu.current_thread_id,
            main_thread_cont: emu.main_thread_cont,
            gateway_return: emu.gateway_return,
            // Flattened thread context
            regs: emu.regs().clone(),
            pre_op_regs: *emu.pre_op_regs(),
            post_op_regs: *emu.post_op_regs(),
            flags: *emu.flags(),
            pre_op_flags: *emu.pre_op_flags(),
            post_op_flags: *emu.post_op_flags(),
            eflags: emu.eflags().clone(),
            fpu: emu.fpu().clone().into(),
            seh: emu.seh(),
            veh: emu.veh(),
            uef: emu.uef(),
            eh_ctx: emu.eh_ctx(),
            tls32: emu.tls32().clone(),
            tls64: emu.tls64().clone(),
            fls: emu.fls().clone(),
            fs: emu.fs().clone(),
            call_stack: emu.call_stack().clone(),
            // API call interception
            banzai: emu.banzai.clone(),
            skip_apicall: emu.skip_apicall,
            its_apicall: emu.its_apicall,
            // Debugging & breakpoints
            bp: emu.bp.clone(),
            break_on_alert: emu.break_on_alert,
            break_on_next_cmp: emu.break_on_next_cmp,
            break_on_next_return: emu.break_on_next_return,
            enabled_ctrlc: emu.enabled_ctrlc,
            running_script: emu.running_script,
            exp: emu.exp,
            // Tracing & statistics
            entropy: emu.entropy,
            last_error: emu.last_error,
        }
    }
}

impl From<SerializableEmu> for Emu {
    fn from(serialized: SerializableEmu) -> Self {
        let trace_file = if let Some(trace_filename) = &serialized.cfg.trace_filename {
            let file = File::open(trace_filename.clone()).unwrap();
            Some(file)
        } else {
            None
        };

        Emu {
            // Configuration & display
            cfg: serialized.cfg.clone(),
            colors: serialized.colors,
            filename: serialized.filename,
            // Memory & address space
            maps: serialized.maps.into(),
            base: serialized.base,
            heap_addr: serialized.heap_addr,
            heap_management: None,
            memory_operations: serialized.memory_operations,
            // Instruction decoding (formatter, cache recreated)
            instruction: serialized.instruction,
            formatter: Default::default(),
            instruction_cache: InstructionCache::new(),
            decoder_position: serialized.decoder_position,
            last_instruction_size: serialized.last_instruction_size,
            rep: serialized.rep,
            // Core execution state
            pos: serialized.pos,
            max_pos: serialized.max_pos,
            tick: serialized.tick,
            is_running: Arc::new(atomic::AtomicU32::new(serialized.is_running)),
            now: serialized.now.to_instant(),
            force_break: serialized.force_break,
            force_reload: serialized.force_reload,
            run_until_ret: serialized.run_until_ret,
            rng: RefCell::new(rand::rng()),
            // Platform & loaded binary
            os: serialized.os,
            pe64: serialized.pe64.map(|x| x.into()),
            pe32: serialized.pe32.map(|x| x.into()),
            elf64: None,    // TODO: not yet serialized
            elf32: None,    // TODO: not yet serialized
            macho64: None,  // TODO: not yet serialized
            tls_callbacks: serialized.tls_callbacks,
            library_loaded: false,
            // Thread management
            threads: serialized.threads.into_iter().map(|t| t.into()).collect(),
            current_thread_id: serialized.current_thread_id,
            main_thread_cont: serialized.main_thread_cont,
            gateway_return: serialized.gateway_return,
            global_locks: GlobalLocks::new(),
            // API call interception (hooks cannot be serialized)
            hooks: Hooks::default(),
            skip_apicall: serialized.skip_apicall,
            its_apicall: serialized.its_apicall,
            is_api_run: false,
            is_break_on_api: false,
            banzai: serialized.banzai,
            // Debugging & breakpoints
            bp: serialized.bp,
            break_on_alert: serialized.break_on_alert,
            break_on_next_cmp: serialized.break_on_next_cmp,
            break_on_next_return: serialized.break_on_next_return,
            enabled_ctrlc: serialized.enabled_ctrlc,
            running_script: serialized.running_script,
            exp: serialized.exp,
            definitions: HashMap::new(),
            stored_contexts: HashMap::new(),
            // Tracing & statistics
            trace_file,
            instruction_count: 0,
            fault_count: 0,
            entropy: 0.0,
            last_error: 0,
            // Win32 resource management
            handle_management: HandleManagement::new(), // TODO: not yet serialized
        }
    }
}

impl Default for SerializableEmu {
    fn default() -> Self {
        SerializableEmu::from(&Emu::new())
    }
}

impl SerializableEmu {
    pub fn set_regs(&mut self, regs: Regs64) {
        self.regs = regs;
    }

    pub fn set_maps(&mut self, maps: SerializableMaps) {
        self.maps = maps;
    }

    pub fn set_pe32(&mut self, pe32: Option<SerializablePE32>) {
        self.pe32 = pe32;
    }

    pub fn set_pe64(&mut self, pe64: Option<SerializablePE64>) {
        self.pe64 = pe64;
    }
}
