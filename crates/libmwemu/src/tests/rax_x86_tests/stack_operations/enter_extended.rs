use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Comprehensive tests for ENTER instruction
//
// ENTER - Create Stack Frame for Procedure Parameters
// Opcode: C8 iw ib
// Format: ENTER imm16, imm8
//
// Creates a stack frame for a procedure with:
// - imm16: Amount of local variable space to allocate
// - imm8: Nesting level (0-31)
//
// ENTER performs:
// 1. PUSH RBP
// 2. Set frame pointer to current stack pointer
// 3. If nesting level > 0, push frame pointers from outer levels
// 4. Subtract allocation size from RSP

// ============================================================================
// ENTER with nesting level 0 (most common case)
// ============================================================================

#[test]
fn test_enter_basic_no_nesting() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // ENTER 0,0 should: PUSH RBP, MOV RBP, RSP
    assert_eq!(emu.regs().rsp, 0x1000 - 8, "RSP decremented by 8 (pushed RBP)");
    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP = RSP after PUSH");

    let saved_rbp = emu.maps.read_qword(0x1000 - 8).unwrap();
    assert_eq!(saved_rbp, 0x2000, "Old RBP saved on stack");
}

#[test]
fn test_enter_allocate_8_bytes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP set after pushing");
    assert_eq!(emu.regs().rsp, 0x1000 - 16, "RSP = RBP - 8 (allocated space)");

    let saved_rbp = emu.maps.read_qword(0x1000 - 8).unwrap();
    assert_eq!(saved_rbp, 0x2000, "Old RBP saved");
}

#[test]
fn test_enter_allocate_16_bytes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x1000 - 24, "RSP = RBP - 16");

    let saved_rbp = emu.maps.read_qword(emu.regs().rbp).unwrap();
    assert_eq!(saved_rbp, 0x2000, "Old RBP saved");
}

#[test]
fn test_enter_allocate_32_bytes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x1000 - 40, "RSP = RBP - 32");
}

#[test]
fn test_enter_allocate_64_bytes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x40, 0x00, 0x00, // ENTER 64, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x1000 - 72, "RSP = RBP - 64");
}

#[test]
fn test_enter_allocate_128_bytes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x80, 0x00, 0x00, // ENTER 128, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x1000 - 136, "RSP = RBP - 128");
}

#[test]
fn test_enter_allocate_256_bytes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x01, 0x00, // ENTER 256, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x1000 - 264, "RSP = RBP - 256");
}

#[test]
fn test_enter_allocate_1024_bytes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x04, 0x00, // ENTER 1024, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x2000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x2000 - 1032, "RSP = RBP - 1024");
}

#[test]
fn test_enter_allocate_max_16bit() {
    let mut emu = emu64();
    let code = [
        0xc8, 0xff, 0xff, 0x00, // ENTER 65535, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x20000;
    emu.regs_mut().rbp = 0x30000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x20000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x20000 - 8 - 65535, "RSP = RBP - 65535");
}

// ============================================================================
// ENTER with nesting level 1
// ============================================================================

#[test]
fn test_enter_nesting_level_1_no_alloc() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x01, // ENTER 0, 1
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 1. PUSH RBP (old RBP = 0x2000)
    // 2. temp = RSP
    // 3. PUSH [temp] (push the old RBP again)
    // 4. RBP = temp
    // 5. RSP -= 0

    assert_eq!(emu.regs().rsp, 0x1000 - 16, "RSP decremented by 16");

    let first_push = emu.maps.read_qword(0x1000 - 8).unwrap();
    assert_eq!(first_push, 0x2000, "First push is old RBP");
}

#[test]
fn test_enter_nesting_level_1_with_alloc() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x01, // ENTER 16, 1
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 32, "RSP decremented by 32");

    let saved_rbp = emu.maps.read_qword(0x1000 - 8).unwrap();
    assert_eq!(saved_rbp, 0x2000, "Old RBP saved");
}

// ============================================================================
// ENTER with various nesting levels
// ============================================================================

#[test]
fn test_enter_nesting_level_2() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x02, // ENTER 0, 2
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x3000, 0x4000);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000 - 24, "RSP decremented by 24");
}

#[test]
fn test_enter_nesting_level_3() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x08, 0x00, 0x03, // ENTER 8, 3
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x3000, 0x4000);
    emu.maps.write_qword(0x4000, 0x5000);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000 - 40, "RSP decremented by 40");
}

#[test]
fn test_enter_nesting_level_4_with_alloc() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x04, // ENTER 32, 4
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000 - 72, "RSP decremented by 72");
}

#[test]
fn test_enter_nesting_level_5() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x05, // ENTER 0, 5
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x3000;
    emu.regs_mut().rbp = 0x4000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3000 - 48, "RSP decremented by 48");
}

// ============================================================================
// ENTER preserves other registers
// ============================================================================

#[test]
fn test_enter_preserves_other_registers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x11, "RAX unchanged");
    assert_eq!(emu.regs().rbx, 0x22, "RBX unchanged");
    assert_eq!(emu.regs().rcx, 0x33, "RCX unchanged");
}

#[test]
fn test_enter_preserves_flags() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC (set carry)
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should be preserved");
}

// ============================================================================
// ENTER followed by LEAVE
// ============================================================================

#[test]
fn test_enter_leave_roundtrip() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored");
}

#[test]
fn test_enter_leave_nested() {
    let mut emu = emu64();
    let code = [
        // Outer function
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Inner function
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE (inner)
        0xc9, // LEAVE (outer)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored after nested ENTER/LEAVE");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored after nested ENTER/LEAVE");
}

// ============================================================================
// ENTER with different stack positions
// ============================================================================

#[test]
fn test_enter_high_stack_address() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x100000;
    emu.regs_mut().rbp = 0x200000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x100000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x100000 - 40, "RSP decremented");

    let saved_rbp = emu.maps.read_qword(emu.regs().rbp).unwrap();
    assert_eq!(saved_rbp, 0x200000, "Old RBP saved");
}

#[test]
fn test_enter_low_stack_address() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x100;
    emu.regs_mut().rbp = 0x200;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x100 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x100 - 24, "RSP decremented");

    let saved_rbp = emu.maps.read_qword(emu.regs().rbp).unwrap();
    assert_eq!(saved_rbp, 0x200, "Old RBP saved");
}

// ============================================================================
// ENTER multiple times (function call chain)
// ============================================================================

#[test]
fn test_enter_multiple_calls() {
    let mut emu = emu64();
    let code = [
        // First function
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Second function
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        // Third function
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000 - 80, "RSP after three ENTERs");
}

#[test]
fn test_enter_with_odd_allocation() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x11, 0x00, 0x00, // ENTER 17, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x1000 - 25, "RSP = RBP - 17");
}

#[test]
fn test_enter_allocation_alignment() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x18, 0x00, 0x00, // ENTER 24, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP set");
    assert_eq!(emu.regs().rsp, 0x1000 - 32, "RSP = RBP - 24");
}

// ============================================================================
// ENTER with frame pointer chain
// ============================================================================

#[test]
fn test_enter_frame_pointer_chain() {
    let mut emu = emu64();
    let code = [
        // First frame
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        // Second frame
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        // Third frame
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000 - 24, "Three frames created");

    let frame3_rbp = emu.regs().rbp;
    let frame2_rbp = emu.maps.read_qword(frame3_rbp).unwrap();
    let frame1_rbp = emu.maps.read_qword(frame2_rbp).unwrap();
    let original_rbp = emu.maps.read_qword(frame1_rbp).unwrap();

    assert_eq!(original_rbp, 0x3000, "Original RBP at end of chain");
}

// ============================================================================
// ENTER with practical function prologue patterns
// ============================================================================

#[test]
fn test_enter_small_local_vars() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x18, 0x00, 0x00, // ENTER 24, 0 (3 x 8-byte vars)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "Frame pointer set");
    assert_eq!(emu.regs().rsp, 0x1000 - 32, "Space for 3 local vars");
}

#[test]
fn test_enter_large_local_array() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x02, 0x00, // ENTER 512, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x2000 - 8, "Frame pointer set");
    assert_eq!(emu.regs().rsp, 0x2000 - 520, "Space for 512-byte buffer");
}

#[test]
fn test_enter_no_locals() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "Frame pointer set");
    assert_eq!(emu.regs().rsp, 0x1000 - 8, "No local space allocated");
}

// ============================================================================
// ENTER with subsequent stack operations
// ============================================================================

#[test]
fn test_enter_then_push() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x50, // PUSH RAX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 32, "RSP after ENTER and PUSH");

    let pushed_val = emu.maps.read_qword(emu.regs().rsp).unwrap();
    assert_eq!(pushed_val, 0x42, "Pushed value on stack");
}

#[test]
fn test_enter_then_mov_to_local() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0x89, 0x45, 0xf8, // MOV [RBP-8], RAX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let local_var = emu.maps.read_qword(emu.regs().rbp - 8).unwrap();
    assert_eq!(local_var, 0xAA, "Local variable set correctly");
}

#[test]
fn test_enter_with_parameter_access() {
    let mut emu = emu64();
    let code = [
        // Simulate: parameters pushed before call, then ENTER
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x50, // PUSH RAX (parameter)
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Access parameter at [RBP+16] (pushed param + return addr + old RBP)
        0x48, 0x8b, 0x5d, 0x10, // MOV RBX, [RBP+16]
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_ne!(emu.regs().rbx, 0, "Parameter accessed through RBP");
}

// ============================================================================
// ENTER with different nesting patterns
// ============================================================================

#[test]
fn test_enter_nesting_level_10() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x0a, // ENTER 0, 10
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x3000;
    emu.regs_mut().rbp = 0x4000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3000 - 88, "RSP with nesting level 10");
}

#[test]
fn test_enter_nesting_level_16() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x10, // ENTER 0, 16
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x4000;
    emu.regs_mut().rbp = 0x5000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x4000 - 136, "RSP with nesting level 16");
}

#[test]
fn test_enter_nesting_level_31() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x1f, // ENTER 0, 31
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x5000;
    emu.regs_mut().rbp = 0x6000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x5000 - 256, "RSP with max nesting level 31");
}

// ============================================================================
// ENTER edge cases
// ============================================================================

#[test]
fn test_enter_zero_allocation_zero_nesting() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 8, "Only RBP pushed");
    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP = RSP");
}

#[test]
fn test_enter_with_same_rbp_rsp() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x1000; // RBP == RSP
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000 - 8, "RBP updated");

    let saved_rbp = emu.maps.read_qword(emu.regs().rbp).unwrap();
    assert_eq!(saved_rbp, 0x1000, "Old RBP (which was RSP) saved");
}

#[test]
fn test_enter_consecutive_same_size() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000 - 120, "Three identical ENTERs");
}

#[test]
fn test_enter_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc8, 0x40, 0x00, 0x00, // ENTER 64, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x3000;
    emu.regs_mut().rbp = 0x4000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 16 + 24 + 40 + 72 = 152
    assert_eq!(emu.regs().rsp, 0x3000 - 152, "Mixed size allocations");
}
