//! Tests for the ADC instruction.
//!
//! ADC - Add with Carry
//!
//! Adds the destination operand, source operand, and the carry (CF) flag,
//! storing the result in the destination operand.
//!
//! Flags affected: OF, SF, ZF, AF, CF, PF
//!
//! Reference: docs/adc.txt

use crate::*;

// ============================================================================
// ADC AL, imm8 (opcode 14 ib)
// ============================================================================

#[test]
fn test_adc_al_imm8_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC AL, 5 when CF=0
    // 14 05 = ADC AL, 5
    // f4 = HLT
    let code = [0x14, 0x05, 0xf4];
    emu.regs_mut().rax = 10;
    emu.flags_mut().load(0x2); // CF=0
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 15, "ADC AL, 5: 10 + 5 + 0 = 15");
}

#[test]
fn test_adc_al_imm8_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC AL, 5 when CF=1
    let code = [0x14, 0x05, 0xf4];
    emu.regs_mut().rax = 10;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF=1
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 16, "ADC AL, 5: 10 + 5 + 1 = 16");
}

#[test]
fn test_adc_al_imm8_carry_propagation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC AL, 0xFF when CF=1, AL=0 -> should produce 0 with CF set
    let code = [0x14, 0xFF, 0xf4];
    emu.regs_mut().rax = 0;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0, "ADC AL, 0xFF: 0 + 0xFF + 1 = 0x100 (wraps to 0)");
    assert!(emu.flags().f_cf, "CF should be set (carry out)");
    assert!(emu.flags().f_zf, "ZF should be set (result = 0)");
}

#[test]
fn test_adc_al_imm8_overflow_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x14, 0x00, 0xf4];
    emu.regs_mut().rax = 0x7F;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x80, "ADC AL, 0: 0x7F + 0 + 1 = 0x80");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(emu.flags().f_sf, "SF should be set (result negative)");
}

#[test]
fn test_adc_al_imm8_double_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFF + 0xFF + 1 = 0x1FF -> 0xFF with carry
    let code = [0x14, 0xFF, 0xf4];
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "ADC AL, 0xFF: 0xFF + 0xFF + 1 = 0x1FF (low byte = 0xFF)");
    assert!(emu.flags().f_cf, "CF should be set");
}

// ============================================================================
// ADC AX/EAX/RAX, imm16/32 (opcode 15)
// ============================================================================

#[test]
fn test_adc_ax_imm16_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC AX, 0x1234 when CF=0
    // 66 15 34 12 = ADC AX, 0x1234
    let code = [0x66, 0x15, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0x1000;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x2234, "ADC AX, 0x1234: 0x1000 + 0x1234 + 0");
}

#[test]
fn test_adc_ax_imm16_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC AX, 0x1234 when CF=1
    let code = [0x66, 0x15, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 0x1000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFF, 0x2235, "ADC AX, 0x1234: 0x1000 + 0x1234 + 1 = 0x2235");
}

#[test]
fn test_adc_eax_imm32_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC EAX, 0x12345678 when CF=1
    // 15 78 56 34 12 = ADC EAX, 0x12345678
    let code = [0x15, 0x78, 0x56, 0x34, 0x12, 0xf4];
    emu.regs_mut().rax = 1;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x1234567A, "ADC EAX, 0x12345678: 1 + 0x12345678 + 1");
}

#[test]
fn test_adc_rax_imm32_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W ADC RAX, imm32 (sign-extended) when CF=1
    // 48 15 ff ff ff ff = ADC RAX, -1
    let code = [0x48, 0x15, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rax = 100;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    // 100 + (-1) + 1 = 100
    assert_eq!(emu.regs().rax, 100, "ADC RAX, -1: 100 + (-1) + 1 = 100");
}

// ============================================================================
// ADC r/m8, imm8 (opcode 80 /2)
// ============================================================================

#[test]
fn test_adc_rm8_imm8_register_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC CL, 10 when CF=0
    // 80 d1 0a = ADC CL, 10
    let code = [0x80, 0xd1, 0x0a, 0xf4];
    emu.regs_mut().rcx = 5;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 15, "ADC CL, 10: 5 + 10 + 0 = 15");
}

#[test]
fn test_adc_rm8_imm8_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC CL, 10 when CF=1
    let code = [0x80, 0xd1, 0x0a, 0xf4];
    emu.regs_mut().rcx = 5;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 16, "ADC CL, 10: 5 + 10 + 1 = 16");
}

#[test]
fn test_adc_rm8_imm8_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC BYTE PTR [RBX], 10 when CF=1
    // 80 13 0a = ADC BYTE PTR [RBX], 10
    let code = [0x80, 0x13, 0x0a, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 25);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 36, "ADC [RBX], 10: 25 + 10 + 1 = 36");
}

// ============================================================================
// ADC r/m16/32/64, imm32 (opcode 81 /2)
// ============================================================================

#[test]
fn test_adc_rm32_imm32_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC ECX, 0x12345678 when CF=1
    // 81 d1 78 56 34 12 = ADC ECX, 0x12345678
    let code = [0x81, 0xd1, 0x78, 0x56, 0x34, 0x12, 0xf4];
    emu.regs_mut().rcx = 1;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x1234567A, "ADC ECX, 0x12345678: 1 + 0x12345678 + 1");
}

#[test]
fn test_adc_rm64_imm32_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W ADC RCX, -1 when CF=1
    // 48 81 d1 ff ff ff ff = ADC RCX, -1
    let code = [0x48, 0x81, 0xd1, 0xff, 0xff, 0xff, 0xff, 0xf4];
    emu.regs_mut().rcx = 0x100000000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    // 0x100000000 + (-1) + 1 = 0x100000000
    assert_eq!(emu.regs().rcx, 0x100000000, "ADC RCX, -1: 0x100000000 + (-1) + 1");
}

#[test]
fn test_adc_rm32_imm32_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC DWORD PTR [RBX], 0x1000 when CF=1
    // 81 13 00 10 00 00 = ADC DWORD PTR [RBX], 0x1000
    let code = [0x81, 0x13, 0x00, 0x10, 0x00, 0x00, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x12346679, "ADC [RBX], 0x1000: with CF=1");
}

// ============================================================================
// ADC r/m16/32/64, imm8 sign-extended (opcode 83 /2)
// ============================================================================

#[test]
fn test_adc_rm32_imm8_positive_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC ECX, 10 when CF=1
    // 83 d1 0a = ADC ECX, 10
    let code = [0x83, 0xd1, 0x0a, 0xf4];
    emu.regs_mut().rcx = 100;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 111, "ADC ECX, 10: 100 + 10 + 1 = 111");
}

#[test]
fn test_adc_rm32_imm8_negative_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC ECX, -10 when CF=1
    // 83 d1 f6 = ADC ECX, -10
    let code = [0x83, 0xd1, 0xf6, 0xf4];
    emu.regs_mut().rcx = 100;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 91, "ADC ECX, -10: 100 + (-10) + 1 = 91");
}

#[test]
fn test_adc_rm64_imm8_sign_extended_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W ADC RCX, -1 when CF=1
    // 48 83 d1 ff = ADC RCX, -1
    let code = [0x48, 0x83, 0xd1, 0xff, 0xf4];
    emu.regs_mut().rcx = 0x100000000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x100000000, "ADC RCX, -1: with CF=1 results in same value");
}

// ============================================================================
// ADC r/m8, r8 (opcode 10 /r)
// ============================================================================

#[test]
fn test_adc_rm8_r8_register_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC AL, CL when CF=0
    // 10 c8 = ADC AL, CL
    let code = [0x10, 0xc8, 0xf4];
    emu.regs_mut().rax = 10;
    emu.regs_mut().rcx = 5;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 15, "ADC AL, CL: 10 + 5 + 0 = 15");
}

#[test]
fn test_adc_rm8_r8_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC AL, CL when CF=1
    let code = [0x10, 0xc8, 0xf4];
    emu.regs_mut().rax = 10;
    emu.regs_mut().rcx = 5;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 16, "ADC AL, CL: 10 + 5 + 1 = 16");
}

#[test]
fn test_adc_rm8_r8_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC [RBX], CL when CF=1
    // 10 0b = ADC [RBX], CL
    let code = [0x10, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 20;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 30);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_byte(DATA_ADDR).unwrap(), 51, "ADC [RBX], CL: 30 + 20 + 1 = 51");
}

// ============================================================================
// ADC r/m16/32/64, r16/32/64 (opcode 11 /r)
// ============================================================================

#[test]
fn test_adc_rm32_r32_register_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC EAX, ECX when CF=0
    // 11 c8 = ADC EAX, ECX
    let code = [0x11, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x11111111;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x23456789, "ADC EAX, ECX with CF=0");
}

#[test]
fn test_adc_rm32_r32_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC EAX, ECX when CF=1
    let code = [0x11, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x7FFFFFFF;
    emu.regs_mut().rcx = 1;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x80000001, "ADC EAX, ECX: 0x7FFFFFFF + 1 + 1");
}

#[test]
fn test_adc_rm64_r64_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W ADC RAX, RCX when CF=1
    // 48 11 c8 = ADC RAX, RCX
    let code = [0x48, 0x11, 0xc8, 0xf4];
    emu.regs_mut().rax = 0xFFFFFFFF_FFFFFFFF;
    emu.regs_mut().rcx = 0;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0, "ADC RAX, RCX: max + 0 + 1 = 0 with overflow");
    assert!(emu.flags().f_cf, "CF should be set on overflow");
}

#[test]
fn test_adc_rm32_r32_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC [RBX], ECX when CF=1
    // 11 0b = ADC [RBX], ECX
    let code = [0x11, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 0x1000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(DATA_ADDR).unwrap(), 0x12346679, "ADC [RBX], ECX with CF=1");
}

// ============================================================================
// ADC r8, r/m8 (opcode 12 /r)
// ============================================================================

#[test]
fn test_adc_r8_rm8_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC CL, AL when CF=1
    // 12 c8 = ADC CL, AL
    let code = [0x12, 0xc8, 0xf4];
    emu.regs_mut().rax = 10;
    emu.regs_mut().rcx = 5;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 16, "ADC CL, AL: 5 + 10 + 1 = 16");
}

#[test]
fn test_adc_r8_rm8_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC CL, [RBX] when CF=1
    // 12 0b = ADC CL, [RBX]
    let code = [0x12, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 20;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_byte(DATA_ADDR, 30);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx & 0xFF, 51, "ADC CL, [RBX]: 20 + 30 + 1 = 51");
}

// ============================================================================
// ADC r16/32/64, r/m16/32/64 (opcode 13 /r)
// ============================================================================

#[test]
fn test_adc_r32_rm32_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC ECX, EAX when CF=1
    // 13 c8 = ADC ECX, EAX
    let code = [0x13, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x11111111;
    emu.regs_mut().rcx = 0x22222222;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x33333334, "ADC ECX, EAX: with CF=1");
}

#[test]
fn test_adc_r64_rm64_register_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.W ADC RCX, RAX when CF=1
    // 48 13 c8 = ADC RCX, RAX
    let code = [0x48, 0x13, 0xc8, 0xf4];
    emu.regs_mut().rax = 0x1111111111111111;
    emu.regs_mut().rcx = 0x2222222222222222;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x3333333333333334, "ADC RCX, RAX with CF=1");
}

#[test]
fn test_adc_r32_rm32_memory_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC ECX, [RBX] when CF=1
    // 13 0b = ADC ECX, [RBX]
    let code = [0x13, 0x0b, 0xf4];
    emu.regs_mut().rbx = DATA_ADDR;
    emu.regs_mut().rcx = 0x1000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.maps.write_dword(DATA_ADDR, 0x12345678);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0x12346679, "ADC ECX, [RBX] with CF=1");
}

// ============================================================================
// Multi-byte Addition (ADC Chain) Tests
// ============================================================================

#[test]
fn test_adc_chain_64bit_addition() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADD EAX, ECX (low 32 bits)
    // ADC EBX, EDX (high 32 bits with carry)
    let code = [
        0x01, 0xc8,       // ADD EAX, ECX
        0x11, 0xd3,       // ADC EBX, EDX
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0x00000001; // EAX = low word of first number
    emu.regs_mut().rcx = 0x00000001; // ECX = low word of second number
    emu.regs_mut().rbx = 0xFFFFFFFF; // EBX = high word of first number
    emu.regs_mut().rdx = 0x00000000; // EDX = high word of second number
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 2, "Low 32 bits: 1 + 1 = 2");
    assert_eq!(emu.regs().rbx, 0xFFFFFFFF, "High 32 bits: 0xFFFFFFFF + 0 + 0");
}

#[test]
fn test_adc_chain_with_carry_propagation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADD EAX, ECX (low, produces carry)
    // ADC EBX, EDX (high, receives carry)
    let code = [
        0x01, 0xc8,       // ADD EAX, ECX
        0x11, 0xd3,       // ADC EBX, EDX
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0xFFFFFFFF; // Low word of first number
    emu.regs_mut().rcx = 0x00000001; // Low word of second number
    emu.regs_mut().rbx = 0x00000000; // High word of first number
    emu.regs_mut().rdx = 0x00000000; // High word of second number
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0, "Low 32 bits: 0xFFFFFFFF + 1 = 0 (with carry)");
    assert_eq!(emu.regs().rbx, 1, "High 32 bits: 0 + 0 + 1 (carry) = 1");
}

#[test]
fn test_adc_chain_128bit_addition() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // word0 + word1 + word2 (with carry) + word3 (with carry)
    let code = [
        0x01, 0xc8,       // ADD EAX, ECX
        0x11, 0xd3,       // ADC EBX, EDX
        0x45, 0x11, 0xc0, // ADC R8D, R8D (simulate adding 0 with carry)
        0xf4,             // HLT
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rcx = 0x00000002; // This will produce carry
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rdx = 0x00000000; // This will receive carry and produce carry
    emu.regs_mut().r8 = 0x00000000;  // This will receive carry
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 1, "word0: 0xFFFFFFFF + 2 = 1 (with carry)");
    assert_eq!(emu.regs().rbx, 0, "word1: 0xFFFFFFFF + 0 + carry = 0 (with carry)");
    assert_eq!(emu.regs().r8, 1, "word2: 0 + 0 + carry = 1");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_adc_zero_flag_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADC producing zero result with carry
    // 0xFF + 0x00 + 1 = 0x100 (0 in 8-bit)
    let code = [0x14, 0x00, 0xf4]; // ADC AL, 0
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0);
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(emu.flags().f_cf, "CF should be set (carry out)");
}

#[test]
fn test_adc_auxiliary_carry_with_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AF test: 0x0F + 0x00 + 1 = 0x10 (carry from bit 3)
    let code = [0x14, 0x00, 0xf4]; // ADC AL, 0
    emu.regs_mut().rax = 0x0F;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x10);
    assert!(emu.flags().f_af, "AF should be set (carry from bit 3)");
}

#[test]
fn test_adc_overflow_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x14, 0x01, 0xf4]; // ADC AL, 1
    emu.regs_mut().rax = 0x7F;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFF, 0x81);
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

#[test]
fn test_adc_parity_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x01 + 0x00 + 1 = 0x02 (odd parity)
    let code = [0x14, 0x00, 0xf4];
    emu.regs_mut().rax = 0x01;
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
fn test_adc_r8_extended_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.B ADC R8D, 100 when CF=1
    // 41 83 d0 64 = ADC R8D, 100
    let code = [0x41, 0x83, 0xd0, 0x64, 0xf4];
    emu.regs_mut().r8 = 50;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 151, "ADC R8D, 100: 50 + 100 + 1 = 151");
}

#[test]
fn test_adc_r15_extended_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // REX.WB ADC R15, RAX when CF=1
    // 49 11 c7 = ADC R15, RAX
    let code = [0x49, 0x11, 0xc7, 0xf4];
    emu.regs_mut().rax = 0x1000;
    emu.regs_mut().r15 = 0x2000;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.regs().r15, 0x3001, "ADC R15, RAX: 0x2000 + 0x1000 + 1 = 0x3001");
}
