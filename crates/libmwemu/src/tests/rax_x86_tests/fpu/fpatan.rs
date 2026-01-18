//! Tests for the FPATAN instruction.
//!
//! FPATAN - Partial Arctangent
//!
//! Computes the arctangent of the source operand in register ST(1) divided by the
//! source operand in register ST(0), stores the result in ST(1), and pops the FPU
//! register stack. The result in register ST(0) has the same sign as the source
//! operand ST(1) and a magnitude less than +π.
//!
//! The FPATAN instruction returns the angle between the X axis and the line from
//! the origin to the point (X,Y), where Y (the ordinate) is ST(1) and X (the abscissa)
//! is ST(0). The angle depends on the sign of X and Y independently, not just on the
//! sign of the ratio Y/X.
//!
//! Opcode: D9 F3
//!
//! Operation: ST(1) := arctan(ST(1) / ST(0)); PopRegisterStack;
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; Set if result was rounded up
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fpatan.txt

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
// FPATAN Tests: First Quadrant (positive X, positive Y)
// ============================================================================

#[test]
fn test_fpatan_first_quadrant_45deg() {
    let mut emu = emu64();    // FLD qword [0x2000]  ; Load X (ST(0))
    // FLD qword [0x2008]  ; Load Y (ST(0), X becomes ST(1))
    // FPATAN              ; ST(1) = arctan(ST(1)/ST(0)), pop
    // FSTP qword [0x3000] ; Store result
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF3,                                  // FPATAN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);  // X = 1.0
    emu.maps.write_f64(0x2008, 1.0);  // Y = 1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::FRAC_PI_4;
    assert!((result - expected).abs() < 1e-15,
        "arctan(1/1) should be π/4, got {} expected {}", result, expected);
}

#[test]
fn test_fpatan_first_quadrant_30deg() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF3,                                  // FPATAN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0_f64.sqrt());  // X = sqrt(3)
    emu.maps.write_f64(0x2008, 1.0);             // Y = 1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::FRAC_PI_6;
    assert!((result - expected).abs() < 1e-14,
        "arctan(1/sqrt(3)) should be π/6");
}

#[test]
fn test_fpatan_first_quadrant_60deg() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);             // X = 1.0
    emu.maps.write_f64(0x2008, 3.0_f64.sqrt());  // Y = sqrt(3)

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::FRAC_PI_3;
    assert!((result - expected).abs() < 1e-14,
        "arctan(sqrt(3)/1) should be π/3");
}

#[test]
fn test_fpatan_first_quadrant_small_angle() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);   // X = 1.0
    emu.maps.write_f64(0x2008, 0.1);   // Y = 0.1

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (0.1_f64).atan();
    assert!((result - expected).abs() < 1e-15,
        "arctan(0.1/1.0) should match Rust atan");
}

#[test]
fn test_fpatan_first_quadrant_large_ratio() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);   // X = 1.0
    emu.maps.write_f64(0x2008, 10.0);  // Y = 10.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (10.0_f64).atan();
    assert!((result - expected).abs() < 1e-14,
        "arctan(10.0/1.0) should match Rust atan");
}

// ============================================================================
// FPATAN Tests: Second Quadrant (negative X, positive Y)
// ============================================================================

#[test]
fn test_fpatan_second_quadrant_135deg() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);  // X = -1.0
    emu.maps.write_f64(0x2008, 1.0);   // Y = 1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 3.0 * std::f64::consts::FRAC_PI_4;
    assert!((result - expected).abs() < 1e-15,
        "arctan(1/-1) should be 3π/4");
}

#[test]
fn test_fpatan_second_quadrant_near_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);  // X = -1.0
    emu.maps.write_f64(0x2008, 0.1);   // Y = 0.1

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (0.1_f64).atan2(-1.0);
    assert!((result - expected).abs() < 1e-15,
        "arctan(0.1/-1.0) should match atan2");
}

#[test]
fn test_fpatan_second_quadrant_large_y() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);  // X = -1.0
    emu.maps.write_f64(0x2008, 10.0);  // Y = 10.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (10.0_f64).atan2(-1.0);
    assert!((result - expected).abs() < 1e-14,
        "arctan(10.0/-1.0) should match atan2");
}

// ============================================================================
// FPATAN Tests: Third Quadrant (negative X, negative Y)
// ============================================================================

#[test]
fn test_fpatan_third_quadrant_225deg() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);  // X = -1.0
    emu.maps.write_f64(0x2008, -1.0);  // Y = -1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -3.0 * std::f64::consts::FRAC_PI_4;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-1/-1) should be -3π/4");
}

#[test]
fn test_fpatan_third_quadrant_near_minus_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);  // X = -1.0
    emu.maps.write_f64(0x2008, -0.1);  // Y = -0.1

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (-0.1_f64).atan2(-1.0);
    assert!((result - expected).abs() < 1e-15,
        "arctan(-0.1/-1.0) should match atan2");
}

#[test]
fn test_fpatan_third_quadrant_large_y() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);   // X = -1.0
    emu.maps.write_f64(0x2008, -10.0);  // Y = -10.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (-10.0_f64).atan2(-1.0);
    assert!((result - expected).abs() < 1e-14,
        "arctan(-10.0/-1.0) should match atan2");
}

// ============================================================================
// FPATAN Tests: Fourth Quadrant (positive X, negative Y)
// ============================================================================

#[test]
fn test_fpatan_fourth_quadrant_minus_45deg() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);   // X = 1.0
    emu.maps.write_f64(0x2008, -1.0);  // Y = -1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -std::f64::consts::FRAC_PI_4;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-1/1) should be -π/4");
}

#[test]
fn test_fpatan_fourth_quadrant_small_angle() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);   // X = 1.0
    emu.maps.write_f64(0x2008, -0.1);  // Y = -0.1

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (-0.1_f64).atan();
    assert!((result - expected).abs() < 1e-15,
        "arctan(-0.1/1.0) should match Rust atan");
}

#[test]
fn test_fpatan_fourth_quadrant_large_y() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);    // X = 1.0
    emu.maps.write_f64(0x2008, -10.0);  // Y = -10.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (-10.0_f64).atan();
    assert!((result - expected).abs() < 1e-14,
        "arctan(-10.0/1.0) should match Rust atan");
}

// ============================================================================
// FPATAN Tests: Special Cases with Zeros
// ============================================================================

#[test]
fn test_fpatan_positive_zero_positive_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);  // X = 1.0
    emu.maps.write_f64(0x2008, 0.0);  // Y = +0.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.abs() < 1e-15 && !result.is_sign_negative(),
        "arctan(+0/+X) should be +0");
}

#[test]
fn test_fpatan_negative_zero_positive_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);   // X = 1.0
    emu.maps.write_f64(0x2008, -0.0);  // Y = -0.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.abs() < 1e-15 && result.is_sign_negative(),
        "arctan(-0/+X) should be -0");
}

#[test]
fn test_fpatan_positive_zero_negative_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);  // X = -1.0
    emu.maps.write_f64(0x2008, 0.0);   // Y = +0.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::PI;
    assert!((result - expected).abs() < 1e-15,
        "arctan(+0/-X) should be +π");
}

#[test]
fn test_fpatan_negative_zero_negative_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);  // X = -1.0
    emu.maps.write_f64(0x2008, -0.0);  // Y = -0.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -std::f64::consts::PI;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-0/-X) should be -π");
}

#[test]
fn test_fpatan_positive_y_positive_zero_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);  // X = +0.0
    emu.maps.write_f64(0x2008, 1.0);  // Y = 1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::FRAC_PI_2;
    assert!((result - expected).abs() < 1e-15,
        "arctan(+Y/+0) should be +π/2");
}

#[test]
fn test_fpatan_negative_y_positive_zero_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);   // X = +0.0
    emu.maps.write_f64(0x2008, -1.0);  // Y = -1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -std::f64::consts::FRAC_PI_2;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-Y/+0) should be -π/2");
}

#[test]
fn test_fpatan_positive_y_negative_zero_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);  // X = -0.0
    emu.maps.write_f64(0x2008, 1.0);   // Y = 1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::FRAC_PI_2;
    assert!((result - expected).abs() < 1e-15,
        "arctan(+Y/-0) should be +π/2");
}

#[test]
fn test_fpatan_negative_y_negative_zero_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);  // X = -0.0
    emu.maps.write_f64(0x2008, -1.0);  // Y = -1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -std::f64::consts::FRAC_PI_2;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-Y/-0) should be -π/2");
}

// ============================================================================
// FPATAN Tests: Special Cases with Infinities
// ============================================================================

#[test]
fn test_fpatan_positive_infinity_positive_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);                    // X = 1.0
    emu.maps.write_f64(0x2008, f64::INFINITY);          // Y = +∞

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::FRAC_PI_2;
    assert!((result - expected).abs() < 1e-15,
        "arctan(+∞/+X) should be +π/2");
}

#[test]
fn test_fpatan_negative_infinity_positive_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);                    // X = 1.0
    emu.maps.write_f64(0x2008, f64::NEG_INFINITY);      // Y = -∞

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -std::f64::consts::FRAC_PI_2;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-∞/+X) should be -π/2");
}

#[test]
fn test_fpatan_positive_infinity_negative_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);                   // X = -1.0
    emu.maps.write_f64(0x2008, f64::INFINITY);          // Y = +∞

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::FRAC_PI_2;
    assert!((result - expected).abs() < 1e-15,
        "arctan(+∞/-X) should be +π/2");
}

#[test]
fn test_fpatan_negative_infinity_negative_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);                   // X = -1.0
    emu.maps.write_f64(0x2008, f64::NEG_INFINITY);      // Y = -∞

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -std::f64::consts::FRAC_PI_2;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-∞/-X) should be -π/2");
}

#[test]
fn test_fpatan_positive_y_positive_infinity_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);          // X = +∞
    emu.maps.write_f64(0x2008, 1.0);                    // Y = 1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.abs() < 1e-15 && !result.is_sign_negative(),
        "arctan(+Y/+∞) should be +0");
}

#[test]
fn test_fpatan_negative_y_positive_infinity_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);          // X = +∞
    emu.maps.write_f64(0x2008, -1.0);                   // Y = -1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.abs() < 1e-15 && result.is_sign_negative(),
        "arctan(-Y/+∞) should be -0");
}

#[test]
fn test_fpatan_positive_y_negative_infinity_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NEG_INFINITY);      // X = -∞
    emu.maps.write_f64(0x2008, 1.0);                    // Y = 1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::PI;
    assert!((result - expected).abs() < 1e-15,
        "arctan(+Y/-∞) should be +π");
}

#[test]
fn test_fpatan_negative_y_negative_infinity_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NEG_INFINITY);      // X = -∞
    emu.maps.write_f64(0x2008, -1.0);                   // Y = -1.0

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -std::f64::consts::PI;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-Y/-∞) should be -π");
}

#[test]
fn test_fpatan_positive_infinity_positive_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);          // X = +∞
    emu.maps.write_f64(0x2008, f64::INFINITY);          // Y = +∞

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::FRAC_PI_4;
    assert!((result - expected).abs() < 1e-15,
        "arctan(+∞/+∞) should be +π/4");
}

#[test]
fn test_fpatan_positive_infinity_negative_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NEG_INFINITY);      // X = -∞
    emu.maps.write_f64(0x2008, f64::INFINITY);          // Y = +∞

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 3.0 * std::f64::consts::FRAC_PI_4;
    assert!((result - expected).abs() < 1e-15,
        "arctan(+∞/-∞) should be +3π/4");
}

#[test]
fn test_fpatan_negative_infinity_positive_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);          // X = +∞
    emu.maps.write_f64(0x2008, f64::NEG_INFINITY);      // Y = -∞

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -std::f64::consts::FRAC_PI_4;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-∞/+∞) should be -π/4");
}

#[test]
fn test_fpatan_negative_infinity_negative_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NEG_INFINITY);      // X = -∞
    emu.maps.write_f64(0x2008, f64::NEG_INFINITY);      // Y = -∞

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -3.0 * std::f64::consts::FRAC_PI_4;
    assert!((result - expected).abs() < 1e-15,
        "arctan(-∞/-∞) should be -3π/4");
}

// ============================================================================
// FPATAN Tests: Various Value Combinations
// ============================================================================

#[test]
fn test_fpatan_large_x_small_y() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1000.0);  // X = 1000.0
    emu.maps.write_f64(0x2008, 0.001);   // Y = 0.001

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (0.001_f64).atan2(1000.0);
    assert!((result - expected).abs() < 1e-15,
        "arctan(0.001/1000.0) should match atan2");
}

#[test]
fn test_fpatan_fractional_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF3,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.25);  // X = 0.25
    emu.maps.write_f64(0x2008, 0.5);   // Y = 0.5

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0_f64.atan();
    assert!((result - expected).abs() < 1e-15,
        "arctan(0.5/0.25) should equal arctan(2)");
}
