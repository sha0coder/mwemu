use crate::{emu::Emu, fpu::FPU};

impl Emu {
    // Helper method to sync FPU instruction pointer with RIP
    pub fn sync_fpu_ip(&mut self) {
        let rip = self.threads[self.current_thread_id].regs.rip;
        self.threads[self.current_thread_id].fpu.set_ip(rip);
    }

    pub fn fpu(&self) -> &FPU {
        &self.threads[self.current_thread_id].fpu
    }

    pub fn fpu_mut(&mut self) -> &mut FPU {
        &mut self.threads[self.current_thread_id].fpu
    }
}
