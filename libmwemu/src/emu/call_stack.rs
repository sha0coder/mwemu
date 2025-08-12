impl Emu {

    
    pub fn call_stack(&self) -> &Vec<String> {
        &self.threads[self.current_thread_id].call_stack
    }
    
    pub fn call_stack_mut(&mut self) -> &mut Vec<String> {
        &mut self.threads[self.current_thread_id].call_stack
    }
}