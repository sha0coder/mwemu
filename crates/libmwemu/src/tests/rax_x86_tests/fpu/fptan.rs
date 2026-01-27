//! Tests for the FPTAN instruction.
//!
//! FPTAN - Partial Tangent
//!
//! Computes the approximate tangent of the source operand in register ST(0), stores
//! the result in ST(0), and pushes a 1.0 onto the FPU register stack. The source
//! operand must be given in radians and must be less than ±2^63.
//!
//! The value 1.0 is pushed onto the register stack after the tangent has been computed
//! to maintain compatibility with the Intel 8087 and Intel 287 math coprocessors.
//!
//! If the source operand is outside the acceptable range, the C2 flag in the FPU status
//! word is set, and the value in register ST(0) remains unchanged.
//!
//! Opcode: D9 F2
//!
//! Operation:
//! IF ST(0) < 2^63 THEN
//!   C2 := 0;
//!   ST(0) := fptan(ST(0));
//!   TOP := TOP - 1;
//!   ST(0) := 1.0;
//! ELSE
//!   C2 := 1;
//! FI;
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; set to 1 if stack overflow occurred
//! - C2: Set to 1 if outside range (-2^63 < source < +2^63);
 // otherwise 0
//! - C0, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fptan.txt

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
// FPTAN Tests: Special Angles
// ============================================================================

#[test]
fn test_fptan_zero() {
    let mut emu = emu64();    // FLD qword [0x2000]  ; Load angle
    // FPTAN               ; Compute tangent, push 1.0
    // FSTP qword [0x3000] ; Store the 1.0
    // FSTP qword [0x3008] ; Store the tangent result
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF2,                                  // FPTAN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-15, "tan(0) should be 0");
}

#[test]
fn test_fptan_pi_over_4() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_4);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - 1.0).abs() < 1e-14, "tan(π/4) should be 1");
}

#[test]
fn test_fptan_pi_over_6() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_6);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = 1.0 / 3.0_f64.sqrt();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-14, "tan(π/6) should be 1/√3");
}

#[test]
fn test_fptan_pi_over_3() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_3);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = 3.0_f64.sqrt();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-13, "tan(π/3) should be √3");
}

#[test]
fn test_fptan_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::PI);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-14, "tan(π) should be approximately 0");
}

// ============================================================================
// FPTAN Tests: Negative Angles
// ============================================================================

#[test]
fn test_fptan_negative_pi_over_4() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -std::f64::consts::FRAC_PI_4);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent + 1.0).abs() < 1e-14, "tan(-π/4) should be -1");
}

#[test]
fn test_fptan_negative_pi_over_6() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -std::f64::consts::FRAC_PI_6);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = -1.0 / 3.0_f64.sqrt();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-14, "tan(-π/6) should be -1/√3");
}

#[test]
fn test_fptan_negative_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -std::f64::consts::PI);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-14, "tan(-π) should be approximately 0");
}

// ============================================================================
// FPTAN Tests: Small Angles
// ============================================================================

#[test]
fn test_fptan_small_positive_angle() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = 0.1_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-15, "tan(0.1) should match Rust tan");
}

#[test]
fn test_fptan_small_negative_angle() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.1);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = (-0.1_f64).tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-15, "tan(-0.1) should match Rust tan");
}

#[test]
fn test_fptan_very_small_angle() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.001);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    let expected = (0.001_f64).tan();
    assert!((tangent - expected).abs() < 1e-15, "tan(0.001) should match Rust tan");
}

// ============================================================================
// FPTAN Tests: Multiple of 2π (periodicity)
// ============================================================================

#[test]
fn test_fptan_two_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0 * std::f64::consts::PI);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-13, "tan(2π) should be approximately 0");
}

#[test]
fn test_fptan_three_pi_over_4() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0 * std::f64::consts::FRAC_PI_4);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent + 1.0).abs() < 1e-13, "tan(3π/4) should be -1");
}

#[test]
fn test_fptan_five_pi_over_4() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0 * std::f64::consts::FRAC_PI_4);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - 1.0).abs() < 1e-13, "tan(5π/4) should be 1");
}

// ============================================================================
// FPTAN Tests: Larger Angles (Range Reduction)
// ============================================================================

#[test]
fn test_fptan_ten_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0 * std::f64::consts::PI);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-10, "tan(10π) should be approximately 0");
}

#[test]
fn test_fptan_hundred() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = 100.0_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() / expected.abs() < 1e-10,
        "tan(100) should approximately match Rust tan");
}

#[test]
fn test_fptan_thousand() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1000.0);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let _tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
}

// ============================================================================
// FPTAN Tests: Various Values
// ============================================================================

#[test]
fn test_fptan_one_radian() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = 1.0_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-14, "tan(1) should match Rust tan");
}

#[test]
fn test_fptan_two_radians() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = 2.0_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-14, "tan(2) should match Rust tan");
}

#[test]
fn test_fptan_half_radian() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.5);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = 0.5_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-15, "tan(0.5) should match Rust tan");
}

#[test]
fn test_fptan_pi_over_8() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_8);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = std::f64::consts::FRAC_PI_8.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-15, "tan(π/8) should match Rust tan");
}

#[test]
fn test_fptan_three_pi_over_8() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0 * std::f64::consts::FRAC_PI_8);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();
    let expected = (3.0 * std::f64::consts::FRAC_PI_8).tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-13, "tan(3π/8) should match Rust tan");
}

// ============================================================================
// FPTAN Tests: Compatibility with FPATAN
// ============================================================================

#[test]
fn test_fptan_fpatan_cotangent_calculation() {
    let mut emu = emu64();    // FPTAN returns tan(x) in ST(1) and 1.0 in ST(0)
    // FDIVR ST(1), ST(0) computes ST(1) = ST(0) / ST(1) = 1.0 / tan(x) = cot(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF2,                                  // FPTAN (ST(0)=1.0, ST(1)=tan)
        0xDE, 0xF9,                                  // FDIVR (ST(1) = ST(0)/ST(1) = 1/tan = cot)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] (store cotangent)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::FRAC_PI_4);

    emu.run(None).unwrap();

    let cotangent = emu.maps.read_f64(0x3008).unwrap();
    // cot(π/4) = 1/tan(π/4) = 1/1 = 1
    assert!((cotangent - 1.0).abs() < 1e-14,
        "cot(π/4) calculated via FPTAN+FDIVR should be 1");
}

#[test]
fn test_fptan_positive_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent == 0.0 && !tangent.is_sign_negative(),
        "tan(+0) should be +0");
}

#[test]
fn test_fptan_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);

    emu.run(None).unwrap();

    let one_value = emu.maps.read_f64(0x3000).unwrap();
    let tangent = emu.maps.read_f64(0x3008).unwrap();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent == 0.0 && tangent.is_sign_negative(),
        "tan(-0) should be -0");
}

#[test]
fn test_fptan_symmetry() {
    let mut emu = emu64();    let angle = 0.7;

    let code_pos = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code_pos);
    emu.maps.write_f64(0x2000, angle);
    emu.run(None).unwrap();
    let tan_pos = emu.maps.read_f64(0x3008).unwrap();

    let code_neg = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xF2,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];

    emu.load_code_bytes(&code_neg);
    emu.maps.write_f64(0x2000, -angle);
    emu.run(None).unwrap();
    let tan_neg = emu.maps.read_f64(0x3008).unwrap();

    assert!((tan_pos + tan_neg).abs() < 1e-15,
        "tan(-x) should equal -tan(x)");
}
