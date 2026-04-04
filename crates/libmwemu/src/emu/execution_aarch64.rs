use std::sync::atomic;

use yaxpeax_arch::{Decoder, U8Reader};
use yaxpeax_arm::armv8::a64::InstDecoder;

use crate::console::Console;
use crate::emu::Emu;
use crate::engine;
use crate::err::MwemuError;

impl Emu {
    /// Single-step one AArch64 instruction.
    pub fn step_aarch64(&mut self) -> bool {
        self.pos += 1;

        let pc = self.regs_aarch64().pc;

        // Fetch code block
        let code = match self.maps.get_mem_by_addr(pc) {
            Some(c) => c,
            None => {
                log::trace!("aarch64: code flow to unmapped address 0x{:x}", pc);
                Console::spawn_console(self);
                return false;
            }
        };

        let block = code.read_bytes(pc, 4);
        if block.len() < 4 {
            log::warn!("aarch64: cannot read 4 bytes at 0x{:x}", pc);
            return false;
        }

        // Decode
        let decoder = InstDecoder::default();
        let mut reader = U8Reader::new(block);
        let ins = match decoder.decode(&mut reader) {
            Ok(ins) => ins,
            Err(e) => {
                log::warn!("aarch64: decode error at 0x{:x}: {:?}", pc, e);
                return false;
            }
        };

        self.aarch64_instruction = Some(ins);
        self.memory_operations.clear();

        if self.cfg.verbose >= 2 {
            log::trace!("{} 0x{:x}: {}", self.pos, pc, ins);
        }

        // Emulate
        let result_ok = engine::aarch64::emulate_instruction(self, &ins);
        self.last_instruction_size = 4;

        // Advance PC (unless branch set force_reload)
        if self.force_reload {
            self.force_reload = false;
        } else {
            self.regs_aarch64_mut().pc += 4;
        }

        result_ok
    }

    /// Main AArch64 emulation loop.
    pub fn run_aarch64(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        self.is_running.store(1, atomic::Ordering::Relaxed);

        let decoder = InstDecoder::default();
        let mut ins = yaxpeax_arm::armv8::a64::Instruction::default();

        loop {
            if self.is_running.load(atomic::Ordering::Relaxed) != 1 {
                break;
            }

            let pc = self.regs_aarch64().pc;

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
            let mut reader = U8Reader::new(block);
            match decoder.decode_into(&mut ins, &mut reader) {
                Ok(()) => {}
                Err(e) => {
                    log::warn!("aarch64: decode error at 0x{:x}: {:?}", pc, e);
                    return Err(MwemuError::new("aarch64 decode error"));
                }
            }

            self.aarch64_instruction = Some(ins);
            self.memory_operations.clear();
            self.pos += 1;
            self.instruction_count += 1;

            // Check limits
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
                        log::info!("timeout reached ({:.1}s)", elapsed);
                        return Ok(pc);
                    }
                }
            }

            if let Some(max_faults) = self.cfg.max_faults {
                if self.fault_count >= max_faults {
                    log::info!("max_faults limit reached ({})", max_faults);
                    return Ok(pc);
                }
            }

            // Verbose
            if self.cfg.verbose >= 2 {
                log::trace!("{} 0x{:x}: {}", self.pos, pc, ins);
            }

            // Breakpoints
            if self.bp.is_bp(pc) {
                log::info!("breakpoint hit at 0x{:x}", pc);
                Console::spawn_console(self);
            }

            // Emulate
            let result_ok = engine::aarch64::emulate_instruction(self, &ins);
            self.last_instruction_size = 4;

            if !result_ok {
                self.fault_count += 1;
                if self.cfg.verbose > 0 {
                    log::warn!("aarch64: instruction failed at 0x{:x}: {}", pc, ins);
                }
            }

            // Advance PC
            if self.force_reload {
                self.force_reload = false;
            } else {
                self.regs_aarch64_mut().pc += 4;
            }

            // Return-based stop
            if self.run_until_ret && ins.opcode == yaxpeax_arm::armv8::a64::Opcode::RET {
                return Ok(self.regs_aarch64().pc);
            }
        }

        Ok(self.regs_aarch64().pc)
    }
}
