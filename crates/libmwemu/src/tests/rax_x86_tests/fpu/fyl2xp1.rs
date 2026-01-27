//! Tests for the FYL2XP1 instruction.
//!
//! FYL2XP1 - Compute y * log2(x+1)
//!
//! Computes (ST(1) * log2(ST(0) + 1.0)), stores result in ST(1), and pops the register stack.
//! ST(0) must be in range: -(1 - sqrt(2)/2) to (1 - sqrt(2)/2) (approximately -0.2929 to 0.2929)
//! Optimized for small values of x where log2(1+x) provides better accuracy than log2(x).
//! Commonly used in compound interest and annuity calculations.
//!
//! Opcode: D9 F9
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow; set if result rounded up, cleared otherwise
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fyl2xp1.txt

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
// FYL2XP1 - Basic Logarithm Calculations
// ============================================================================

#[test]
fn test_fyl2xp1_zero() {
    let mut emu = emu64();    // log2(1 + 0) = log2(1) = 0, so y * log2(1+0) = 0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 5.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.0
        0xD9, 0xF9,                                  // FYL2XP1 ; ST(1) * log2(ST(0) + 1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);  // x
    emu.maps.write_f64(0x2008, 5.0);  // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 0.0).abs() < 1e-15, "y * log2(1+0) should be 0.0");
}

#[test]
fn test_fyl2xp1_one() {
    let mut emu = emu64();    // log2(1 + 1) = log2(2) = 1, so 1 * log2(1+1) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 1.0
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);  // x
    emu.maps.write_f64(0x2008, 1.0);  // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 1.0).abs() < 1e-10, "1 * log2(1+1) should be 1.0");
}

#[test]
fn test_fyl2xp1_small_positive() {
    let mut emu = emu64();    // log2(1 + 0.1) with y = 1
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.1
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);  // x
    emu.maps.write_f64(0x2008, 1.0);  // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1.1f64.log2();
    assert!((result - expected).abs() < 1e-10, "log2(1.1) should match");
}

#[test]
fn test_fyl2xp1_small_negative() {
    let mut emu = emu64();    // log2(1 - 0.1) with y = 1
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = -0.1
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.1);  // x
    emu.maps.write_f64(0x2008, 1.0);   // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 0.9f64.log2();
    assert!((result - expected).abs() < 1e-10, "log2(0.9) should match");
}

// ============================================================================
// FYL2XP1 - With Multiplier
// ============================================================================

#[test]
fn test_fyl2xp1_with_multiplier() {
    let mut emu = emu64();    // 3 * log2(1 + 0.5) = 3 * log2(1.5)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 3.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.5
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.5);  // x
    emu.maps.write_f64(0x2008, 3.0);  // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 3.0 * 1.5f64.log2();
    assert!((result - expected).abs() < 1e-10, "3 * log2(1.5) should match");
}

#[test]
fn test_fyl2xp1_negative_multiplier() {
    let mut emu = emu64();    // -2 * log2(1 + 0.25)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = -2.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.25
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.25);   // x
    emu.maps.write_f64(0x2008, -2.0);   // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = -2.0 * 1.25f64.log2();
    assert!((result - expected).abs() < 1e-10, "-2 * log2(1.25) should match");
}

#[test]
fn test_fyl2xp1_fractional_multiplier() {
    let mut emu = emu64();    // 0.5 * log2(1 + 0.2)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 0.5
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.2
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.2);  // x
    emu.maps.write_f64(0x2008, 0.5);  // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 0.5 * 1.2f64.log2();
    assert!((result - expected).abs() < 1e-10, "0.5 * log2(1.2) should match");
}

// ============================================================================
// FYL2XP1 - Optimal Range Tests
// ============================================================================

#[test]
fn test_fyl2xp1_near_lower_bound() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let x = -(1.0 - 2.0f64.sqrt() / 2.0) + 0.01; // Just inside range
    emu.maps.write_f64(0x2000, x);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (1.0 + x).log2();
    assert!((result - expected).abs() < 1e-10, "Near lower bound should work");
}

#[test]
fn test_fyl2xp1_near_upper_bound() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let x = (1.0 - 2.0f64.sqrt() / 2.0) - 0.01; // Just inside range
    emu.maps.write_f64(0x2000, x);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (1.0 + x).log2();
    assert!((result - expected).abs() < 1e-10, "Near upper bound should work");
}

#[test]
fn test_fyl2xp1_very_small_x() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.001
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.001);  // x
    emu.maps.write_f64(0x2008, 1.0);    // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1.001f64.log2();
    assert!((result - expected).abs() < 1e-12, "Very small x should be accurate");
}

// ============================================================================
// FYL2XP1 - Compound Interest Applications
// ============================================================================

#[test]
fn test_fyl2xp1_compound_interest_case() {
    let mut emu = emu64();    // (1 + r)^n using y * log2(1+x) where x is interest rate
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.05
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.05);  // 5% interest
    emu.maps.write_f64(0x2008, 1.0);   // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1.05f64.log2();
    assert!((result - expected).abs() < 1e-12, "Compound interest calculation");
}

#[test]
fn test_fyl2xp1_annuity_calculation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.025);  // 2.5% rate
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1.025f64.log2();
    assert!((result - expected).abs() < 1e-12, "Annuity calculation");
}

// ============================================================================
// FYL2XP1 - Logarithm Base Conversion
// ============================================================================

#[test]
fn test_fyl2xp1_natural_log_conversion() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1/log2(e)
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.1
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);  // x
    let log2_e = std::f64::consts::E.log2();
    emu.maps.write_f64(0x2008, 1.0 / log2_e);  // y = 1/log2(e)

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1.1f64.ln();
    assert!((result - expected).abs() < 1e-10, "ln(1.1) conversion should match");
}

#[test]
fn test_fyl2xp1_log10_conversion() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1/log2(10)
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.1
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);  // x
    let log2_10 = 10.0f64.log2();
    emu.maps.write_f64(0x2008, 1.0 / log2_10);  // y = 1/log2(10)

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 1.1f64.log10();
    assert!((result - expected).abs() < 1e-10, "log10(1.1) conversion should match");
}

// ============================================================================
// FYL2XP1 - Special Values
// ============================================================================

#[test]
fn test_fyl2xp1_with_zero_y() {
    let mut emu = emu64();    // 0 * log2(1+x) = 0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 0.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.1
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);  // x
    emu.maps.write_f64(0x2008, 0.0);  // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "0 * log2(1+x) should be 0");
}

#[test]
fn test_fyl2xp1_with_infinity_y() {
    let mut emu = emu64();    // infinity * log2(1+x) for x > 0 should be infinity
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = infinity
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.1
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);            // x
    emu.maps.write_f64(0x2008, f64::INFINITY);  // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && !result.is_sign_negative(), "infinity * log2(1.1) should be infinity");
}

#[test]
fn test_fyl2xp1_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = -0.0
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);  // x
    emu.maps.write_f64(0x2008, 1.0);   // y

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 0.0).abs() < 1e-15, "log2(1 + (-0)) should be 0");
}

// ============================================================================
// FYL2XP1 - Multiple Operations
// ============================================================================

#[test]
fn test_fyl2xp1_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x1
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] ; y2
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; x2
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.1);   // x1
    emu.maps.write_f64(0x2008, 1.0);   // y1
    emu.maps.write_f64(0x2010, 0.25);  // x2
    emu.maps.write_f64(0x2018, 2.0);   // y2

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    let expected1 = 1.1f64.log2();
    let expected2 = 2.0 * 1.25f64.log2();
    assert!((result1 - expected1).abs() < 1e-10, "First FYL2XP1");
    assert!((result2 - expected2).abs() < 1e-10, "Second FYL2XP1");
}

// ============================================================================
// FYL2XP1 - Precision Tests
// ============================================================================

#[test]
fn test_fyl2xp1_precision_small_values() {
    let mut emu = emu64();    // FYL2XP1 should give better precision than FYL2X for small x
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e-10);  // Very small x
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (1.0_f64 + 1e-10).log2();
    assert!((result - expected).abs() < 1e-15, "High precision for small x");
}

#[test]
fn test_fyl2xp1_various_small_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_cases = vec![0.01, 0.05, 0.1, 0.15, 0.2, 0.25];

    for x in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, x);
        emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        let expected = (1.0 + x).log2();
        assert!((result - expected).abs() < 1e-12, "log2(1+{}) precision", x);
    }
}

#[test]
fn test_fyl2xp1_negative_small_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_cases = vec![-0.01, -0.05, -0.1, -0.15, -0.2, -0.25];

    for x in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, x);
        emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        let expected = (1.0 + x).log2();
        assert!((result - expected).abs() < 1e-12, "log2(1+{}) precision", x);
    }
}

// ============================================================================
// FYL2XP1 - Edge Cases
// ============================================================================

#[test]
fn test_fyl2xp1_boundary_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let x = 1.0 - 2.0f64.sqrt() / 2.0;  // Upper boundary
    emu.maps.write_f64(0x2000, x);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (1.0 + x).log2();
    assert!((result - expected).abs() < 1e-10, "At upper boundary");
}

#[test]
fn test_fyl2xp1_boundary_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let x = -(1.0 - 2.0f64.sqrt() / 2.0);  // Lower boundary
    emu.maps.write_f64(0x2000, x);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = (1.0 + x).log2();
    assert!((result - expected).abs() < 1e-10, "At lower boundary");
}

#[test]
fn test_fyl2xp1_mid_range() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; x = 0.0
        0xD9, 0xF9,                                  // FYL2XP1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "log2(1) should be exactly 0");
}
