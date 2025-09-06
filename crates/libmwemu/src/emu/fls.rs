use crate::emu::Emu;

impl Emu {
    pub fn fls(&self) -> &Vec<u32> {
        &self.threads[self.current_thread_id].fls
    }

    pub fn fls_mut(&mut self) -> &mut Vec<u32> {
        &mut self.threads[self.current_thread_id].fls
    }
}
