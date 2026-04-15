use std::io::Write as _;
use std::sync::atomic;

use iced_x86::{Decoder, DecoderOptions, Instruction};

use crate::debug::console::Console;
use crate::emu::{ArchState, Emu};
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::emu::disassemble::InstructionCache;
use crate::err::MwemuError;

use crate::syscall::windows::syscall64::memory as win_syscall64_memory;
use crate::windows::peb::peb64;
use crate::{engine, serialization, windows::constants};

mod control;
mod decode;
mod multithreaded;
mod rep;

macro_rules! round_to {
    ($num:expr, $dec:expr) => {{
        let factor = 10f64.powi($dec);
        ($num * factor).round() / factor
    }};
}

/// Maximum nesting depth for emulator-driven call64/call32/linux_call64 invocations.
/// Normal emulated `call` instructions do not increment this counter — only explicit
/// host-side calls (loader bootstrap, TLS callbacks, etc.) do. A depth beyond this
/// most likely indicates a bug in the emulator infrastructure.
const MAX_CALL_DEPTH: u32 = 32;

impl Emu {
    #[inline]
    pub fn stop(&mut self) {
        self.process_terminated = true;
        self.is_running.store(0, atomic::Ordering::Relaxed);
    }

    /// Decode and execute one instruction at the current PC.
    /// Returns (instruction_size, emulation_ok).
    /// Dispatches to x86 or aarch64 decode/execute internally.
    pub fn decode_and_execute(&mut self) -> (usize, bool) {
        let pc = self.pc();

        // Fetch code
        let code = match self.maps.get_mem_by_addr(pc) {
            Some(c) => c,
            None => {
                log::trace!("code flow to unmapped address 0x{:x}", pc);
                Console::spawn_console(self);
                return (0, false);
            }
        };

        self.memory_operations.clear();

        if self.cfg.arch.is_aarch64() {
            // --- AArch64 decode & execute ---
            let block = code.read_bytes(pc, 4);
            if block.len() < 4 {
                log::warn!("aarch64: cannot read 4 bytes at 0x{:x}", pc);
                return (0, false);
            }

            let decoder = yaxpeax_arm::armv8::a64::InstDecoder::default();
            let mut reader = yaxpeax_arch::U8Reader::new(block);
            let ins = match yaxpeax_arch::Decoder::decode(&decoder, &mut reader) {
                Ok(ins) => ins,
                Err(e) => {
                    log::warn!("aarch64: decode error at 0x{:x}: {:?}", pc, e);
                    return (0, false);
                }
            };

            if self.cfg.verbose >= 2 {
                log::trace!("{} 0x{:x}: {}", self.pos, pc, ins);
            }

            self.last_decoded = Some(DecodedInstruction::AArch64(ins));
            let result_ok = engine::aarch64::emulate_instruction(self, &ins);
            self.last_instruction_size = 4;
            (4, result_ok)
        } else {
            // --- x86 decode & execute ---
            let block = code.read_from(pc).to_vec();
            let mut decoder = if self.cfg.is_x64() {
                Decoder::with_ip(64, &block, pc, DecoderOptions::NONE)
            } else {
                Decoder::with_ip(32, &block, pc, DecoderOptions::NONE)
            };

            let ins = decoder.decode();
            let sz = ins.len();
            let position = decoder.position();

            self.set_x86_instruction(Some(ins));
            self.set_x86_decoder_position(position);
            self.last_decoded = Some(DecodedInstruction::X86(ins));

            let result_ok = engine::emulate_instruction(self, &ins, sz, true);
            self.last_instruction_size = sz;
            (sz, result_ok)
        }
    }

    /// Advance the program counter by `sz` bytes.
    /// Respects force_reload (branch already set PC).
    /// Dispatches to RIP, EIP, or PC based on architecture.
    #[inline]
    pub fn advance_pc(&mut self, sz: usize) {
        if self.force_reload {
            self.force_reload = false;
        } else if self.cfg.arch.is_aarch64() {
            self.regs_aarch64_mut().pc += sz as u64;
        } else if self.cfg.is_x64() {
            self.regs_mut().rip += sz as u64;
        } else {
            let eip = self.regs().get_eip() + sz as u64;
            self.regs_mut().set_eip(eip);
        }
    }

    /// Call a 32bits function at addr, passing argument in an array of u64 but will cast to u32.
    /// The calling convention is stack, like winapi32.
    pub fn call32(&mut self, addr: u64, args: &[u32]) -> Result<u32, MwemuError> {
        if addr == self.regs().get_eip() {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call32, change eip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }
        let orig_stack = self.regs().get_esp();
        for arg in args.iter().rev() {
            self.stack_push32(*arg);
        }
        let ret_addr = self.regs().get_eip();
        self.stack_push32(ret_addr as u32);
        self.regs_mut().set_eip(addr);
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;
        self.regs_mut().set_esp(orig_stack);
        Ok(self.regs().get_eax() as u32)
    }

    /// Call 64bits function at addr using Microsoft x64 ABI, passing argument in an array of u64.
    /// The calling convention is registers rcx/rdx/r8/r9 and then stack. Like windows64.
    /// Dont use for linux64 syscall like convention, for this is linux_call64()
    pub fn call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        if addr == self.regs().rip {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call64, change rip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }

        let n = args.len();
        if n >= 1 {
            self.regs_mut().rcx = args[0];
        }
        if n >= 2 {
            self.regs_mut().rdx = args[1];
        }
        if n >= 3 {
            self.regs_mut().r8 = args[2];
        }
        if n >= 4 {
            self.regs_mut().r9 = args[3];
        }

        // stack pointer backup, for restoring when function returns.
        let orig_stack = self.regs().rsp;

        // padding
        let extra_args = if n > 4 { (n - 4) * 8 } else { 0 };
        let total = extra_args + 32 + 8;
        let padding = (16 - (self.regs().rsp as usize + total) % 16) % 16;
        self.regs_mut().rsp -= padding as u64;

        // shadow space (32bits)
        for _ in 0..4 {
            self.stack_push64(0);
        }

        // stack parameters
        if n > 4 {
            for arg in args.iter().skip(4).rev() {
                self.stack_push64(*arg);
            }
        }

        // return address
        let ret_addr = self.regs().rip;
        self.stack_push64(ret_addr);

        // trigger function
        self.regs_mut().rip = addr;

        // emulate the function until return address is reached
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;

        // recover stack and  return rax
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
    }

    /// Call a 64bits function at addr, passing arguments in an array of u64.
    /// The calling convention is registers RDI, RSI, RDX, RCX, R8, R9 and then stack. Like linux64.
    pub fn linux_call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        if addr == self.regs().rip {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call64, change rip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }

        let n = args.len();
        if n >= 1 {
            self.regs_mut().rdi = args[0];
        }
        if n >= 2 {
            self.regs_mut().rsi = args[1];
        }
        if n >= 3 {
            self.regs_mut().rdx = args[2];
        }
        if n >= 4 {
            self.regs_mut().rcx = args[3];
        }
        if n >= 5 {
            self.regs_mut().r8 = args[4];
        }
        if n >= 6 {
            self.regs_mut().r9 = args[5];
        }

        // stack pointer backup, for restoring when function returns.
        let orig_stack = self.regs().rsp;

        // padding
        let extra_args = if n > 6 { (n - 6) * 8 } else { 0 };
        let total = extra_args + 8;
        let padding = (16 - (self.regs().rsp as usize + total) % 16) % 16;
        self.regs_mut().rsp -= padding as u64;

        // stack parameters
        if n > 6 {
            for arg in args.iter().skip(6).rev() {
                self.stack_push64(*arg);
            }
        }

        // return address
        let ret_addr = self.regs().rip;
        self.stack_push64(ret_addr);

        // trigger function
        self.regs_mut().rip = addr;

        // emulate the function until return address is reached
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;

        // recover stack and  return rax
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
    }

    /// Call a 64-bit function using AArch64 AAPCS64 calling convention.
    /// Args in x0-x7, return value in x0, LR = return address.
    pub fn aarch64_call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        let current_pc = self.pc();
        if addr == current_pc {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting aarch64_call64, change pc.",
                ));
            } else {
                self.set_pc(0);
            }
        }

        // Load args into x0-x7
        let n = args.len().min(8);
        for i in 0..n {
            self.regs_aarch64_mut().x[i] = args[i];
        }
        if args.len() > 8 {
            log::warn!("aarch64_call64: more than 8 args not yet supported");
        }

        // Save SP
        let orig_sp = self.regs_aarch64().sp;

        // 16-byte align SP
        let sp = self.regs_aarch64().sp;
        let aligned_sp = sp & !0xF;
        self.regs_aarch64_mut().sp = aligned_sp;

        // Set return address in LR (x30)
        let ret_addr = self.pc();
        self.regs_aarch64_mut().x[30] = ret_addr;

        // Jump to target
        self.set_pc(addr);

        // Emulate the function until return address is reached
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;

        // Restore SP and return x0
        self.regs_aarch64_mut().sp = orig_sp;
        Ok(self.regs_aarch64().x[0])
    }

    /// Start emulation until a ret instruction is found.
    /// It will return the address or MwemuError.
    #[inline]
    pub fn run_until_ret(&mut self) -> Result<u64, MwemuError> {
        self.run_until_ret = true;
        self.run(None)
    }

    /// Emulate a single step from the current point.
    /// Works for both x86 and aarch64. Handles hooks, threading, exit_position.
    #[allow(deprecated)]
    pub fn step(&mut self) -> bool {
        if self.process_terminated {
            return false;
        }

        if !self.os.is_linux() && self.cfg.arch.is_64bits() && self.cfg.ssdt_use_ldr_initialize_thunk {
            peb64::ensure_peb_system_dependent_07(self);
        }

        // Multi-threaded dispatch (uses scheduler which calls decode_and_execute internally)
        if self.cfg.enable_threading && self.threads.len() > 1 {
            return self.step_multi_threaded();
        }

        self.pos += 1;

        // exit position check
        if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
            log::trace!("exit position reached");
            if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    self,
                    self.cfg.dump_filename.as_ref().unwrap(),
                );
            }
            if self.cfg.trace_regs && self.cfg.trace_filename.is_some() {
                self.trace_file
                    .as_ref()
                    .unwrap()
                    .flush()
                    .expect("failed to flush trace file");
            }
            return false;
        }

        // Decode and execute (arch-dispatched)
        let (sz, result_ok) = self.decode_and_execute();
        if sz == 0 {
            return false;
        }

        // Pre-instruction hook
        if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
            let pc = self.pc();
            let decoded = self.last_decoded.unwrap();
            let skip = !hook_fn(self, pc, &decoded, sz);
            self.hooks.hook_on_pre_instruction = Some(hook_fn);
            if skip {
                self.advance_pc(sz);
                return true;
            }
        }

        // Post-instruction hook
        if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
            let pc = self.pc();
            let decoded = self.last_decoded.unwrap();
            hook_fn(self, pc, &decoded, sz, result_ok);
            self.hooks.hook_on_post_instruction = Some(hook_fn);
        }

        // Advance PC
        self.advance_pc(sz);

        result_ok
    }

    pub fn update_entropy(&mut self) {
        let prev_entropy = self.entropy;

        let mem = match self.maps.get_mem_by_addr(self.pc()) {
            Some(n) => n,
            None => {
                self.entropy = 0.0;
                if self.entropy != prev_entropy {
                    log::trace!(
                        "{}:0x{:x} entropy changed {} ->  {}",
                        self.pos,
                        self.pc(),
                        prev_entropy,
                        self.entropy
                    );
                }
                return;
            }
        };

        let data = mem.get_bytes();

        if data.is_empty() {
            self.entropy = 0.0;
            if self.entropy != prev_entropy {
                log::trace!(
                    "{}:0x{:x} entropy changed {} ->  {}",
                    self.pos,
                    self.pc(),
                    prev_entropy,
                    self.entropy
                );
            }
            return;
        }

        let mut counts = [0usize; 256];
        for &b in data {
            counts[b as usize] += 1;
        }
        let len = data.len() as f64;
        self.entropy = round_to!(
            counts
                .iter()
                .filter(|&&c| c > 0)
                .map(|&c| {
                    let p = c as f64 / len;
                    -p * p.log2()
                })
                .sum::<f64>(),
            3
        );

        if self.entropy != prev_entropy {
            log::trace!(
                "{}:0x{:x} entropy changed {} ->  {}",
                self.pos,
                self.pc(),
                prev_entropy,
                self.entropy
            );
        }
    }

    /// Emulate a single step from the current point (single-threaded implementation).
    /// this don't reset the emu.pos, that mark the number of emulated instructions and point to
    /// the current emulation moment.
    /// If you do a loop with emu.step() will have more control of the emulator but it will be
    /// slow.
    /// Is more convinient using run and run_to or even setting breakpoints.
    #[deprecated(
        since = "0.1.0",
        note = "Use step() instead, which automatically handles threading"
    )]
    pub fn step_single_threaded(&mut self) -> bool {
        self.pos += 1;

        // exit
        if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
            log::trace!("exit position reached");

            if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    self,
                    self.cfg.dump_filename.as_ref().unwrap(),
                );
            }

            if self.cfg.trace_regs && self.cfg.trace_filename.is_some() {
                self.trace_file
                    .as_ref()
                    .unwrap()
                    .flush()
                    .expect("failed to flush trace file");
            }

            return false;
        }

        // code
        let rip = self.regs().rip;
        let code = match self.maps.get_mem_by_addr(rip) {
            Some(c) => c,
            None => {
                log::trace!(
                    "redirecting code flow to non maped address 0x{:x}",
                    self.regs().rip
                );
                Console::spawn_console(self);
                return false;
            }
        };

        // block
        let block = code.read_from(rip).to_vec(); // reduce code block for more speed

        // decoder
        let mut decoder;
        if self.cfg.is_x64() {
            decoder = Decoder::with_ip(64, &block, self.regs().rip, DecoderOptions::NONE);
        } else {
            decoder = Decoder::with_ip(32, &block, self.regs().get_eip(), DecoderOptions::NONE);
        }

        // get first instruction from iterator
        let ins = decoder.decode();
        let sz = ins.len();
        let addr = ins.ip();
        let position = decoder.position();

        // clear
        self.memory_operations.clear();

        // format
        self.set_x86_instruction(Some(ins));
        self.set_x86_decoder_position(position);

        // Run pre-instruction hook
        let decoded = DecodedInstruction::X86(ins);
        self.last_decoded = Some(decoded);
        if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
            let rip = self.regs().rip;
            let skip = !hook_fn(self, rip, &decoded, sz);
            self.hooks.hook_on_pre_instruction = Some(hook_fn);
            if skip {
                // update eip/rip
                if self.force_reload {
                    self.force_reload = false;
                } else if self.cfg.is_x64() {
                    self.regs_mut().rip += sz as u64;
                } else {
                    let eip = self.regs().get_eip() + sz as u64;
                    self.regs_mut().set_eip(eip);
                }
                return true; // skip instruction emulation
            }
        }
        // emulate
        let result_ok = engine::emulate_instruction(self, &ins, sz, true);
        //tracing::trace_instruction(self, self.pos);
        self.last_instruction_size = sz;

        // Run post-instruction hook
        if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
            let rip = self.regs().rip;
            hook_fn(self, rip, &decoded, sz, result_ok);
            self.hooks.hook_on_post_instruction = Some(hook_fn);
        }

        // update eip/rip
        if self.force_reload {
            self.force_reload = false;
        } else if self.cfg.is_x64() {
            self.regs_mut().rip += sz as u64;
        } else {
            let eip = self.regs().get_eip() + sz as u64;
            self.regs_mut().set_eip(eip);
        }

        result_ok
    }

    /// Run until a specific position (emu.pos)
    /// This don't reset the emu.pos, will meulate from current position to
    /// selected end_pos included.
    pub fn run_to(&mut self, end_pos: u64) -> Result<u64, MwemuError> {
        self.max_pos = Some(end_pos);
        let r = self.run(None);
        self.max_pos = None;
        return r;
    }

    /// Start or continue emulation.
    /// For emulating forever: run(None)
    /// For emulating until an address: run(Some(0x11223344))
    /// self.pos is not set to zero, can be used to continue emulation.
    /// Automatically dispatches to single or multi-threaded execution based on cfg.enable_threading.
    #[allow(deprecated)]
    pub fn run(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        // Reset instruction cache for the active architecture
        match &mut self.arch_state {
            ArchState::X86 {
                instruction_cache, ..
            } => *instruction_cache = InstructionCache::new(),
            ArchState::AArch64 {
                instruction_cache, ..
            } => *instruction_cache = InstructionCache::new(),
        }
        if !self.os.is_linux()
            && self.cfg.arch.is_64bits()
            && self.cfg.ssdt_use_ldr_initialize_thunk
            && self.maps.get_map_by_name("peb").is_some()
        {
            peb64::ensure_peb_system_dependent_07(self);
        }
        if !self.cfg.arch.is_aarch64() {
            *self.x86_instruction_cache() = InstructionCache::new();
        }
        if self.cfg.enable_threading && self.threads.len() > 1 {
            self.run_multi_threaded(end_addr)
        } else {
            self.run_single_threaded(end_addr)
        }
    }

    /// Unified single-threaded emulation loop for both x86 and aarch64.
    ///
    /// Both architectures share identical structure: outer cache-miss → inner
    /// cache-hit decode loop.  The only x86-specific section is REP prefix
    /// handling (~30 lines), guarded by `decoded.is_x86()`.
    #[deprecated(
        since = "0.1.0",
        note = "Use run() instead, which automatically handles threading"
    )]
    pub fn run_single_threaded(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        let is_aarch64 = self.cfg.arch.is_aarch64();

        if self.process_terminated {
            return Err(MwemuError::new("process terminated (NtTerminateProcess)"));
        }
        self.ensure_run_start_pc_mapped(self.pc())?;

        self.is_running.store(1, atomic::Ordering::Relaxed);
        self.install_ctrlc_handler_if_enabled();

        let mut looped: Vec<u64> = Vec::new();
        let mut prev_addr: u64 = 0;
        let mut repeat_counter: u32 = 0;

        let arch = if self.cfg.is_x64() { 64 } else { 32 };
        let mut x86_ins: Instruction = Instruction::default();
        let mut aarch64_ins = yaxpeax_arm::armv8::a64::Instruction::default();
        let mut block: Vec<u8> = Vec::with_capacity(constants::BLOCK_LEN + 1);
        block.resize(constants::BLOCK_LEN, 0x0);

        loop {
            while self.is_running.load(atomic::Ordering::Relaxed) == 1 {
                let pc = self.pc();

                // Outer-loop limit checks: must run BEFORE attempting to fetch code,
                // otherwise PC sitting one past the end (e.g. after final loop iteration
                // under run_to) errors out as "unmapped" instead of cleanly stopping.
                if let Some(limit_pc) = self.reached_outer_run_limit(pc, end_addr) {
                    return Ok(limit_pc);
                }

                self.fill_code_block(pc, &mut block)?;
                self.ensure_instruction_cache_populated(pc, &block, arch, is_aarch64)?;

                // Inner decode loop
                let mut sz: usize = 0;
                let mut addr: u64 = 0;

                let mut inner_running = self.instruction_cache_can_decode();
                let mut aarch64_decode_offset: u64 = 0;

                while inner_running {
                    // Decode next instruction from cache
                    let decoded: DecodedInstruction;
                    if is_aarch64 {
                        if self.rep.is_none() {
                            match &mut self.arch_state {
                                ArchState::AArch64 {
                                    instruction_cache,
                                    instruction,
                                    ..
                                } => {
                                    instruction_cache.decode_out(&mut aarch64_ins);
                                    *instruction = Some(aarch64_ins);
                                }
                                _ => unreachable!(),
                            }
                            sz = 4;
                            addr = pc + aarch64_decode_offset;
                            aarch64_decode_offset += 4;
                        }
                        decoded = DecodedInstruction::AArch64(aarch64_ins);
                    } else {
                        if self.rep.is_none() {
                            match &mut self.arch_state {
                                ArchState::X86 {
                                    instruction_cache, ..
                                } => {
                                    instruction_cache.decode_out(&mut x86_ins);
                                }
                                _ => unreachable!(),
                            }
                            sz = x86_ins.len();
                            addr = x86_ins.ip();

                            if end_addr.is_some() && Some(addr) == end_addr {
                                return Ok(self.pc());
                            }

                            if self.max_pos.is_some() && Some(self.pos) >= self.max_pos {
                                return Ok(self.pc());
                            }
                        }
                        self.set_x86_instruction(Some(x86_ins));
                        match &self.arch_state {
                            ArchState::X86 {
                                instruction_cache, ..
                            } => {
                                self.set_x86_decoder_position(
                                    instruction_cache.current_instruction_slot,
                                );
                            }
                            _ => unreachable!(),
                        }
                        decoded = DecodedInstruction::X86(x86_ins);
                    }

                    // aarch64 end_addr / max_pos checks (x86 checked above during decode)
                    if is_aarch64 {
                        if let Some(end) = end_addr {
                            if addr == end {
                                return Ok(self.pc());
                            }
                        }
                        if self.max_pos.is_some() && Some(self.pos) >= self.max_pos {
                            return Ok(self.pc());
                        }
                    }

                    self.last_decoded = Some(decoded);
                    self.memory_operations.clear();
                    self.pos += 1;
                    self.instruction_count += 1;

                    // --- Limits ---
                    if let Some(limit_pc) = self.check_runtime_limits(self.pc()) {
                        return Ok(limit_pc);
                    }

                    self.update_verbose_at();

                    // --- verbose_range activation (-X a,b) ---
                    if self.cfg.verbose_start != 0 {
                        let in_range = self.pos >= self.cfg.verbose_start
                            && (self.cfg.verbose_end == 0 || self.pos <= self.cfg.verbose_end);
                        if in_range {
                            if self.cfg.verbose_range_saved.is_none() {
                                self.cfg.verbose_range_saved = Some(self.cfg.verbose);
                            }
                            self.cfg.verbose = 3;
                        } else if let Some(orig) = self.cfg.verbose_range_saved.take() {
                            self.cfg.verbose = orig;
                        }
                    }

                    // --- Exit position ---
                    if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
                        log::trace!("exit position reached");

                        if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                            serialization::Serialization::dump(
                                self,
                                self.cfg.dump_filename.as_ref().unwrap(),
                            );
                        }

                        if self.cfg.trace_regs && self.cfg.trace_filename.is_some() {
                            self.trace_file
                                .as_ref()
                                .unwrap()
                                .flush()
                                .expect("failed to flush trace file");
                        }

                        return Ok(self.pc());
                    }

                    // --- Breakpoints ---
                    if (self.exp != u64::MAX && self.exp == self.pos)
                        || self.bp.is_bp_instruction(self.pos)
                        || self.bp.is_bp(addr)
                        || (self.cfg.console2 && self.cfg.console_addr == addr)
                    {
                        if self.running_script {
                            return Ok(self.pc());
                        }

                        self.cfg.console2 = false;
                        if self.cfg.verbose >= 2 {
                            let output = self.format_instruction(&decoded);
                            log::trace!("-------");
                            log::trace!("{} 0x{:x}: {}", self.pos, addr, output);
                        }
                        Console::spawn_console(self);
                        if self.force_break {
                            self.force_break = false;
                            break;
                        }
                    }

                    // --- Loop detection (skip during REP) ---
                    if self.rep.is_none() {
                        self.observe_loop_progress(
                            addr,
                            &mut prev_addr,
                            &mut repeat_counter,
                            &mut looped,
                            "infinite loop found",
                        )?;
                    }

                    // --- Pre-instruction tracing ---
                    self.trace_pre_step_state(self.pos);

                    // --- Pre-instruction hook ---
                    if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
                        let hook_pc = self.pc();
                        let skip = !hook_fn(self, hook_pc, &decoded, sz);
                        self.hooks.hook_on_pre_instruction = Some(hook_fn);
                        if skip {
                            // Check can_decode for next iteration
                            inner_running = self.instruction_cache_can_decode();
                            continue;
                        }
                    }

                    // --- x86 REP prefix handling ---
                    if !is_aarch64 && self.handle_x86_rep_pre_execution(x86_ins, sz) {
                        inner_running = self.instruction_cache_can_decode();
                        continue;
                    }

                    // --- Entropy ---
                    if self.cfg.entropy && self.pos % 10000 == 0 {
                        self.update_entropy();
                    }

                    // --- Verbose output ---
                    if self.cfg.verbose >= 2 && is_aarch64 {
                        log::trace!("{} 0x{:x}: {}", self.pos, addr, aarch64_ins);
                    }

                    if !is_aarch64 {
                        win_syscall64_memory::ntdll_heap_list_walk_fixup(self, &x86_ins, addr);
                    }

                    // --- Emulate ---
                    let emulation_ok = if is_aarch64 {
                        engine::aarch64::emulate_instruction(self, &aarch64_ins)
                    } else {
                        engine::emulate_instruction(self, &x86_ins, sz, false)
                    };
                    self.last_instruction_size = sz;

                    if self.is_running.load(atomic::Ordering::Relaxed) == 0 {
                        return Ok(self.pc());
                    }

                    // --- x86 REP post-execution state machine ---
                    if !is_aarch64 {
                        self.update_x86_rep_state_after_execution(x86_ins);
                    }

                    // --- Post-instruction hook ---
                    if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
                        let hook_pc = self.pc();
                        hook_fn(self, hook_pc, &decoded, sz, emulation_ok);
                        self.hooks.hook_on_post_instruction = Some(hook_fn);
                    }

                    // --- Post-execution tracing ---
                    if self.cfg.inspect {
                        self.trace_memory_inspection();
                    }

                    if self.cfg.trace_regs
                        && self.cfg.trace_filename.is_some()
                        && self.pos >= self.cfg.trace_start
                    {
                        self.capture_post_op();
                        self.write_to_trace_file();
                    }

                    // --- Register trace (aarch64) ---
                    if is_aarch64 && self.cfg.trace_regs {
                        let regs = self.regs_aarch64();
                        log::trace!(
                            "  x0=0x{:x} x1=0x{:x} x2=0x{:x} x3=0x{:x} x8=0x{:x} x9=0x{:x} sp=0x{:x} lr=0x{:x}",
                            regs.x[0],
                            regs.x[1],
                            regs.x[2],
                            regs.x[3],
                            regs.x[8],
                            regs.x[9],
                            regs.sp,
                            regs.x[30]
                        );
                    }

                    // --- Failure handling ---
                    if !emulation_ok {
                        self.fault_count += 1;
                        if self.cfg.console_enabled {
                            Console::spawn_console(self);
                        } else if self.running_script {
                            return Ok(self.pc());
                        } else {
                            return Err(MwemuError::new(&format!(
                                "emulation error at pos = {} pc = 0x{:x}",
                                self.pos, addr
                            )));
                        }
                    }

                    // --- PC advance ---
                    if self.force_reload {
                        self.force_reload = false;
                        break; // break inner loop to re-fetch from new PC
                    }

                    if is_aarch64 {
                        self.regs_aarch64_mut().pc += 4;
                    } else if self.rep.is_none() {
                        if self.cfg.is_x64() {
                            self.regs_mut().rip += sz as u64;
                        } else {
                            let new_eip = self.regs().get_eip() + sz as u64;
                            self.regs_mut().set_eip(new_eip);
                        }
                    }

                    if self.force_break {
                        self.force_break = false;
                        break;
                    }

                    // --- Return-based stop ---
                    // TODO: re-enable this. Correct semantics for `run_until_ret()` on
                    // BOTH arches (main's run_aarch64 has the equivalent check at
                    // execution_aarch64.rs:185). Currently disabled because main's x86
                    // path lacks this check entirely: instead, ret.rs returns true
                    // without updating rip when run_until_ret is set, the loop then
                    // advances rip += sz past the ret, execution falls through to
                    // whatever bytes follow, and eventually crashes into unmapped
                    // memory — at which point run() returns Err and callers using
                    // `let _ = emu.run_until_ret()` silently swallow it. The test
                    // tests::string_ops_tests::test_scasb relies on this quirk: its
                    // `jz +7` is intentionally aimed at a `ret` that's expected to
                    // act as a nop so execution falls through to `mov rbx, 1`. To
                    // turn this on, fix the test bytecode (jz offset 0x07 -> 0x08 so
                    // it skips both the `mov rbx, 0` AND its trailing ret, landing
                    // directly on `mov rbx, 1`), then uncomment the block below.
                    //
                    // if self.run_until_ret && decoded.is_return() {
                    //     return Ok(self.pc());
                    // }

                    // Check can_decode for next iteration
                    inner_running = self.instruction_cache_can_decode();
                } // end inner decode loop

                if self.is_api_run && self.is_break_on_api {
                    self.is_api_run = false;
                    break;
                }
            } // end running loop

            if self.is_break_on_api {
                return Ok(0);
            }

            self.is_running.store(1, atomic::Ordering::Relaxed);
            Console::spawn_console(self);
        } // end infinite loop
    } // end run_single_threaded
}
