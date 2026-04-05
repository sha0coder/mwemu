use std::sync::{atomic, Arc};

use crate::debug::console::Console;
use crate::emu::Emu;
use crate::engine;
use crate::err::MwemuError;
use crate::serialization;

impl Emu {
    /// Main AArch64 emulation loop — full feature parity with x86.
    /// Supports: hooks, breakpoints (address + instruction count), loop detection,
    /// entropy, inspect mode, tracing (regs, flags, string, file), verbose_at,
    /// exit_position with dump, run_until_ret, ctrl-C, and all execution limits.
    pub fn run_aarch64(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        // Validate PC points to mapped memory
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

        let decoder = yaxpeax_arm::armv8::a64::InstDecoder::default();
        let mut ins = yaxpeax_arm::armv8::a64::Instruction::default();

        loop {
            while self.is_running.load(atomic::Ordering::Relaxed) == 1 {
                let pc = self.pc();

                // Check end address
                if let Some(end) = end_addr {
                    if pc == end {
                        return Ok(pc);
                    }
                }

                // Check position limit
                if self.max_pos.is_some() && Some(self.pos) >= self.max_pos {
                    return Ok(pc);
                }

                // Fetch
                let code = match self.maps.get_mem_by_addr(pc) {
                    Some(c) => c,
                    None => {
                        log::trace!("aarch64: code flow to unmapped address 0x{:x}", pc);
                        Console::spawn_console(self);
                        return Err(MwemuError::new("cannot read program counter"));
                    }
                };

                let block = code.read_bytes(pc, 4);
                if block.len() < 4 {
                    return Err(MwemuError::new("cannot read 4 bytes for aarch64 instruction"));
                }

                // Decode
                let mut reader = yaxpeax_arch::U8Reader::new(block);
                match yaxpeax_arch::Decoder::decode_into(&decoder, &mut ins, &mut reader) {
                    Ok(()) => {}
                    Err(e) => {
                        log::warn!("aarch64: decode error at 0x{:x}: {:?}", pc, e);
                        return Err(MwemuError::new("aarch64 decode error"));
                    }
                }

                self.memory_operations.clear();
                self.pos += 1;
                self.instruction_count += 1;
                let sz: usize = 4;
                let addr = pc;

                // --- Limits ---
                if let Some(max) = self.cfg.max_instructions {
                    if self.instruction_count >= max {
                        log::info!("max_instructions limit reached ({})", max);
                        return Ok(pc);
                    }
                }

                if let Some(timeout) = self.cfg.timeout_secs {
                    if self.instruction_count % 10000 == 0 {
                        let elapsed = self.now.elapsed().as_secs_f64();
                        if elapsed >= timeout {
                            log::info!("timeout reached ({:.1}s >= {:.1}s)", elapsed, timeout);
                            return Ok(pc);
                        }
                    }
                }

                if let Some(max) = self.cfg.max_faults {
                    if self.fault_count >= max {
                        log::info!("max_faults limit reached ({})", max);
                        return Ok(pc);
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
                    return Ok(pc);
                }

                // --- Breakpoints (address + instruction count + exp) ---
                if self.exp == self.pos
                    || self.bp.is_bp_instruction(self.pos)
                    || self.bp.is_bp(addr)
                    || (self.cfg.console2 && self.cfg.console_addr == addr)
                {
                    if self.running_script {
                        return Ok(pc);
                    }
                    self.cfg.console2 = false;
                    if self.cfg.verbose >= 2 {
                        log::trace!("-------");
                        log::trace!("{} 0x{:x}: {}", self.pos, pc, ins);
                    }
                    Console::spawn_console(self);
                    if self.force_break {
                        self.force_break = false;
                        break;
                    }
                }

                // --- Loop detection ---
                if addr == prev_addr {
                    repeat_counter += 1;
                } else {
                    repeat_counter = 0;
                }
                prev_addr = addr;
                if repeat_counter == 100 {
                    log::trace!("infinite loop at 0x{:x}: {}", pc, ins);
                    return Err(MwemuError::new("infinite loop found"));
                }

                if self.cfg.loops {
                    looped.push(addr);
                    let count = looped.iter().filter(|&&a| a == addr).count() as u32;
                    if count > 2 {
                        log::trace!("    loop: {} iterations", count);
                    }
                }

                // --- Entropy ---
                if self.cfg.entropy && self.pos % 10000 == 0 {
                    self.update_entropy();
                }

                // --- Verbose output ---
                if self.cfg.verbose >= 2 {
                    log::trace!("{} 0x{:x}: {}", self.pos, pc, ins);
                }

                // --- Emulate ---
                let emulation_ok = engine::aarch64::emulate_instruction(self, &ins);
                self.last_instruction_size = sz;

                if self.is_running.load(atomic::Ordering::Relaxed) == 0 {
                    return Ok(self.pc());
                }

                // --- Inspect mode ---
                if self.cfg.inspect {
                    self.trace_memory_inspection();
                }

                // --- Trace string ---
                if self.cfg.trace_string {
                    self.trace_string();
                }

                // --- Register trace ---
                if self.cfg.trace_regs {
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
                        return Ok(pc);
                    } else {
                        return Err(MwemuError::new(&format!(
                            "emulation error at pos = {} pc = 0x{:x}",
                            self.pos, pc
                        )));
                    }
                }

                // --- Advance PC ---
                if self.force_reload {
                    self.force_reload = false;
                } else {
                    self.regs_aarch64_mut().pc += 4;
                }

                if self.force_break {
                    self.force_break = false;
                    break;
                }

                // --- Return-based stop ---
                if self.run_until_ret && ins.opcode == yaxpeax_arm::armv8::a64::Opcode::RET {
                    return Ok(self.pc());
                }

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
    }
}
