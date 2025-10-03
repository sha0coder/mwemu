use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::sync::atomic;
use std::sync::Arc;

use iced_x86::Instruction;
use serde::{Deserialize, Serialize};

use crate::banzai::Banzai;
use crate::breakpoint::Breakpoints;
use crate::colors::Colors;
use crate::config::Config;
use crate::eflags::Eflags;
use crate::emu::Emu;
use crate::flags::Flags;
use crate::global_locks::GlobalLocks;
use crate::hooks::Hooks;
use crate::regs64::Regs64;
use crate::serialization::fpu::SerializableFPU;
use crate::serialization::instant::SerializableInstant;
use crate::serialization::maps::SerializableMaps;
use crate::serialization::pe32::SerializablePE32;
use crate::serialization::pe64::SerializablePE64;
use crate::serialization::thread_context::SerializableThreadContext;
use crate::structures::MemoryOperation;

use crate::emu::disassemble::InstructionCache;

#[derive(Serialize, Deserialize)]
pub struct SerializableEmu {
    pub regs: Regs64,
    pub pre_op_regs: Regs64,
    pub post_op_regs: Regs64,
    pub flags: Flags,
    pub pre_op_flags: Flags,
    pub post_op_flags: Flags,
    pub eflags: Eflags,
    pub fpu: SerializableFPU,
    pub maps: SerializableMaps,
    //pub hooks: Hooks, // not possible
    pub exp: u64,
    pub break_on_alert: bool,
    pub bp: Breakpoints,
    pub seh: u64,
    pub veh: u64,
    pub uef: u64,
    pub eh_ctx: u32,
    pub cfg: Config,
    pub colors: Colors,
    pub pos: u64,
    pub max_pos: Option<u64>,
    pub force_break: bool,
    pub force_reload: bool,
    pub tls_callbacks: Vec<u64>,
    pub tls32: Vec<u32>,
    pub tls64: Vec<u64>,
    pub fls: Vec<u32>,
    pub instruction: Option<Instruction>,
    pub decoder_position: usize,
    pub memory_operations: Vec<MemoryOperation>,
    pub main_thread_cont: u64,
    pub gateway_return: u64,
    pub is_running: u32,
    pub break_on_next_cmp: bool,
    pub break_on_next_return: bool,
    pub filename: String,
    pub enabled_ctrlc: bool,
    pub run_until_ret: bool,
    pub running_script: bool,
    pub banzai: Banzai,
    pub mnemonic: String,
    pub linux: bool,
    pub fs: BTreeMap<u64, u64>,
    pub now: SerializableInstant,
    pub skip_apicall: bool,
    pub its_apicall: Option<u64>,
    pub last_instruction_size: usize,
    pub pe64: Option<SerializablePE64>,
    pub pe32: Option<SerializablePE32>,
    pub rep: Option<u64>,
    pub tick: usize,
    pub base: u64,
    pub call_stack: Vec<(u64, u64)>,
    pub heap_addr: u64,
    pub threads: Vec<SerializableThreadContext>,
    pub current_thread_id: usize,
    pub entropy: f64,
}

impl<'a> From<&'a Emu> for SerializableEmu {
    fn from(emu: &'a Emu) -> Self {
        SerializableEmu {
            regs: emu.regs().clone(),
            pre_op_regs: *emu.pre_op_regs(),
            post_op_regs: *emu.post_op_regs(),
            flags: *emu.flags(),
            pre_op_flags: *emu.pre_op_flags(),
            post_op_flags: *emu.post_op_flags(),
            eflags: emu.eflags().clone(),
            fpu: emu.fpu().clone().into(),
            maps: emu.maps.clone().into(),
            exp: emu.exp,
            break_on_alert: emu.break_on_alert,
            bp: emu.bp.clone(),
            seh: emu.seh(),
            veh: emu.veh(),
            uef: emu.uef(),
            eh_ctx: emu.eh_ctx(),
            cfg: emu.cfg.clone(),
            colors: emu.colors.clone(),
            pos: emu.pos,
            max_pos: emu.max_pos,
            force_break: emu.force_break,
            force_reload: emu.force_reload,
            tls_callbacks: emu.tls_callbacks.clone(),
            tls32: emu.tls32().clone(),
            tls64: emu.tls64().clone(),
            fls: emu.fls().clone(),
            instruction: emu.instruction,
            decoder_position: emu.decoder_position,
            memory_operations: emu.memory_operations.clone(),
            main_thread_cont: emu.main_thread_cont,
            gateway_return: emu.gateway_return,
            is_running: emu.is_running.load(std::sync::atomic::Ordering::Relaxed),
            break_on_next_cmp: emu.break_on_next_cmp,
            break_on_next_return: emu.break_on_next_return,
            filename: emu.filename.clone(),
            enabled_ctrlc: emu.enabled_ctrlc,
            run_until_ret: emu.run_until_ret,
            running_script: emu.running_script,
            banzai: emu.banzai.clone(),
            mnemonic: emu.mnemonic.clone(),
            linux: emu.linux,
            fs: emu.fs().clone(),
            now: SerializableInstant::from(emu.now),
            skip_apicall: emu.skip_apicall,
            its_apicall: emu.its_apicall,
            last_instruction_size: emu.last_instruction_size,
            pe64: emu.pe64.as_ref().map(|x| x.into()),
            pe32: emu.pe32.as_ref().map(|x| x.into()),
            rep: emu.rep,
            tick: emu.tick,
            base: emu.base,
            call_stack: emu.call_stack().clone(),
            heap_addr: emu.heap_addr,
            threads: emu.threads.iter().map(|t| t.into()).collect(),
            current_thread_id: emu.current_thread_id,
            entropy: emu.entropy,
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
            instruction_cache: InstructionCache::new(),
            maps: serialized.maps.into(),
            hooks: Hooks::default(), // not possible
            exp: serialized.exp,
            break_on_alert: serialized.break_on_alert,
            bp: serialized.bp,
            cfg: serialized.cfg.clone(),
            colors: serialized.colors,
            pos: serialized.pos,
            max_pos: serialized.max_pos,
            force_break: serialized.force_break,
            force_reload: serialized.force_reload,
            tls_callbacks: serialized.tls_callbacks,
            instruction: serialized.instruction,
            decoder_position: serialized.decoder_position,
            memory_operations: serialized.memory_operations,
            main_thread_cont: serialized.main_thread_cont,
            gateway_return: serialized.gateway_return,
            is_running: Arc::new(atomic::AtomicU32::new(serialized.is_running)),
            break_on_next_cmp: serialized.break_on_next_cmp,
            break_on_next_return: serialized.break_on_next_return,
            filename: serialized.filename,
            enabled_ctrlc: serialized.enabled_ctrlc,
            run_until_ret: serialized.run_until_ret,
            running_script: serialized.running_script,
            banzai: serialized.banzai,
            mnemonic: serialized.mnemonic,
            linux: serialized.linux,
            now: serialized.now.to_instant(),
            skip_apicall: serialized.skip_apicall,
            its_apicall: serialized.its_apicall,
            last_instruction_size: serialized.last_instruction_size,
            pe64: serialized.pe64.map(|x| x.into()),
            pe32: serialized.pe32.map(|x| x.into()),
            rep: serialized.rep,
            tick: serialized.tick,
            trace_file: trace_file,
            base: serialized.base,
            formatter: Default::default(),
            heap_addr: serialized.heap_addr,
            rng: RefCell::new(rand::rng()),
            threads: serialized.threads.into_iter().map(|t| t.into()).collect(),
            current_thread_id: serialized.current_thread_id,
            global_locks: GlobalLocks::new(), // Reset locks on deserialization
            definitions: HashMap::new(),
            stored_contexts: HashMap::new(),
            entropy: 0.0,
            heap_management: None,
        }
    }
}

impl Default for SerializableEmu {
    fn default() -> Self {
        let emu = Emu::new();
        SerializableEmu {
            regs: emu.regs().clone(),
            pre_op_regs: emu.pre_op_regs().clone(),
            post_op_regs: emu.post_op_regs().clone(),
            flags: emu.flags().clone(),
            pre_op_flags: emu.pre_op_flags().clone(),
            post_op_flags: emu.post_op_flags().clone(),
            eflags: emu.eflags().clone(),
            fpu: SerializableFPU::default(),
            maps: SerializableMaps::default(),
            exp: emu.exp,
            break_on_alert: emu.break_on_alert,
            bp: emu.bp.clone(),
            seh: emu.seh(),
            veh: emu.veh(),
            uef: emu.uef().clone(),
            eh_ctx: emu.eh_ctx().clone(),
            cfg: emu.cfg.clone(),
            colors: emu.colors.clone(),
            pos: emu.pos,
            max_pos: emu.max_pos,
            force_break: emu.force_break,
            force_reload: emu.force_reload,
            tls_callbacks: emu.tls_callbacks.clone(),
            tls32: emu.tls32().clone(),
            tls64: emu.tls64().clone(),
            fls: emu.fls().clone(),
            instruction: emu.instruction,
            decoder_position: emu.decoder_position,
            memory_operations: emu.memory_operations.clone(),
            main_thread_cont: emu.main_thread_cont,
            gateway_return: emu.gateway_return,
            is_running: emu.is_running.load(std::sync::atomic::Ordering::Relaxed),
            break_on_next_cmp: emu.break_on_next_cmp,
            break_on_next_return: emu.break_on_next_return,
            filename: emu.filename.clone(),
            enabled_ctrlc: emu.enabled_ctrlc,
            run_until_ret: emu.run_until_ret,
            running_script: emu.running_script,
            banzai: emu.banzai.clone(),
            mnemonic: emu.mnemonic.clone(),
            linux: emu.linux,
            fs: emu.fs().clone(),
            now: SerializableInstant::from(emu.now),
            skip_apicall: emu.skip_apicall,
            its_apicall: emu.its_apicall,
            last_instruction_size: emu.last_instruction_size,
            pe64: emu.pe64.as_ref().map(|x| x.into()),
            pe32: emu.pe32.as_ref().map(|x| x.into()),
            rep: emu.rep,
            tick: emu.tick,
            base: emu.base,
            call_stack: emu.call_stack().clone(),
            heap_addr: emu.heap_addr,
            threads: emu.threads.iter().map(|t| t.into()).collect(),
            current_thread_id: emu.current_thread_id,
            entropy: emu.entropy,
        }
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
