use crate::emu::Emu;
use log;
use std::io::Write;

/// Thread scheduling and management functions for the emulator
pub struct ThreadScheduler;

impl ThreadScheduler {
    /// Schedule and switch to the next runnable thread (round-robin)
    /// Always tries to switch to give each thread equal time
    /// Returns true if a thread switch occurred, false otherwise
    pub fn schedule_next_thread(emu: &mut Emu) -> bool {
        // Single thread case - no scheduling needed
        if emu.threads.len() <= 1 {
            return false;
        }

        let current_tick = emu.tick;
        let current_thread_id = emu.current_thread_id;

        // Round-robin: always try to switch to the next thread
        // This ensures fair scheduling - each thread gets one instruction
        for i in 1..=emu.threads.len() {
            let thread_idx = (current_thread_id + i) % emu.threads.len();

            // Skip back to current thread only if no other threads are runnable
            if thread_idx == current_thread_id {
                // We've checked all other threads, none are runnable
                // Check if current thread can continue
                if Self::is_thread_runnable(emu, current_thread_id) {
                    return false; // Stay on current thread
                }
                // Current thread also can't run
                break;
            }

            if Self::is_thread_runnable(emu, thread_idx) {
                // Found a runnable thread - switch to it
                /*log::debug!(
                    "🔄 Thread switch: {} -> {} at step {}",
                    current_thread_id,
                    thread_idx,
                    emu.pos
                );
                log::debug!(
                    "   From RIP: 0x{:x} -> To RIP: 0x{:x}",
                    emu.threads[current_thread_id].regs.rip,
                    emu.threads[thread_idx].regs.rip
                );*/

                Self::switch_to_thread(emu, thread_idx);
                return true;
            }
        }

        // No threads are runnable (including current)
        // Try to advance time if threads are just sleeping
        if Self::advance_to_next_wake(emu) {
            // Recursively try scheduling again after time advance
            return Self::schedule_next_thread(emu);
        }

        // All threads are permanently blocked
        Self::log_thread_states(emu);
        log::error!("⚠️ All threads blocked or suspended - deadlock detected");

        false
    }

    /// Check if a specific thread is runnable
    fn is_thread_runnable(emu: &Emu, thread_idx: usize) -> bool {
        if thread_idx >= emu.threads.len() {
            return false;
        }

        let thread = &emu.threads[thread_idx];
        !thread.suspended && thread.wake_tick <= emu.tick && thread.blocked_on_cs.is_none()
    }

    /// Advance emulator tick to the next thread wake time
    /// Returns true if time was advanced, false if no threads are waiting
    fn advance_to_next_wake(emu: &mut Emu) -> bool {
        let current_tick = emu.tick;
        let mut next_wake = usize::MAX;

        // Find the earliest wake time among suspended threads
        for thread in &emu.threads {
            if !thread.suspended && thread.wake_tick > current_tick {
                next_wake = next_wake.min(thread.wake_tick);
            }
        }

        if next_wake != usize::MAX && next_wake > current_tick {
            log::info!(
                "⏰ Advancing tick from {} to {} (all threads sleeping)",
                current_tick,
                next_wake
            );
            emu.tick = next_wake;
            return true;
        }

        false
    }

    /// Log the current state of all threads for debugging
    pub fn log_thread_states(emu: &Emu) {
        log::info!("=== Thread States ===");
        for (i, thread) in emu.threads.iter().enumerate() {
            let status = Self::get_thread_status_string(emu, i);
            let marker = if i == emu.current_thread_id {
                ">>>"
            } else {
                "   "
            };

            log::info!(
                "{} Thread[{}]: ID=0x{:x}, RIP=0x{:x}, Status={}",
                marker,
                i,
                thread.id,
                thread.regs.rip,
                status
            );
        }
        log::info!("Current tick: {}", emu.tick);
    }

    /// Get a human-readable status string for a thread
    fn get_thread_status_string(emu: &Emu, thread_idx: usize) -> String {
        let thread = &emu.threads[thread_idx];

        if thread.suspended {
            "SUSPENDED".to_string()
        } else if thread.wake_tick > emu.tick {
            format!("SLEEPING(wake={})", thread.wake_tick)
        } else if thread.blocked_on_cs.is_some() {
            "BLOCKED_CS".to_string()
        } else {
            "RUNNABLE".to_string()
        }
    }

    /// Switch execution context to a different thread
    pub fn switch_to_thread(emu: &mut Emu, thread_id: usize) -> bool {
        if thread_id >= emu.threads.len() {
            log::error!("Invalid thread ID: {}", thread_id);
            return false;
        }

        if thread_id == emu.current_thread_id {
            return true; // Already on this thread
        }

        // Save current thread's FPU state
        emu.threads[emu.current_thread_id].fpu = emu.fpu().clone();

        // Switch to new thread
        emu.current_thread_id = thread_id;

        // Restore new thread's FPU state
        *emu.fpu_mut() = emu.threads[thread_id].fpu.clone();

        // Don't set force_reload - we want the thread to continue from its current position
        // force_reload would prevent IP advancement which causes instructions to execute twice

        /*log::trace!(
            "Switched to thread {} (ID: 0x{:x})",
            thread_id,
            emu.threads[thread_id].id
        );*/

        true
    }

    /// Execute a single instruction for a specific thread
    /// This consolidates the duplicated logic from step_thread
    pub fn execute_thread_instruction(emu: &mut Emu, thread_id: usize) -> bool {
        // Switch to target thread if needed
        if emu.current_thread_id != thread_id {
            if !Self::switch_to_thread(emu, thread_id) {
                return false;
            }
        }

        let rip = emu.regs().rip;

        // Check if RIP points to valid memory
        let code = match emu.maps.get_mem_by_addr(rip) {
            Some(c) => c,
            None => {
                log::info!(
                    "Thread {} (ID: 0x{:x}) RIP 0x{:x} points to unmapped memory",
                    thread_id,
                    emu.threads[thread_id].id,
                    rip
                );
                crate::console::Console::spawn_console(emu);
                return false;
            }
        };

        // Read and decode instruction
        let block = code.read_from(rip).to_vec();
        let ins = if emu.cfg.is_64bits {
            iced_x86::Decoder::with_ip(64, &block, rip, iced_x86::DecoderOptions::NONE).decode()
        } else {
            let eip = emu.regs().get_eip();
            iced_x86::Decoder::with_ip(32, &block, eip, iced_x86::DecoderOptions::NONE).decode()
        };

        let sz = ins.len();
        let position = if emu.cfg.is_64bits {
            iced_x86::Decoder::with_ip(64, &block, rip, iced_x86::DecoderOptions::NONE).position()
        } else {
            let eip = emu.regs().get_eip();
            iced_x86::Decoder::with_ip(32, &block, eip, iced_x86::DecoderOptions::NONE).position()
        };

        // Prepare for execution
        emu.memory_operations.clear();
        emu.instruction = Some(ins);
        emu.decoder_position = position;

        // Pre-instruction hook
        if let Some(hook_fn) = emu.hooks.hook_on_pre_instruction {
            if !hook_fn(emu, rip, &ins, sz) {
                Self::advance_ip(emu, sz);
                return true;
            }
        }

        // Execute the instruction
        let result_ok = crate::engine::emulate_instruction(emu, &ins, sz, true);
        emu.last_instruction_size = sz;

        // Post-instruction hook
        if let Some(hook_fn) = emu.hooks.hook_on_post_instruction {
            let instruction = emu.instruction.take().unwrap();
            hook_fn(emu, rip, &instruction, sz, result_ok);
            emu.instruction = Some(instruction);
        }

        // Advance instruction pointer
        Self::advance_ip(emu, sz);

        result_ok
    }

    /// Advance the instruction pointer by the given size
    /// Handles both 32-bit and 64-bit modes, and respects force_reload flag
    pub fn advance_ip(emu: &mut Emu, sz: usize) {
        if emu.force_reload {
            // Don't advance IP if force_reload is set
            // This allows hooks or other code to manually set the IP
            emu.force_reload = false;
        } else if emu.cfg.is_64bits {
            // 64-bit mode: advance RIP
            emu.regs_mut().rip += sz as u64;
        } else {
            // 32-bit mode: advance EIP
            let eip = emu.regs().get_eip() + sz as u64;
            emu.regs_mut().set_eip(eip);
        }
    }

    /// Main thread scheduling step - replaces the complex logic in step()
    pub fn step_with_scheduling(emu: &mut Emu) -> bool {
        emu.pos += 1;

        // Check exit condition
        if emu.cfg.exit_position != 0 && emu.pos == emu.cfg.exit_position {
            log::info!("Exit position reached");
            Self::handle_exit(emu);
            return false;
        }

        // If only one thread, execute it directly
        if emu.threads.len() == 1 {
            if Self::is_thread_runnable(emu, 0) {
                return Self::execute_thread_instruction(emu, 0);
            } else {
                log::error!("Single thread is not runnable");
                return false;
            }
        }

        // Multi-threaded execution with scheduling

        // First, try to continue with current thread if it's still runnable
        if Self::is_thread_runnable(emu, emu.current_thread_id) {
            // Give current thread another timeslice
            return Self::execute_thread_instruction(emu, emu.current_thread_id);
        }

        // Current thread can't run, find another
        for i in 1..emu.threads.len() {
            let thread_idx = (emu.current_thread_id + i) % emu.threads.len();
            if Self::is_thread_runnable(emu, thread_idx) {
                log::debug!(
                    "Switching from thread {} to {}",
                    emu.current_thread_id,
                    thread_idx
                );
                return Self::execute_thread_instruction(emu, thread_idx);
            }
        }

        // No threads are immediately runnable - try advancing time
        if Self::advance_to_next_wake(emu) {
            // Time advanced, try again
            return Self::step_with_scheduling(emu);
        }

        // All threads are blocked
        Self::log_thread_states(emu);
        log::error!("All threads are blocked or suspended");
        false
    }

    /// Handle emulator exit
    fn handle_exit(emu: &mut Emu) {
        if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
            crate::serialization::Serialization::dump_to_file(
                emu,
                emu.cfg.dump_filename.as_ref().unwrap(),
            );
        }

        if emu.cfg.trace_regs && emu.cfg.trace_filename.is_some() {
            emu.trace_file
                .as_ref()
                .unwrap()
                .flush()
                .expect("failed to flush trace file");
        }
    }
}
