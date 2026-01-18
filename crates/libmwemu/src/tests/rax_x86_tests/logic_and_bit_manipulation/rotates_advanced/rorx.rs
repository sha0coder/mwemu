// RORX (Rotate Right Logical Without Affecting Flags) instruction tests
//
// Opcodes:
// VEX.LZ.F2.0F3A.W0 F0 /r ib    RORX r32, r/m32, imm8
// VEX.LZ.F2.0F3A.W1 F0 /r ib    RORX r64, r/m64, imm8
//
// RORX rotates the source operand right by imm8 bits without affecting flags.
// Unlike ROR, RORX:
// - Has separate destination and source operands
// - Does NOT affect any flags
// - Only accepts immediate count (no CL variant)
// - Requires BMI2 CPU feature
//
// Flags:
// - None affected (unlike ROR which sets CF and OF)

use crate::*;

// ============================================================================
// 32-bit RORX tests
// ============================================================================

#[test]
fn test_rorx_eax_ebx_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX EAX, EBX, imm8 (VEX.LZ.F2.0F3A.W0 F0 /r ib)
    // VEX encoding: C4 E3 7B F0 C3 04
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x81234567, "EAX: 0x12345678 RORX 4 = 0x81234567");
    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX: source unchanged");
}

#[test]
fn test_rorx_eax_no_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX should not affect any flags
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_PF | flags::F_AF |
                  flags::F_ZF | flags::F_SF | flags::F_OF);
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.flags().dump(), initial_flags, "Flags: RORX does not affect flags");
}

#[test]
fn test_rorx_eax_rotate_8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x78123456, "EAX: 0x12345678 RORX 8 = 0x78123456");
}

#[test]
fn test_rorx_eax_rotate_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x01, // RORX EAX, EBX, 1
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x091A2B3C, "EAX: 0x12345678 RORX 1 = 0x091A2B3C");
}

#[test]
fn test_rorx_eax_full_rotation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x20, // RORX EAX, EBX, 32
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "EAX: full rotation returns to original");
}

#[test]
fn test_rorx_eax_count_masked() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x28, // RORX EAX, EBX, 0x28 (40)
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x28 & 0x1F = 8
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x78123456, "EAX: count masked to 8");
}

// ============================================================================
// 64-bit RORX tests
// ============================================================================

#[test]
fn test_rorx_rax_rbx_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX RAX, RBX, imm8 (VEX.LZ.F2.0F3A.W1 F0 /r ib)
    // VEX encoding: C4 E3 FB F0 C3 04
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x04, // RORX RAX, RBX, 4
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0123456789ABCDEF, "RAX: 0x123456789ABCDEF0 RORX 4");
    assert_eq!(emu.regs().rbx, 0x123456789ABCDEF0, "RBX: source unchanged");
}

#[test]
fn test_rorx_rax_no_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX should not affect any flags (64-bit)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x10, // RORX RAX, RBX, 16
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_ZF | flags::F_OF);
    let initial_flags = emu.flags().dump();
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.flags().dump(), initial_flags, "Flags: RORX does not affect flags");
}

#[test]
fn test_rorx_rax_rotate_16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x10, // RORX RAX, RBX, 16
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xDEF0123456789ABC, "RAX: RORX 16");
}

#[test]
fn test_rorx_rax_rotate_32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x20, // RORX RAX, RBX, 32
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x9ABCDEF012345678, "RAX: RORX 32");
}

#[test]
fn test_rorx_rax_full_rotation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x40, // RORX RAX, RBX, 64
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX: full rotation returns to original");
}

#[test]
fn test_rorx_rax_count_masked() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x50, // RORX RAX, RBX, 0x50 (80)
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x50 & 0x3F = 16
    assert_eq!(emu.regs().rax, 0xDEF0123456789ABC, "RAX: count masked to 16");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_rorx_r8d_r9d_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX R8D, R9D, imm8
    let code = [
        0xc4, 0x43, 0x7b, 0xf0, 0xc1, 0x08, // RORX R8D, R9D, 8
        0xf4,
    ];
    emu.regs_mut().r9 = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 0x78123456, "R8D: RORX from R9D");
}

#[test]
fn test_rorx_r14_r15_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX R14, R15, imm8
    let code = [
        0xc4, 0x43, 0xfb, 0xf0, 0xf7, 0x10, // RORX R14, R15, 16
        0xf4,
    ];
    emu.regs_mut().r15 = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r14, 0xDEF0123456789ABC, "R14: RORX from R15");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rorx_eax_dword_ptr() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX EAX, dword ptr [DATA_ADDR], imm8
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0x04, 0x25, // RORX EAX, dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x08, // imm8 = 8
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    let result = emu.maps.read_dword(DATA_ADDR).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x78123456, "EAX: RORX from memory");
    assert_eq!(result, 0x12345678, "Memory: unchanged");
}

#[test]
fn test_rorx_rax_qword_ptr() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX RAX, qword ptr [DATA_ADDR], imm8
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0x04, 0x25, // RORX RAX, qword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8 = 16
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x123456789ABCDEF0);

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(emu.regs().rax, 0xDEF0123456789ABC, "RAX: RORX from memory");
    assert_eq!(result, 0x123456789ABCDEF0, "Memory: unchanged");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_rorx_separate_dest_source() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RORX can use different registers for dest and source
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF; // Will be overwritten
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x81234567, "EAX: new rotated value");
    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "EBX: preserved");
}

#[test]
fn test_rorx_vs_ror_flags() {
    let DATA_ADDR = 0x7000;
    let code_ror = [
        0xc1, 0xc8, 0x04, // ROR EAX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.flags_mut().load(0x02);
    emu.load_code_bytes(&code_ror);
    emu.run(None).unwrap();

    // ROR should set CF
    let ror_cf = emu.flags().f_cf;

    let code_rorx = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rbx = 0x12345678;
    emu.flags_mut().load(0x2 | flags::F_CF); // Set CF
    emu.load_code_bytes(&code_rorx);
    emu.run(None).unwrap();

    // RORX should not change CF
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x81234567, "RORX result");
    assert!(emu.flags().f_cf, "RORX: CF preserved");
}

#[test]
fn test_rorx_byte_swap() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x78123456, "EAX: bytes rotated");
}

#[test]
fn test_rorx_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x01, // RORX EAX, EBX, 1
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX: all ones stay all ones");
}

#[test]
fn test_rorx_alternating_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x01, // RORX EAX, EBX, 1
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 1010_1010...
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x55555555, "EAX: alternating bits rotated");
}

#[test]
fn test_rorx_nibble_swap() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x81234567, "EAX: nibbles rotated");
}
