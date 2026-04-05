use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::eflags::Eflags;
use crate::flags::Flags;
use crate::regs64::Regs64;
use crate::serialization::fpu::SerializableFPU;
use crate::threading::context::{ArchThreadState, ThreadContext};

#[derive(Serialize, Deserialize)]
pub struct SerializableThreadContext {
    pub id: u64,
    pub suspended: bool,
    pub wake_tick: usize,
    pub blocked_on_cs: Option<u64>,
    pub handle: u64,
    // x86-specific fields (present only for x86 threads)
    pub regs: Regs64,
    pub pre_op_regs: Regs64,
    pub post_op_regs: Regs64,
    pub flags: Flags,
    pub pre_op_flags: Flags,
    pub post_op_flags: Flags,
    pub eflags: Eflags,
    pub fpu: SerializableFPU,
    pub seh: u64,
    pub veh: u64,
    pub uef: u64,
    pub eh_ctx: u32,
    pub tls32: Vec<u32>,
    pub tls64: Vec<u64>,
    pub fls: Vec<u32>,
    pub fs: BTreeMap<u64, u64>,
    pub call_stack: Vec<(u64, u64)>,
}

impl From<&ThreadContext> for SerializableThreadContext {
    fn from(thread: &ThreadContext) -> Self {
        match &thread.arch {
            ArchThreadState::X86 {
                regs, pre_op_regs, post_op_regs,
                flags, pre_op_flags, post_op_flags,
                eflags, fpu, seh, veh, uef, eh_ctx,
                tls32, tls64, fls, fs, call_stack,
            } => SerializableThreadContext {
                id: thread.id,
                suspended: thread.suspended,
                wake_tick: thread.wake_tick,
                blocked_on_cs: thread.blocked_on_cs,
                handle: thread.handle,
                regs: *regs,
                pre_op_regs: *pre_op_regs,
                post_op_regs: *post_op_regs,
                flags: *flags,
                pre_op_flags: *pre_op_flags,
                post_op_flags: *post_op_flags,
                eflags: eflags.clone(),
                fpu: fpu.clone().into(),
                seh: *seh,
                veh: *veh,
                uef: *uef,
                eh_ctx: *eh_ctx,
                tls32: tls32.clone(),
                tls64: tls64.clone(),
                fls: fls.clone(),
                fs: fs.clone(),
                call_stack: call_stack.clone(),
            },
            ArchThreadState::AArch64 { .. } => {
                // TODO: implement aarch64 serialization
                panic!("aarch64 thread serialization not yet implemented")
            }
        }
    }
}

impl From<SerializableThreadContext> for ThreadContext {
    fn from(serialized: SerializableThreadContext) -> Self {
        ThreadContext {
            id: serialized.id,
            suspended: serialized.suspended,
            wake_tick: serialized.wake_tick,
            blocked_on_cs: serialized.blocked_on_cs,
            handle: serialized.handle,
            arch: ArchThreadState::X86 {
                regs: serialized.regs,
                pre_op_regs: serialized.pre_op_regs,
                post_op_regs: serialized.post_op_regs,
                flags: serialized.flags,
                pre_op_flags: serialized.pre_op_flags,
                post_op_flags: serialized.post_op_flags,
                eflags: serialized.eflags,
                fpu: serialized.fpu.into(),
                seh: serialized.seh,
                veh: serialized.veh,
                uef: serialized.uef,
                eh_ctx: serialized.eh_ctx,
                tls32: serialized.tls32,
                tls64: serialized.tls64,
                fls: serialized.fls,
                fs: serialized.fs,
                call_stack: serialized.call_stack,
            },
        }
    }
}
