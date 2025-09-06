use crate::emu::Emu;

impl Emu {
    pub fn tls32(&self) -> &Vec<u32> {
        &self.threads[self.current_thread_id].tls32
    }

    pub fn tls32_mut(&mut self) -> &mut Vec<u32> {
        &mut self.threads[self.current_thread_id].tls32
    }

    pub fn tls64(&self) -> &Vec<u64> {
        &self.threads[self.current_thread_id].tls64
    }

    pub fn tls64_mut(&mut self) -> &mut Vec<u64> {
        &mut self.threads[self.current_thread_id].tls64
    }
}
