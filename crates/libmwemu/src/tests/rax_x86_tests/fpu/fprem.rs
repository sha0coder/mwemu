//! Tests for the FPREM instruction.
//!
//! FPREM - Partial Remainder
//!
//! Computes the remainder from dividing ST(0) by ST(1) and stores result in ST(0).
//! Remainder = ST(0) - (Q * ST(1)) where Q is truncated quotient toward zero.
//! May require multiple executions if C2 flag is set (partial remainder).
//! Stores three least significant bits of quotient in C3, C1, C0.
//!
//! Opcode: D9 F8
//!
//! Flags affected:
//! - C0: Set to bit 2 (Q2) of quotient
//! - C1: Set to bit 0 (Q0) of quotient (or 0 if stack underflow)
//! - C2: Set to 0 if reduction complete, 1 if incomplete
//! - C3: Set to bit 1 (Q1) of quotient
//!
//! Reference: /Users/int/dev/rax/docs/fprem.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Helper function to write f64 to memory
fn write_f64(mem: u64, addr: u64, val: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// Helper function to read f64 from memory
fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FPREM - Basic Remainder Operations
// ============================================================================

#[test]
fn test_fprem_basic_positive() {
    let mut emu = emu64();    // 7.0 % 3.0 = 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; divisor (3.0)
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; dividend (7.0)
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; result
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "7.0 % 3.0 should be 1.0");
}

#[test]
fn test_fprem_exact_division() {
    let mut emu = emu64();    // 9.0 % 3.0 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; dividend
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 9.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "9.0 % 3.0 should be 0.0");
}

#[test]
fn test_fprem_small_dividend() {
    let mut emu = emu64();    // 2.0 % 5.0 = 2.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; dividend
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "2.0 % 5.0 should be 2.0");
}

#[test]
fn test_fprem_fractional() {
    let mut emu = emu64();    // 5.5 % 2.0 = 1.5
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.5);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.5, "5.5 % 2.0 should be 1.5");
}

// ============================================================================
// FPREM - Negative Dividends
// ============================================================================

#[test]
fn test_fprem_negative_dividend() {
    let mut emu = emu64();    // -7.0 % 3.0 = -1.0 (sign follows dividend)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -7.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, -1.0, "-7.0 % 3.0 should be -1.0");
}

#[test]
fn test_fprem_negative_divisor() {
    let mut emu = emu64();    // 7.0 % -3.0 = 1.0 (sign follows dividend)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);
    emu.maps.write_f64(0x2008, -3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "7.0 % -3.0 should be 1.0");
}

#[test]
fn test_fprem_both_negative() {
    let mut emu = emu64();    // -7.0 % -3.0 = -1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -7.0);
    emu.maps.write_f64(0x2008, -3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, -1.0, "-7.0 % -3.0 should be -1.0");
}

// ============================================================================
// FPREM - Special Values: Zero
// ============================================================================

#[test]
fn test_fprem_zero_dividend() {
    let mut emu = emu64();    // 0.0 % 5.0 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "0.0 % 5.0 should be 0.0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

#[test]
fn test_fprem_negative_zero_dividend() {
    let mut emu = emu64();    // -0.0 % 5.0 = -0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "-0.0 % 5.0 should be -0.0");
    assert!(result.is_sign_negative(), "Result should be negative zero");
}

// ============================================================================
// FPREM - Divisors
// ============================================================================

#[test]
fn test_fprem_divisor_one() {
    let mut emu = emu64();    // 5.5 % 1.0 = 0.5
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.5);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.5, "5.5 % 1.0 should be 0.5");
}

#[test]
fn test_fprem_small_divisor() {
    let mut emu = emu64();    // 10.0 % 0.5 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 0.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "10.0 % 0.5 should be 0.0");
}

#[test]
fn test_fprem_large_divisor() {
    let mut emu = emu64();    // 5.0 % 10.0 = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 5.0, "5.0 % 10.0 should be 5.0");
}

#[test]
fn test_fprem_fractional_divisor() {
    let mut emu = emu64();    // 7.0 % 1.5 = 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);
    emu.maps.write_f64(0x2008, 1.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "7.0 % 1.5 should be 1.0");
}

// ============================================================================
// FPREM - Special Values: Infinity
// ============================================================================

#[test]
fn test_fprem_finite_mod_infinity() {
    let mut emu = emu64();    // 5.0 % infinity = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, f64::INFINITY);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 5.0, "Finite % infinity should be the finite value");
}

// ============================================================================
// FPREM - Mathematical Constants
// ============================================================================

#[test]
fn test_fprem_pi_modulo() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::PI);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::PI - 3.0;
    assert!((result - expected).abs() < 1e-10, "PI % 1.0 computation");
}

#[test]
fn test_fprem_angle_reduction() {
    let mut emu = emu64();    // 10.0 % (2*PI) for angle reduction
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let angle = 10.0;
    let two_pi = 2.0 * std::f64::consts::PI;
    emu.maps.write_f64(0x2000, angle);
    emu.maps.write_f64(0x2008, two_pi);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result >= 0.0 && result < two_pi, "Angle should be reduced to [0, 2π)");
}

// ============================================================================
// FPREM - Multiple Operations
// ============================================================================

#[test]
fn test_fprem_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 3.0);
    emu.maps.write_f64(0x2010, 15.0);
    emu.maps.write_f64(0x2018, 4.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 1.0, "10.0 % 3.0 = 1.0");
    assert_eq!(result2, 3.0, "15.0 % 4.0 = 3.0");
}

// ============================================================================
// FPREM - Edge Cases
// ============================================================================

#[test]
fn test_fprem_very_large_dividend() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1000000.0);
    emu.maps.write_f64(0x2008, 7.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "1000000.0 % 7.0 = 1.0");
}

#[test]
fn test_fprem_very_small_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e-10);
    emu.maps.write_f64(0x2008, 3e-11);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1e-10 % 3e-11;
    assert!((result - expected).abs() < 1e-20, "Very small values");
}

#[test]
fn test_fprem_same_values() {
    let mut emu = emu64();    // x % x = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "x % x should be 0");
}

#[test]
fn test_fprem_preserves_divisor() {
    let mut emu = emu64();    // FPREM only modifies ST(0), ST(1) (divisor) remains unchanged
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; dividend
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; remainder
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; divisor
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let remainder = emu.maps.read_f64(0x3000).unwrap();
    let divisor = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(remainder, 1.0, "Remainder should be 1.0");
    assert_eq!(divisor, 3.0, "Divisor should remain 3.0");
}

// ============================================================================
// FPREM - Partial Remainder (C2 Flag)
// ============================================================================

#[test]
fn test_fprem_completion() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);
    emu.maps.write_f64(0x2008, 7.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "100.0 % 7.0 = 2.0");
}

// ============================================================================
// FPREM - Various Combinations
// ============================================================================

#[test]
fn test_fprem_power_of_two_divisor() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 17.0);
    emu.maps.write_f64(0x2008, 8.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "17.0 % 8.0 = 1.0");
}

#[test]
fn test_fprem_various_divisors() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_cases = vec![
        (10.0, 3.0, 1.0),
        (10.0, 4.0, 2.0),
        (10.0, 7.0, 3.0),
        (20.0, 6.0, 2.0),
        (100.0, 11.0, 1.0),
    ];

    for (dividend, divisor, expected) in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, dividend);
        emu.maps.write_f64(0x2008, divisor);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, expected, "{} % {} should be {}", dividend, divisor, expected);
    }
}

#[test]
fn test_fprem_decimal_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_cases = vec![
        (3.14, 1.0, 0.14),
        (2.71, 0.5, 0.21),
        (9.99, 2.0, 1.99),
    ];

    for (dividend, divisor, expected) in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, dividend);
        emu.maps.write_f64(0x2008, divisor);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        assert!((result - expected).abs() < 1e-10, "{} % {} should be approximately {}", dividend, divisor, expected);
    }
}

#[test]
fn test_fprem_irrational_dividend() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF8,                                  // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let sqrt2 = 2.0f64.sqrt();
    emu.maps.write_f64(0x2000, sqrt2);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = sqrt2 - 1.0;
    assert!((result - expected).abs() < 1e-10, "sqrt(2) % 1.0");
}
