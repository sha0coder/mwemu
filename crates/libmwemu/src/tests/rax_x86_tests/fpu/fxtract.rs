//! Tests for the FXTRACT instruction.
//!
//! FXTRACT - Extract Exponent and Significand
//!
//! Separates the source value in the ST(0) register into its exponent and significand,
//! stores the exponent in ST(0), and pushes the significand onto the register stack.
//! Following this operation, the new top-of-stack register ST(0) contains the value
//! of the original significand expressed as a floating-point value. The sign and
//! significand of this value are the same as those found in the source operand, and
//! the exponent is 3FFFH (biased value for a true exponent of zero). The ST(1) register
//! contains the value of the original operand's true (unbiased) exponent expressed as
//! a floating-point value.
//!
//! Opcode: D9 F4
//!
//! Operation:
//! TEMP := Significand(ST(0));
//! ST(0) := Exponent(ST(0));
//! TOP := TOP - 1;
//! ST(0) := TEMP;
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; set to 1 if stack overflow occurred
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fxtract.txt

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
// FXTRACT Tests: Powers of Two
// ============================================================================

#[test]
fn test_fxtract_one() {
    let mut emu = emu64();    // 1.0 has exponent 0 and significand 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF4,                                  // FXTRACT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] (significand)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] (exponent)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.0).abs() < 1e-15, "Significand of 1.0 should be 1.0");
    assert!((exponent - 0.0).abs() < 1e-15, "Exponent of 1.0 should be 0.0");
}

#[test]
fn test_fxtract_two() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.0).abs() < 1e-15, "Significand of 2.0 should be 1.0");
    assert!((exponent - 1.0).abs() < 1e-15, "Exponent of 2.0 should be 1.0");
}

#[test]
fn test_fxtract_four() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 4.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.0).abs() < 1e-15, "Significand of 4.0 should be 1.0");
    assert!((exponent - 2.0).abs() < 1e-15, "Exponent of 4.0 should be 2.0");
}

#[test]
fn test_fxtract_eight() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 8.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.0).abs() < 1e-15, "Significand of 8.0 should be 1.0");
    assert!((exponent - 3.0).abs() < 1e-15, "Exponent of 8.0 should be 3.0");
}

#[test]
fn test_fxtract_large_power_of_two() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1024.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.0).abs() < 1e-15, "Significand of 1024.0 should be 1.0");
    assert!((exponent - 10.0).abs() < 1e-15, "Exponent of 1024.0 should be 10.0");
}

// ============================================================================
// FXTRACT Tests: Fractional Powers of Two
// ============================================================================

#[test]
fn test_fxtract_half() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.5);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.0).abs() < 1e-15, "Significand of 0.5 should be 1.0");
    assert!((exponent - (-1.0)).abs() < 1e-15, "Exponent of 0.5 should be -1.0");
}

#[test]
fn test_fxtract_quarter() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.25);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.0).abs() < 1e-15, "Significand of 0.25 should be 1.0");
    assert!((exponent - (-2.0)).abs() < 1e-15, "Exponent of 0.25 should be -2.0");
}

#[test]
fn test_fxtract_small_power_of_two() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0 / 1024.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.0).abs() < 1e-15, "Significand of 2^-10 should be 1.0");
    assert!((exponent - (-10.0)).abs() < 1e-15, "Exponent of 2^-10 should be -10.0");
}

// ============================================================================
// FXTRACT Tests: Non-Power-of-Two Values
// ============================================================================

#[test]
fn test_fxtract_three() {
    let mut emu = emu64();    // 3.0 = 1.5 * 2^1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.5).abs() < 1e-15, "Significand of 3.0 should be 1.5");
    assert!((exponent - 1.0).abs() < 1e-15, "Exponent of 3.0 should be 1.0");
}

#[test]
fn test_fxtract_five() {
    let mut emu = emu64();    // 5.0 = 1.25 * 2^2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.25).abs() < 1e-15, "Significand of 5.0 should be 1.25");
    assert!((exponent - 2.0).abs() < 1e-15, "Exponent of 5.0 should be 2.0");
}

#[test]
fn test_fxtract_six() {
    let mut emu = emu64();    // 6.0 = 1.5 * 2^2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 6.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.5).abs() < 1e-15, "Significand of 6.0 should be 1.5");
    assert!((exponent - 2.0).abs() < 1e-15, "Exponent of 6.0 should be 2.0");
}

#[test]
fn test_fxtract_ten() {
    let mut emu = emu64();    // 10.0 = 1.25 * 2^3
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - 1.25).abs() < 1e-15, "Significand of 10.0 should be 1.25");
    assert!((exponent - 3.0).abs() < 1e-15, "Exponent of 10.0 should be 3.0");
}

#[test]
fn test_fxtract_pi() {
    let mut emu = emu64();    // π ≈ 3.14159... = 1.5708... * 2^1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::PI);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    // π / 2 ≈ 1.5707963...
    assert!((significand - std::f64::consts::PI / 2.0).abs() < 1e-15,
        "Significand of π should be π/2");
    assert!((exponent - 1.0).abs() < 1e-15, "Exponent of π should be 1.0");
}

// ============================================================================
// FXTRACT Tests: Negative Values
// ============================================================================

#[test]
fn test_fxtract_negative_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - (-1.0)).abs() < 1e-15, "Significand of -1.0 should be -1.0");
    assert!((exponent - 0.0).abs() < 1e-15, "Exponent of -1.0 should be 0.0");
}

#[test]
fn test_fxtract_negative_two() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -2.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - (-1.0)).abs() < 1e-15, "Significand of -2.0 should be -1.0");
    assert!((exponent - 1.0).abs() < 1e-15, "Exponent of -2.0 should be 1.0");
}

#[test]
fn test_fxtract_negative_half() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.5);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - (-1.0)).abs() < 1e-15, "Significand of -0.5 should be -1.0");
    assert!((exponent - (-1.0)).abs() < 1e-15, "Exponent of -0.5 should be -1.0");
}

#[test]
fn test_fxtract_negative_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -std::f64::consts::PI);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!((significand - (-std::f64::consts::PI / 2.0)).abs() < 1e-15,
        "Significand of -π should be -π/2");
    assert!((exponent - 1.0).abs() < 1e-15, "Exponent of -π should be 1.0");
}

// ============================================================================
// FXTRACT Tests: Special Cases
// ============================================================================

#[test]
fn test_fxtract_positive_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!(significand == 0.0 && !significand.is_sign_negative(),
        "Significand of +0 should be +0");
    assert!(exponent.is_infinite() && exponent.is_sign_negative(),
        "Exponent of +0 should be -∞");
}

#[test]
fn test_fxtract_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!(significand == 0.0 && significand.is_sign_negative(),
        "Significand of -0 should be -0");
    assert!(exponent.is_infinite() && exponent.is_sign_negative(),
        "Exponent of -0 should be -∞");
}

// ============================================================================
// FXTRACT Tests: Large Values
// ============================================================================

#[test]
fn test_fxtract_large_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0e100);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!(significand >= 1.0 && significand < 2.0,
        "Significand should be in [1.0, 2.0)");
    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!((reconstructed - 1.0e100).abs() / 1.0e100 < 1e-15,
        "Reconstruction should match original value");
}

#[test]
fn test_fxtract_small_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0e-100);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    assert!(significand >= 1.0 && significand < 2.0,
        "Significand should be in [1.0, 2.0)");
    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!((reconstructed - 1.0e-100).abs() / 1.0e-100 < 1e-15,
        "Reconstruction should match original value");
}

// ============================================================================
// FXTRACT Tests: Reconstruction (FXTRACT + FSCALE reversal)
// ============================================================================

#[test]
fn test_fxtract_reconstruction_with_fscale() {
    let mut emu = emu64();    // FXTRACT; FSCALE; FSTP ST(1);
    let code = [
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
fn test_fxtract_reconstruction_manually() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    let original = 7.5;
    emu.maps.write_f64(0x2000, original);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!((reconstructed - original).abs() < 1e-15,
        "Manual reconstruction should match original: {} * 2^{} = {}",
        significand, exponent, reconstructed);
}

// ============================================================================
// FXTRACT Tests: Various Values
// ============================================================================

#[test]
fn test_fxtract_hundred() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    // 100 = 1.5625 * 2^6
    assert!((significand - 1.5625).abs() < 1e-15, "Significand of 100.0");
    assert!((exponent - 6.0).abs() < 1e-15, "Exponent of 100.0");
}

#[test]
fn test_fxtract_point_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!((reconstructed - 0.1).abs() < 1e-16,
        "Reconstruction of 0.1 should match");
}

#[test]
fn test_fxtract_e() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::E);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    // e ≈ 2.718... = 1.359... * 2^1
    assert!((exponent - 1.0).abs() < 1e-15, "Exponent of e should be 1");
    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!((reconstructed - std::f64::consts::E).abs() < 1e-15,
        "Reconstruction of e should match");
}

#[test]
fn test_fxtract_sqrt_two() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF4,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::SQRT_2);

    emu.run(None).unwrap();

    let significand = emu.maps.read_f64(0x3000).unwrap();
    let exponent = emu.maps.read_f64(0x3008).unwrap();

    // √2 ≈ 1.414... = 1.414... * 2^0
    assert!((exponent - 0.0).abs() < 1e-15, "Exponent of √2 should be 0");
    assert!((significand - std::f64::consts::SQRT_2).abs() < 1e-15,
        "Significand of √2 should be √2 itself");
}

#[test]
fn test_fxtract_range_verification() {
    let mut emu = emu64();    let test_values = [1.5, 3.7, 9.9, 15.3, 27.8, 50.5, 99.9, 127.5];

    for &value in &test_values {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xD9, 0xF4,
            0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
            0xF4,
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, value);

    emu.run(None).unwrap();

        let significand = emu.maps.read_f64(0x3000).unwrap();
        let exponent = emu.maps.read_f64(0x3008).unwrap();

        assert!(significand >= 1.0 && significand < 2.0,
            "Significand of {} should be in [1.0, 2.0), got {}", value, significand);

        let reconstructed = significand * 2.0_f64.powf(exponent);
        assert!((reconstructed - value).abs() < 1e-14,
            "Reconstruction of {} failed", value);
    }
}
