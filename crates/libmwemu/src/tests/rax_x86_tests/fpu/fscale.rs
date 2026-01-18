//! Tests for the FSCALE instruction.
//!
//! FSCALE - Scale
//!
//! Truncates the value in the source operand (toward 0) to an integral value and
//! adds that value to the exponent of the destination operand. The destination and
//! source operands are floating-point values located in registers ST(0) and ST(1),
//! respectively. This instruction provides rapid multiplication or division by integral
//! powers of 2.
//!
//! In most cases, only the exponent is changed and the mantissa (significand) remains
//! unchanged. However, when the value being scaled in ST(0) is a denormal value, the
//! mantissa is also changed and the result may turn out to be a normalized number.
//! Similarly, if overflow or underflow results from a scale operation, the resulting
//! mantissa will differ from the source's mantissa.
//!
//! Opcode: D9 FD
//!
//! Operation: ST(0) := ST(0) * 2^RoundTowardZero(ST(1))
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; Set if result was rounded up
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fscale.txt

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
// FSCALE Tests: Scaling by Positive Integer Powers
// ============================================================================

#[test]
fn test_fscale_multiply_by_2() {
    let mut emu = emu64();    // FLD qword [0x2000]  ; Load value to scale (ST(0))
    // FLD qword [0x2008]  ; Load scale factor (ST(0), value becomes ST(1))
    // FSCALE              ; ST(0) = ST(1) * 2^trunc(ST(0))
    // FSTP qword [0x3000] ; Store result
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFD,                                  // FSCALE
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0) (clean stack)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);   // Value to scale
    emu.maps.write_f64(0x2008, 1.0);   // Scale factor

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 10.0).abs() < 1e-15, "5.0 * 2^1 should be 10.0");
}

#[test]
fn test_fscale_multiply_by_4() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 12.0).abs() < 1e-15, "3.0 * 2^2 should be 12.0");
}

#[test]
fn test_fscale_multiply_by_8() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.5);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 60.0).abs() < 1e-14, "7.5 * 2^3 should be 60.0");
}

#[test]
fn test_fscale_multiply_by_1024() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 1024.0).abs() < 1e-13, "1.0 * 2^10 should be 1024.0");
}

// ============================================================================
// FSCALE Tests: Scaling by Negative Integer Powers (Division)
// ============================================================================

#[test]
fn test_fscale_divide_by_2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, -1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 5.0).abs() < 1e-15, "10.0 * 2^-1 should be 5.0");
}

#[test]
fn test_fscale_divide_by_4() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 20.0);
    emu.maps.write_f64(0x2008, -2.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 5.0).abs() < 1e-15, "20.0 * 2^-2 should be 5.0");
}

#[test]
fn test_fscale_divide_by_8() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 64.0);
    emu.maps.write_f64(0x2008, -3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 8.0).abs() < 1e-15, "64.0 * 2^-3 should be 8.0");
}

#[test]
fn test_fscale_divide_by_1024() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2048.0);
    emu.maps.write_f64(0x2008, -10.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 2.0).abs() < 1e-14, "2048.0 * 2^-10 should be 2.0");
}

// ============================================================================
// FSCALE Tests: Scale Factor Zero
// ============================================================================

#[test]
fn test_fscale_zero_scale_factor() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 42.0);
    emu.maps.write_f64(0x2008, 0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 42.0).abs() < 1e-15, "42.0 * 2^0 should be 42.0");
}

#[test]
fn test_fscale_positive_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 99.5);
    emu.maps.write_f64(0x2008, 0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 99.5).abs() < 1e-14, "Value should remain unchanged");
}

#[test]
fn test_fscale_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 77.7);
    emu.maps.write_f64(0x2008, -0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 77.7).abs() < 1e-14, "Value should remain unchanged");
}

// ============================================================================
// FSCALE Tests: Fractional Scale Factors (Truncation)
// ============================================================================

#[test]
fn test_fscale_truncate_positive_fraction() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.9);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    // 2.9 truncates to 2, so 1.0 * 2^2 = 4.0
    assert!((result - 4.0).abs() < 1e-15,
        "Scale factor 2.9 should truncate to 2");
}

#[test]
fn test_fscale_truncate_negative_fraction() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 16.0);
    emu.maps.write_f64(0x2008, -2.9);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    // -2.9 truncates to -2, so 16.0 * 2^-2 = 4.0
    assert!((result - 4.0).abs() < 1e-15,
        "Scale factor -2.9 should truncate to -2");
}

#[test]
fn test_fscale_truncate_small_positive_fraction() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 0.9);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    // 0.9 truncates to 0, so 5.0 * 2^0 = 5.0
    assert!((result - 5.0).abs() < 1e-15,
        "Scale factor 0.9 should truncate to 0");
}

#[test]
fn test_fscale_truncate_small_negative_fraction() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);
    emu.maps.write_f64(0x2008, -0.9);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    // -0.9 truncates to 0, so 7.0 * 2^0 = 7.0
    assert!((result - 7.0).abs() < 1e-15,
        "Scale factor -0.9 should truncate to 0");
}

// ============================================================================
// FSCALE Tests: Negative Values
// ============================================================================

#[test]
fn test_fscale_negative_value_positive_scale() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -3.0);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - (-12.0)).abs() < 1e-15, "-3.0 * 2^2 should be -12.0");
}

#[test]
fn test_fscale_negative_value_negative_scale() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -8.0);
    emu.maps.write_f64(0x2008, -2.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - (-2.0)).abs() < 1e-15, "-8.0 * 2^-2 should be -2.0");
}

// ============================================================================
// FSCALE Tests: Special Cases with Zeros
// ============================================================================

#[test]
fn test_fscale_zero_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result == 0.0 && !result.is_sign_negative(),
        "0.0 * 2^5 should be +0.0");
}

#[test]
fn test_fscale_negative_zero_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result == 0.0 && result.is_sign_negative(),
        "-0.0 * 2^5 should be -0.0");
}

// ============================================================================
// FSCALE Tests: FXTRACT Reversal
// ============================================================================

#[test]
fn test_fscale_fxtract_reversal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF4,                                  // FXTRACT (ST(0)=sig, ST(1)=exp)
        0xD9, 0xFD,                                  // FSCALE (ST(0) = sig * 2^exp)
        0xDD, 0xD9,                                  // FSTP ST(1) (pop exponent)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let original = 123.456;
    emu.maps.write_f64(0x2000, original);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - original).abs() < 1e-14,
        "FXTRACT followed by FSCALE should restore original value");
}

#[test]
fn test_fscale_fxtract_reversal_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xD9, 0xFD,
        0xDD, 0xD9,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    let original = -789.123;
    emu.maps.write_f64(0x2000, original);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - original).abs() < 1e-13,
        "FXTRACT followed by FSCALE should restore negative value");
}

#[test]
fn test_fscale_fxtract_reversal_small() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xD9, 0xFD,
        0xDD, 0xD9,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    let original = 0.00123;
    emu.maps.write_f64(0x2000, original);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - original).abs() < 1e-17,
        "FXTRACT followed by FSCALE should restore small value");
}

// ============================================================================
// FSCALE Tests: Large Scale Factors
// ============================================================================

#[test]
fn test_fscale_large_positive_scale() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 100.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0_f64.powf(100.0);
    assert!((result - expected).abs() / expected < 1e-15,
        "1.0 * 2^100 should match");
}

#[test]
fn test_fscale_large_negative_scale() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, -100.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0_f64.powf(-100.0);
    assert!((result - expected).abs() / expected < 1e-15,
        "1.0 * 2^-100 should match");
}

// ============================================================================
// FSCALE Tests: Various Values
// ============================================================================

#[test]
fn test_fscale_pi_scale_by_4() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::PI);
    emu.maps.write_f64(0x2008, 4.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::PI * 16.0;
    assert!((result - expected).abs() < 1e-13,
        "π * 2^4 should be π * 16");
}

#[test]
fn test_fscale_e_scale_by_minus_3() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::E);
    emu.maps.write_f64(0x2008, -3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::E / 8.0;
    assert!((result - expected).abs() < 1e-15,
        "e * 2^-3 should be e / 8");
}

#[test]
fn test_fscale_series() {
    let mut emu = emu64();    let test_cases = [
        (1.0, 0.0, 1.0),
        (1.0, 1.0, 2.0),
        (1.0, 2.0, 4.0),
        (1.0, 3.0, 8.0),
        (2.0, 2.0, 8.0),
        (3.0, 1.0, 6.0),
        (5.0, -1.0, 2.5),
        (100.0, -3.0, 12.5),
    ];

    for &(value, scale, expected) in &test_cases {
        let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFD,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, value);
        emu.maps.write_f64(0x2008, scale);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        assert!((result - expected).abs() < 1e-14,
            "{} * 2^{} should be {}, got {}", value, scale, expected, result);
    }
}
