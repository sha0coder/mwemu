//! Tests for the SUB instruction.
//!
//! SUB - Subtract
//!
//! Subtracts the second operand (source operand) from the first operand (destination operand)
//! and stores the result in the destination operand.
//!
//! Flags affected: OF, SF, ZF, AF, CF, PF
//!
//! Reference: docs/sub.txt

use crate::*;

// ============================================================================
// SUB AL, imm8 (opcode 2C ib)
// ============================================================================

#[test]
fn test_sub_al_imm8_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB AL, 5
    // 2C 05 = SUB AL, 5
    // f4 = HLT
    let code = [0x2c, 0x05, 0xf4];
    emu.regs_mut().rax = 15;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 10, "SUB AL, 5: 15 - 5 = 10");
    assert!(!emu.flags().f_zf, "ZF should be clear (result != 0)");
    assert!(!emu.flags().f_sf, "SF should be clear (result positive)");
    assert!(!emu.flags().f_cf, "CF should be clear (no borrow)");
}

#[test]
fn test_sub_al_imm8_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB AL, 0x55 when AL = 0x55
    let code = [0x2c, 0x55, 0xf4];
    emu.regs_mut().rax = 0x55;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0, "SUB AL, 0x55: 0x55 - 0x55 = 0");
    assert!(emu.flags().f_zf, "ZF should be set (result = 0)");
    assert!(!emu.flags().f_cf, "CF should be clear (no borrow)");
}

#[test]
fn test_sub_al_imm8_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB AL, 1 when AL = 0 -> 0xFF with borrow
    let code = [0x2c, 0x01, 0xf4];
    emu.regs_mut().rax = 0;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "SUB AL, 1: 0 - 1 = 0xFF (with borrow)");
    assert!(!emu.flags().f_zf, "ZF should be clear");
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
    assert!(emu.flags().f_sf, "SF should be set (result negative)");
}

#[test]
fn test_sub_al_imm8_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2c, 0x01, 0xf4];
    emu.regs_mut().rax = 0x80;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x7F, "SUB AL, 1: 0x80 - 1 = 0x7F");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(!emu.flags().f_sf, "SF should be clear (result positive)");
    assert!(!emu.flags().f_cf, "CF should be clear (no unsigned borrow)");
}

#[test]
fn test_sub_al_imm8_negative_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 5 - 10 = -5 (0xFB in unsigned)
    let code = [0x2c, 0x0a, 0xf4];
    emu.regs_mut().rax = 5;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0xFB, "SUB AL, 10: 5 - 10 = 0xFB (-5)");
    assert!(emu.flags().f_sf, "SF should be set (result negative)");
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
    assert!(!emu.flags().f_of, "OF should be clear (no signed overflow)");
}

#[test]
fn test_sub_al_imm8_parity() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2c, 0x03, 0xf4];
    emu.regs_mut().rax = 0x0F;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x0C);
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_sub_al_imm8_preserves_high_bytes() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2c, 0x05, 0xf4];
    emu.regs_mut().rax = 0xDEADBEEF_1234567D;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x7D - 0x05);
    assert_eq!(emu.regs().rax & !0xFF, 0xDEADBEEF_12345600, "High bytes should be preserved");
}

// ============================================================================
// SUB AX/EAX/RAX, imm16/32 (opcode 2D)
// ============================================================================

#[test]
fn test_sub_ax_imm16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB AX, 0x1234 (16-bit mode with 66 prefix)
    // 66 2D 34 12 = SUB AX, 0x1234
    let code = [0x66, 0x2d, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0x5678;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x4444, "SUB AX, 0x1234: 0x5678 - 0x1234 = 0x4444");
}

#[test]
fn test_sub_eax_imm32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB EAX, 0x12345678
    // 2D 78 56 34 12 = SUB EAX, 0x12345678
    let code = [0x2d, 0x78, 0x56, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFFF_23456789;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    // 32-bit operation clears high 32 bits
    assert_eq!(emu.regs().rax, 0x11111111, "SUB EAX clears high 32 bits of RAX");
}

#[test]
fn test_sub_rax_imm32_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SUB RAX, imm32 (sign-extended)
    // 48 2D ff ff ff ff = SUB RAX, -1 (sign-extended to 64 bits)
    let code = [0x48, 0x2d, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rax = 100;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    // 100 - (-1) = 101
    assert_eq!(emu.regs().rax, 101, "SUB RAX, -1 (sign-extended): 100 - (-1) = 101");
}

// ============================================================================
// SUB r/m8, imm8 (opcode 80 /5)
// ============================================================================

#[test]
fn test_sub_rm8_imm8_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB CL, 10
    // 80 e9 0a = SUB CL, 10
    let code = [0x80, 0xe9, 0x0a, 0xf4];
    emu.regs_mut().rcx = 25;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 15, "SUB CL, 10: 25 - 10 = 15");
}

#[test]
fn test_sub_rm8_imm8_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB BYTE PTR [RBX], 10
    // 80 2b 0a = SUB BYTE PTR [RBX], 10
    let code = [0x80, 0x2b, 0x0a, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 50);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 40, "SUB [RBX], 10: 50 - 10 = 40");
}

// ============================================================================
// SUB r/m16/32/64, imm32 (opcode 81 /5)
// ============================================================================

#[test]
fn test_sub_rm32_imm32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB ECX, 0x12345678
    // 81 e9 78 56 34 12 = SUB ECX, 0x12345678
    let code = [0x81, 0xe9, 0x78, 0x56, 0x34, 0x12, 0xf4];
    emu.regs_mut().rcx = 0x23456789;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x11111111, "SUB ECX, 0x12345678");
}

#[test]
fn test_sub_rm64_imm32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SUB RCX, 0xFFFFFFFF (-1 sign-extended)
    // 48 81 e9 ff ff ff ff = SUB RCX, -1
    let code = [0x48, 0x81, 0xe9, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rcx = 0x100000000;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x100000001, "SUB RCX, -1: 0x100000000 - (-1) = 0x100000001");
}

#[test]
fn test_sub_rm32_imm32_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB DWORD PTR [RBX], 0x1000
    // 81 2b 00 10 00 00 = SUB DWORD PTR [RBX], 0x1000
    let code = [0x81, 0x2b, 0x00, 0x10, 0x00, 0x00, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x12344678, "SUB [RBX], 0x1000");
}

// ============================================================================
// SUB r/m16/32/64, imm8 sign-extended (opcode 83 /5)
// ============================================================================

#[test]
fn test_sub_rm32_imm8_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB ECX, 10 (using sign-extended imm8)
    // 83 e9 0a = SUB ECX, 10
    let code = [0x83, 0xe9, 0x0a, 0xf4];
    emu.regs_mut().rcx = 100;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 90, "SUB ECX, 10: 100 - 10 = 90");
}

#[test]
fn test_sub_rm32_imm8_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB ECX, -10 (0xF6 sign-extended) - effectively adds 10
    // 83 e9 f6 = SUB ECX, -10
    let code = [0x83, 0xe9, 0xf6, 0xf4];
    emu.regs_mut().rcx = 100;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 110, "SUB ECX, -10: 100 - (-10) = 110");
}

#[test]
fn test_sub_rm64_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SUB RCX, -1 (0xFF sign-extended to 64-bit)
    // 48 83 e9 ff = SUB RCX, -1
    let code = [0x48, 0x83, 0xe9, 0xff, 0xf4];
    emu.regs_mut().rcx = 0x100000000;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x100000001, "SUB RCX, -1: 0x100000000 + 1 = 0x100000001");
}

#[test]
fn test_sub_rm16_imm8_sign_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB CX, -1 (with 66 prefix)
    // 66 83 e9 ff = SUB CX, -1
    let code = [0x66, 0x83, 0xe9, 0xff, 0xf4];
    emu.regs_mut().rcx = 0x1000;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFFFF, 0x1001, "SUB CX, -1: 0x1000 + 1 = 0x1001");
}

// ============================================================================
// SUB r/m8, r8 (opcode 28 /r)
// ============================================================================

#[test]
fn test_sub_rm8_r8_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB AL, CL
    // 28 c8 = SUB AL, CL
    let code = [0x28, 0xc8, 0xf4];
    emu.regs_mut().rax = 15;
    emu.regs_mut().rcx = 5;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 10, "SUB AL, CL: 15 - 5 = 10");
}

#[test]
fn test_sub_rm8_r8_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB [RBX], CL
    // 28 0b = SUB [RBX], CL
    let code = [0x28, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 20;
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 50);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 30, "SUB [RBX], CL: 50 - 20 = 30");
}

#[test]
fn test_sub_rm8_r8_same_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB AL, AL (result is always 0)
    // 28 c0 = SUB AL, AL
    let code = [0x28, 0xc0, 0xf4];
    emu.regs_mut().rax = 50;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0, "SUB AL, AL: 50 - 50 = 0");
    assert!(emu.flags().f_zf, "ZF should be set (result = 0)");
}

// ============================================================================
// SUB r/m16/32/64, r16/32/64 (opcode 29 /r)
// ============================================================================

#[test]
fn test_sub_rm32_r32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB EAX, ECX
    // 29 c8 = SUB EAX, ECX
    let code = [0x29, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x23456789;
    emu.regs_mut().rcx = 0x11111111;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x12345678, "SUB EAX, ECX");
}

#[test]
fn test_sub_rm64_r64_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SUB RAX, RCX
    // 48 29 c8 = SUB RAX, RCX
    let code = [0x48, 0x29, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x00000001_00000000;
    emu.regs_mut().rcx = 0x00000000_00000001;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x00000000_FFFFFFFF, "SUB RAX, RCX");
}

#[test]
fn test_sub_rm64_r64_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SUB RAX, RCX with borrow
    // 48 29 c8 = SUB RAX, RCX
    let code = [0x48, 0x29, 0xc8, 0xf4];
    emu.regs_mut().rax = 0;
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "SUB RAX, RCX: 0 - 1 = max");
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
}

#[test]
fn test_sub_rm32_r32_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB [RBX], ECX
    // 29 0b = SUB [RBX], ECX
    let code = [0x29, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 0x1000;
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x12344678, "SUB [RBX], ECX");
}

#[test]
fn test_sub_rm64_r64_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SUB [RBX], RCX
    // 48 29 0b = SUB [RBX], RCX
    let code = [0x48, 0x29, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 0x1;
    emu.load_code_bytes(&code);

    emu.maps.write_qword(DATA_ADDR, 0x100000000);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(DATA_ADDR).unwrap(), 0xFFFFFFFF, "SUB [RBX], RCX (64-bit)");
}

// ============================================================================
// SUB r8, r/m8 (opcode 2A /r)
// ============================================================================

#[test]
fn test_sub_r8_rm8_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB CL, AL
    // 2A c8 = SUB CL, AL
    let code = [0x2a, 0xc8, 0xf4];
    emu.regs_mut().rax = 5;
    emu.regs_mut().rcx = 15;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 10, "SUB CL, AL: 15 - 5 = 10");
}

#[test]
fn test_sub_r8_rm8_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB CL, [RBX]
    // 2A 0b = SUB CL, [RBX]
    let code = [0x2a, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 50;
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 20);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 30, "SUB CL, [RBX]: 50 - 20 = 30");
}

// ============================================================================
// SUB r16/32/64, r/m16/32/64 (opcode 2B /r)
// ============================================================================

#[test]
fn test_sub_r32_rm32_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB ECX, EAX
    // 2B c8 = SUB ECX, EAX
    let code = [0x2b, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x11111111;
    emu.regs_mut().rcx = 0x33333333;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x22222222, "SUB ECX, EAX");
}

#[test]
fn test_sub_r64_rm64_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SUB RCX, RAX
    // 48 2B c8 = SUB RCX, RAX
    let code = [0x48, 0x2b, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x1111111111111111;
    emu.regs_mut().rcx = 0x3333333333333333;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x2222222222222222, "SUB RCX, RAX (64-bit)");
}

#[test]
fn test_sub_r32_rm32_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB ECX, [RBX]
    // 2B 0b = SUB ECX, [RBX]
    let code = [0x2b, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 0x12345678;
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x1000);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x12344678, "SUB ECX, [RBX]");
}

#[test]
fn test_sub_r64_rm64_memory() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SUB RCX, [RBX]
    // 48 2B 0b = SUB RCX, [RBX]
    let code = [0x48, 0x2b, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 0x200000000;
    emu.load_code_bytes(&code);

    emu.maps.write_qword(DATA_ADDR, 0x100000000);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x100000000, "SUB RCX, [RBX] (64-bit)");
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_sub_r8_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.B SUB R8D, 50
    // 41 83 e8 32 = SUB R8D, 50
    let code = [0x41, 0x83, 0xe8, 0x32, 0xf4];
    emu.regs_mut().r8 = 150;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 100, "SUB R8D, 50: 150 - 50 = 100");
}

#[test]
fn test_sub_r15_extended() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.WB SUB R15, RAX
    // 49 29 c7 = SUB R15, RAX
    let code = [0x49, 0x29, 0xc7, 0xf4];
    emu.regs_mut().rax = 0x1000;
    emu.regs_mut().r15 = 0x3000;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r15, 0x2000, "SUB R15, RAX: 0x3000 - 0x1000 = 0x2000");
}

// ============================================================================
// Flag Edge Cases
// ============================================================================

#[test]
fn test_sub_flags_32bit_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 32-bit signed overflow: 0x80000000 - 1 = 0x7FFFFFFF
    let code = [0x83, 0xe8, 0x01, 0xf4]; // SUB EAX, 1
    emu.regs_mut().rax = 0x80000000;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x7FFFFFFF);
    assert!(emu.flags().f_of, "OF should be set (32-bit signed overflow)");
    assert!(!emu.flags().f_sf, "SF should be clear (result positive in 32-bit)");
    assert!(!emu.flags().f_cf, "CF should be clear (no unsigned borrow)");
}

#[test]
fn test_sub_flags_64bit_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 64-bit signed overflow: 0x8000000000000000 - 1 = 0x7FFFFFFFFFFFFFFF
    let code = [0x48, 0x83, 0xe8, 0x01, 0xf4]; // SUB RAX, 1
    emu.regs_mut().rax = 0x8000000000000000;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x7FFFFFFFFFFFFFFF);
    assert!(emu.flags().f_of, "OF should be set (64-bit signed overflow)");
    assert!(!emu.flags().f_sf, "SF should be clear (result positive)");
}

#[test]
fn test_sub_flags_64bit_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 64-bit unsigned borrow: 0 - 1 = 0xFFFFFFFFFFFFFFFF
    let code = [0x48, 0x83, 0xe8, 0x01, 0xf4]; // SUB RAX, 1
    emu.regs_mut().rax = 0;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF);
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
    assert!(emu.flags().f_sf, "SF should be set (result negative)");
}

#[test]
fn test_sub_auxiliary_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AF is set when there's a borrow from bit 4 to bit 3
    // 0x10 - 0x01 = 0x0F (borrow from bit 4)
    let code = [0x2c, 0x01, 0xf4]; // SUB AL, 1
    emu.regs_mut().rax = 0x10;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x0F);
    assert!(emu.flags().f_af, "AF should be set (borrow from bit 4)");
}

#[test]
fn test_sub_no_auxiliary_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x1F - 0x01 = 0x1E (no borrow from bit 4)
    let code = [0x2c, 0x01, 0xf4]; // SUB AL, 1
    emu.regs_mut().rax = 0x1F;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x1E);
    assert!(!emu.flags().f_af, "AF should be clear (no borrow from bit 4)");
}

// ============================================================================
// Complex Addressing Mode Tests
// ============================================================================

#[test]
fn test_sub_with_displacement() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB EAX, [RBX + 8]
    // 2B 43 08 = SUB EAX, [RBX + 8]
    let code = [0x2b, 0x43, 0x08, 0xf4];
    emu.regs_mut().rax = 150;
    emu.regs_mut().rbx = DATA_ADDR;
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR + 8, 50);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 100, "SUB EAX, [RBX + 8]: 150 - 50 = 100");
}

#[test]
fn test_sub_with_sib() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB EAX, [RBX + RCX*4]
    // 2B 04 8b = SUB EAX, [RBX + RCX*4]
    let code = [0x2b, 0x04, 0x8b, 0xf4];
    emu.regs_mut().rax = 125;
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 2; // index = 2, scale = 4, so offset = 8
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR + 8, 25);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 100, "SUB EAX, [RBX + RCX*4]: 125 - 25 = 100");
}

// ============================================================================
// 16-bit Operand Tests
// ============================================================================

#[test]
fn test_sub_rm16_r16() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB AX, CX (with 66 prefix)
    // 66 29 c8 = SUB AX, CX
    let code = [0x66, 0x29, 0xc8, 0xf4];
    emu.regs_mut().rax = 0xDEAD_5678;
    emu.regs_mut().rcx = 0xBEEF_1234;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x4444, "SUB AX, CX: 0x5678 - 0x1234 = 0x4444");
    assert_eq!(emu.regs().rax & 0xFFFF0000, 0xDEAD0000, "High word of EAX should be preserved");
}

#[test]
fn test_sub_16bit_borrow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB AX, 1 when AX = 0
    let code = [0x66, 0x83, 0xe8, 0x01, 0xf4]; // SUB AX, 1
    emu.regs_mut().rax = 0;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "SUB AX, 1: 0 - 1 = 0xFFFF");
    assert!(emu.flags().f_cf, "CF should be set (16-bit borrow)");
    assert!(emu.flags().f_sf, "SF should be set");
}

// ============================================================================
// Chained SUB Tests
// ============================================================================

#[test]
fn test_sub_chain_multi_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB EAX, EBX
    // SUB EAX, ECX
    // SUB EAX, EDX
    let code = [
        0x29, 0xd8, // SUB EAX, EBX
        0x29, 0xc8, // SUB EAX, ECX
        0x29, 0xd0, // SUB EAX, EDX
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 100;
    emu.regs_mut().rbx = 10;
    emu.regs_mut().rcx = 20;
    emu.regs_mut().rdx = 30;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 40, "100 - 10 - 20 - 30 = 40");
}

#[test]
fn test_sub_self_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB RAX, RAX (always zeros the register)
    let code = [
        0x48, 0x29, 0xc0, // SUB RAX, RAX
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0xDEADBEEF_CAFEBABE;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0, "SUB RAX, RAX always produces 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}
