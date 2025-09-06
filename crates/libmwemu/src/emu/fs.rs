use std::collections::BTreeMap;

use crate::emu::Emu;

impl Emu {
    pub fn fs(&self) -> &BTreeMap<u64, u64> {
        &self.threads[self.current_thread_id].fs
    }

    pub fn fs_mut(&mut self) -> &mut BTreeMap<u64, u64> {
        &mut self.threads[self.current_thread_id].fs
    }
}
