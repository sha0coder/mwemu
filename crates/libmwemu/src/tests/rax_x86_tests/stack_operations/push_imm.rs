use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Comprehensive tests for PUSH with immediate operands
//
// PUSH imm8 - Opcode: 6A ib (sign-extended to 64-bit)
// PUSH imm32 - Opcode: 68 id (sign-extended to 64-bit)
//
// In 64-bit mode, both push 64-bit values (sign-extended from their size)

// ============================================================================
// PUSH imm8 - 8-bit immediate sign-extended to 64-bit
// ============================================================================

#[test]
fn test_push_imm8_zero() {
    let code = [
        0x6a, 0x00, // PUSH 0
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP decremented by 8");
    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0, "Zero pushed");
}

#[test]
fn test_push_imm8_positive_small() {
    let code = [
        0x6a, 0x01, // PUSH 1
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 1, "Small positive value");
}

#[test]
fn test_push_imm8_positive_medium() {
    let code = [
        0x6a, 0x42, // PUSH 0x42 (66)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x42, "Medium positive value");
}

#[test]
fn test_push_imm8_max_positive() {
    let code = [
        0x6a, 0x7f, // PUSH 127 (max positive 8-bit signed)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 127, "Max positive 8-bit signed");
}

#[test]
fn test_push_imm8_negative_one() {
    let code = [
        0x6a, 0xff, // PUSH -1 (0xFF sign-extended)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFFFF, "-1 sign-extended to 64-bit");
}

#[test]
fn test_push_imm8_negative_small() {
    let code = [
        0x6a, 0xfe, // PUSH -2
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFFFE, "-2 sign-extended");
}

#[test]
fn test_push_imm8_negative_medium() {
    let code = [
        0x6a, 0xf0, // PUSH -16
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFFF0, "-16 sign-extended");
}

#[test]
fn test_push_imm8_min_negative() {
    let code = [
        0x6a, 0x80, // PUSH -128 (min 8-bit signed)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFF80, "-128 sign-extended");
}

#[test]
fn test_push_imm8_boundary_values() {
    let code = [
        0x6a, 0x7f, // PUSH 127
        0x6a, 0x80, // PUSH -128
        0x6a, 0x00, // PUSH 0
        0x6a, 0xff, // PUSH -1
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 32, "Four values pushed");
    assert_eq!(emu.maps.read_qword(0x1000 - 8).unwrap(), 127, "First push");
    assert_eq!(
        emu.maps.read_qword(0x1000 - 16).unwrap(),
        0xFFFFFFFFFFFFFF80,
        "Second push"
    );
    assert_eq!(emu.maps.read_qword(0x1000 - 24).unwrap(), 0, "Third push");
    assert_eq!(
        emu.maps.read_qword(0x1000 - 32).unwrap(),
        0xFFFFFFFFFFFFFFFF,
        "Fourth push"
    );
}

// ============================================================================
// PUSH imm32 - 32-bit immediate sign-extended to 64-bit
// ============================================================================

#[test]
fn test_push_imm32_zero() {
    let code = [
        0x68, 0x00, 0x00, 0x00, 0x00, // PUSH 0
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP decremented by 8");
    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0, "Zero pushed");
}

#[test]
fn test_push_imm32_small_positive() {
    let code = [
        0x68, 0x01, 0x00, 0x00, 0x00, // PUSH 1
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 1, "Small positive value");
}

#[test]
fn test_push_imm32_medium_positive() {
    let code = [
        0x68, 0x78, 0x56, 0x34, 0x12, // PUSH 0x12345678
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x12345678, "Medium positive value");
}

#[test]
fn test_push_imm32_large_positive() {
    let code = [
        0x68, 0xff, 0xff, 0xff, 0x7f, // PUSH 0x7FFFFFFF (max positive 32-bit signed)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x7FFFFFFF, "Max positive 32-bit signed");
}

#[test]
fn test_push_imm32_negative_one() {
    let code = [
        0x68, 0xff, 0xff, 0xff, 0xff, // PUSH -1
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFFFF, "-1 sign-extended");
}

#[test]
fn test_push_imm32_negative_small() {
    let code = [
        0x68, 0xfe, 0xff, 0xff, 0xff, // PUSH -2
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFFFE, "-2 sign-extended");
}

#[test]
fn test_push_imm32_negative_large() {
    let code = [
        0x68, 0x00, 0x00, 0x00, 0x80, // PUSH 0x80000000 (-2147483648, min 32-bit signed)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFF80000000, "Min 32-bit signed extended");
}

#[test]
fn test_push_imm32_pattern_deadbeef() {
    let code = [
        0x68, 0xef, 0xbe, 0xad, 0xde, // PUSH 0xDEADBEEF
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFDEADBEEF, "0xDEADBEEF sign-extended");
}

#[test]
fn test_push_imm32_pattern_cafebabe() {
    let code = [
        0x68, 0xbe, 0xba, 0xfe, 0xca, // PUSH 0xCAFEBABE
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFCAFEBABE, "0xCAFEBABE sign-extended");
}

// ============================================================================
// Multiple PUSH immediate operations
// ============================================================================

#[test]
fn test_push_imm8_sequence() {
    let code = [
        0x6a, 0x01, // PUSH 1
        0x6a, 0x02, // PUSH 2
        0x6a, 0x03, // PUSH 3
        0x6a, 0x04, // PUSH 4
        0x6a, 0x05, // PUSH 5
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 40, "Five values pushed");
    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp).unwrap(),
        5,
        "Top of stack"
    );
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 8).unwrap(), 4);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 16).unwrap(), 3);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 24).unwrap(), 2);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 32).unwrap(), 1);
}

#[test]
fn test_push_imm32_sequence() {
    let code = [
        0x68, 0x01, 0x00, 0x00, 0x00, // PUSH 1
        0x68, 0x02, 0x00, 0x00, 0x00, // PUSH 2
        0x68, 0x03, 0x00, 0x00, 0x00, // PUSH 3
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 24, "Three values pushed");
    assert_eq!(emu.maps.read_qword(emu.regs().rsp).unwrap(), 3);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 8).unwrap(), 2);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 16).unwrap(), 1);
}

#[test]
fn test_push_imm_mixed_sizes() {
    let code = [
        0x6a, 0x11, // PUSH 0x11 (imm8)
        0x68, 0x22, 0x22, 0x22, 0x22, // PUSH 0x22222222 (imm32)
        0x6a, 0x33, // PUSH 0x33 (imm8)
        0x68, 0x44, 0x44, 0x44, 0x44, // PUSH 0x44444444 (imm32)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 32, "Four values pushed");
    assert_eq!(emu.maps.read_qword(0x1000 - 32).unwrap(), 0x44444444);
    assert_eq!(emu.maps.read_qword(0x1000 - 24).unwrap(), 0x33);
    assert_eq!(emu.maps.read_qword(0x1000 - 16).unwrap(), 0x22222222);
    assert_eq!(emu.maps.read_qword(0x1000 - 8).unwrap(), 0x11);
}

// ============================================================================
// PUSH immediate with POP roundtrip
// ============================================================================

#[test]
fn test_push_imm8_pop_roundtrip() {
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x42, "Value popped into RAX");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack balanced");
}

#[test]
fn test_push_imm32_pop_roundtrip() {
    let code = [
        0x68, 0x78, 0x56, 0x34, 0x12, // PUSH 0x12345678
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "Value popped into RAX");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack balanced");
}

#[test]
fn test_push_imm_negative_pop() {
    let code = [
        0x6a, 0xff, // PUSH -1
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "-1 in RAX");
}

// ============================================================================
// PUSH immediate preserves registers and flags
// ============================================================================

#[test]
fn test_push_imm_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0xc7, 0xc3, 0xbb, 0x00, 0x00, 0x00, // MOV RBX, 0xBB
        0x6a, 0x42, // PUSH 0x42
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xAA, "RAX unchanged");
    assert_eq!(emu.regs().rbx, 0xBB, "RBX unchanged");
}

#[test]
fn test_push_imm_preserves_flags() {
    let code = [
        0xf9, // STC (set carry)
        0x6a, 0x42, // PUSH 0x42
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_ne!(emu.flags().dump() & 0x01, 0, "CF preserved");
}

// ============================================================================
// PUSH immediate for function parameters
// ============================================================================

#[test]
fn test_push_imm_function_params() {
    let code = [
        // Push parameters in reverse order
        0x6a, 0x03, // PUSH 3 (param 3)
        0x6a, 0x02, // PUSH 2 (param 2)
        0x6a, 0x01, // PUSH 1 (param 1)
        // Function would be called here
        // Clean up stack
        0x48, 0x83, 0xc4, 0x18, // ADD RSP, 24
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "Stack cleaned up");
}

#[test]
fn test_push_imm_large_params() {
    let code = [
        0x68, 0x00, 0x10, 0x00, 0x00, // PUSH 0x1000
        0x68, 0x00, 0x20, 0x00, 0x00, // PUSH 0x2000
        0x68, 0x00, 0x30, 0x00, 0x00, // PUSH 0x3000
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x2000-(0x2000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x2000;
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_qword(emu.regs().rsp).unwrap(), 0x3000);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 8).unwrap(), 0x2000);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 16).unwrap(), 0x1000);
}

// ============================================================================
// PUSH immediate edge cases
// ============================================================================

#[test]
fn test_push_imm8_all_ones() {
    let code = [
        0x6a, 0xff, // PUSH 0xFF (-1)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFFFF, "All ones");
}

#[test]
fn test_push_imm32_all_ones() {
    let code = [
        0x68, 0xff, 0xff, 0xff, 0xff, // PUSH 0xFFFFFFFF
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFFFF, "All ones");
}

#[test]
fn test_push_imm_at_low_stack() {
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x10-(0x10 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x10;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x08, "RSP at low address");
    let val = emu.maps.read_qword(0x08).unwrap();
    assert_eq!(val, 0x42, "Value pushed");
}

#[test]
fn test_push_imm_rapid_sequence() {
    let code = [
        0x6a, 0x00, 0x6a, 0x01, 0x6a, 0x02, 0x6a, 0x03, 0x6a, 0x04, 0x6a, 0x05, 0x6a, 0x06, 0x6a,
        0x07, 0x6a, 0x08, 0x6a, 0x09, 0x6a, 0x0a, 0x6a, 0x0b, 0x6a, 0x0c, 0x6a, 0x0d, 0x6a, 0x0e,
        0x6a, 0x0f, 0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x2000-(0x2000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x2000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000 - 128, "16 values pushed");
    for i in 0..16 {
        let val = emu.maps.read_qword(emu.regs().rsp + i * 8).unwrap();
        assert_eq!(val, 15 - i, "Value at position {}", i);
    }
}

// ============================================================================
// PUSH immediate with specific bit patterns
// ============================================================================

#[test]
fn test_push_imm32_alternating_bits() {
    let code = [
        0x68, 0x55, 0x55, 0x55, 0x55, // PUSH 0x55555555
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x55555555, "Alternating bits");
}

#[test]
fn test_push_imm32_inverse_alternating() {
    let code = [
        0x68, 0xaa, 0xaa, 0xaa, 0xaa, // PUSH 0xAAAAAAAA
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(
        val, 0xFFFFFFFFAAAAAAAA,
        "Inverse alternating (sign-extended)"
    );
}

#[test]
fn test_push_imm32_high_bit_set() {
    let code = [
        0x68, 0x00, 0x00, 0x00, 0x80, // PUSH 0x80000000
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFF80000000, "High bit triggers sign extension");
}

#[test]
fn test_push_imm32_high_bit_clear() {
    let code = [
        0x68, 0xff, 0xff, 0xff, 0x7f, // PUSH 0x7FFFFFFF
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x7FFFFFFF, "High bit clear, no sign extension");
}

// ============================================================================
// PUSH immediate for constants
// ============================================================================

#[test]
fn test_push_imm_powers_of_two() {
    let code = [
        0x6a, 0x01, // PUSH 1 (2^0)
        0x6a, 0x02, // PUSH 2 (2^1)
        0x6a, 0x04, // PUSH 4 (2^2)
        0x6a, 0x08, // PUSH 8 (2^3)
        0x6a, 0x10, // PUSH 16 (2^4)
        0x6a, 0x20, // PUSH 32 (2^5)
        0x6a, 0x40, // PUSH 64 (2^6)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_qword(emu.regs().rsp).unwrap(), 64);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 8).unwrap(), 32);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 16).unwrap(), 16);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 24).unwrap(), 8);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 32).unwrap(), 4);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 40).unwrap(), 2);
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 48).unwrap(), 1);
}

#[test]
fn test_push_imm_ascii_values() {
    let code = [
        0x6a, 0x41, // PUSH 'A'
        0x6a, 0x42, // PUSH 'B'
        0x6a, 0x43, // PUSH 'C'
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_qword(emu.regs().rsp).unwrap(), 0x43, "'C'");
    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp + 8).unwrap(),
        0x42,
        "'B'"
    );
    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp + 16).unwrap(),
        0x41,
        "'A'"
    );
}

#[test]
fn test_push_imm_boolean_values() {
    let code = [
        0x6a, 0x00, // PUSH 0 (false)
        0x6a, 0x01, // PUSH 1 (true)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.maps.read_qword(emu.regs().rsp).unwrap(), 1, "true");
    assert_eq!(emu.maps.read_qword(emu.regs().rsp + 8).unwrap(), 0, "false");
}

// ============================================================================
// PUSH immediate with subsequent arithmetic
// ============================================================================

#[test]
fn test_push_imm_then_add() {
    let code = [
        0x6a, 0x05, // PUSH 5
        0x58, // POP RAX
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 8, "5 + 3 = 8");
}

#[test]
fn test_push_imm_multiple_ops() {
    let code = [
        0x6a, 0x0a, // PUSH 10
        0x6a, 0x05, // PUSH 5
        0x58, // POP RAX (5)
        0x5b, // POP RBX (10)
        0x48, 0x01, 0xd8, // ADD RAX, RBX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 15, "5 + 10 = 15");
}

#[test]
fn test_push_imm_stack_based_calc() {
    let code = [
        0x6a, 0x03, // PUSH 3
        0x6a, 0x04, // PUSH 4
        0x6a, 0x05, // PUSH 5
        0x59, // POP RCX (5)
        0x5b, // POP RBX (4)
        0x58, // POP RAX (3)
        // RAX = 3, RBX = 4, RCX = 5
        0x48, 0x0f, 0xaf, 0xc3, // IMUL RAX, RBX (3 * 4 = 12)
        0x48, 0x01, 0xc8, // ADD RAX, RCX (12 + 5 = 17)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 17, "3 * 4 + 5 = 17");
}
