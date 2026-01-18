//! Tests for the SBB instruction.
//!
//! SBB - Subtract with Borrow
//!
//! Adds the source operand and the carry (CF) flag, and subtracts the result
//! from the destination operand. Operation: DEST = DEST - SRC - CF
//!
//! Flags affected: OF, SF, ZF, AF, CF, PF
//!
//! Reference: docs/sbb.txt

use crate::*;

// ============================================================================
// SBB AL, imm8 (opcode 1C ib)
// ============================================================================

#[test]
fn test_sbb_al_imm8_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, 5 when CF=0
    // 1C 05 = SBB AL, 5
    // f4 = HLT
    let code = [0x1c, 0x05, 0xf4];
    emu.regs_mut().rax = 15;
    emu.flags_mut().load(0x2); // CF=0
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 10, "SBB AL, 5: 15 - 5 - 0 = 10");
}

#[test]
fn test_sbb_al_imm8_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, 5 when CF=1
    let code = [0x1c, 0x05, 0xf4];
    emu.regs_mut().rax = 15;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF=1
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 9, "SBB AL, 5: 15 - 5 - 1 = 9");
}

#[test]
fn test_sbb_al_imm8_borrow_propagation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, 0 when CF=1, AL=0 -> should produce 0xFF with borrow
    let code = [0x1c, 0x00, 0xf4];
    emu.regs_mut().rax = 0;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "SBB AL, 0: 0 - 0 - 1 = 0xFF (borrow)");
    assert!(emu.flags().f_cf, "CF should be set (borrow out)");
    assert!(emu.flags().f_sf, "SF should be set (result negative)");
}

#[test]
fn test_sbb_al_imm8_underflow_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x1c, 0x00, 0xf4];
    emu.regs_mut().rax = 0x80;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x7F, "SBB AL, 0: 0x80 - 0 - 1 = 0x7F");
    assert!(emu.flags().f_of, "OF should be set (signed underflow)");
    assert!(!emu.flags().f_sf, "SF should be clear (result positive)");
}

#[test]
fn test_sbb_al_imm8_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 10 - 9 - 1 = 0
    let code = [0x1c, 0x09, 0xf4];
    emu.regs_mut().rax = 10;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0, "SBB AL, 9: 10 - 9 - 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set (result = 0)");
}

// ============================================================================
// SBB AX/EAX/RAX, imm16/32 (opcode 1D)
// ============================================================================

#[test]
fn test_sbb_ax_imm16_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AX, 0x1234 when CF=0
    // 66 1D 34 12 = SBB AX, 0x1234
    let code = [0x66, 0x1d, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0x5678;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x4444, "SBB AX, 0x1234: 0x5678 - 0x1234 - 0");
}

#[test]
fn test_sbb_ax_imm16_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AX, 0x1234 when CF=1
    let code = [0x66, 0x1d, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0x5678;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x4443, "SBB AX, 0x1234: 0x5678 - 0x1234 - 1 = 0x4443");
}

#[test]
fn test_sbb_eax_imm32_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB EAX, 0x12345678 when CF=1
    // 1D 78 56 34 12 = SBB EAX, 0x12345678
    let code = [0x1d, 0x78, 0x56, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0x23456789;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x11111110, "SBB EAX, 0x12345678: with CF=1");
}

#[test]
fn test_sbb_rax_imm32_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SBB RAX, imm32 (sign-extended) when CF=1
    // 48 1D ff ff ff ff = SBB RAX, -1
    let code = [0x48, 0x1d, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rax = 100;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    // 100 - (-1) - 1 = 100
    assert_eq!(emu.regs().rax, 100, "SBB RAX, -1: 100 - (-1) - 1 = 100");
}

// ============================================================================
// SBB r/m8, imm8 (opcode 80 /3)
// ============================================================================

#[test]
fn test_sbb_rm8_imm8_register_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB CL, 10 when CF=0
    // 80 d9 0a = SBB CL, 10
    let code = [0x80, 0xd9, 0x0a, 0xf4];
    emu.regs_mut().rcx = 25;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 15, "SBB CL, 10: 25 - 10 - 0 = 15");
}

#[test]
fn test_sbb_rm8_imm8_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB CL, 10 when CF=1
    let code = [0x80, 0xd9, 0x0a, 0xf4];
    emu.regs_mut().rcx = 25;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 14, "SBB CL, 10: 25 - 10 - 1 = 14");
}

#[test]
fn test_sbb_rm8_imm8_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB BYTE PTR [RBX], 10 when CF=1
    // 80 1b 0a = SBB BYTE PTR [RBX], 10
    let code = [0x80, 0x1b, 0x0a, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 50);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 39, "SBB [RBX], 10: 50 - 10 - 1 = 39");
}

// ============================================================================
// SBB r/m16/32/64, imm32 (opcode 81 /3)
// ============================================================================

#[test]
fn test_sbb_rm32_imm32_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB ECX, 0x12345678 when CF=1
    // 81 d9 78 56 34 12 = SBB ECX, 0x12345678
    let code = [0x81, 0xd9, 0x78, 0x56, 0x34, 0x12, 0xf4];
    emu.regs_mut().rcx = 0x23456789;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x11111110, "SBB ECX, 0x12345678: with CF=1");
}

#[test]
fn test_sbb_rm64_imm32_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SBB RCX, -1 when CF=1
    // 48 81 d9 ff ff ff ff = SBB RCX, -1
    let code = [0x48, 0x81, 0xd9, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rcx = 0x100000000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    // 0x100000000 - (-1) - 1 = 0x100000000
    assert_eq!(emu.regs().rcx, 0x100000000, "SBB RCX, -1: with CF=1");
}

#[test]
fn test_sbb_rm32_imm32_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB DWORD PTR [RBX], 0x1000 when CF=1
    // 81 1b 00 10 00 00 = SBB DWORD PTR [RBX], 0x1000
    let code = [0x81, 0x1b, 0x00, 0x10, 0x00, 0x00, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x12344677, "SBB [RBX], 0x1000: with CF=1");
}

// ============================================================================
// SBB r/m16/32/64, imm8 sign-extended (opcode 83 /3)
// ============================================================================

#[test]
fn test_sbb_rm32_imm8_positive_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB ECX, 10 when CF=1
    // 83 d9 0a = SBB ECX, 10
    let code = [0x83, 0xd9, 0x0a, 0xf4];
    emu.regs_mut().rcx = 100;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 89, "SBB ECX, 10: 100 - 10 - 1 = 89");
}

#[test]
fn test_sbb_rm32_imm8_negative_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB ECX, -10 when CF=1
    // 83 d9 f6 = SBB ECX, -10
    let code = [0x83, 0xd9, 0xf6, 0xf4];
    emu.regs_mut().rcx = 100;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 109, "SBB ECX, -10: 100 - (-10) - 1 = 109");
}

#[test]
fn test_sbb_rm64_imm8_sign_extended_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SBB RCX, -1 when CF=1
    // 48 83 d9 ff = SBB RCX, -1
    let code = [0x48, 0x83, 0xd9, 0xff, 0xf4];
    emu.regs_mut().rcx = 0x100000000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x100000000, "SBB RCX, -1: with CF=1 results in same value");
}

// ============================================================================
// SBB r/m8, r8 (opcode 18 /r)
// ============================================================================

#[test]
fn test_sbb_rm8_r8_register_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, CL when CF=0
    // 18 c8 = SBB AL, CL
    let code = [0x18, 0xc8, 0xf4];
    emu.regs_mut().rax = 15;
    emu.regs_mut().rcx = 5;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 10, "SBB AL, CL: 15 - 5 - 0 = 10");
}

#[test]
fn test_sbb_rm8_r8_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB AL, CL when CF=1
    let code = [0x18, 0xc8, 0xf4];
    emu.regs_mut().rax = 15;
    emu.regs_mut().rcx = 5;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 9, "SBB AL, CL: 15 - 5 - 1 = 9");
}

#[test]
fn test_sbb_rm8_r8_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB [RBX], CL when CF=1
    // 18 0b = SBB [RBX], CL
    let code = [0x18, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 20;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 50);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 29, "SBB [RBX], CL: 50 - 20 - 1 = 29");
}

// ============================================================================
// SBB r/m16/32/64, r16/32/64 (opcode 19 /r)
// ============================================================================

#[test]
fn test_sbb_rm32_r32_register_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB EAX, ECX when CF=0
    // 19 c8 = SBB EAX, ECX
    let code = [0x19, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x23456789;
    emu.regs_mut().rcx = 0x11111111;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x12345678, "SBB EAX, ECX with CF=0");
}

#[test]
fn test_sbb_rm32_r32_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB EAX, ECX when CF=1
    let code = [0x19, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x80000000;
    emu.regs_mut().rcx = 0;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x7FFFFFFF, "SBB EAX, ECX: 0x80000000 - 0 - 1 = 0x7FFFFFFF");
}

#[test]
fn test_sbb_rm64_r64_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SBB RAX, RCX when CF=1
    // 48 19 c8 = SBB RAX, RCX
    let code = [0x48, 0x19, 0xc8, 0xf4];
    emu.regs_mut().rax = 0;
    emu.regs_mut().rcx = 0;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "SBB RAX, RCX: 0 - 0 - 1 = max with borrow");
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
}

#[test]
fn test_sbb_rm32_r32_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB [RBX], ECX when CF=1
    // 19 0b = SBB [RBX], ECX
    let code = [0x19, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 0x1000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x12344677, "SBB [RBX], ECX with CF=1");
}

// ============================================================================
// SBB r8, r/m8 (opcode 1A /r)
// ============================================================================

#[test]
fn test_sbb_r8_rm8_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB CL, AL when CF=1
    // 1A c8 = SBB CL, AL
    let code = [0x1a, 0xc8, 0xf4];
    emu.regs_mut().rax = 5;
    emu.regs_mut().rcx = 15;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 9, "SBB CL, AL: 15 - 5 - 1 = 9");
}

#[test]
fn test_sbb_r8_rm8_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB CL, [RBX] when CF=1
    // 1A 0b = SBB CL, [RBX]
    let code = [0x1a, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 50;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 20);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 29, "SBB CL, [RBX]: 50 - 20 - 1 = 29");
}

// ============================================================================
// SBB r16/32/64, r/m16/32/64 (opcode 1B /r)
// ============================================================================

#[test]
fn test_sbb_r32_rm32_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB ECX, EAX when CF=1
    // 1B c8 = SBB ECX, EAX
    let code = [0x1b, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x11111111;
    emu.regs_mut().rcx = 0x33333333;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x22222221, "SBB ECX, EAX: with CF=1");
}

#[test]
fn test_sbb_r64_rm64_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W SBB RCX, RAX when CF=1
    // 48 1B c8 = SBB RCX, RAX
    let code = [0x48, 0x1b, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x1111111111111111;
    emu.regs_mut().rcx = 0x3333333333333333;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x2222222222222221, "SBB RCX, RAX with CF=1");
}

#[test]
fn test_sbb_r32_rm32_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB ECX, [RBX] when CF=1
    // 1B 0b = SBB ECX, [RBX]
    let code = [0x1b, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 0x12345678;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x1000);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x12344677, "SBB ECX, [RBX] with CF=1");
}

// ============================================================================
// Multi-byte Subtraction (SBB Chain) Tests
// ============================================================================

#[test]
fn test_sbb_chain_64bit_subtraction() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB EAX, ECX (low 32 bits)
    // SBB EBX, EDX (high 32 bits with borrow)
    let code = [
        0x29, 0xc8,       // SUB EAX, ECX
        0x19, 0xd3,       // SBB EBX, EDX
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x00000000; // Low word of first number
    emu.regs_mut().rcx = 0x00000001; // Low word of second number
    emu.regs_mut().rbx = 0x00000001; // High word of first number
    emu.regs_mut().rdx = 0x00000000; // High word of second number
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "Low 32 bits: 0 - 1 = 0xFFFFFFFF (borrow)");
    assert_eq!(emu.regs().rbx, 0, "High 32 bits: 1 - 0 - 1 (borrow) = 0");
}

#[test]
fn test_sbb_chain_with_borrow_propagation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SUB EAX, ECX (low, produces borrow)
    // SBB EBX, EDX (high, receives borrow)
    let code = [
        0x29, 0xc8,       // SUB EAX, ECX
        0x19, 0xd3,       // SBB EBX, EDX
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rcx = 0x00000001;
    emu.regs_mut().rbx = 0x00000001;
    emu.regs_mut().rdx = 0x00000000;
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "Low 32 bits with borrow");
    assert_eq!(emu.regs().rbx, 0, "High 32 bits: 1 - 0 - 1 (borrow)");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_sbb_zero_flag_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SBB producing zero result with carry
    // 10 - 9 - 1 = 0
    let code = [0x1c, 0x09, 0xf4]; // SBB AL, 9
    emu.regs_mut().rax = 10;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0);
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear (no borrow)");
}

#[test]
fn test_sbb_auxiliary_carry_with_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AF test: 0x10 - 0x00 - 1 = 0x0F (borrow from bit 4)
    let code = [0x1c, 0x00, 0xf4]; // SBB AL, 0
    emu.regs_mut().rax = 0x10;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x0F);
    assert!(emu.flags().f_af, "AF should be set (borrow from bit 4)");
}

#[test]
fn test_sbb_overflow_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x1c, 0x00, 0xf4]; // SBB AL, 0
    emu.regs_mut().rax = 0x80;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x7F);
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

#[test]
fn test_sbb_parity_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x03 - 0x00 - 1 = 0x02 (even parity)
    let code = [0x1c, 0x00, 0xf4];
    emu.regs_mut().rax = 0x03;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x02);
    assert!(!emu.flags().f_pf, "PF should be clear (odd parity)");
}

// ============================================================================
// Extended Register Tests
// ============================================================================

#[test]
fn test_sbb_r8_extended_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.B SBB R8D, 50 when CF=1
    // 41 83 d8 32 = SBB R8D, 50
    let code = [0x41, 0x83, 0xd8, 0x32, 0xf4];
    emu.regs_mut().r8 = 150;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 99, "SBB R8D, 50: 150 - 50 - 1 = 99");
}

#[test]
fn test_sbb_r15_extended_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.WB SBB R15, RAX when CF=1
    // 49 19 c7 = SBB R15, RAX
    let code = [0x49, 0x19, 0xc7, 0xf4];
    emu.regs_mut().rax = 0x1000;
    emu.regs_mut().r15 = 0x3000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r15, 0x1FFF, "SBB R15, RAX: 0x3000 - 0x1000 - 1 = 0x1FFF");
}
