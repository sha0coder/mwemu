use crate::{eflags::Eflags, emu::Emu, flags::Flags, threading::context::ArchThreadState};

impl Emu {
    pub fn pre_op_flags(&self) -> &Flags {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { pre_op_flags, .. } => pre_op_flags,
            _ => panic!("pre_op_flags called on aarch64 emu"),
        }
    }

    pub fn pre_op_flags_mut(&mut self) -> &mut Flags {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { pre_op_flags, .. } => pre_op_flags,
            _ => panic!("pre_op_flags_mut called on aarch64 emu"),
        }
    }

    pub fn post_op_flags(&self) -> &Flags {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { post_op_flags, .. } => post_op_flags,
            _ => panic!("post_op_flags called on aarch64 emu"),
        }
    }

    pub fn post_op_flags_mut(&mut self) -> &mut Flags {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { post_op_flags, .. } => post_op_flags,
            _ => panic!("post_op_flags_mut called on aarch64 emu"),
        }
    }

    pub fn eflags(&self) -> &Eflags {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eflags, .. } => eflags,
            _ => panic!("eflags called on aarch64 emu"),
        }
    }

    pub fn eflags_mut(&mut self) -> &mut Eflags {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eflags, .. } => eflags,
            _ => panic!("eflags_mut called on aarch64 emu"),
        }
    }

    pub fn set_pre_op_flags(&mut self, new_flags: Flags) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { pre_op_flags, .. } => *pre_op_flags = new_flags,
            _ => panic!("set_pre_op_flags called on aarch64 emu"),
        }
    }

    pub fn set_post_op_flags(&mut self, new_flags: Flags) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { post_op_flags, .. } => *post_op_flags = new_flags,
            _ => panic!("set_post_op_flags called on aarch64 emu"),
        }
    }

    pub fn flags(&self) -> &Flags {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { flags, .. } => flags,
            _ => panic!("flags() called on aarch64 emu"),
        }
    }

    pub fn flags_mut(&mut self) -> &mut Flags {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { flags, .. } => flags,
            _ => panic!("flags_mut() called on aarch64 emu"),
        }
    }
}
