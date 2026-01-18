use crate::*;

// CMP — Compare Two Operands
//
// Opcodes:
// - 3C ib           CMP AL, imm8
// - 3D iw           CMP AX, imm16
// - 3D id           CMP EAX, imm32
// - REX.W + 3D id   CMP RAX, imm32 (sign-extended)
// - 80 /7 ib        CMP r/m8, imm8
// - 81 /7 iw        CMP r/m16, imm16
// - 81 /7 id        CMP r/m32, imm32
// - REX.W + 81 /7 id CMP r/m64, imm32 (sign-extended)
// - 83 /7 ib        CMP r/m16, imm8 (sign-extended)
// - 83 /7 ib        CMP r/m32, imm8 (sign-extended)
// - REX.W + 83 /7 ib CMP r/m64, imm8 (sign-extended)
// - 38 /r           CMP r/m8, r8
// - 39 /r           CMP r/m16, r16
// - 39 /r           CMP r/m32, r32
// - REX.W + 39 /r   CMP r/m64, r64
// - 3A /r           CMP r8, r/m8
// - 3B /r           CMP r16, r/m16
// - 3B /r           CMP r32, r/m32
// - REX.W + 3B /r   CMP r64, r/m64
//
// Operation: temp := DEST - SRC (result discarded, only flags set)
// Flags: CF, OF, SF, ZF, AF, PF are set according to result
// Note: CMP does NOT modify the destination operand

// ============================================================================
// 8-bit CMP Tests
// ============================================================================

#[test]
fn test_cmp_al_imm8_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x42, 0xf4]; // CMP AL, 0x42; HLT
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_cmp_al_imm8_greater() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x20, 0xf4]; // CMP AL, 0x20
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL should not be modified");
    assert!(!emu.flags().f_zf, "ZF should be clear (not equal)");
    assert!(!emu.flags().f_cf, "CF should be clear (AL > imm8)");
}

#[test]
fn test_cmp_al_imm8_less() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x80, 0xf4]; // CMP AL, 0x80
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL should not be modified");
    assert!(!emu.flags().f_zf, "ZF should be clear (not equal)");
    assert!(emu.flags().f_cf, "CF should be set (AL < imm8 unsigned)");
}

#[test]
fn test_cmp_r8_r8_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x38, 0xd8, 0xf4]; // CMP AL, BL
    emu.regs_mut().rax = 0x42;
    emu.regs_mut().rbx = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL should not be modified");
    assert_eq!(emu.regs().rbx & 0xFF, 0x42, "BL should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_r8_r8_less() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x38, 0xd8, 0xf4]; // CMP AL, BL
    emu.regs_mut().rax = 0x10;
    emu.regs_mut().rbx = 0x20;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (AL < BL)");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_cmp_all_8bit_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for reg_num in 8..=15 {
        let modrm = 0xC0 | ((reg_num - 8) << 3) | (reg_num - 8);
        let code = [0x45, 0x38, modrm, 0xf4]; // CMP R*B, R*B

        match reg_num {
            8 => emu.regs_mut().r8 = 0x42,
            9 => emu.regs_mut().r9 = 0x42,
            10 => emu.regs_mut().r10 = 0x42,
            11 => emu.regs_mut().r11 = 0x42,
            12 => emu.regs_mut().r12 = 0x42,
            13 => emu.regs_mut().r13 = 0x42,
            14 => emu.regs_mut().r14 = 0x42,
            15 => emu.regs_mut().r15 = 0x42,
            _ => unreachable!(),
        }

        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert!(emu.flags().f_zf, "ZF should be set for R{} - R{}", reg_num, reg_num);
    }
}

#[test]
fn test_cmp_extended_r8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x45, 0x38, 0xc8, 0xf4]; // CMP R8B, R9B
    emu.regs_mut().r8 = 0x50;
    emu.regs_mut().r9 = 0x30;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8 & 0xFF, 0x50, "R8B should not be modified");
    assert_eq!(emu.regs().r9 & 0xFF, 0x30, "R9B should not be modified");
    assert!(!emu.flags().f_cf, "CF should be clear (R8B > R9B)");
}

// ============================================================================
// 16-bit CMP Tests
// ============================================================================

#[test]
fn test_cmp_ax_imm16_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x3D, 0x34, 0x12, 0xf4]; // CMP AX, 0x1234
    emu.regs_mut().rax = 0x1234;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_ax_imm16_greater() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x3D, 0x00, 0x10, 0xf4]; // CMP AX, 0x1000
    emu.regs_mut().rax = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x2000, "AX should not be modified");
    assert!(!emu.flags().f_cf, "CF should be clear (AX > imm16)");
}

#[test]
fn test_cmp_ax_imm16_less() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x3D, 0x00, 0x80, 0xf4]; // CMP AX, 0x8000
    emu.regs_mut().rax = 0x4000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x4000, "AX should not be modified");
    assert!(emu.flags().f_cf, "CF should be set (AX < imm16)");
}

#[test]
fn test_cmp_r16_r16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x39, 0xd8, 0xf4]; // CMP AX, BX
    emu.regs_mut().rax = 0x3000;
    emu.regs_mut().rbx = 0x3000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x3000, "AX should not be modified");
    assert_eq!(emu.regs().rbx & 0xFFFF, 0x3000, "BX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_r16_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x83, 0xf8, 0xFF, 0xf4]; // CMP AX, -1
    emu.regs_mut().rax = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "AX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_extended_r16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x39, 0xda, 0xf4]; // CMP R10W, R11W
    emu.regs_mut().r10 = 0x8000;
    emu.regs_mut().r11 = 0x4000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10 & 0xFFFF, 0x8000, "R10W should not be modified");
    assert!(!emu.flags().f_cf, "CF should be clear (R10W > R11W)");
}

// ============================================================================
// 32-bit CMP Tests
// ============================================================================

#[test]
fn test_cmp_eax_imm32_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3D, 0x78, 0x56, 0x34, 0x12, 0xf4]; // CMP EAX, 0x12345678
    emu.regs_mut().rax = 0x12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_eax_imm32_greater() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3D, 0x00, 0x00, 0x00, 0x10, 0xf4]; // CMP EAX, 0x10000000
    emu.regs_mut().rax = 0x20000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x20000000, "EAX should not be modified");
    assert!(!emu.flags().f_cf, "CF should be clear (EAX > imm32)");
}

#[test]
fn test_cmp_eax_imm32_less() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3D, 0x00, 0x00, 0x00, 0x80, 0xf4]; // CMP EAX, 0x80000000
    emu.regs_mut().rax = 0x40000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x40000000, "EAX should not be modified");
    assert!(emu.flags().f_cf, "CF should be set (EAX < imm32)");
}

#[test]
fn test_cmp_r32_r32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x39, 0xd8, 0xf4]; // CMP EAX, EBX
    emu.regs_mut().rax = 0x30000000;
    emu.regs_mut().rbx = 0x30000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x30000000, "EAX should not be modified");
    assert_eq!(emu.regs().rbx, 0x30000000, "EBX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_r32_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x83, 0xf8, 0x7F, 0xf4]; // CMP EAX, 127
    emu.regs_mut().rax = 0x0000007F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000007F, "EAX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_extended_r32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x45, 0x39, 0xec, 0xf4]; // CMP R12D, R13D
    emu.regs_mut().r12 = 0x70000000;
    emu.regs_mut().r13 = 0x30000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0x70000000, "R12D should not be modified");
    assert!(!emu.flags().f_cf, "CF should be clear (R12D > R13D)");
}

// ============================================================================
// 64-bit CMP Tests
// ============================================================================

#[test]
fn test_cmp_rax_imm32_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x3D, 0x78, 0x56, 0x34, 0x12, 0xf4]; // CMP RAX, 0x12345678
    emu.regs_mut().rax = 0x0000000012345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000012345678, "RAX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_rax_imm32_greater() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x3D, 0x00, 0x00, 0x00, 0x10, 0xf4]; // CMP RAX, 0x10000000
    emu.regs_mut().rax = 0x0000000020000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000020000000, "RAX should not be modified");
    assert!(!emu.flags().f_cf, "CF should be clear (RAX > imm32)");
}

#[test]
fn test_cmp_rax_imm32_less() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x3D, 0xFF, 0xFF, 0xFF, 0x7F, 0xf4]; // CMP RAX, 0x7FFFFFFF
    emu.regs_mut().rax = 0x0000000010000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000010000000, "RAX should not be modified");
    assert!(emu.flags().f_cf, "CF should be set (RAX < imm32)");
}

#[test]
fn test_cmp_r64_r64_equal() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x39, 0xd8, 0xf4]; // CMP RAX, RBX
    emu.regs_mut().rax = 0x1234567890ABCDEF;
    emu.regs_mut().rbx = 0x1234567890ABCDEF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1234567890ABCDEF, "RAX should not be modified");
    assert_eq!(emu.regs().rbx, 0x1234567890ABCDEF, "RBX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_r64_r64_less() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x39, 0xd8, 0xf4]; // CMP RAX, RBX
    emu.regs_mut().rax = 0x1000000000000000;
    emu.regs_mut().rbx = 0x2000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (RAX < RBX)");
}

#[test]
fn test_cmp_r64_r64_greater() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x39, 0xd8, 0xf4]; // CMP RAX, RBX
    emu.regs_mut().rax = 0x3000000000000000;
    emu.regs_mut().rbx = 0x1000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (RAX > RBX)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
}

#[test]
fn test_cmp_r64_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x83, 0xf8, 0xFF, 0xf4]; // CMP RAX, -1
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_all_64bit_registers() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for i in 0..16 {
        let (rex, modrm) = if i < 8 {
            (0x48, 0xC0 | (i as u8))
        } else {
            (0x4C, 0xC0 | ((i - 8) as u8))
        };

        let code = [rex, 0x39, modrm, 0xf4]; // CMP RAX, reg
        let test_val = 0x1234567890ABCDEF;
        emu.regs_mut().rax = test_val;
        emu.regs_mut().rcx = test_val;
        emu.regs_mut().rdx = test_val;
        emu.regs_mut().rbx = test_val;
        emu.regs_mut().rsp = test_val;
        emu.regs_mut().rbp = test_val;
        emu.regs_mut().rsi = test_val;
        emu.regs_mut().rdi = test_val;
        emu.regs_mut().r8 = test_val;
        emu.regs_mut().r9 = test_val;
        emu.regs_mut().r10 = test_val;
        emu.regs_mut().r11 = test_val;
        emu.regs_mut().r12 = test_val;
        emu.regs_mut().r13 = test_val;
        emu.regs_mut().r14 = test_val;
        emu.regs_mut().r15 = test_val;

        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(emu.regs().rax, test_val, "RAX should not be modified");
        assert!(emu.flags().f_zf, "ZF should be set for equal values");
    }
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_cmp_byte_ptr_imm8() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x80, 0x3D, 0xF9, 0x0F, 0x00, 0x00, 0x42, // CMP BYTE PTR [rip+0x0FF9], 0x42
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 0x42);

    emu.run(None).unwrap();
    let result = emu.maps.read_byte(DATA_ADDR).unwrap();

    assert_eq!(result, 0x42, "Memory should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_qword_ptr_r64() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x39, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // CMP QWORD PTR [rip+0x0FF6], RBX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let test_val = 0x1234567890ABCDEF;
    emu.maps.write_qword(DATA_ADDR, test_val);
    emu.regs_mut().rbx = test_val;

    emu.run(None).unwrap();
    let result = emu.maps.read_qword(DATA_ADDR).unwrap();

    assert_eq!(result, test_val, "Memory should not be modified");
    assert_eq!(emu.regs().rbx, test_val, "RBX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_r64_from_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x3B, 0x05, 0xF9, 0x0F, 0x00, 0x00, // CMP RAX, QWORD PTR [rip+0x0FF6]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 0x1000000000000000);
    emu.regs_mut().rax = 0x1000000000000000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1000000000000000, "RAX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_cmp_zero_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x42, 0xf4]; // CMP AL, 0x42
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_cmp_carry_flag_set() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x50, 0xf4]; // CMP AL, 0x50
    emu.regs_mut().rax = 0x40;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (AL < imm8)");
}

#[test]
fn test_cmp_carry_flag_clear() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x30, 0xf4]; // CMP AL, 0x30
    emu.regs_mut().rax = 0x40;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(!emu.flags().f_cf, "CF should be clear (AL > imm8)");
}

#[test]
fn test_cmp_sign_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x90, 0xf4]; // CMP AL, 0x90
    emu.regs_mut().rax = 0x40;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_sf, "SF should be set (negative result)");
}

#[test]
fn test_cmp_overflow_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x01, 0xf4]; // CMP AL, 1
    emu.regs_mut().rax = 0x80; // Min negative i8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_cmp_parity_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x02, 0xf4]; // CMP AL, 2
    emu.regs_mut().rax = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_pf, "PF should be set");
}

#[test]
fn test_cmp_auxiliary_flag() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x05, 0xf4]; // CMP AL, 5
    emu.regs_mut().rax = 0x12;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// Edge Cases and Special Tests
// ============================================================================

#[test]
fn test_cmp_zero_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x00, 0xf4]; // CMP AL, 0
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "AL should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_cmp_self() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x38, 0xc0, 0xf4]; // CMP AL, AL
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "AL should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set (equal)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_cmp_max_unsigned_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x39, 0xd8, 0xf4]; // CMP RAX, RBX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX should not be modified");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_signed_negative_comparison() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0xFF, 0xf4]; // CMP AL, -1
    emu.regs_mut().rax = 0xFE; // -2 in i8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFE, "AL should not be modified");
    assert!(emu.flags().f_cf, "CF should be set (0xFE < 0xFF unsigned)");
}

#[test]
fn test_cmp_preserves_all_bits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x3C, 0x78, 0xf4]; // CMP AL, 0x78
    emu.regs_mut().rax = 0xDEADBEEF12345678;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xDEADBEEF12345678, "RAX should be completely unchanged");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_cmp_anti_commutativity() {
    // CMP A, B should have inverted CF compared to CMP B, A
    let code1 = [0x38, 0xd8, 0xf4]; // CMP AL, BL
    let code2 = [0x38, 0xc3, 0xf4]; // CMP BL, AL

    let mut emu1 = emu64();
    emu1.regs_mut().rax = 0x10;
    emu1.regs_mut().rbx = 0x20;
    emu1.load_code_bytes(&code1);
    emu1.run(None).unwrap();

    let mut emu2 = emu64();
    emu2.regs_mut().rax = 0x10;
    emu2.regs_mut().rbx = 0x20;
    emu2.load_code_bytes(&code2);
    emu2.run(None).unwrap();

    // AL < BL should set CF, but BL > AL should clear CF
    assert!(emu1.flags().f_cf, "CF should be set for AL < BL");
    assert!(!emu2.flags().f_cf, "CF should be clear for BL > AL");
}
