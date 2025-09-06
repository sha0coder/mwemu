use crate::{emu::Emu, regs64::Regs64};

impl Emu {
    // Forwarding methods for thread-specific fields
    pub fn regs(&self) -> &Regs64 {
        &self.threads[self.current_thread_id].regs
    }

    pub fn regs_mut(&mut self) -> &mut Regs64 {
        &mut self.threads[self.current_thread_id].regs
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
