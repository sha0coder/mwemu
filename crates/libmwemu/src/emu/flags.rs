use crate::{eflags::Eflags, emu::Emu, flags::Flags};

impl Emu {
    pub fn pre_op_flags(&self) -> &Flags {
        &self.threads[self.current_thread_id].pre_op_flags
    }

    pub fn pre_op_flags_mut(&mut self) -> &mut Flags {
        &mut self.threads[self.current_thread_id].pre_op_flags
    }

    pub fn post_op_flags(&self) -> &Flags {
        &self.threads[self.current_thread_id].post_op_flags
    }

    pub fn post_op_flags_mut(&mut self) -> &mut Flags {
        &mut self.threads[self.current_thread_id].post_op_flags
    }

    pub fn eflags(&self) -> &Eflags {
        &self.threads[self.current_thread_id].eflags
    }

    pub fn eflags_mut(&mut self) -> &mut Eflags {
        &mut self.threads[self.current_thread_id].eflags
    }

    pub fn set_pre_op_flags(&mut self, flags: Flags) {
        self.threads[self.current_thread_id].pre_op_flags = flags;
    }

    pub fn set_post_op_flags(&mut self, flags: Flags) {
        self.threads[self.current_thread_id].post_op_flags = flags;
    }

    pub fn flags(&self) -> &Flags {
        &self.threads[self.current_thread_id].flags
    }

    pub fn flags_mut(&mut self) -> &mut Flags {
        &mut self.threads[self.current_thread_id].flags
    }
}
