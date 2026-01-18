use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Comprehensive tests for LEAVE instruction
//
// LEAVE - High Level Procedure Exit
// Opcode: C9
//
// LEAVE performs:
// 1. MOV RSP, RBP (restore stack pointer to frame pointer)
// 2. POP RBP (restore old frame pointer)
//
// This instruction is the complement of ENTER for function epilogues

// ============================================================================
// Basic LEAVE functionality
// ============================================================================

#[test]
fn test_leave_basic() {
    let mut emu = emu64();
    let code = [
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2FF8; // Stack pointer below frame pointer
    emu.regs_mut().rbp = 0x3000; // Frame pointer
    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x3000, 0x4000);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3008, "RSP = old RBP + 8");
    assert_eq!(emu.regs().rbp, 0x4000, "RBP restored from stack");
}

#[test]
fn test_leave_after_enter() {
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

    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored to original");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored to original");
}

#[test]
fn test_leave_with_local_variables() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0 (allocate space for locals)
        // Simulate using local variables
        0x48, 0xc7, 0x45, 0xf8, 0xaa, 0x00, 0x00, 0x00, // MOV QWORD [RBP-8], 0xAA
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored despite local var usage");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored");
}

#[test]
fn test_leave_small_frame() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
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
fn test_leave_large_frame() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x04, 0x00, // ENTER 1024, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "RSP restored after large allocation");
    assert_eq!(emu.regs().rbp, 0x3000, "RBP restored");
}

// ============================================================================
// LEAVE with nested function calls
// ============================================================================

#[test]
fn test_leave_nested_functions() {
    let mut emu = emu64();
    let code = [
        // Outer function
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        // Inner function
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE (inner)
        0xc9, // LEAVE (outer)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP fully restored after nested calls");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP fully restored after nested calls");
}

#[test]
fn test_leave_triple_nested() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0 (func1)
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0 (func2)
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0 (func3)
        0xc9, // LEAVE (func3)
        0xc9, // LEAVE (func2)
        0xc9, // LEAVE (func1)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "RSP restored through 3 levels");
    assert_eq!(emu.regs().rbp, 0x3000, "RBP restored through 3 levels");
}

#[test]
fn test_leave_deep_nesting() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x3000;
    emu.regs_mut().rbp = 0x4000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3000, "RSP restored through deep nesting");
    assert_eq!(emu.regs().rbp, 0x4000, "RBP restored through deep nesting");
}

// ============================================================================
// LEAVE preserves other registers
// ============================================================================

#[test]
fn test_leave_preserves_registers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
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
fn test_leave_preserves_flags() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC (set carry)
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should be preserved");
}

// ============================================================================
// LEAVE with manual stack frame setup
// ============================================================================

#[test]
fn test_leave_manual_frame() {
    let mut emu = emu64();
    let code = [
        // Manual ENTER equivalent
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16
        // LEAVE should undo all of this
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
fn test_leave_with_push_pop_in_function() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        // Function body would be here
        0x5b, // POP RBX
        0x58, // POP RAX
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.regs_mut().rax = 0xAAAA;
    emu.regs_mut().rbx = 0xBBBB;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored");
    assert_eq!(emu.regs().rax, 0xAAAA, "RAX restored via PUSH/POP");
    assert_eq!(emu.regs().rbx, 0xBBBB, "RBX restored via PUSH/POP");
}

// ============================================================================
// LEAVE without corresponding ENTER
// ============================================================================

#[test]
fn test_leave_standalone() {
    let mut emu = emu64();
    let code = [
        0xc9, // LEAVE (without prior ENTER)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2FF8;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x3000, 0x4000);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3008, "RSP = RBP + 8");
    assert_eq!(emu.regs().rbp, 0x4000, "RBP from stack");
}

#[test]
fn test_leave_with_modified_stack() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        // Manually adjust RSP
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0xc9, // LEAVE (should still work correctly)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // LEAVE uses RBP, not RSP, so RSP modification doesn't matter
    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored via RBP");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored");
}

// ============================================================================
// LEAVE at different stack positions
// ============================================================================

#[test]
fn test_leave_high_stack_address() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x100000;
    emu.regs_mut().rbp = 0x200000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x100000, "RSP restored at high address");
    assert_eq!(emu.regs().rbp, 0x200000, "RBP restored");
}

#[test]
fn test_leave_low_stack_address() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x100;
    emu.regs_mut().rbp = 0x200;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x100, "RSP restored at low address");
    assert_eq!(emu.regs().rbp, 0x200, "RBP restored");
}

// ============================================================================
// LEAVE with zero-sized frames
// ============================================================================

#[test]
fn test_leave_zero_sized_frame() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
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

// ============================================================================
// LEAVE in typical function epilogue patterns
// ============================================================================

#[test]
fn test_leave_typical_epilogue() {
    let mut emu = emu64();
    let code = [
        // Prologue
        0xc8, 0x18, 0x00, 0x00, // ENTER 24, 0
        // Function body
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42 (return value)
        // Epilogue
        0xc9, // LEAVE
        0xf4, // HLT (would be RET)
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x42, "Return value set");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack cleaned up");
    assert_eq!(emu.regs().rbp, 0x2000, "Frame pointer restored");
}

#[test]
fn test_leave_with_saved_registers() {
    let mut emu = emu64();
    let code = [
        // Prologue
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x50, // PUSH RAX (callee-save)
        0x53, // PUSH RBX (callee-save)
        // Function body
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        // Epilogue
        0x5b, // POP RBX
        0x58, // POP RAX
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.regs_mut().rax = 0xAAAA;
    emu.regs_mut().rbx = 0xBBBB;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xAAAA, "RAX restored");
    assert_eq!(emu.regs().rbx, 0xBBBB, "RBX restored");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack balanced");
}

// ============================================================================
// LEAVE multiple times in sequence
// ============================================================================

#[test]
fn test_leave_sequence() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0 again
        0xc9, // LEAVE again
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored after sequence");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored after sequence");
}

// ============================================================================
// LEAVE with frame pointer chain verification
// ============================================================================

#[test]
fn test_leave_frame_chain() {
    let mut emu = emu64();
    let code = [
        // Create frame chain
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0 (frame 1)
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0 (frame 2)
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0 (frame 3)
        // Unwind chain
        0xc9, // LEAVE (frame 3)
        0xc9, // LEAVE (frame 2)
        0xc9, // LEAVE (frame 1)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "Full chain unwound");
    assert_eq!(emu.regs().rbp, 0x3000, "Original frame pointer restored");
}

#[test]
fn test_leave_with_varying_frames() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x40, 0x00, 0x00, // ENTER 64, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "All frames unwound");
    assert_eq!(emu.regs().rbp, 0x3000, "Frame pointer fully restored");
}

// ============================================================================
// LEAVE edge cases
// ============================================================================

#[test]
fn test_leave_rbp_equals_rsp() {
    let mut emu = emu64();
    let code = [
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x3000;
    emu.regs_mut().rbp = 0x3000; // RBP == RSP
    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x3000, 0x4000);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3008, "RSP incremented");
    assert_eq!(emu.regs().rbp, 0x4000, "RBP loaded from stack");
}

#[test]
fn test_leave_with_zero_saved_rbp() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x0000; // RBP is zero
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x0000, "Zero RBP restored");
}

#[test]
fn test_leave_stack_grows_correctly() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x01, 0x00, // ENTER 256, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "Large frame properly unwound");
}

// ============================================================================
// LEAVE with interleaved operations
// ============================================================================

#[test]
fn test_leave_after_stack_arithmetic() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        // Do some stack operations
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16 (allocate more)
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16 (deallocate)
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "LEAVE restores correctly despite SUB/ADD");
}

#[test]
fn test_leave_after_many_pushes() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x50, 0x50, 0x50, 0x50, // PUSH RAX x4
        0x58, 0x58, 0x58, 0x58, // POP RAX x4
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "Stack balanced after PUSH/POP");
}

// ============================================================================
// LEAVE in real-world scenarios
// ============================================================================

#[test]
fn test_leave_recursive_function_simulation() {
    let mut emu = emu64();
    let code = [
        // Recursion level 1
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Recursion level 2
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Recursion level 3
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Unwind
        0xc9, // LEAVE (level 3)
        0xc9, // LEAVE (level 2)
        0xc9, // LEAVE (level 1)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x3000;
    emu.regs_mut().rbp = 0x4000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3000, "Recursion fully unwound");
    assert_eq!(emu.regs().rbp, 0x4000, "Base frame restored");
}

#[test]
fn test_leave_with_mixed_operations() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        // Save registers
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        // Do work
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        // Restore registers
        0x5b, // POP RBX
        0x58, // POP RAX
        // Clean up frame
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.regs_mut().rax = 0x1111;
    emu.regs_mut().rbx = 0x2222;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1111, "RAX restored by POP");
    assert_eq!(emu.regs().rbx, 0x2222, "RBX restored by POP");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack cleaned by LEAVE");
}

#[test]
fn test_leave_exception_handler_pattern() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0 (exception handler frame)
        // Handler code
        0x48, 0xc7, 0x45, 0xf8, 0xff, 0x00, 0x00, 0x00, // MOV [RBP-8], 0xFF
        // Exit handler
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "Handler frame unwound");
}

#[test]
fn test_leave_tail_call_preparation() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Prepare for tail call
        0xc9, // LEAVE (clean up current frame before tail call)
        // Would JMP to tail call target here
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "Frame cleaned for tail call");
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored for tail call");
}

#[test]
fn test_leave_coroutine_switch_pattern() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        // Save context
        0x50, 0x53, 0x51, 0x52, // PUSH RAX, RBX, RCX, RDX
        // Restore context
        0x5a, 0x59, 0x5b, 0x58, // POP RDX, RCX, RBX, RAX
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.regs_mut().rbp = 0x3000;
    emu.regs_mut().rax = 0xAA;
    emu.regs_mut().rbx = 0xBB;
    emu.regs_mut().rcx = 0xCC;
    emu.regs_mut().rdx = 0xDD;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "Context switch cleaned up");
    assert_eq!(emu.regs().rax, 0xAA, "Registers preserved");
}
