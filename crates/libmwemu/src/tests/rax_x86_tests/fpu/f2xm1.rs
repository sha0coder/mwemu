//! Tests for the F2XM1 instruction.
//!
//! F2XM1 - Compute 2^X - 1
//!
//! Computes the exponential value of 2 to the power of ST(0) minus 1.
//! The source operand must lie in the range -1.0 to +1.0.
//! If the source is outside this range, the result is undefined.
//! Result is stored back in ST(0).
//!
//! Opcode: D9 F0
//!
//! Formula: ST(0) := (2^ST(0)) - 1
//!
//! Used for exponentiation: x^y := 2^(y * log2(x))
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow; set if result rounded up, cleared otherwise
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/f2xm1.txt

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
// F2XM1 - Zero
// ============================================================================

#[test]
fn test_f2xm1_zero() {
    let mut emu = emu64();    // 2^0 - 1 = 1 - 1 = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 0.0).abs() < 1e-10, "2^0 - 1 should be 0");
}

#[test]
fn test_f2xm1_positive_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "2^(+0) - 1 = +0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

#[test]
fn test_f2xm1_negative_zero() {
    let mut emu = emu64();    // 2^(-0) - 1 = 1 - 1 = -0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "2^(-0) - 1 = -0");
    assert!(result.is_sign_negative(), "Result should be negative zero");
}

// ============================================================================
// F2XM1 - One
// ============================================================================

#[test]
fn test_f2xm1_one() {
    let mut emu = emu64();    // 2^1 - 1 = 2 - 1 = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 1.0).abs() < 1e-10, "2^1 - 1 should be 1.0");
}

// ============================================================================
// F2XM1 - Positive Values
// ============================================================================

#[test]
fn test_f2xm1_half() {
    let mut emu = emu64();    // 2^0.5 - 1 = sqrt(2) - 1 ≈ 0.414
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(0.5) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.5 - 1 should be approximately 0.414");
}

#[test]
fn test_f2xm1_quarter() {
    let mut emu = emu64();    // 2^0.25 - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.25);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(0.25) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.25 - 1 calculation");
}

#[test]
fn test_f2xm1_three_quarters() {
    let mut emu = emu64();    // 2^0.75 - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.75);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(0.75) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.75 - 1 calculation");
}

#[test]
fn test_f2xm1_small_positive() {
    let mut emu = emu64();    // 2^0.1 - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(0.1) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.1 - 1 calculation");
}

#[test]
fn test_f2xm1_very_small_positive() {
    let mut emu = emu64();    // 2^0.01 - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.01);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(0.01) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.01 - 1 calculation");
}

// ============================================================================
// F2XM1 - Negative Values
// ============================================================================

#[test]
fn test_f2xm1_negative_one() {
    let mut emu = emu64();    // 2^(-1) - 1 = 0.5 - 1 = -0.5
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - (-0.5)).abs() < 1e-10, "2^(-1) - 1 should be -0.5");
}

#[test]
fn test_f2xm1_negative_half() {
    let mut emu = emu64();    // 2^(-0.5) - 1 = 1/sqrt(2) - 1 ≈ -0.293
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(-0.5) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^(-0.5) - 1 calculation");
}

#[test]
fn test_f2xm1_negative_quarter() {
    let mut emu = emu64();    // 2^(-0.25) - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.25);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(-0.25) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^(-0.25) - 1 calculation");
}

#[test]
fn test_f2xm1_small_negative() {
    let mut emu = emu64();    // 2^(-0.1) - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.1);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(-0.1) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^(-0.1) - 1 calculation");
}

#[test]
fn test_f2xm1_very_small_negative() {
    let mut emu = emu64();    // 2^(-0.01) - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.01);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(-0.01) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^(-0.01) - 1 calculation");
}

// ============================================================================
// F2XM1 - Range Limits
// ============================================================================

#[test]
fn test_f2xm1_upper_limit() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(1.0) - 1.0;
    assert!((result - expected).abs() < 1e-10, "Upper limit: 2^1 - 1 = 1.0");
}

#[test]
fn test_f2xm1_lower_limit() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(-1.0) - 1.0;
    assert!((result - expected).abs() < 1e-10, "Lower limit: 2^(-1) - 1 = -0.5");
}

#[test]
fn test_f2xm1_near_upper_limit() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.99);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(0.99) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.99 - 1 calculation");
}

#[test]
fn test_f2xm1_near_lower_limit() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.99);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(-0.99) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^(-0.99) - 1 calculation");
}

// ============================================================================
// F2XM1 - Precision Tests
// ============================================================================

#[test]
fn test_f2xm1_various_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_values = vec![
        -0.9, -0.75, -0.5, -0.25, -0.1,
        0.1, 0.25, 0.5, 0.75, 0.9,
    ];

    for val in test_values {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, val);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        let expected = 2.0f64.powf(val) - 1.0;
        assert!((result - expected).abs() < 1e-10, "2^{} - 1 should match", val);
    }
}

#[test]
fn test_f2xm1_symmetric_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_pairs = vec![
        (0.5, -0.5),
        (0.25, -0.25),
        (0.75, -0.75),
    ];

    for (pos, neg) in test_pairs {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, pos);
    emu.run(None).unwrap();
        let result_pos = emu.maps.read_f64(0x3000).unwrap();

        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, neg);
    emu.run(None).unwrap();
        let result_neg = emu.maps.read_f64(0x3000).unwrap();

        let expected_pos = 2.0f64.powf(pos) - 1.0;
        let expected_neg = 2.0f64.powf(neg) - 1.0;

        assert!((result_pos - expected_pos).abs() < 1e-10, "2^{} - 1", pos);
        assert!((result_neg - expected_neg).abs() < 1e-10, "2^{} - 1", neg);
    }
}

// ============================================================================
// F2XM1 - Multiple Operations
// ============================================================================

#[test]
fn test_f2xm1_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.5);
    emu.maps.write_f64(0x2008, -0.5);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    let expected1 = 2.0f64.powf(0.5) - 1.0;
    let expected2 = 2.0f64.powf(-0.5) - 1.0;

    assert!((result1 - expected1).abs() < 1e-10, "First F2XM1");
    assert!((result2 - expected2).abs() < 1e-10, "Second F2XM1");
}

// ============================================================================
// F2XM1 - Edge Cases
// ============================================================================

#[test]
fn test_f2xm1_tiny_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.001);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0f64.powf(0.001) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.001 - 1 calculation");
}

#[test]
fn test_f2xm1_precision_boundary() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_values = vec![0.125, 0.375, 0.625, 0.875];

    for val in test_values {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, val);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        let expected = 2.0f64.powf(val) - 1.0;
        assert!((result - expected).abs() < 1e-10, "2^{} - 1", val);
    }
}

#[test]
fn test_f2xm1_fractional_precision() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF0,                                  // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_values = vec![0.2, 0.3, 0.4, 0.6, 0.7, 0.8];

    for val in test_values {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, val);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        let expected = 2.0f64.powf(val) - 1.0;
        assert!((result - expected).abs() < 1e-10, "2^{} - 1 should match", val);
    }
}
