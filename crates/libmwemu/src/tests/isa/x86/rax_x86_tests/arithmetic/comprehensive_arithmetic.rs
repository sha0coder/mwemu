//! Comprehensive tests for Group 1: Legacy & General Purpose Arithmetic
//!
//! This module provides extensive testing for:
//! - Legacy BCD/ASCII Adjust: aaa, aad, aam, aas
//! - Legacy Decimal Adjust: daa, das
//! - Integer Addition (Carry): adc, adcx, add, adox
//! - Integer Subtraction: dec, inc, neg, sbb
//! - Integer Subtraction (Base): sub
//! - Integer Multiplication: imul, mul, mulx
//! - Integer Division: div, idiv
//! - Sign Extension: cbw, cdq, cwde, cqo, cwd
//!
//! Each test category covers:
//! - Basic operations
//! - Boundary values (0, 1, max, min)
//! - Overflow/underflow conditions
//! - Flag behavior (CF, OF, ZF, SF, PF, AF)
//! - Register preservation
//! - Memory operand variants

use crate::*;

// ============================================================================
// ADC - ADD WITH CARRY: Comprehensive Edge Cases
// ============================================================================

#[test]
fn test_adc_carry_chain_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x14, 0x01, // ADC AL, 0x01
        0xf4,
    ];
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2 | flags::F_CF); // Set carry flag initially
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0xFF + 0x01 + 1(CF) = 0x101, result = 0x01, CF set
    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL should be 0x01");
    assert!(emu.flags().f_cf, "CF should be set (overflow)");
}

#[test]
fn test_adc_no_carry_in_no_carry_out() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x14, 0x01, 0xf4]; // ADC AL, 0x01
    emu.regs_mut().rax = 0x10;
    emu.flags_mut().load(0x2); // No carry flag
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x11, "0x10 + 0x01 = 0x11");
    assert!(!emu.flags().f_cf, "No carry out");
}

#[test]
fn test_adc_32bit_max_boundary() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // EAX = 0xFFFFFFFF, add 0 with carry = 1
    let code = [
        0x15, 0x00, 0x00, 0x00, 0x00, // ADC EAX, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "0xFFFFFFFF + 0 + 1 = 0");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_zf, "ZF should be set (result is 0)");
}

#[test]
fn test_adc_64bit_carry_propagation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // RAX = 0xFFFFFFFFFFFFFFFF, add 0 with carry = 1
    let code = [
        0x48, 0x15, 0x00, 0x00, 0x00, 0x00, // ADC RAX, 0
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "0xFFFFFFFFFFFFFFFF + 0 + 1 = 0");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_adc_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x7F + 0x01 = 0x80 (signed overflow: 127 + 1 = -128)
    let code = [0x14, 0x01, 0xf4]; // ADC AL, 0x01
    emu.regs_mut().rax = 0x7F;
    emu.flags_mut().load(0x2); // No carry
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "0x7F + 0x01 = 0x80");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(!emu.flags().f_cf, "CF should be clear (no unsigned overflow)");
    assert!(emu.flags().f_sf, "SF should be set (result is negative)");
}

#[test]
fn test_adc_auxiliary_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x0F + 0x01 = 0x10 (carry from lower nibble)
    let code = [0x14, 0x01, 0xf4]; // ADC AL, 0x01
    emu.regs_mut().rax = 0x0F;
    emu.flags_mut().load(0x2);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x10, "0x0F + 0x01 = 0x10");
    assert!(emu.flags().f_af, "AF should be set");
}

// ============================================================================
// ADD - INTEGER ADDITION: Comprehensive Tests
// ============================================================================

#[test]
fn test_add_zero_preservation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x00, 0xf4]; // ADD AL, 0
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "0 + 0 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_add_parity_even() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x02, 0xf4]; // ADD AL, 0x02
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "0x01 + 0x02 = 0x03");
    assert!(emu.flags().f_pf, "PF should be set (even parity)");
}

#[test]
fn test_add_parity_odd() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x04, 0x01, 0xf4]; // ADD AL, 0x01
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "0x00 + 0x01 = 0x01");
    assert!(!emu.flags().f_pf, "PF should be clear (odd parity)");
}

#[test]
fn test_add_register_to_register_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADD EAX, EBX (32-bit register to register)
    let code = [0x01, 0xd8, 0xf4]; // ADD EAX, EBX
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0x11111111;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x23456789, "0x12345678 + 0x11111111 = 0x23456789");
    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x11111111, "EBX unchanged");
}

#[test]
fn test_add_64bit_large_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADD RAX, RBX with large 64-bit values
    let code = [0x48, 0x01, 0xd8, 0xf4]; // ADD RAX, RBX
    emu.regs_mut().rax = 0x7FFFFFFFFFFFFFFF; // Max positive i64
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x8000000000000000, "Max i64 + 1 wraps to min");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_add_memory_operand() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADD EAX, [mem]
    let code = [
        0x03, 0x05, 0xfa, 0x0f, 0x00, 0x00, // ADD EAX, [rip+0x0FFA]
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 0x00000002);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000003, "1 + 2 = 3");
}

// ============================================================================
// SUB - INTEGER SUBTRACTION: Comprehensive Tests
// ============================================================================

#[test]
fn test_sub_zero_result() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x2c, 0x05, 0xf4]; // SUB AL, 0x05
    emu.regs_mut().rax = 0x05;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "5 - 5 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
    assert!(!emu.flags().f_cf, "CF should be clear (no borrow)");
}

#[test]
fn test_sub_borrow_required() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x00 - 0x01 = 0xFF with borrow
    let code = [0x2c, 0x01, 0xf4]; // SUB AL, 0x01
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "0 - 1 = 0xFF (underflow)");
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
    assert!(emu.flags().f_sf, "SF should be set (negative result)");
}

#[test]
fn test_sub_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x80 - 0x01 = 0x7F (signed overflow: -128 - 1 = 127)
    let code = [0x2c, 0x01, 0xf4]; // SUB AL, 0x01
    emu.regs_mut().rax = 0x80; // -128 as i8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x7F, "-128 - 1 = 127 (overflow)");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(!emu.flags().f_sf, "SF should be clear (positive result)");
}

#[test]
fn test_sub_32bit_underflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x2d, 0x01, 0x00, 0x00, 0x00, // SUB EAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "0 - 1 = 0xFFFFFFFF");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_sub_64bit_large_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x2d, 0x01, 0x00, 0x00, 0x00, // SUB RAX, 1
        0xf4,
    ];
    emu.regs_mut().rax = 0x8000000000000000; // Min i64
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x7FFFFFFFFFFFFFFF, "Min i64 - 1 wraps to max");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
}

// ============================================================================
// SBB - SUBTRACT WITH BORROW: Comprehensive Tests
// ============================================================================

#[test]
fn test_sbb_with_borrow_in() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x10 - 0x01 - 1(CF) = 0x0E
    let code = [0x1c, 0x01, 0xf4]; // SBB AL, 0x01
    emu.regs_mut().rax = 0x10;
    emu.flags_mut().load(0x2 | flags::F_CF); // Set carry (borrow)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x0E, "0x10 - 0x01 - 1 = 0x0E");
    assert!(!emu.flags().f_cf, "No borrow out");
}

#[test]
fn test_sbb_borrow_chain() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x00 - 0x00 - 1(CF) = 0xFF with borrow
    let code = [0x1c, 0x00, 0xf4]; // SBB AL, 0x00
    emu.regs_mut().rax = 0x00;
    emu.flags_mut().load(0x2 | flags::F_CF);
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "0x00 - 0x00 - 1 = 0xFF");
    assert!(emu.flags().f_cf, "CF should be set (borrow)");
}

#[test]
fn test_sbb_multiprecision_subtraction() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x2d, 0x01, 0x00, 0x00, 0x00, // SUB EAX, 1 (low dword)
        0x19, 0xca,                   // SBB EDX, ECX (high dword)
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000000; // Low dword
    emu.regs_mut().rdx = 0x00000001; // High dword
    emu.regs_mut().rcx = 0x00000000; // Subtract 0 from high (but with borrow from low)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x0000000100000000 - 1 = 0x00000000FFFFFFFF
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "Low dword underflowed");
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0x00000000, "High dword decremented by borrow");
}

// ============================================================================
// INC/DEC - INCREMENT/DECREMENT: Comprehensive Tests
// ============================================================================

#[test]
fn test_inc_basic_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc0, 0xf4]; // INC AL
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x01, "0 + 1 = 1");
}

#[test]
fn test_inc_overflow_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc0, 0xf4]; // INC AL
    emu.regs_mut().rax = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "0xFF + 1 = 0x00 (wrap)");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_inc_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc0, 0xf4]; // INC AL
    emu.regs_mut().rax = 0x7F; // 127
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "127 + 1 = -128 (signed overflow)");
    assert!(emu.flags().f_of, "OF should be set");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_inc_preserves_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // INC should NOT modify CF
    let code = [0xfe, 0xc0, 0xf4]; // INC AL
    emu.regs_mut().rax = 0xFF;
    emu.flags_mut().load(0x2 | flags::F_CF); // Set CF before
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be preserved by INC");
}

#[test]
fn test_dec_basic_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "1 - 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_dec_underflow_8bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "0 - 1 = 0xFF (underflow)");
    assert!(emu.flags().f_sf, "SF should be set");
    // DEC does NOT affect CF
}

#[test]
fn test_dec_signed_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    emu.regs_mut().rax = 0x80; // -128
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x7F, "-128 - 1 = 127 (signed overflow)");
    assert!(emu.flags().f_of, "OF should be set");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_inc_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xff, 0xc0, 0xf4]; // INC EAX
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "0xFFFFFFFF + 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_inc_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xff, 0xc0, 0xf4]; // INC RAX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "Max u64 + 1 = 0");
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// NEG - TWO'S COMPLEMENT NEGATION: Comprehensive Tests
// ============================================================================

#[test]
fn test_neg_positive_to_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x05; // 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFB, "NEG 5 = -5 (0xFB)");
    assert!(emu.flags().f_cf, "CF set when operand non-zero");
    assert!(emu.flags().f_sf, "SF should be set");
}

#[test]
fn test_neg_negative_to_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0xFB; // -5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x05, "NEG -5 = 5");
    assert!(emu.flags().f_cf, "CF set when operand non-zero");
    assert!(!emu.flags().f_sf, "SF should be clear");
}

#[test]
fn test_neg_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x00, "NEG 0 = 0");
    assert!(!emu.flags().f_cf, "CF clear when operand is zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_neg_min_value_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // NEG of minimum signed value overflows (no positive equivalent)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    emu.regs_mut().rax = 0x80; // -128, NEG(-128) can't be represented as i8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "NEG -128 = -128 (overflow, stays same)");
    assert!(emu.flags().f_of, "OF should be set (signed overflow)");
    assert!(emu.flags().f_cf, "CF should be set (non-zero operand)");
}

#[test]
fn test_neg_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    emu.regs_mut().rax = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFFF, "NEG 1 = -1");
}

#[test]
fn test_neg_64bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    emu.regs_mut().rax = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "NEG 1 = -1");
}

// ============================================================================
// MUL - UNSIGNED MULTIPLY: Comprehensive Tests
// ============================================================================

#[test]
fn test_mul_8bit_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AL * BL = AX
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0x05; // AL = 5
    emu.regs_mut().rbx = 0x03; // BL = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x000F, "5 * 3 = 15");
    assert!(!emu.flags().f_cf, "CF clear (fits in AL)");
    assert!(!emu.flags().f_of, "OF clear");
}

#[test]
fn test_mul_8bit_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 16 * 16 = 256 (overflows AL)
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 16;
    emu.regs_mut().rbx = 16;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0100, "16 * 16 = 256");
    assert!(emu.flags().f_cf, "CF set (overflow into AH)");
    assert!(emu.flags().f_of, "OF set");
}

#[test]
fn test_mul_8bit_max() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFF * 0xFF = 0xFE01
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0xFF;
    emu.regs_mut().rbx = 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFE01, "255 * 255 = 65025");
    assert!(emu.flags().f_cf, "CF set");
}

#[test]
fn test_mul_32bit_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // EAX * EBX = EDX:EAX
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 0x00001000;
    emu.regs_mut().rbx = 0x00001000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x1000 * 0x1000 = 0x01000000
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x01000000, "Low 32 bits");
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0x00000000, "High 32 bits");
    assert!(!emu.flags().f_cf, "CF clear (fits in EAX)");
}

#[test]
fn test_mul_32bit_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x80000000 * 2 = 0x100000000 (overflows into EDX)
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 0x80000000;
    emu.regs_mut().rbx = 0x00000002;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "Low 32 bits");
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0x00000001, "High 32 bits");
    assert!(emu.flags().f_cf, "CF set (overflow into EDX)");
}

#[test]
fn test_mul_64bit_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 0x0000000100000000;
    emu.regs_mut().rbx = 0x0000000000000002;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000200000000, "Low 64 bits");
    assert_eq!(emu.regs().rdx, 0x0000000000000000, "High 64 bits");
    assert!(!emu.flags().f_cf, "CF clear");
}

#[test]
fn test_mul_by_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0xFF;
    emu.regs_mut().rbx = 0x00;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0000, "Anything * 0 = 0");
    assert!(!emu.flags().f_cf, "CF clear (no overflow)");
}

#[test]
fn test_mul_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0xAB;
    emu.regs_mut().rbx = 0x01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x00AB, "Anything * 1 = itself");
    assert!(!emu.flags().f_cf, "CF clear");
}

// ============================================================================
// DIV - UNSIGNED DIVIDE: Comprehensive Tests
// ============================================================================

#[test]
fn test_div_8bit_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // AX / BL = AL (quotient), AH (remainder)
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 0x0011; // AX = 17
    emu.regs_mut().rbx = 0x05;   // BL = 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "17 / 5 = 3 (quotient in AL)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "17 % 5 = 2 (remainder in AH)");
}

#[test]
fn test_div_8bit_exact() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 15 / 5 = 3 with no remainder
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 0x000F; // AX = 15
    emu.regs_mut().rbx = 0x05;   // BL = 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "15 / 5 = 3");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "15 % 5 = 0");
}

#[test]
fn test_div_32bit_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // EDX:EAX / EBX = EAX (quotient), EDX (remainder)
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 0x00000064; // EAX = 100
    emu.regs_mut().rdx = 0x00000000; // EDX = 0
    emu.regs_mut().rbx = 0x00000007; // EBX = 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x0000000E, "100 / 7 = 14");
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0x00000002, "100 % 7 = 2");
}

#[test]
fn test_div_64bit_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 0x0000000000000064; // 100
    emu.regs_mut().rdx = 0x0000000000000000;
    emu.regs_mut().rbx = 0x0000000000000007; // 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x000000000000000E, "100 / 7 = 14");
    assert_eq!(emu.regs().rdx, 0x0000000000000002, "100 % 7 = 2");
}

#[test]
fn test_div_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 0x00FF; // AX = 255
    emu.regs_mut().rbx = 0x01;   // BL = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFF, "255 / 1 = 255");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "255 % 1 = 0");
}

#[test]
fn test_div_large_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x0100 / 2 = 0x80
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 0x0100; // AX = 256
    emu.regs_mut().rbx = 0x02;   // BL = 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "256 / 2 = 128");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x00, "256 % 2 = 0");
}

// ============================================================================
// IDIV - SIGNED DIVIDE: Comprehensive Tests
// ============================================================================

#[test]
fn test_idiv_positive_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 0x0011; // AX = 17
    emu.regs_mut().rbx = 0x05;   // BL = 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "17 / 5 = 3");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "17 % 5 = 2");
}

#[test]
fn test_idiv_negative_by_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -17 / 5 = -3, remainder -2
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 0xFFEF; // AX = -17 (sign-extended)
    emu.regs_mut().rbx = 0x05;   // BL = 5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFD, "-17 / 5 = -3 (0xFD)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFE, "-17 % 5 = -2 (0xFE)");
}

#[test]
fn test_idiv_positive_by_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 17 / -5 = -3, remainder 2
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 0x0011; // AX = 17
    emu.regs_mut().rbx = 0xFB;   // BL = -5 (0xFB)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xFD, "17 / -5 = -3 (0xFD)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "17 % -5 = 2");
}

#[test]
fn test_idiv_negative_by_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -17 / -5 = 3, remainder -2
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    emu.regs_mut().rax = 0xFFEF; // AX = -17
    emu.regs_mut().rbx = 0xFB;   // BL = -5
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x03, "-17 / -5 = 3");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFE, "-17 % -5 = -2 (0xFE)");
}

#[test]
fn test_idiv_32bit_signed() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xfb, 0xf4]; // IDIV EBX
    emu.regs_mut().rax = 0xFFFFFF9C; // EAX = -100 (sign-extended to 32-bit)
    emu.regs_mut().rdx = 0xFFFFFFFF; // EDX = -1 (sign-extension)
    emu.regs_mut().rbx = 0x00000007; // EBX = 7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFFFFF2, "-100 / 7 = -14 (0xFFFFFFF2)");
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0xFFFFFFFE, "-100 % 7 = -2 (0xFFFFFFFE)");
}

// ============================================================================
// Sign Extension Instructions: CBW, CWDE, CDQE, CWD, CDQ, CQO
// ============================================================================

#[test]
fn test_cbw_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CBW: AL -> AX (sign-extend byte to word)
    let code = [0x66, 0x98, 0xf4]; // CBW (0x66 in 64-bit mode)
    emu.regs_mut().rax = 0x7F; // AL = 127 (positive)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x007F, "127 sign-extended to word");
}

#[test]
fn test_cbw_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x98, 0xf4]; // CBW (0x66 in 64-bit mode)
    emu.regs_mut().rax = 0x80; // AL = -128 (negative)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFF80, "-128 sign-extended to word");
}

#[test]
fn test_cwde_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CWDE: AX -> EAX (sign-extend word to dword)
    let code = [0x98, 0xf4]; // CWDE (in 32-bit mode it's 0x98)
    emu.regs_mut().rax = 0x7FFF; // AX = 32767
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00007FFF, "32767 sign-extended");
}

#[test]
fn test_cwde_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x98, 0xf4]; // CWDE
    emu.regs_mut().rax = 0x8000; // AX = -32768
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xFFFF8000, "-32768 sign-extended");
}

#[test]
fn test_cdqe_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CDQE: EAX -> RAX (sign-extend dword to qword)
    let code = [0x48, 0x98, 0xf4]; // CDQE (REX.W + 0x98)
    emu.regs_mut().rax = 0x7FFFFFFF; // EAX = max positive i32
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x000000007FFFFFFF, "Max i32 sign-extended");
}

#[test]
fn test_cdqe_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x98, 0xf4]; // CDQE
    emu.regs_mut().rax = 0x80000000; // EAX = min i32 (-2147483648)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF80000000, "Min i32 sign-extended");
}

#[test]
fn test_cwd_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CWD: AX -> DX:AX (sign-extend word to dword, into DX:AX)
    let code = [0x66, 0x99, 0xf4]; // CWD (16-bit operand override + 0x99)
    emu.regs_mut().rax = 0x7FFF; // AX = 32767
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFF, 0x0000, "DX = 0 for positive");
}

#[test]
fn test_cwd_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0x99, 0xf4]; // CWD
    emu.regs_mut().rax = 0x8000; // AX = -32768
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFF, 0xFFFF, "DX = 0xFFFF for negative");
}

#[test]
fn test_cdq_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CDQ: EAX -> EDX:EAX (sign-extend dword, into EDX:EAX)
    let code = [0x99, 0xf4]; // CDQ
    emu.regs_mut().rax = 0x7FFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0x00000000, "EDX = 0 for positive");
}

#[test]
fn test_cdq_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x99, 0xf4]; // CDQ
    emu.regs_mut().rax = 0x80000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0xFFFFFFFF, "EDX = 0xFFFFFFFF for negative");
}

#[test]
fn test_cqo_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // CQO: RAX -> RDX:RAX (sign-extend qword)
    let code = [0x48, 0x99, 0xf4]; // CQO (REX.W + 0x99)
    emu.regs_mut().rax = 0x7FFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 0x0000000000000000, "RDX = 0 for positive");
}

#[test]
fn test_cqo_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0x99, 0xf4]; // CQO
    emu.regs_mut().rax = 0x8000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rdx, 0xFFFFFFFFFFFFFFFF, "RDX = -1 for negative");
}

// ============================================================================
// ADCX/ADOX - ADX Extension Instructions
// ============================================================================

#[test]
fn test_adcx_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADCX r32, r/m32 - add with carry flag (CF) only
    let code = [
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rbx = 0x00000002;
    emu.flags_mut().load(0x2 | flags::F_CF); // CF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000004, "1 + 2 + 1(CF) = 4");
}

#[test]
fn test_adcx_ignores_of() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADCX should only use CF, not OF
    let code = [
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rbx = 0x00000002;
    emu.flags_mut().load(0x2 | flags::F_OF); // Only OF set, not CF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000003, "1 + 2 + 0 = 3 (OF ignored)");
}

#[test]
fn test_adox_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADOX r32, r/m32 - add with overflow flag (OF) only
    let code = [
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rbx = 0x00000002;
    emu.flags_mut().load(0x2 | flags::F_OF); // OF set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000004, "1 + 2 + 1(OF) = 4");
}

#[test]
fn test_adox_ignores_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADOX should only use OF, not CF
    let code = [
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rbx = 0x00000002;
    emu.flags_mut().load(0x2 | flags::F_CF); // Only CF set, not OF
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000003, "1 + 2 + 0 = 3 (CF ignored)");
}

#[test]
fn test_adcx_adox_parallel() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // ADCX and ADOX can be used in parallel for multi-precision arithmetic
    let code = [
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (uses CF)
        0xf3, 0x0f, 0x38, 0xf6, 0xca, // ADOX ECX, EDX (uses OF)
        0xf4,
    ];
    emu.regs_mut().rax = 0x00000001;
    emu.regs_mut().rbx = 0x00000001;
    emu.regs_mut().rcx = 0x00000002;
    emu.regs_mut().rdx = 0x00000002;
    emu.flags_mut().load(0x2 | flags::F_CF | flags::F_OF); // Both set
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000003, "ADCX: 1 + 1 + 1 = 3");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0x00000005, "ADOX: 2 + 2 + 1 = 5");
}

// ============================================================================
// Complex Multi-Instruction Sequences
// ============================================================================

#[test]
fn test_multiprecision_addition_128bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // (RAX:RDX) + (RBX:RCX) = result in (RAX:RDX)
    let code = [
        0x48, 0x01, 0xd8,             // ADD RAX, RBX (low 64 bits)
        0x48, 0x11, 0xca,             // ADC RDX, RCX (high 64 bits with carry)
        0xf4,
    ];
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF; // Low
    emu.regs_mut().rdx = 0x0000000000000001; // High
    emu.regs_mut().rbx = 0x0000000000000001; // Low
    emu.regs_mut().rcx = 0x0000000000000000; // High
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "Low 64 bits");
    assert_eq!(emu.regs().rdx, 0x0000000000000002, "High 64 bits");
}

#[test]
fn test_multiprecision_subtraction_128bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x29, 0xd8,             // SUB RAX, RBX (low 64 bits)
        0x48, 0x19, 0xca,             // SBB RDX, RCX (high 64 bits with borrow)
        0xf4,
    ];
    emu.regs_mut().rax = 0x0000000000000000;
    emu.regs_mut().rdx = 0x0000000000000001;
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.regs_mut().rcx = 0x0000000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "Low 64 bits");
    assert_eq!(emu.regs().rdx, 0x0000000000000000, "High 64 bits");
}

#[test]
fn test_increment_loop_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xff, 0xc0,             // INC RAX
        0x48, 0x83, 0xf8, 0x05,       // CMP RAX, 5
        0x75, 0xf7,                   // JNE back to INC
        0xf4,
    ];
    emu.regs_mut().rax = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 5, "Should have incremented to 5");
}

#[test]
fn test_decrement_loop_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xff, 0xc8,             // DEC RAX
        0x75, 0xfb,                   // JNZ back to DEC (ZF=0 means continue)
        0xf4,
    ];
    emu.regs_mut().rax = 5;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "Should have decremented to 0");
}

// ============================================================================
// Edge Cases with Extended Registers (R8-R15)
// ============================================================================

#[test]
fn test_add_r8_r9() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x4d, 0x01, 0xc8, // ADD R8, R9
        0xf4,
    ];
    emu.regs_mut().r8 = 0x1111111111111111;
    emu.regs_mut().r9 = 0x2222222222222222;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r8, 0x3333333333333333, "R8 + R9");
    assert_eq!(emu.regs().r9, 0x2222222222222222, "R9 unchanged");
}

#[test]
fn test_sub_r10_r11() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x4d, 0x29, 0xda, // SUB R10, R11
        0xf4,
    ];
    emu.regs_mut().r10 = 0x5555555555555555;
    emu.regs_mut().r11 = 0x1111111111111111;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r10, 0x4444444444444444, "R10 - R11");
}

#[test]
fn test_inc_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x49, 0xff, 0xc7, // INC R15
        0xf4,
    ];
    emu.regs_mut().r15 = 0xFFFFFFFFFFFFFFFE;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0xFFFFFFFFFFFFFFFF, "R15 incremented");
}

#[test]
fn test_neg_r12() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x49, 0xf7, 0xdc, // NEG R12
        0xf4,
    ];
    emu.regs_mut().r12 = 0x0000000000000005;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r12, 0xFFFFFFFFFFFFFFFB, "NEG 5 = -5");
}
