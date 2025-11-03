use std::collections::BTreeMap;

use crate::{eflags::Eflags, flags::Flags, fpu::FPU, regs64::Regs64};

#[derive(Clone)]
pub struct ThreadContext {
    pub id: u64,                    // Thread ID (e.g., 0x1000, 0x1001, etc.)
    pub suspended: bool,            // Whether thread is suspended
    pub wake_tick: usize,           // Global tick when thread can next run (0 = runnable)
    pub blocked_on_cs: Option<u64>, // Pointer to critical section if blocked
    pub regs: Regs64,
    pub pre_op_regs: Regs64,
    pub post_op_regs: Regs64,
    pub flags: Flags,
    pub pre_op_flags: Flags,
    pub post_op_flags: Flags,
    pub eflags: Eflags,
    pub fpu: FPU,
    pub seh: u64,
    pub veh: u64,
    pub uef: u64,
    pub eh_ctx: u32,
    pub tls32: Vec<u32>,
    pub tls64: Vec<u64>,
    pub fls: Vec<u32>,
    pub fs: BTreeMap<u64, u64>,
    pub call_stack: Vec<(u64, u64)>,
    pub handle: u64,
}

impl ThreadContext {
    pub fn new(id: u64) -> Self {
        ThreadContext {
            id,
            suspended: false,
            wake_tick: 0, // 0 means runnable
            blocked_on_cs: None,
            regs: Regs64::new(),
            pre_op_regs: Regs64::new(),
            post_op_regs: Regs64::new(),
            flags: Flags::new(),
            pre_op_flags: Flags::new(),
            post_op_flags: Flags::new(),
            eflags: Eflags::new(),
            fpu: FPU::new(),
            seh: 0,
            veh: 0,
            uef: 0,
            eh_ctx: 0,
            tls32: Vec::new(),
            tls64: Vec::new(),
            fls: Vec::new(),
            fs: BTreeMap::new(),
            call_stack: Vec::with_capacity(10000),
            handle: 0
        }
    }
}
