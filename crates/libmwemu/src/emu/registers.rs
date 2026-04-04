use crate::{emu::Emu, regs64::Regs64, regs_aarch64::RegsAarch64};

impl Emu {
    // Forwarding methods for thread-specific fields
    pub fn regs(&self) -> &Regs64 {
        &self.threads[self.current_thread_id].regs
    }

    pub fn regs_mut(&mut self) -> &mut Regs64 {
        &mut self.threads[self.current_thread_id].regs
    }

    // AArch64 register accessors
    pub fn regs_aarch64(&self) -> &RegsAarch64 {
        self.threads[self.current_thread_id]
            .regs_aarch64
            .as_ref()
            .expect("regs_aarch64 called on non-aarch64 emu")
    }

    pub fn regs_aarch64_mut(&mut self) -> &mut RegsAarch64 {
        self.threads[self.current_thread_id]
            .regs_aarch64
            .as_mut()
            .expect("regs_aarch64_mut called on non-aarch64 emu")
    }

    // Unified program counter for shared code paths
    pub fn pc(&self) -> u64 {
        if self.cfg.arch.is_aarch64() {
            self.regs_aarch64().pc
        } else {
            self.regs().rip
        }
    }

    pub fn set_pc(&mut self, addr: u64) {
        if self.cfg.arch.is_aarch64() {
            self.regs_aarch64_mut().pc = addr;
        } else {
            self.regs_mut().rip = addr;
        }
    }

    pub fn set_pre_op_regs(&mut self, regs: Regs64) {
        self.threads[self.current_thread_id].pre_op_regs = regs;
    }

    pub fn set_post_op_regs(&mut self, regs: Regs64) {
        self.threads[self.current_thread_id].post_op_regs = regs;
    }

    pub fn pre_op_regs(&self) -> &Regs64 {
        &self.threads[self.current_thread_id].pre_op_regs
    }

    pub fn pre_op_regs_mut(&mut self) -> &mut Regs64 {
        &mut self.threads[self.current_thread_id].pre_op_regs
    }

    pub fn post_op_regs(&self) -> &Regs64 {
        &self.threads[self.current_thread_id].post_op_regs
    }

    pub fn post_op_regs_mut(&mut self) -> &mut Regs64 {
        &mut self.threads[self.current_thread_id].post_op_regs
    }
}
