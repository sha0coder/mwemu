//! Tests for the FSQRT instruction.
//!
//! FSQRT - Square Root
//!
//! Computes the square root of the source value in the ST(0) register and
//! stores the result in ST(0).
//! The operation is ST(0) := SquareRoot(ST(0))
//!
//! Opcode: D9 FA
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; Set if result was rounded up
//! - C0, C2, C3: Undefined
//!
//! Exceptions:
//! - #IA: Source operand is a negative value (except -0), SNaN, or unsupported format
//! - #D: Source operand is a denormal value
//! - #P: Value cannot be represented exactly in destination format
//!
//! Reference: /Users/int/dev/rax/docs/fsqrt.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Helper function to write f64 to memory
fn write_f64(mem: u64, addr: u64, val: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// Helper function to read f64 from memory
fn read_f64(mem: u64, addr: u64) -> f64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FSQRT - Perfect Squares
// ============================================================================

#[test]
fn test_fsqrt_perfect_square_4() {
    let mut emu = emu64();    // FLD qword [0x2000]  ; DD 04 25 00 20 00 00
    // FSQRT               ; D9 FA
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 4.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "FSQRT of 4.0 should be 2.0");
}

#[test]
fn test_fsqrt_perfect_square_9() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 9.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 3.0, "FSQRT of 9.0 should be 3.0");
}

#[test]
fn test_fsqrt_perfect_square_16() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 16.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 4.0, "FSQRT of 16.0 should be 4.0");
}

#[test]
fn test_fsqrt_perfect_square_25() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 25.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 5.0, "FSQRT of 25.0 should be 5.0");
}

#[test]
fn test_fsqrt_perfect_square_100() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 10.0, "FSQRT of 100.0 should be 10.0");
}

#[test]
fn test_fsqrt_perfect_square_144() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 144.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 12.0, "FSQRT of 144.0 should be 12.0");
}

// ============================================================================
// FSQRT - Non-Perfect Squares
// ============================================================================

#[test]
fn test_fsqrt_two() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0_f64.sqrt();
    assert!((result - expected).abs() < 1e-15, "FSQRT of 2.0 should be approximately {}", expected);
}

#[test]
fn test_fsqrt_three() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 3.0_f64.sqrt();
    assert!((result - expected).abs() < 1e-15, "FSQRT of 3.0 should be approximately {}", expected);
}

#[test]
fn test_fsqrt_five() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 5.0_f64.sqrt();
    assert!((result - expected).abs() < 1e-15, "FSQRT of 5.0 should be approximately {}", expected);
}

#[test]
fn test_fsqrt_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::PI);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::PI.sqrt();
    assert!((result - expected).abs() < 1e-15, "FSQRT of PI should be approximately {}", expected);
}

#[test]
fn test_fsqrt_e() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::E);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::E.sqrt();
    assert!((result - expected).abs() < 1e-15, "FSQRT of E should be approximately {}", expected);
}

// ============================================================================
// FSQRT - Special Cases: Zero
// ============================================================================

#[test]
fn test_fsqrt_positive_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "FSQRT of +0.0 should be +0.0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

#[test]
fn test_fsqrt_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, -0.0, "FSQRT of -0.0 should be -0.0");
    assert!(result.is_sign_negative(), "Result should be negative zero");
}

// ============================================================================
// FSQRT - Special Cases: Infinity
// ============================================================================

#[test]
fn test_fsqrt_positive_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, f64::INFINITY, "FSQRT of +infinity should be +infinity");
}

#[test]
fn test_fsqrt_negative_infinity() {
    let mut emu = emu64();    // FSQRT of negative infinity should produce NaN (invalid operation)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NEG_INFINITY);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_nan(), "FSQRT of -infinity should produce NaN");
}

// ============================================================================
// FSQRT - Special Cases: NaN
// ============================================================================

#[test]
fn test_fsqrt_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_nan(), "FSQRT of NaN should remain NaN");
}

// ============================================================================
// FSQRT - Negative Numbers (Invalid Operation)
// ============================================================================

#[test]
fn test_fsqrt_negative_one() {
    let mut emu = emu64();    // FSQRT of negative number (except -0) should produce NaN
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_nan(), "FSQRT of -1.0 should produce NaN (invalid operation)");
}

#[test]
fn test_fsqrt_negative_small() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_nan(), "FSQRT of -0.5 should produce NaN");
}

#[test]
fn test_fsqrt_negative_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -100.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_nan(), "FSQRT of -100.0 should produce NaN");
}

// ============================================================================
// FSQRT - Fractional Values
// ============================================================================

#[test]
fn test_fsqrt_one_quarter() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.25);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.5, "FSQRT of 0.25 should be 0.5");
}

#[test]
fn test_fsqrt_one_sixteenth() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0625);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.25, "FSQRT of 0.0625 should be 0.25");
}

#[test]
fn test_fsqrt_decimal_fraction() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.01);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.1, "FSQRT of 0.01 should be 0.1");
}

// ============================================================================
// FSQRT - Large Values
// ============================================================================

#[test]
fn test_fsqrt_large_perfect_square() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10000.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 100.0, "FSQRT of 10000.0 should be 100.0");
}

#[test]
fn test_fsqrt_very_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e100);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1e50;
    assert_eq!(result, expected, "FSQRT of 1e100 should be 1e50");
}

// ============================================================================
// FSQRT - Small Values
// ============================================================================

#[test]
fn test_fsqrt_very_small() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e-100);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1e-50;
    assert!((result - expected).abs() < 1e-60, "FSQRT of 1e-100 should be approximately 1e-50");
}

#[test]
fn test_fsqrt_min_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::MIN_POSITIVE);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result > 0.0, "FSQRT of MIN_POSITIVE should be positive");
    assert!(result.is_finite(), "Result should be finite");
}

// ============================================================================
// FSQRT - Precision Tests
// ============================================================================

#[test]
fn test_fsqrt_precision_1() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "FSQRT of 1.0 should be exactly 1.0");
}

#[test]
fn test_fsqrt_precision_check() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_values = vec![2.0, 3.0, 5.0, 7.0, 10.0, 50.0, 123.456];

    for val in test_values {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, val);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        let expected = val.sqrt();
        let rel_error = ((result - expected) / expected).abs();
        assert!(rel_error < 1e-15, "FSQRT of {} has relative error {} (expected < 1e-15)", val, rel_error);
    }
}

// ============================================================================
// FSQRT - Idempotency and Combinations
// ============================================================================

#[test]
fn test_fsqrt_twice() {
    let mut emu = emu64();    // FSQRT(FSQRT(16)) should be 2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xD9, 0xFA,                                  // FSQRT (second time)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 16.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "FSQRT(FSQRT(16)) should be 2");
}

#[test]
fn test_fsqrt_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 64.0);
    emu.maps.write_f64(0x2008, 81.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 8.0, "FSQRT(64) should be 8");
    assert_eq!(result2, 9.0, "FSQRT(81) should be 9");
}

#[test]
fn test_fsqrt_with_multiplication() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC9,                                  // FMULP (multiply and pop)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 4.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 6.0, "FSQRT(4) * 3 should be 6");
}

// ============================================================================
// FSQRT - Additional Edge Cases
// ============================================================================

#[test]
fn test_fsqrt_max_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::MAX);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_finite(), "FSQRT of MAX should be finite");
    assert!(result > 0.0, "FSQRT of MAX should be positive");
}

#[test]
fn test_fsqrt_inverse_property() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xD8, 0xC8,                                  // FMUL ST(0), ST(0) (square the result)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let test_val = 42.0;
    emu.maps.write_f64(0x2000, test_val);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - test_val).abs() < 1e-14, "FSQRT(x)^2 should equal x");
}

#[test]
fn test_fsqrt_power_of_two() {
    let mut emu = emu64();    // FSQRT(256) = 16
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 256.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 16.0, "FSQRT(256) should be 16");
}

#[test]
fn test_fsqrt_nested() {
    let mut emu = emu64();    // FSQRT(FSQRT(256)) should be 4
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xD9, 0xFA,                                  // FSQRT (second)
        0xD9, 0xFA,                                  // FSQRT (third)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 256.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "FSQRT(FSQRT(FSQRT(256))) should be 2");
}
