//! Tests for the FSINCOS instruction.
//!
//! FSINCOS - Simultaneous Sine and Cosine
//!
//! Computes both the approximate sine and cosine of the source operand in register ST(0),
//! stores the sine in ST(0), and pushes the cosine onto the top of the FPU register stack.
//! This instruction is faster than executing FSIN and FCOS in succession.
//! The source operand must be given in radians and must be within the range -2^63 to +2^63.
//!
//! Opcode: D9 FB
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow; set to 1 if stack overflow; set if result rounded up
//! - C2: Set to 1 if outside range (-2^63 < source < +2^63);
 // otherwise 0
//! - C0, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fsincos.txt

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
// FSINCOS - Special Angles
// ============================================================================

#[test]
fn test_fsincos_zero() {
    let mut emu = emu64();    // sin(0) = 0, cos(0) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!((sine - 0.0).abs() < 1e-15, "sin(0) should be 0");
    assert!((cosine - 1.0).abs() < 1e-15, "cos(0) should be 1");
}

#[test]
fn test_fsincos_pi_over_2() {
    let mut emu = emu64();    // sin(π/2) = 1, cos(π/2) = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_2);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!((sine - 1.0).abs() < 1e-15, "sin(π/2) should be 1");
    assert!(cosine.abs() < 1e-15, "cos(π/2) should be 0");
}

#[test]
fn test_fsincos_pi() {
    let mut emu = emu64();    // sin(π) = 0, cos(π) = -1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::PI);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.abs() < 1e-15, "sin(π) should be 0");
    assert!((cosine + 1.0).abs() < 1e-15, "cos(π) should be -1");
}

#[test]
fn test_fsincos_3pi_over_2() {
    let mut emu = emu64();    // sin(3π/2) = -1, cos(3π/2) = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0 * std::f64::consts::FRAC_PI_2);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!((sine + 1.0).abs() < 1e-15, "sin(3π/2) should be -1");
    assert!(cosine.abs() < 1e-15, "cos(3π/2) should be 0");
}

#[test]
fn test_fsincos_2pi() {
    let mut emu = emu64();    // sin(2π) = 0, cos(2π) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0 * std::f64::consts::PI);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.abs() < 1e-15, "sin(2π) should be 0");
    assert!((cosine - 1.0).abs() < 1e-15, "cos(2π) should be 1");
}

#[test]
fn test_fsincos_pi_over_4() {
    let mut emu = emu64();    // sin(π/4) = cos(π/4) = √2/2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_4);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    let expected = (std::f64::consts::FRAC_PI_4).sin();
    assert!((sine - expected).abs() < 1e-15, "sin(π/4) should be √2/2");
    assert!((cosine - expected).abs() < 1e-15, "cos(π/4) should be √2/2");
    assert!((sine - cosine).abs() < 1e-15, "sin(π/4) = cos(π/4)");
}

#[test]
fn test_fsincos_pi_over_6() {
    let mut emu = emu64();    // sin(π/6) = 0.5, cos(π/6) = √3/2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_6);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!((sine - 0.5).abs() < 1e-15, "sin(π/6) should be 0.5");
    let expected_cos = (std::f64::consts::FRAC_PI_6).cos();
    assert!((cosine - expected_cos).abs() < 1e-15, "cos(π/6) should be √3/2");
}

#[test]
fn test_fsincos_pi_over_3() {
    let mut emu = emu64();    // sin(π/3) = √3/2, cos(π/3) = 0.5
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::PI / 3.0);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    let expected_sin = (std::f64::consts::PI / 3.0).sin();
    assert!((sine - expected_sin).abs() < 1e-15, "sin(π/3) should be √3/2");
    assert!((cosine - 0.5).abs() < 1e-15, "cos(π/3) should be 0.5");
}

// ============================================================================
// FSINCOS - Negative Angles
// ============================================================================

#[test]
fn test_fsincos_negative_zero() {
    let mut emu = emu64();    // sin(-0) = -0, cos(-0) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.abs() < 1e-15, "sin(-0) should be 0");
    assert!((cosine - 1.0).abs() < 1e-15, "cos(-0) should be 1");
}

#[test]
fn test_fsincos_negative_pi_over_2() {
    let mut emu = emu64();    // sin(-π/2) = -1, cos(-π/2) = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -std::f64::consts::FRAC_PI_2);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!((sine + 1.0).abs() < 1e-15, "sin(-π/2) should be -1");
    assert!(cosine.abs() < 1e-15, "cos(-π/2) should be 0");
}

#[test]
fn test_fsincos_negative_pi() {
    let mut emu = emu64();    // sin(-π) = 0, cos(-π) = -1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -std::f64::consts::PI);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.abs() < 1e-15, "sin(-π) should be 0");
    assert!((cosine + 1.0).abs() < 1e-15, "cos(-π) should be -1");
}

#[test]
fn test_fsincos_negative_pi_over_4() {
    let mut emu = emu64();    // sin(-π/4) = -√2/2, cos(-π/4) = √2/2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -std::f64::consts::FRAC_PI_4);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    let expected = (-std::f64::consts::FRAC_PI_4).sin();
    assert!((sine - expected).abs() < 1e-15, "sin(-π/4) should be -√2/2");
    assert!((cosine + sine).abs() < 1e-15, "cos(-π/4) should equal -sin(-π/4)");
}

// ============================================================================
// FSINCOS - Trigonometric Identities
// ============================================================================

#[test]
fn test_fsincos_pythagorean_identity() {
    let mut emu = emu64();    // sin²(x) + cos²(x) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xD9, 0xC0,                                  // FLD ST(0) (duplicate cosine)
        0xD8, 0xC8,                                  // FMUL ST(0), ST(0) (cos²)
        0xD9, 0xCA,                                  // FXCH ST(2)
        0xD8, 0xC8,                                  // FMUL ST(0), ST(0) (sin²)
        0xDE, 0xC2,                                  // FADDP (add sin² + cos²)
        0xDD, 0xD8,                                  // FSTP ST(0) (pop extra)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.7);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 1.0).abs() < 1e-14, "sin²(x) + cos²(x) should equal 1");
}

#[test]
fn test_fsincos_odd_even_symmetry() {
    let mut emu = emu64();    // sin(-x) = -sin(x), cos(-x) = cos(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    let angle = 0.5;

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, angle);
    emu.run(None).unwrap();
    let cos_positive = emu.maps.read_f64(0x3000).unwrap();
    let sin_positive = emu.maps.read_f64(0x3008).unwrap();

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -angle);
    emu.run(None).unwrap();
    let cos_negative = emu.maps.read_f64(0x3000).unwrap();
    let sin_negative = emu.maps.read_f64(0x3008).unwrap();

    assert!((sin_positive + sin_negative).abs() < 1e-15, "sin(-x) should equal -sin(x)");
    assert!((cos_positive - cos_negative).abs() < 1e-15, "cos(-x) should equal cos(x)");
}

#[test]
fn test_fsincos_periodicity() {
    let mut emu = emu64();    // sin(x + 2π) = sin(x), cos(x + 2π) = cos(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    let angle = std::f64::consts::FRAC_PI_6;

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, angle);
    emu.run(None).unwrap();
    let cos1 = emu.maps.read_f64(0x3000).unwrap();
    let sin1 = emu.maps.read_f64(0x3008).unwrap();

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, angle + 2.0 * std::f64::consts::PI);
    emu.run(None).unwrap();
    let cos2 = emu.maps.read_f64(0x3000).unwrap();
    let sin2 = emu.maps.read_f64(0x3008).unwrap();

    assert!((sin1 - sin2).abs() < 1e-14, "sin(x + 2π) should equal sin(x)");
    assert!((cos1 - cos2).abs() < 1e-14, "cos(x + 2π) should equal cos(x)");
}

// ============================================================================
// FSINCOS - Range Reduction
// ============================================================================

#[test]
fn test_fsincos_large_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0 * std::f64::consts::PI);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.abs() <= 1.0, "sin(large value) should be in [-1, 1]");
    assert!(cosine.abs() <= 1.0, "cos(large value) should be in [-1, 1]");
}

#[test]
fn test_fsincos_large_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -100.0 * std::f64::consts::PI);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.abs() <= 1.0, "sin(large negative) should be in [-1, 1]");
    assert!(cosine.abs() <= 1.0, "cos(large negative) should be in [-1, 1]");
}

// ============================================================================
// FSINCOS - Small Angles
// ============================================================================

#[test]
fn test_fsincos_small_angle() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let small_angle = 0.001;
    emu.maps.write_f64(0x2000, small_angle);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!((sine - small_angle).abs() < 1e-7, "sin(small x) ≈ x");
    assert!((cosine - 1.0).abs() < 1e-5, "cos(small x) ≈ 1");
}

// ============================================================================
// FSINCOS - Various Angles
// ============================================================================

#[test]
fn test_fsincos_various_angles() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    let test_angles = vec![0.1, 0.2, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];

    for angle in test_angles {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, angle);

    emu.run(None).unwrap();

        let cosine = emu.maps.read_f64(0x3000).unwrap();
        let sine = emu.maps.read_f64(0x3008).unwrap();
        let expected_sin = angle.sin();
        let expected_cos = angle.cos();
        assert!((sine - expected_sin).abs() < 1e-14, "sin({}) error too large", angle);
        assert!((cosine - expected_cos).abs() < 1e-14, "cos({}) error too large", angle);
    }
}

// ============================================================================
// FSINCOS - Special Values
// ============================================================================

#[test]
fn test_fsincos_infinity() {
    let mut emu = emu64();    // FSINCOS of infinity should produce NaN
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.is_nan(), "sin(infinity) should produce NaN");
    assert!(cosine.is_nan(), "cos(infinity) should produce NaN");
}

#[test]
fn test_fsincos_neg_infinity() {
    let mut emu = emu64();    // FSINCOS of -infinity should produce NaN
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NEG_INFINITY);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.is_nan(), "sin(-infinity) should produce NaN");
    assert!(cosine.is_nan(), "cos(-infinity) should produce NaN");
}

#[test]
fn test_fsincos_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);

    emu.run(None).unwrap();

    let cosine = emu.maps.read_f64(0x3000).unwrap();
    let sine = emu.maps.read_f64(0x3008).unwrap();
    assert!(sine.is_nan(), "sin(NaN) should be NaN");
    assert!(cosine.is_nan(), "cos(NaN) should be NaN");
}

// ============================================================================
// FSINCOS - Performance Benefits
// ============================================================================

#[test]
fn test_fsincos_multiple_angles() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cos1
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sin1
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010] ; cos2
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00,  // FSTP qword [0x3018] ; sin2
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_4);
    emu.maps.write_f64(0x2008, std::f64::consts::FRAC_PI_3);

    emu.run(None).unwrap();

    let cos1 = emu.maps.read_f64(0x3000).unwrap();
    let sin1 = emu.maps.read_f64(0x3008).unwrap();
    let cos2 = emu.maps.read_f64(0x3010).unwrap();
    let sin2 = emu.maps.read_f64(0x3018).unwrap();

    let expected_sin1 = (std::f64::consts::FRAC_PI_4).sin();
    let expected_cos1 = (std::f64::consts::FRAC_PI_4).cos();
    let expected_sin2 = (std::f64::consts::PI / 3.0).sin();
    let expected_cos2 = (std::f64::consts::PI / 3.0).cos();

    assert!((sin1 - expected_sin1).abs() < 1e-15, "sin(π/4)");
    assert!((cos1 - expected_cos1).abs() < 1e-15, "cos(π/4)");
    assert!((sin2 - expected_sin2).abs() < 1e-15, "sin(π/3)");
    assert!((cos2 - expected_cos2).abs() < 1e-15, "cos(π/3)");
}

// ============================================================================
// FSINCOS - All Quadrants
// ============================================================================

#[test]
fn test_fsincos_all_quadrants() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_6);
    emu.run(None).unwrap();
    let cos_q1 = emu.maps.read_f64(0x3000).unwrap();
    let sin_q1 = emu.maps.read_f64(0x3008).unwrap();
    assert!(sin_q1 > 0.0 && cos_q1 > 0.0, "Quadrant I: both should be positive");

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0 * std::f64::consts::PI / 3.0);
    emu.run(None).unwrap();
    let cos_q2 = emu.maps.read_f64(0x3000).unwrap();
    let sin_q2 = emu.maps.read_f64(0x3008).unwrap();
    assert!(sin_q2 > 0.0 && cos_q2 < 0.0, "Quadrant II: sin>0, cos<0");

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 4.0 * std::f64::consts::PI / 3.0);
    emu.run(None).unwrap();
    let cos_q3 = emu.maps.read_f64(0x3000).unwrap();
    let sin_q3 = emu.maps.read_f64(0x3008).unwrap();
    assert!(sin_q3 < 0.0 && cos_q3 < 0.0, "Quadrant III: both should be negative");

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0 * std::f64::consts::PI / 3.0);
    emu.run(None).unwrap();
    let cos_q4 = emu.maps.read_f64(0x3000).unwrap();
    let sin_q4 = emu.maps.read_f64(0x3008).unwrap();
    assert!(sin_q4 < 0.0 && cos_q4 > 0.0, "Quadrant IV: sin<0, cos>0");
}

#[test]
fn test_fsincos_bound_check() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFB,                                  // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; sine
        0xF4,                                        // HLT
    ];

    let test_angles = vec![0.1, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 50.0];

    for angle in test_angles {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, angle);

    emu.run(None).unwrap();

        let cosine = emu.maps.read_f64(0x3000).unwrap();
        let sine = emu.maps.read_f64(0x3008).unwrap();
        assert!(sine >= -1.0 && sine <= 1.0, "sin({}) must be in [-1, 1], got {}", angle, sine);
        assert!(cosine >= -1.0 && cosine <= 1.0, "cos({}) must be in [-1, 1], got {}", angle, cosine);
    }
}
