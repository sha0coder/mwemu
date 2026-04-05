use std::io::Write as _;
use std::sync::{atomic, Arc};

use iced_x86::{Code, Decoder, DecoderOptions, Instruction, Mnemonic};

use crate::debug::console::Console;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::emu::disassemble::InstructionCache;
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::{windows::constants, engine, serialization};

macro_rules! round_to {
    ($num:expr, $dec:expr) => {{
        let factor = 10f64.powi($dec);
        ($num * factor).round() / factor
    }};
}

impl Emu {
    #[inline]
    pub fn stop(&mut self) {
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
        self.run(Some(ret_addr))?;
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
        self.run(Some(ret_addr))?;

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
        self.run(Some(ret_addr))?;

        // recover stack and  return rax
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
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
        // Multi-threaded dispatch (uses scheduler which calls decode_and_execute internally)
        if self.cfg.enable_threading && self.threads.len() > 1 {
            return self.step_multi_threaded();
        }

        self.pos += 1;

        // exit position check
        if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
            log::trace!("exit position reached");
            if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                serialization::Serialization::dump_to_file(
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
                serialization::Serialization::dump_to_file(
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

    /// Emulate a single step from the current point (multi-threaded implementation).
    /// this don't reset the emu.pos, that mark the number of emulated instructions and point to
    /// the current emulation moment.
    /// If you do a loop with emu.step() will have more control of the emulator but it will be
    /// slow.
    /// Is more convinient using run and run_to or even setting breakpoints.
    #[deprecated(
        since = "0.1.0",
        note = "Use step() instead, which automatically handles threading"
    )]
    pub fn step_multi_threaded(&mut self) -> bool {
        self.pos += 1;

        // exit
        if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
            log::trace!("exit position reached");

            if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                serialization::Serialization::dump_to_file(
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

        // Thread scheduling - find next runnable thread
        let num_threads = self.threads.len();
        let current_tick = self.tick;

        // Debug logging for threading
        if num_threads > 1 {
            /*log::trace!("=== THREAD SCHEDULER DEBUG ===");
            log::trace!("Step {}: {} threads, current_thread_id={}, tick={}",
                    self.pos, num_threads, self.current_thread_id, current_tick);

            for (i, thread) in self.threads.iter().enumerate() {
                let status = if thread.suspended {
                    "SUSPENDED".to_string()
                } else if thread.wake_tick > current_tick {
                    format!("SLEEPING(wake={})", thread.wake_tick)
                } else if thread.blocked_on_cs.is_some() {
                    "BLOCKED_CS".to_string()
                } else {
                    "RUNNABLE".to_string()
                };

                let marker = if i == self.current_thread_id { ">>> " } else { "    " };
                log::trace!("{}Thread[{}]: ID=0x{:x}, RIP=0x{:x}, Status={}",
                        marker, i, thread.id, thread.regs_x86().rip, status);
            }*/
        }

        // Check if current thread can run
        let current_can_run = !self.threads[self.current_thread_id].suspended
            && self.threads[self.current_thread_id].wake_tick <= current_tick
            && self.threads[self.current_thread_id].blocked_on_cs.is_none();

        if num_threads > 1 {
            //log::debug!("Current thread {} can run: {}", self.current_thread_id, current_can_run);

            // Round-robin scheduling: try each thread starting from next one
            for i in 0..num_threads {
                let thread_idx = (self.current_thread_id + i + 1) % num_threads;
                let thread = &self.threads[thread_idx];

                /*log::debug!("Checking thread {}: suspended={}, wake_tick={}, blocked={}",
                thread_idx, thread.suspended, thread.wake_tick,
                thread.blocked_on_cs.is_some());*/

                // Check if thread is runnable
                if !thread.suspended
                    && thread.wake_tick <= current_tick
                    && thread.blocked_on_cs.is_none()
                {
                    // Found a runnable thread, execute it
                    if thread_idx != self.current_thread_id {
                        /*log::trace!("🔄 THREAD SWITCH: {} -> {} (step {})",
                                self.current_thread_id, thread_idx, self.pos);
                        log::trace!("   From RIP: 0x{:x} -> To RIP: 0x{:x}",
                                self.threads[self.current_thread_id].regs_x86().rip,
                                thread.regs_x86().rip);*/
                    }
                    return crate::threading::scheduler::ThreadScheduler::execute_thread_instruction(
                        self, thread_idx,
                    );
                }
            }

            log::debug!("No other threads runnable, checking current thread");
        }

        // If no other threads are runnable, try current thread
        if current_can_run {
            /*if num_threads > 1 {
                log::debug!("Continuing with current thread {}", self.current_thread_id);
            }*/
            return crate::threading::scheduler::ThreadScheduler::execute_thread_instruction(
                self,
                self.current_thread_id,
            );
        }

        // All threads are blocked or suspended - advance time to next wake point
        let mut next_wake = usize::MAX;
        for thread in &self.threads {
            if !thread.suspended && thread.wake_tick > current_tick {
                next_wake = next_wake.min(thread.wake_tick);
            }
        }

        if next_wake != usize::MAX && next_wake > current_tick {
            // Advance time to next wake point
            self.tick = next_wake;
            log::trace!(
                "⏰ All threads blocked, advancing tick from {} to {}",
                current_tick,
                next_wake
            );
            // Try scheduling again
            return self.step();
        }

        // All threads are permanently blocked or suspended
        log::trace!("💀 All threads are blocked/suspended, cannot continue execution");
        if num_threads > 1 {
            log::trace!("Final thread states:");
            for (i, thread) in self.threads.iter().enumerate() {
                log::trace!(
                    "  Thread[{}]: ID=0x{:x}, suspended={}, wake_tick={}, blocked={}",
                    i,
                    thread.id,
                    thread.suspended,
                    thread.wake_tick,
                    thread.blocked_on_cs.is_some()
                );
            }
        }
        false
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
            super::ArchState::X86 { instruction_cache, .. } => *instruction_cache = InstructionCache::new(),
            super::ArchState::AArch64 { instruction_cache, .. } => *instruction_cache = InstructionCache::new(),
        }
        if self.cfg.enable_threading && self.threads.len() > 1 {
            self.run_multi_threaded(end_addr)
        } else {
            self.run_single_threaded(end_addr)
        }
    }

    /// Start or continue emulation (multi-threaded implementation).
    /// For emulating forever: run(None)
    /// For emulating until an address: run(Some(0x11223344))
    /// self.pos is not set to zero, can be used to continue emulation.
    #[deprecated(
        since = "0.1.0",
        note = "Use run() instead, which automatically handles threading"
    )]
    pub fn run_multi_threaded(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        todo!()
    } // end run

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

        match self.maps.get_mem_by_addr(self.pc()) {
            Some(_) => {}
            None => {
                log::trace!("Cannot start emulation, pc pointing to unmapped area");
                return Err(MwemuError::new(
                    "program counter pointing to unmapped memory",
                ));
            }
        };

        self.is_running.store(1, atomic::Ordering::Relaxed);
        let is_running2 = Arc::clone(&self.is_running);

        if self.enabled_ctrlc {
            ctrlc::set_handler(move || {
                log::trace!("Ctrl-C detected, spawning console");
                is_running2.store(0, atomic::Ordering::Relaxed);
            })
            .expect("ctrl-c handler failed");
        }

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

                // Read code bytes into block (before cache lookup to avoid borrow conflict)
                {
                    let code = match self.maps.get_mem_by_addr(pc) {
                        Some(c) => c,
                        None => {
                            log::trace!("code flow to unmapped address 0x{:x}", pc);
                            Console::spawn_console(self);
                            return Err(MwemuError::new("cannot read program counter"));
                        }
                    };
                    let block_temp = code.read_bytes(pc, constants::BLOCK_LEN);
                    let block_temp_len = block_temp.len();
                    if block_temp_len != block.len() {
                        block.resize(block_temp_len, 0);
                    }
                    block.clone_from_slice(block_temp);
                }

                // Cache miss → decode and insert
                let cache_hit = match &mut self.arch_state {
                    super::ArchState::X86 { instruction_cache, .. } => {
                        instruction_cache.lookup_entry(pc, 0)
                    }
                    super::ArchState::AArch64 { instruction_cache, .. } => {
                        instruction_cache.lookup_entry(pc, 0)
                    }
                };

                if !cache_hit {
                    // Empty code block detection
                    if !is_aarch64 {
                        let mut zeros = 0;
                        for b in block.iter() {
                            if *b == 0 { zeros += 1; } else { break; }
                        }
                        if !self.cfg.allow_empty_code_blocks && zeros > 100 {
                            if self.cfg.verbose > 0 {
                                log::trace!("{} empty code block at 0x{:x}", self.pos, pc);
                            }
                            return Err(MwemuError::new("empty code block"));
                        }
                    }

                    if block.is_empty() {
                        return Err(MwemuError::new("cannot read code block, weird address."));
                    }

                    match &mut self.arch_state {
                        super::ArchState::X86 { instruction_cache, .. } => {
                            let mut decoder = Decoder::with_ip(arch, &block, pc, DecoderOptions::NONE);
                            self.rep = None;
                            let addition = if block.len() < 16 { block.len() } else { 16 };
                            instruction_cache.insert_from_decoder(&mut decoder, addition, pc);
                        }
                        super::ArchState::AArch64 { instruction_cache, .. } => {
                            instruction_cache.insert_from_block(&block, pc);
                        }
                    }
                }

                // Inner decode loop
                let mut sz: usize = 0;
                let mut addr: u64 = 0;

                let can_decode_initially = match &self.arch_state {
                    super::ArchState::X86 { instruction_cache, .. } => instruction_cache.can_decode(),
                    super::ArchState::AArch64 { instruction_cache, .. } => instruction_cache.can_decode(),
                };
                let mut inner_running = can_decode_initially;
                let mut aarch64_decode_offset: u64 = 0;

                while inner_running {
                    // Decode next instruction from cache
                    let decoded: DecodedInstruction;
                    if is_aarch64 {
                        if self.rep.is_none() {
                            match &mut self.arch_state {
                                super::ArchState::AArch64 { instruction_cache, instruction, .. } => {
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
                                super::ArchState::X86 { instruction_cache, .. } => {
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
                            super::ArchState::X86 { instruction_cache, .. } => {
                                self.set_x86_decoder_position(instruction_cache.current_instruction_slot);
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
                    if let Some(max) = self.cfg.max_instructions {
                        if self.instruction_count >= max {
                            log::info!("max_instructions limit reached ({})", max);
                            return Ok(self.pc());
                        }
                    }

                    if let Some(timeout) = self.cfg.timeout_secs {
                        if self.instruction_count % 10000 == 0 {
                            let elapsed = self.now.elapsed().as_secs_f64();
                            if elapsed >= timeout {
                                log::info!("timeout reached ({:.1}s >= {:.1}s)", elapsed, timeout);
                                return Ok(self.pc());
                            }
                        }
                    }

                    if let Some(max) = self.cfg.max_faults {
                        if self.fault_count >= max {
                            log::info!("max_faults limit reached ({})", max);
                            return Ok(self.pc());
                        }
                    }

                    // --- verbose_at activation ---
                    if let Some(vpos) = self.cfg.verbose_at {
                        if vpos == self.pos {
                            self.cfg.verbose = 3;
                            self.cfg.trace_mem = true;
                            self.cfg.trace_regs = true;
                        }
                    }

                    // --- Exit position ---
                    if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
                        log::trace!("exit position reached");

                        if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                            serialization::Serialization::dump_to_file(
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
                    if self.exp == self.pos
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
                        if addr == prev_addr {
                            repeat_counter += 1;
                        } else {
                            repeat_counter = 0;
                        }
                        prev_addr = addr;
                        if repeat_counter == 100 {
                            log::trace!("infinite loop at 0x{:x}", addr);
                            return Err(MwemuError::new("infinite loop found"));
                        }

                        if self.cfg.loops {
                            looped.push(addr);
                            let count = looped.iter().filter(|&&a| a == addr).count() as u32;
                            if count > 2 {
                                log::trace!("    loop: {} iterations", count);
                            }
                        }
                    }

                    // --- Pre-instruction tracing ---
                    if self.cfg.trace_regs
                        && self.cfg.trace_filename.is_some()
                        && self.pos >= self.cfg.trace_start
                    {
                        self.capture_pre_op();
                    }

                    if self.cfg.trace_reg {
                        for reg in self.cfg.reg_names.clone().iter() {
                            self.trace_specific_register(reg);
                        }
                    }

                    if self.cfg.trace_flags {
                        self.flags().print_trace(self.pos);
                    }

                    if self.cfg.trace_string {
                        self.trace_string();
                    }

                    // --- Pre-instruction hook ---
                    if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
                        let hook_pc = self.pc();
                        let skip = !hook_fn(self, hook_pc, &decoded, sz);
                        self.hooks.hook_on_pre_instruction = Some(hook_fn);
                        if skip {
                            // Check can_decode for next iteration
                            inner_running = match &self.arch_state {
                                super::ArchState::X86 { instruction_cache, .. } => instruction_cache.can_decode(),
                                super::ArchState::AArch64 { instruction_cache, .. } => instruction_cache.can_decode(),
                            };
                            continue;
                        }
                    }

                    // --- x86 REP prefix handling ---
                    if !is_aarch64 {
                        let x86 = x86_ins;
                        let is_ret = matches!(x86.code(), Code::Retnw | Code::Retnd | Code::Retnq);

                        if !is_ret
                            && (x86.has_rep_prefix() || x86.has_repe_prefix() || x86.has_repne_prefix())
                        {
                            if self.rep.is_none() {
                                self.rep = Some(0);
                            }

                            if self.regs().rcx == 0 {
                                self.rep = None;
                                if self.cfg.is_x64() {
                                    self.regs_mut().rip += sz as u64;
                                } else {
                                    let new_eip = self.regs().get_eip() + sz as u64;
                                    self.regs_mut().set_eip(new_eip);
                                }
                                inner_running = match &self.arch_state {
                                    super::ArchState::X86 { instruction_cache, .. } => instruction_cache.can_decode(),
                                    super::ArchState::AArch64 { instruction_cache, .. } => instruction_cache.can_decode(),
                                };
                                continue;
                            }
                        }
                    }

                    // --- Entropy ---
                    if self.cfg.entropy && self.pos % 10000 == 0 {
                        self.update_entropy();
                    }

                    // --- Verbose output ---
                    if self.cfg.verbose >= 2 && is_aarch64 {
                        log::trace!("{} 0x{:x}: {}", self.pos, addr, aarch64_ins);
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
                        if let Some(rep_count) = self.rep {
                            if self.cfg.verbose >= 3 {
                                log::trace!("    rcx: {}", self.regs().rcx);
                            }
                            if self.regs().rcx > 0 {
                                self.regs_mut().rcx -= 1;
                                if self.regs().rcx == 0 {
                                    self.rep = None;
                                } else {
                                    self.rep = Some(rep_count + 1);
                                }
                            }

                            let is_string_movement = matches!(
                                x86_ins.mnemonic(),
                                Mnemonic::Movsb | Mnemonic::Movsw | Mnemonic::Movsd | Mnemonic::Movsq
                                    | Mnemonic::Stosb | Mnemonic::Stosw | Mnemonic::Stosd | Mnemonic::Stosq
                                    | Mnemonic::Lodsb | Mnemonic::Lodsw | Mnemonic::Lodsd | Mnemonic::Lodsq
                            );
                            let is_string_comparison = matches!(
                                x86_ins.mnemonic(),
                                Mnemonic::Cmpsb | Mnemonic::Cmpsw | Mnemonic::Cmpsd | Mnemonic::Cmpsq
                                    | Mnemonic::Scasb | Mnemonic::Scasw | Mnemonic::Scasd | Mnemonic::Scasq
                            );
                            if is_string_movement {
                                // do not clear rep
                            } else if is_string_comparison {
                                if x86_ins.has_repe_prefix() && !self.flags().f_zf {
                                    self.rep = None;
                                }
                                if x86_ins.has_repne_prefix() && self.flags().f_zf {
                                    self.rep = None;
                                }
                            } else {
                                self.rep = None;
                            }
                        }
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
                            regs.x[0], regs.x[1], regs.x[2], regs.x[3],
                            regs.x[8], regs.x[9], regs.sp, regs.x[30]
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
                    if self.run_until_ret && decoded.is_return() {
                        return Ok(self.pc());
                    }

                    // Check can_decode for next iteration
                    inner_running = match &self.arch_state {
                        super::ArchState::X86 { instruction_cache, .. } => instruction_cache.can_decode(),
                        super::ArchState::AArch64 { instruction_cache, .. } => instruction_cache.can_decode(),
                    };
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
