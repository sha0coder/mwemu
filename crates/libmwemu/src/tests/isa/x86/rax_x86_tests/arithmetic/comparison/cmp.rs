//! Tests for the CMP instruction.
//!
//! CMP - Compare Two Operands
//!
//! Compares the first operand with the second operand by performing a subtraction
//! (first - second) and setting flags accordingly, without storing the result.
//!
//! Flags affected: OF, SF, ZF, AF, CF, PF
//!
//! Reference: docs/cmp.txt

use crate::*;

// ============================================================================
// CMP AL, imm8 (opcode 3C ib)
// ============================================================================

#[test]
fn test_cmp_al_imm8_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP AL, 10 when AL = 10
    // 3C 0a = CMP AL, 10
    // f4 = HLT
    let code = [0x3c, 0x0a, 0xf4];
    emu.regs_mut().rax = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 10, "CMP should not modify operands");
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
    assert!(!emu.flags().f_cf, "CF should be clear (no borrow)");
}

#[test]
fn test_cmp_al_imm8_greater() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP AL, 5 when AL = 10 (10 > 5, so 10 - 5 = 5 > 0)
    let code = [0x3c, 0x05, 0xf4];
    emu.regs_mut().rax = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_zf, "ZF should be clear (not equal)");
    assert!(!emu.flags().f_cf, "CF should be clear (no borrow, first >= second)");
    assert!(!emu.flags().f_sf, "SF should be clear (positive result)");
}

#[test]
fn test_cmp_al_imm8_less() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP AL, 15 when AL = 10 (10 < 15, so 10 - 15 = -5 < 0)
    let code = [0x3c, 0x0f, 0xf4];
    emu.regs_mut().rax = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_zf, "ZF should be clear (not equal)");
    assert!(emu.flags().f_cf, "CF should be set (borrow, first < second)");
    assert!(emu.flags().f_sf, "SF should be set (negative result)");
}

#[test]
fn test_cmp_al_imm8_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP 0x80, 1 -> 0x80 - 1 = 0x7F (signed overflow)
    let code = [0x3c, 0x01, 0xf4];
    emu.regs_mut().rax = 0x80;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

#[test]
fn test_cmp_al_imm8_preserves_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3c, 0x42, 0xf4];
    emu.regs_mut().rax = 0xDEADBEEF_12345678;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xDEADBEEF_12345678, "CMP should not modify RAX");
}

// ============================================================================
// CMP AX/EAX/RAX, imm16/32 (opcode 3D)
// ============================================================================

#[test]
fn test_cmp_ax_imm16_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP AX, 0x1234 when AX = 0x1234
    // 66 3D 34 12 = CMP AX, 0x1234
    let code = [0x66, 0x3d, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
}

#[test]
fn test_cmp_eax_imm32_greater() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP EAX, 0x12345678 when EAX = 0x23456789
    // 3D 78 56 34 12 = CMP EAX, 0x12345678
    let code = [0x3d, 0x78, 0x56, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0x23456789;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_zf, "ZF should be clear (not equal)");
    assert!(!emu.flags().f_cf, "CF should be clear (first > second)");
}

#[test]
fn test_cmp_rax_imm32_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W CMP RAX, -1 (sign-extended to 64-bit)
    // 48 3D ff ff ff ff = CMP RAX, -1
    let code = [0x48, 0x3d, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF should be set (equal to -1)");
}

// ============================================================================
// CMP r/m8, imm8 (opcode 80 /7)
// ============================================================================

#[test]
fn test_cmp_rm8_imm8_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP CL, 10
    // 80 f9 0a = CMP CL, 10
    let code = [0x80, 0xf9, 0x0a, 0xf4];
    emu.regs_mut().rcx = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 10, "CMP should not modify CL");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_rm8_imm8_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP BYTE PTR [RBX], 25
    // 80 3b 19 = CMP BYTE PTR [RBX], 25
    let code = [0x80, 0x3b, 0x19, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 25);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 25, "CMP should not modify memory");
}

// ============================================================================
// CMP r/m16/32/64, imm32 (opcode 81 /7)
// ============================================================================

#[test]
fn test_cmp_rm32_imm32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP ECX, 0x12345678
    // 81 f9 78 56 34 12 = CMP ECX, 0x12345678
    let code = [0x81, 0xf9, 0x78, 0x56, 0x34, 0x12, 0xf4];
    emu.regs_mut().rcx = 0x12345678;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
}

#[test]
fn test_cmp_rm64_imm32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W CMP RCX, -1 (sign-extended)
    // 48 81 f9 ff ff ff ff = CMP RCX, -1
    let code = [0x48, 0x81, 0xf9, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rcx = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// CMP r/m16/32/64, imm8 sign-extended (opcode 83 /7)
// ============================================================================

#[test]
fn test_cmp_rm32_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP ECX, 10
    // 83 f9 0a = CMP ECX, 10
    let code = [0x83, 0xf9, 0x0a, 0xf4];
    emu.regs_mut().rcx = 100;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_cf, "CF should be clear (100 > 10)");
}

#[test]
fn test_cmp_rm32_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP ECX, -10 (0xF6 sign-extended to 0xFFFFFFF6)
    // 83 f9 f6 = CMP ECX, -10
    let code = [0x83, 0xf9, 0xf6, 0xf4];
    emu.regs_mut().rcx = 100;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    // 100 compared to -10 (as unsigned: 0xFFFFFFF6)
    // 100 - 0xFFFFFFF6 produces borrow
    assert!(emu.flags().f_cf, "CF should be set (100 < 0xFFFFFFF6 unsigned)");
}

// ============================================================================
// CMP r/m8, r8 (opcode 38 /r)
// ============================================================================

#[test]
fn test_cmp_rm8_r8_register_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP AL, CL when both are 10
    // 38 c8 = CMP AL, CL
    let code = [0x38, 0xc8, 0xf4];
    emu.regs_mut().rax = 10;
    emu.regs_mut().rcx = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 10, "AL should not change");
    assert_eq!(emu.regs().rcx, 10, "CL should not change");
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
}

#[test]
fn test_cmp_rm8_r8_register_greater() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP AL, CL when AL = 15, CL = 10
    let code = [0x38, 0xc8, 0xf4];
    emu.regs_mut().rax = 15;
    emu.regs_mut().rcx = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_cf, "CF should be clear (AL > CL)");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_cmp_rm8_r8_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP [RBX], CL
    // 38 0b = CMP [RBX], CL
    let code = [0x38, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 30;
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 30);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 30, "Memory should not change");
}

// ============================================================================
// CMP r/m16/32/64, r16/32/64 (opcode 39 /r)
// ============================================================================

#[test]
fn test_cmp_rm32_r32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP EAX, ECX
    // 39 c8 = CMP EAX, ECX
    let code = [0x39, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x12345678;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
}

#[test]
fn test_cmp_rm64_r64_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W CMP RAX, RCX
    // 48 39 c8 = CMP RAX, RCX
    let code = [0x48, 0x39, 0xc8, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFFF_00000000;
    emu.regs_mut().rcx = 0x00000000_FFFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_cf, "CF should be clear (RAX > RCX unsigned)");
}

// ============================================================================
// CMP r8, r/m8 (opcode 3A /r)
// ============================================================================

#[test]
fn test_cmp_r8_rm8_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP CL, AL
    // 3A c8 = CMP CL, AL
    let code = [0x3a, 0xc8, 0xf4];
    emu.regs_mut().rax = 10;
    emu.regs_mut().rcx = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
}

#[test]
fn test_cmp_r8_rm8_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP CL, [RBX]
    // 3A 0b = CMP CL, [RBX]
    let code = [0x3a, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 50;
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 30);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_cf, "CF should be clear (50 > 30)");
}

// ============================================================================
// CMP r16/32/64, r/m16/32/64 (opcode 3B /r)
// ============================================================================

#[test]
fn test_cmp_r32_rm32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP ECX, EAX
    // 3B c8 = CMP ECX, EAX
    let code = [0x3b, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x11111111;
    emu.regs_mut().rcx = 0x22222222;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_cf, "CF should be clear (ECX > EAX)");
}

#[test]
fn test_cmp_r64_rm64_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W CMP RCX, RAX
    // 48 3B c8 = CMP RCX, RAX
    let code = [0x48, 0x3b, 0xc8, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rcx = 0x1111111111111111;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "CF should be set (RCX < RAX)");
}

// ============================================================================
// Condition Code Tests
// ============================================================================

#[test]
fn test_cmp_for_je_condition() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP EAX, 42 when EAX = 42
    let code = [0x3d, 0x2a, 0x00, 0x00, 0x00, 0xf4];
    emu.regs_mut().rax = 42;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF=1 for JE condition");
}

#[test]
fn test_cmp_for_jne_condition() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP EAX, 42 when EAX = 50
    let code = [0x3d, 0x2a, 0x00, 0x00, 0x00, 0xf4];
    emu.regs_mut().rax = 50;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_zf, "ZF=0 for JNE condition");
}

#[test]
fn test_cmp_for_jb_condition() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP EAX, 100 when EAX = 50
    let code = [0x3d, 0x64, 0x00, 0x00, 0x00, 0xf4];
    emu.regs_mut().rax = 50;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "CF=1 for JB condition (50 < 100)");
}

#[test]
fn test_cmp_for_jae_condition() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP EAX, 50 when EAX = 100
    let code = [0x3d, 0x32, 0x00, 0x00, 0x00, 0xf4];
    emu.regs_mut().rax = 100;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_cf, "CF=0 for JAE condition (100 >= 50)");
}

#[test]
fn test_cmp_for_jl_condition() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP EAX, 0 when EAX = -10 (0xFFFFFFF6)
    let code = [0x3d, 0x00, 0x00, 0x00, 0x00, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFF6; // -10 in two's complement
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    // -10 - 0 = -10 (negative), so SF=1, OF=0 -> SF^OF = 1
    assert!(emu.flags().f_sf, "SF=1");
    assert!(!emu.flags().f_of, "OF=0");
}

#[test]
fn test_cmp_for_jg_condition() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP EAX, 0 when EAX = 10
    let code = [0x3d, 0x00, 0x00, 0x00, 0x00, 0xf4];
    emu.regs_mut().rax = 10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(!emu.flags().f_zf, "ZF=0 (not equal)");
    assert!(!emu.flags().f_sf, "SF=0 (positive result)");
    assert!(!emu.flags().f_of, "OF=0 (no overflow)");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_cmp_zero_with_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP 0, 0 should set ZF and clear CF, SF, OF
    let code = [0x3d, 0x00, 0x00, 0x00, 0x00, 0xf4];
    emu.regs_mut().rax = 0;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_sf, "SF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_cmp_max_with_max() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP 0xFFFFFFFF, 0xFFFFFFFF
    let code = [0x3d, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
}

#[test]
fn test_cmp_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP should set parity flag based on result
    // 0x0F - 0x03 = 0x0C (binary 00001100, 2 bits = even parity)
    let code = [0x3c, 0x03, 0xf4];
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_cmp_auxiliary_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CMP should set AF when borrow from bit 4
    // 0x10 - 0x01 = 0x0F (borrow from bit 4)
    let code = [0x3c, 0x01, 0xf4];
    emu.regs_mut().rax = 0x10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert!(emu.flags().f_af, "AF should be set (borrow from bit 4)");
}

// ============================================================================
// Extended Registers
// ============================================================================

#[test]
fn test_cmp_r8_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.B CMP R8D, 100
    // 41 83 f8 64 = CMP R8D, 100
    let code = [0x41, 0x83, 0xf8, 0x64, 0xf4];
    emu.regs_mut().r8 = 150;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 150, "R8 should not change");
    assert!(!emu.flags().f_cf, "CF should be clear (150 > 100)");
}

#[test]
fn test_cmp_r15_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.WB CMP R15, RAX
    // 49 39 c7 = CMP R15, RAX
    let code = [0x49, 0x39, 0xc7, 0xf4];
    emu.regs_mut().rax = 0x1000;
    emu.regs_mut().r15 = 0x1000;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r15, 0x1000, "R15 should not change");
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
}
