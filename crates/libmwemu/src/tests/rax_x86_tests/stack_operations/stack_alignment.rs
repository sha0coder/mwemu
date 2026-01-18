use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Comprehensive tests for stack alignment requirements
//
// In x86-64, the stack should be aligned to 16 bytes before CALL instructions
// This is required by the System V AMD64 ABI and Windows x64 ABI
// The CPU doesn't enforce this, but functions may assume it for performance

// ============================================================================
// Basic stack alignment tests
// ============================================================================

#[test]
fn test_stack_alignment_16_byte() {
    let mut emu = emu64();
    let code = [
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000; // 16-byte aligned
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "RSP is 16-byte aligned");
}

#[test]
fn test_stack_misalignment_8_byte() {
    let mut emu = emu64();
    let code = [
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1008; // 8-byte aligned, not 16-byte aligned
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_ne!(emu.regs().rsp & 0x0F, 0, "RSP is not 16-byte aligned");
    assert_eq!(emu.regs().rsp & 0x07, 0, "RSP is 8-byte aligned");
}

#[test]
fn test_single_push_breaks_alignment() {
    let mut emu = emu64();
    let code = [
        0x50, // PUSH RAX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000; // 16-byte aligned
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 8, "RSP offset by 8 from 16-byte boundary");
}

#[test]
fn test_two_pushes_restore_alignment() {
    let mut emu = emu64();
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000; // 16-byte aligned
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "RSP is 16-byte aligned after two pushes");
}

#[test]
fn test_odd_pushes_misalign() {
    let mut emu = emu64();
    let code = [
        0x50, 0x53, 0x51, // PUSH RAX, RBX, RCX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 8, "Three pushes misalign stack");
}

#[test]
fn test_four_pushes_aligned() {
    let mut emu = emu64();
    let code = [
        0x50, 0x53, 0x51, 0x52, // PUSH RAX, RBX, RCX, RDX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "Four pushes maintain alignment");
}

// ============================================================================
// Stack alignment with ENTER/LEAVE
// ============================================================================

#[test]
fn test_enter_maintains_alignment() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 8, "ENTER pushes RBP (8 bytes), misaligning");
}

#[test]
fn test_enter_with_even_allocation() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // PUSH RBP (8) + SUB RSP, 16 = 24 bytes total, misaligned
    assert_eq!(emu.regs().rsp & 0x0F, 8, "ENTER 16,0 results in misalignment");
}

#[test]
fn test_enter_with_alignment_padding() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // PUSH RBP (8) + SUB RSP, 8 = 16 bytes total, aligned
    assert_eq!(emu.regs().rsp & 0x0F, 0, "ENTER 8,0 maintains alignment");
}

#[test]
fn test_enter_leave_alignment_roundtrip() {
    let mut emu = emu64();
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "ENTER/LEAVE roundtrip preserves alignment");
}

// ============================================================================
// Stack alignment with immediate pushes
// ============================================================================

#[test]
fn test_push_imm_alignment() {
    let mut emu = emu64();
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 8, "PUSH immediate breaks alignment");
}

#[test]
fn test_multiple_push_imm_pattern() {
    let mut emu = emu64();
    let code = [
        0x6a, 0x01, 0x6a, 0x02, 0x6a, 0x03, 0x6a, 0x04,
        0x6a, 0x05, 0x6a, 0x06, 0x6a, 0x07, 0x6a, 0x08,
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 8 pushes * 8 bytes = 64 bytes, maintains alignment
    assert_eq!(emu.regs().rsp & 0x0F, 0, "Eight pushes maintain alignment");
}

// ============================================================================
// Stack alignment with SUB RSP
// ============================================================================

#[test]
fn test_sub_rsp_16_aligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "SUB RSP, 16 maintains alignment");
}

#[test]
fn test_sub_rsp_8_misaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 8, "SUB RSP, 8 breaks alignment");
}

#[test]
fn test_sub_rsp_32_aligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xec, 0x20, // SUB RSP, 32
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "SUB RSP, 32 maintains alignment");
}

#[test]
fn test_sub_rsp_48_aligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xec, 0x30, // SUB RSP, 48
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "SUB RSP, 48 maintains alignment");
}

#[test]
fn test_sub_rsp_odd_value() {
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xec, 0x18, // SUB RSP, 24
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 8, "SUB RSP, 24 breaks alignment");
}

// ============================================================================
// Stack alignment with AND instruction
// ============================================================================

#[test]
fn test_align_stack_with_and() {
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xe4, 0xf0, // AND RSP, 0xFFFFFFFFFFFFFFF0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1008; // Misaligned
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "AND aligns stack to 16 bytes");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack aligned down");
}

#[test]
fn test_align_already_aligned_stack() {
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xe4, 0xf0, // AND RSP, 0xFFFFFFFFFFFFFFF0
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000; // Already aligned
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "Already aligned stack unchanged");
}

// ============================================================================
// Function prologue/epilogue alignment patterns
// ============================================================================

#[test]
fn test_function_prologue_alignment() {
    let mut emu = emu64();
    let code = [
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // PUSH RBP (8) + SUB 16 = 24 bytes, misaligned
    assert_eq!(emu.regs().rsp & 0x0F, 8, "Standard prologue results in misalignment");
}

#[test]
fn test_function_prologue_with_saved_regs() {
    let mut emu = emu64();
    let code = [
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x53, // PUSH RBX (callee-save)
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8 (locals)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // PUSH RBP (8) + PUSH RBX (8) + SUB 8 = 24 bytes, misaligned
    assert_eq!(emu.regs().rsp & 0x0F, 8, "Prologue with saved reg misaligned");
}

#[test]
fn test_function_prologue_aligned_locals() {
    let mut emu = emu64();
    let code = [
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8 (adjust for alignment)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // PUSH RBP (8) + SUB 8 = 16 bytes, aligned
    assert_eq!(emu.regs().rsp & 0x0F, 0, "Aligned prologue with 8-byte adjust");
}

// ============================================================================
// Stack alignment restoration
// ============================================================================

#[test]
fn test_push_pop_alignment_restoration() {
    let mut emu = emu64();
    let code = [
        0x50, // PUSH RAX (break alignment)
        0x58, // POP RAX (restore alignment)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "PUSH/POP restores alignment");
}

#[test]
fn test_sub_add_alignment_restoration() {
    let mut emu = emu64();
    let code = [
        0x48, 0x83, 0xec, 0x18, // SUB RSP, 24 (misalign)
        0x48, 0x83, 0xc4, 0x18, // ADD RSP, 24 (restore)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "SUB/ADD restores alignment");
}

// ============================================================================
// Complex alignment scenarios
// ============================================================================

#[test]
fn test_nested_function_alignment() {
    let mut emu = emu64();
    let code = [
        // Outer function
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        // Inner function
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "Nested functions maintain alignment");
}

#[test]
fn test_alignment_with_parameter_passing() {
    let mut emu = emu64();
    let code = [
        // Push 6 parameters (odd number)
        0x6a, 0x01, 0x6a, 0x02, 0x6a, 0x03,
        0x6a, 0x04, 0x6a, 0x05, 0x6a, 0x06,
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 6 * 8 = 48 bytes, maintains alignment
    assert_eq!(emu.regs().rsp & 0x0F, 0, "Six parameters maintain alignment");
}

#[test]
fn test_alignment_with_odd_parameters() {
    let mut emu = emu64();
    let code = [
        // Push 3 parameters (odd number)
        0x6a, 0x01, 0x6a, 0x02, 0x6a, 0x03,
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 3 * 8 = 24 bytes, breaks alignment
    assert_eq!(emu.regs().rsp & 0x0F, 8, "Three parameters break alignment");
}

// ============================================================================
// Stack alignment verification utilities
// ============================================================================

#[test]
fn test_check_multiple_alignment_points() {
    let mut emu = emu64();
    let code = [
        0x50, // PUSH RAX (point 1: misaligned)
        0x50, // PUSH RAX (point 2: aligned)
        0x50, // PUSH RAX (point 3: misaligned)
        0x50, // PUSH RAX (point 4: aligned)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "Four pushes result in alignment");
}

#[test]
fn test_alignment_at_various_addresses() {
    let mut emu = emu64();
    let code = [0xf4]; // HLT

    for addr in [0x1000, 0x2000, 0x3000, 0x4000, 0x10000, 0x100000].iter() {
        emu.regs_mut().rsp = *addr;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rsp & 0x0F, 0, "Address 0x{:x} is 16-byte aligned", addr);
    }
}

#[test]
fn test_pushfq_popfq_alignment() {
    let mut emu = emu64();
    let code = [
        0x48, 0x9c, // PUSHFQ (8 bytes)
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "PUSHFQ/POPFQ preserves alignment");
}

#[test]
fn test_large_stack_frame_alignment() {
    let mut emu = emu64();
    let code = [
        0x48, 0x81, 0xec, 0x00, 0x10, 0x00, 0x00, // SUB RSP, 0x1000 (4096 bytes)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x10000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp & 0x0F, 0, "Large allocation maintains alignment");
    assert_eq!(emu.regs().rsp, 0xF000, "4KB frame allocated");
}

#[test]
fn test_alignment_after_mixed_operations() {
    let mut emu = emu64();
    let code = [
        0x50, // PUSH RAX
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0x53, // PUSH RBX
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // PUSH(8) + SUB(8) + PUSH(8) + SUB(8) = 32 bytes, aligned
    assert_eq!(emu.regs().rsp & 0x0F, 0, "Mixed operations maintain alignment");
}
