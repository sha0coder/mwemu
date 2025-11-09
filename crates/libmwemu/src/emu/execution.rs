use std::io::Write as _;
use std::sync::{atomic, Arc};

use iced_x86::{Code, Decoder, DecoderOptions, Formatter as _, Instruction, Mnemonic};

use crate::console::Console;
use crate::emu::disassemble::InstructionCache;
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::{constants, engine, serialization};

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
    /// This doesn't reset the emu.pos, which marks the number of emulated instructions and points to
    /// the current emulation moment.
    /// Automatically dispatches to single or multi-threaded execution based on cfg.enable_threading.
    #[allow(deprecated)]
    pub fn step(&mut self) -> bool {
        if self.cfg.enable_threading && self.threads.len() > 1 {
            self.step_multi_threaded()
        } else {
            self.step_single_threaded()
        }
    }

    pub fn update_entropy(&mut self) {
        let prev_entropy = self.entropy;

        let mem = match self.maps.get_mem_by_addr(self.regs().rip) {
            Some(n) => n,
            None => {
                self.entropy = 0.0;
                if self.entropy != prev_entropy {
                    log::info!(
                        "{}:0x{:x} entropy changed {} ->  {}",
                        self.pos,
                        self.regs().rip,
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
                log::info!(
                    "{}:0x{:x} entropy changed {} ->  {}",
                    self.pos,
                    self.regs().rip,
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
            log::info!(
                "{}:0x{:x} entropy changed {} ->  {}",
                self.pos,
                self.regs().rip,
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
            log::info!("exit position reached");

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
                log::info!(
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
        if self.cfg.is_64bits {
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
        self.instruction = Some(ins);
        self.decoder_position = position;

        // Run pre-instruction hook
        if let Some(hook_fn) = self.hooks.hook_on_pre_instruction {
            if !hook_fn(self, self.regs().rip, &ins, sz) {
                // update eip/rip
                if self.force_reload {
                    self.force_reload = false;
                } else if self.cfg.is_64bits {
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
        if let Some(hook_fn) = self.hooks.hook_on_post_instruction {
            hook_fn(self, self.regs().rip, &ins, sz, result_ok)
        }

        // update eip/rip
        if self.force_reload {
            self.force_reload = false;
        } else if self.cfg.is_64bits {
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
            log::info!("exit position reached");

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
            /*log::info!("=== THREAD SCHEDULER DEBUG ===");
            log::info!("Step {}: {} threads, current_thread_id={}, tick={}",
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
                log::info!("{}Thread[{}]: ID=0x{:x}, RIP=0x{:x}, Status={}",
                        marker, i, thread.id, thread.regs.rip, status);
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
                        /*log::info!("🔄 THREAD SWITCH: {} -> {} (step {})",
                                self.current_thread_id, thread_idx, self.pos);
                        log::info!("   From RIP: 0x{:x} -> To RIP: 0x{:x}",
                                self.threads[self.current_thread_id].regs.rip,
                                thread.regs.rip);*/
                    }
                    return crate::threading::ThreadScheduler::execute_thread_instruction(
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
            return crate::threading::ThreadScheduler::execute_thread_instruction(
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
            log::info!(
                "⏰ All threads blocked, advancing tick from {} to {}",
                current_tick,
                next_wake
            );
            // Try scheduling again
            return self.step();
        }

        // All threads are permanently blocked or suspended
        log::info!("💀 All threads are blocked/suspended, cannot continue execution");
        if num_threads > 1 {
            log::info!("Final thread states:");
            for (i, thread) in self.threads.iter().enumerate() {
                log::info!(
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
        let instruction_cache = InstructionCache::new();
        self.instruction_cache = instruction_cache;
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

    /// Start or continue emulation (single-threaded implementation).
    /// For emulating forever: run(None)
    /// For emulating until an address: run(Some(0x11223344))
    /// self.pos is not set to zero, can be used to continue emulation.
    #[deprecated(
        since = "0.1.0",
        note = "Use run() instead, which automatically handles threading"
    )]
    pub fn run_single_threaded(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        //self.stack_lvl.clear();
        //self.stack_lvl_idx = 0;
        //self.stack_lvl.push(0);

        match self.maps.get_mem_by_addr(self.regs().rip) {
            Some(mem) => {}
            None => {
                log::info!("Cannot start emulation, pc pointing to unmapped area");
                return Err(MwemuError::new(
                    "program counter pointing to unmapped memory",
                ));
            }
        };

        self.is_running.store(1, atomic::Ordering::Relaxed);
        let is_running2 = Arc::clone(&self.is_running);

        if self.enabled_ctrlc {
            ctrlc::set_handler(move || {
                log::info!("Ctrl-C detected, spawning console");
                is_running2.store(0, atomic::Ordering::Relaxed);
            })
            .expect("ctrl-c handler failed");
        }

        let mut looped: Vec<u64> = Vec::new();
        let mut prev_addr: u64 = 0;
        //let mut prev_prev_addr:u64 = 0;
        let mut repeat_counter: u32 = 0;

        /*
        if end_addr.is_none() && self.max_pos.is_none() {
            log::info!(" ----- emulation -----");
        } else if !self.max_pos.is_none() {
            log::info!(" ----- emulation to {} -----", self.max_pos.unwrap());
        } else {
            log::info!(" ----- emulation to 0x{:x} -----", end_addr.unwrap());
        }*/

        //self.pos = 0;
        let arch = if self.cfg.is_64bits { 64 } else { 32 };
        let mut ins: Instruction = Instruction::default();
        // we using a single block to store all the instruction to optimize for without
        // the need of Reallocate everytime
        let mut block: Vec<u8> = Vec::with_capacity(constants::BLOCK_LEN + 1);
        block.resize(constants::BLOCK_LEN, 0x0);
        self.instruction_cache = InstructionCache::new();
        loop {
            while self.is_running.load(atomic::Ordering::Relaxed) == 1 {
                //log::info!("reloading rip 0x{:x}", self.regs().rip);

                let rip = self.regs().rip;
                let code = match self.maps.get_mem_by_addr(rip) {
                    Some(c) => c,
                    None => {
                        log::info!("redirecting code flow to non mapped address 0x{:x}", rip);
                        Console::spawn_console(self);
                        return Err(MwemuError::new("cannot read program counter"));
                    }
                };

                if !self.instruction_cache.lookup_entry(rip, 0) {
                    // we just need to read 0x300 bytes because x86 require that the instruction is 16 bytes long
                    // reading anymore would be a waste of time
                    let block_sz = constants::BLOCK_LEN;
                    let block_temp = code.read_bytes(rip, block_sz);
                    let block_temp_len = block_temp.len();
                    if block_temp_len != block.len() {
                        block.resize(block_temp_len, 0);
                    }
                    block.clone_from_slice(block_temp);
                    if block.len() == 0 {
                        return Err(MwemuError::new("cannot read code block, weird address."));
                    }
                    let mut decoder =
                        Decoder::with_ip(arch, &block, self.regs().rip, DecoderOptions::NONE);

                    self.rep = None;
                    let addition = if block_temp_len < 16 {
                        block_temp_len
                    } else {
                        16
                    };
                    self.instruction_cache
                        .insert_from_decoder(&mut decoder, addition, rip);
                }

                let mut sz = 0;
                let mut addr = 0;
                while self.instruction_cache.can_decode() {
                    if self.rep.is_none() {
                        self.instruction_cache.decode_out(&mut ins);
                        sz = ins.len();
                        addr = ins.ip();

                        if end_addr.is_some() && Some(addr) == end_addr {
                            return Ok(self.regs().rip);
                        }

                        if self.max_pos.is_some() && Some(self.pos) >= self.max_pos {
                            return Ok(self.regs().rip);
                        }
                    }

                    self.instruction = Some(ins);
                    self.decoder_position = self.instruction_cache.current_instruction_slot;
                    self.memory_operations.clear();
                    self.pos += 1;

                    // turn on verbosity after a lot of pos
                    if let Some(vpos) = self.cfg.verbose_at {
                        if vpos == self.pos {
                            self.cfg.verbose = 3;
                            self.cfg.trace_mem = true;
                            self.cfg.trace_regs = true;
                        }
                    }

                    if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
                        log::info!("exit position reached");

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

                        return Ok(self.regs().rip);
                    }

                    if self.exp == self.pos
                        || self.bp.is_bp_instruction(self.pos)
                        || self.bp.is_bp(addr)
                        || (self.cfg.console2 && self.cfg.console_addr == addr)
                    {
                        if self.running_script {
                            return Ok(self.regs().rip);
                        }

                        self.cfg.console2 = false;
                        if self.cfg.verbose >= 2 {
                            let mut output = String::new();
                            self.formatter.format(&ins, &mut output);
                            log::info!("-------");
                            log::info!("{} 0x{:x}: {}", self.pos, ins.ip(), output);
                        }
                        Console::spawn_console(self);
                        if self.force_break {
                            self.force_break = false;
                            break;
                        }
                    }

                    // prevent infinite loop
                    if self.rep.is_none() {
                        if addr == prev_addr {
                            // || addr == prev_prev_addr {
                            repeat_counter += 1;
                        }
                        //prev_prev_addr = prev_addr;
                        prev_addr = addr;
                        if repeat_counter == 100 {
                            log::info!(
                                "infinite loop!  opcode: {}",
                                ins.op_code().op_code_string()
                            );
                            return Err(MwemuError::new("inifinite loop found"));
                        }

                        if self.cfg.loops {
                            // loop detector
                            looped.push(addr);
                            let mut count: u32 = 0;
                            for a in looped.iter() {
                                if addr == *a {
                                    count += 1;
                                }
                            }
                            if count > 2 {
                                log::info!("    loop: {} interations", count);
                            }
                            /*
                            if count > self.loop_limit {
                            panic!("/!\\ iteration limit reached");
                            }*/
                            //TODO: if more than x addresses remove the bottom ones
                        }
                    }

                    if self.cfg.trace_regs
                        && self.cfg.trace_filename.is_some()
                        && self.pos >= self.cfg.trace_start
                    {
                        self.capture_pre_op();
                    }

                    if self.cfg.trace_reg {
                        for reg in self.cfg.reg_names.iter() {
                            self.trace_specific_register(reg);
                        }
                    }

                    if self.cfg.trace_flags {
                        self.flags().print_trace(self.pos);
                    }

                    if self.cfg.trace_string {
                        self.trace_string();
                    }

                    //let mut info_factory = InstructionInfoFactory::new();
                    //let info = info_factory.info(&ins);

                    if let Some(hook_fn) = self.hooks.hook_on_pre_instruction {
                        if !hook_fn(self, self.regs().rip, &ins, sz) {
                            continue;
                        }
                    }

                    // trace pre instruction
                    /*{
                        if self.pos >= 100_000_000 {
                            tracing::trace_instruction(self, self.pos);
                        }
                    }*/

                    let is_ret = match ins.code() {
                        Code::Retnw | Code::Retnd | Code::Retnq => true,
                        _ => false,
                    };

                    if !is_ret
                        && (ins.has_rep_prefix() || ins.has_repe_prefix() || ins.has_repne_prefix())
                    {
                        if self.rep.is_none() {
                            self.rep = Some(0);
                        }

                        // if rcx is 0 in first rep step, skip instruction.
                        if self.regs_mut().rcx == 0 {
                            self.rep = None;
                            if self.cfg.is_64bits {
                                self.regs_mut().rip += sz as u64;
                            } else {
                                let new_eip = self.regs().get_eip() + sz as u64;
                                self.regs_mut().set_eip(new_eip);
                            }
                            continue;
                        }
                    }

                    if self.pos % 10000 == 0 {
                        if self.cfg.entropy {
                            self.update_entropy();
                        }
                    }

                    /*************************************/
                    let emulation_ok = engine::emulate_instruction(self, &ins, sz, false);
                    //tracing::trace_instruction(self, self.pos);
                    /*************************************/

                    if let Some(rep_count) = self.rep {
                        if self.cfg.verbose >= 3 {
                            log::info!("    rcx: {}", self.regs().rcx);
                        }
                        if self.regs().rcx > 0 {
                            self.regs_mut().rcx -= 1;
                            if self.regs_mut().rcx == 0 {
                                self.rep = None;
                            } else {
                                self.rep = Some(rep_count + 1);
                            }
                        }

                        // repe and repe are the same on x86 (0xf3) so you have to check if it is movement or comparison
                        let is_string_movement = matches!(
                            ins.mnemonic(),
                            Mnemonic::Movsb
                                | Mnemonic::Movsw
                                | Mnemonic::Movsd
                                | Mnemonic::Movsq
                                | Mnemonic::Stosb
                                | Mnemonic::Stosw
                                | Mnemonic::Stosd
                                | Mnemonic::Stosq
                                | Mnemonic::Lodsb
                                | Mnemonic::Lodsw
                                | Mnemonic::Lodsd
                                | Mnemonic::Lodsq
                        );
                        let is_string_comparison = matches!(
                            ins.mnemonic(),
                            Mnemonic::Cmpsb
                                | Mnemonic::Cmpsw
                                | Mnemonic::Cmpsd
                                | Mnemonic::Cmpsq
                                | Mnemonic::Scasb
                                | Mnemonic::Scasw
                                | Mnemonic::Scasd
                                | Mnemonic::Scasq
                        );
                        if is_string_movement {
                            // do not clear rep if it is a string movement
                        } else if is_string_comparison {
                            if ins.has_repe_prefix() && !self.flags().f_zf {
                                self.rep = None;
                            }
                            if ins.has_repne_prefix() && self.flags().f_zf {
                                self.rep = None;
                            }
                        } else {
                            self.rep = None;
                            //unimplemented!("string instruction not supported");
                        }
                    }

                    if let Some(hook_fn) = self.hooks.hook_on_post_instruction {
                        hook_fn(self, self.regs().rip, &ins, sz, emulation_ok)
                    }

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

                    if !emulation_ok {
                        if self.cfg.console_enabled {
                            Console::spawn_console(self);
                        } else {
                            if self.running_script {
                                return Ok(self.regs().rip);
                            } else {
                                return Err(MwemuError::new(&format!(
                                    "emulation error at pos = {} rip = 0x{:x}",
                                    self.pos,
                                    self.regs().rip
                                )));
                            }
                        }
                    }

                    if self.force_reload {
                        self.force_reload = false;
                        break;
                    }

                    if self.rep.is_none() {
                        if self.cfg.is_64bits {
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
                } // end decoder loop
            } // end running loop

            self.is_running.store(1, atomic::Ordering::Relaxed);
            Console::spawn_console(self);
        } // end infinite loop
    } // end run {
}
