use crate::tests::helpers;

#[test]
// test hooks system basic functionality
pub fn hooks_system() {
    helpers::setup();

    let mut hooks = crate::hooks::Hooks::new();

    // Test initial state - all hooks should be None
    assert!(hooks.hook_on_interrupt.is_none());
    assert!(hooks.hook_on_exception.is_none());
    assert!(hooks.hook_on_memory_read.is_none());
    assert!(hooks.hook_on_memory_write.is_none());
    assert!(hooks.hook_on_pre_instruction.is_none());
    assert!(hooks.hook_on_post_instruction.is_none());
    assert!(hooks.hook_on_winapi_call.is_none());

    // Test setting hooks
    hooks.hook_on_interrupt = Some(|_emu, _addr, _interrupt| true);
    assert!(hooks.hook_on_interrupt.is_some());

    hooks.hook_on_exception = Some(|_emu, _addr, _ex_type| true);
    assert!(hooks.hook_on_exception.is_some());

    hooks.hook_on_memory_read = Some(|_emu, _ip, _addr, _sz| {});
    assert!(hooks.hook_on_memory_read.is_some());

    hooks.hook_on_memory_write = Some(|_emu, _ip, _addr, _sz, value| value);
    assert!(hooks.hook_on_memory_write.is_some());

    hooks.hook_on_pre_instruction = Some(|_emu, _addr, _ins, _sz| true);
    assert!(hooks.hook_on_pre_instruction.is_some());

    hooks.hook_on_post_instruction = Some(|_emu, _addr, _ins, _sz, _ok| {});
    assert!(hooks.hook_on_post_instruction.is_some());

    hooks.hook_on_winapi_call = Some(|_emu, _addr, _called_addr| true);
    assert!(hooks.hook_on_winapi_call.is_some());

    // Test if all hooks are set
    assert!(!hooks.hook_on_interrupt.is_none());
    assert!(!hooks.hook_on_exception.is_none());
    assert!(!hooks.hook_on_memory_read.is_none());
    assert!(!hooks.hook_on_memory_write.is_none());
    assert!(!hooks.hook_on_pre_instruction.is_none());
    assert!(!hooks.hook_on_post_instruction.is_none());
    assert!(!hooks.hook_on_winapi_call.is_none());
}
