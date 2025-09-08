use crate::emu::Emu;

impl Emu {
    pub fn call_stack(&self) -> &Vec<(u64, u64)> {
        &self.threads[self.current_thread_id].call_stack
    }

    pub fn call_stack_mut(&mut self) -> &mut Vec<(u64, u64)> {
        &mut self.threads[self.current_thread_id].call_stack
    }
}
