use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::eflags::Eflags;
use crate::flags::Flags;
use crate::regs64::Regs64;
use crate::serialization::fpu::SerializableFPU;
use crate::thread_context::ThreadContext;

#[derive(Serialize, Deserialize)]
pub struct SerializableThreadContext {
    pub id: u64,
    pub suspended: bool,
    pub wake_tick: usize,
    pub blocked_on_cs: Option<u64>,
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
    pub call_stack: Vec<(u64, u64)>, // the first address is the source of the call location and the second address is the destination of the call
    pub handle: u64
}

impl From<&ThreadContext> for SerializableThreadContext {
    fn from(thread: &ThreadContext) -> Self {
        SerializableThreadContext {
            id: thread.id,
            suspended: thread.suspended,
            wake_tick: thread.wake_tick,
            blocked_on_cs: thread.blocked_on_cs,
            regs: thread.regs,
            pre_op_regs: thread.pre_op_regs,
            post_op_regs: thread.post_op_regs,
            flags: thread.flags,
            pre_op_flags: thread.pre_op_flags,
            post_op_flags: thread.post_op_flags,
            eflags: thread.eflags.clone(),
            fpu: thread.fpu.clone().into(),
            seh: thread.seh,
            veh: thread.veh,
            uef: thread.uef,
            eh_ctx: thread.eh_ctx,
            tls32: thread.tls32.clone(),
            tls64: thread.tls64.clone(),
            fls: thread.fls.clone(),
            fs: thread.fs.clone(),
            call_stack: thread.call_stack.clone(),
            handle: thread.handle
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
            handle: serialized.handle,
        }
    }
}
