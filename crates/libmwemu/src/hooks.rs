use crate::emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::exception::types;

// Hook types using Box<dyn FnMut> for state capture instead of bare fn pointers.
// return: false will ignore interrupt handling like 0x80 -> linux
pub type TypeHookOnInterrupt =
    Box<dyn FnMut(&mut emu::Emu, u64, u64) -> bool>;
// return: allow handle exception?
pub type TypeHookOnException =
    Box<dyn FnMut(&mut emu::Emu, u64, types::ExceptionType) -> bool>;
// memory read is pre-read you can modify the value that is going to be read.
pub type TypeHookOnMemoryRead =
    Box<dyn FnMut(&mut emu::Emu, u64, u64, u32)>;
// the memory write is pre but you can change the value is going to be written.
pub type TypeHookOnMemoryWrite =
    Box<dyn FnMut(&mut emu::Emu, u64, u64, u32, u128) -> u128>;

// [BREAKING API CHANGE] returning false will skip the handling of the instruction
// Changed from &iced_x86::Instruction to &DecodedInstruction for arch parity.
// x86 hook users call ins.as_x86() to get the familiar type.
pub type TypeHookOnPreInstruction =
    Box<dyn FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize) -> bool>;

pub type TypeHookOnPostInstruction =
    Box<dyn FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize, bool)>;
pub type TypeHookOnWinApiCall =
    Box<dyn FnMut(&mut emu::Emu, u64, u64) -> bool>;

pub struct Hooks {
    pub hook_on_interrupt: Option<TypeHookOnInterrupt>,
    pub hook_on_exception: Option<TypeHookOnException>,
    pub hook_on_memory_read: Option<TypeHookOnMemoryRead>,
    pub hook_on_memory_write: Option<TypeHookOnMemoryWrite>,
    pub hook_on_pre_instruction: Option<TypeHookOnPreInstruction>,
    pub hook_on_post_instruction: Option<TypeHookOnPostInstruction>,
    pub hook_on_winapi_call: Option<TypeHookOnWinApiCall>,
}

impl Default for Hooks {
    fn default() -> Self {
        Self::new()
    }
}

impl Hooks {
    pub fn new() -> Hooks {
        Hooks {
            hook_on_interrupt: None,
            hook_on_exception: None,
            hook_on_memory_read: None,
            hook_on_memory_write: None,
            hook_on_pre_instruction: None,
            hook_on_post_instruction: None,
            hook_on_winapi_call: None,
        }
    }

    pub fn on_interrupt(&mut self, hook: impl FnMut(&mut emu::Emu, u64, u64) -> bool + 'static) {
        self.hook_on_interrupt = Some(Box::new(hook));
    }

    pub fn disable_interrupt(&mut self) {
        self.hook_on_interrupt = None;
    }

    pub fn on_exception(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, types::ExceptionType) -> bool + 'static,
    ) {
        self.hook_on_exception = Some(Box::new(hook));
    }

    pub fn disable_exception(&mut self) {
        self.hook_on_exception = None;
    }

    pub fn on_memory_read(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64, u32) + 'static,
    ) {
        self.hook_on_memory_read = Some(Box::new(hook));
    }

    pub fn disable_memory_read(&mut self) {
        self.hook_on_memory_read = None;
    }

    pub fn on_memory_write(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64, u32, u128) -> u128 + 'static,
    ) {
        self.hook_on_memory_write = Some(Box::new(hook));
    }

    pub fn disable_memory_write(&mut self) {
        self.hook_on_memory_write = None;
    }

    pub fn on_pre_instruction(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize) -> bool + 'static,
    ) {
        self.hook_on_pre_instruction = Some(Box::new(hook));
    }

    pub fn disable_pre_instruction(&mut self) {
        self.hook_on_pre_instruction = None;
    }

    pub fn on_post_instruction(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize, bool) + 'static,
    ) {
        self.hook_on_post_instruction = Some(Box::new(hook));
    }

    pub fn disable_post_instruction(&mut self) {
        self.hook_on_post_instruction = None;
    }

    pub fn on_winapi_call(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64) -> bool + 'static,
    ) {
        self.hook_on_winapi_call = Some(Box::new(hook));
    }

    pub fn disable_winapi_call(&mut self) {
        self.hook_on_winapi_call = None;
    }
}
