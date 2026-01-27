//! Tests for the FUCOM, FUCOMP, and FUCOMPP instructions.
//!
//! FUCOM/FUCOMP/FUCOMPP - Unordered Compare Floating-Point Values
//!
//! FUCOM compares ST(0) with a source value and sets condition code flags
//! C0, C2, and C3 in the FPU status word according to the results.
//! Unlike FCOM, FUCOM does not generate an exception for QNaN operands.
//!
//! FUCOMP performs the same comparison and then pops the register stack.
//! FUCOMPP compares ST(0) with ST(1) and pops the stack twice.
//!
//! Comparison Results:
//! - ST(0) > SRC: C3=0, C2=0, C0=0
//! - ST(0) < SRC: C3=0, C2=0, C0=1
//! - ST(0) = SRC: C3=1, C2=0, C0=0
//! - Unordered:   C3=1, C2=1, C0=1 (NaN operand, no exception)
//!
//! Opcodes:
//! - FUCOM: DD E0+i
//! - FUCOMP: DD E8+i
//! - FUCOMPP: DA E9
//!
//! Flags affected:
//! - C1: Set to 0
//! - C0, C2, C3: Set according to comparison result
//!
//! Reference: /Users/int/dev/rax/docs/fucom:fucomp:fucompp.txt

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
// FUCOM - Unordered Compare ST(0) with ST(i)
// ============================================================================

#[test]
fn test_fucom_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 5.0);
    assert_eq!(val2, 5.0);
}

#[test]
fn test_fucom_greater_than() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (10.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 10.0);
    assert_eq!(val2, 5.0);
}

#[test]
fn test_fucom_less_than() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (7.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (3.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 3.0);
    assert_eq!(val2, 7.0);
}

#[test]
fn test_fucom_st0_implicit() {
    let mut emu = emu64();    // FUCOM without operand compares ST(0) with ST(1)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 2.0);
    assert_eq!(val2, 1.0);
}

#[test]
fn test_fucom_st2() {
    let mut emu = emu64();    // FUCOM with ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDD, 0xE2,                                  // FUCOM ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    let val3 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(val1, 3.0);
    assert_eq!(val2, 2.0);
    assert_eq!(val3, 1.0);
}

// ============================================================================
// FUCOM - NaN Comparisons (Unordered, No Exception)
// ============================================================================

#[test]
fn test_fucom_nan_vs_number() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (NaN)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, f64::NAN);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert!(val1.is_nan());
    assert_eq!(val2, 5.0);
}

#[test]
fn test_fucom_number_vs_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (NaN)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (10.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 10.0);
    assert!(val2.is_nan());
}

#[test]
fn test_fucom_nan_vs_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (NaN)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (NaN)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);
    emu.maps.write_f64(0x2008, f64::NAN);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert!(val1.is_nan());
    assert!(val2.is_nan());
}

#[test]
fn test_fucom_positive_negative_zero() {
    let mut emu = emu64();    // +0.0 should equal -0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (-0.0)
        0xD9, 0xEE,                                  // FLDZ (+0.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 0.0);
    assert_eq!(val2, -0.0);
}

#[test]
fn test_fucom_infinity_greater() {
    let mut emu = emu64();    // +infinity > finite
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (100.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);
    emu.maps.write_f64(0x2008, f64::INFINITY);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, f64::INFINITY);
    assert_eq!(val2, 100.0);
}

#[test]
fn test_fucom_negative_infinity_less() {
    let mut emu = emu64();    // -infinity < finite
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (0.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (-inf)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);
    emu.maps.write_f64(0x2008, f64::NEG_INFINITY);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, f64::NEG_INFINITY);
    assert_eq!(val2, 0.0);
}

#[test]
fn test_fucom_infinities_equal() {
    let mut emu = emu64();    // +inf == +inf
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (+inf)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);
    emu.maps.write_f64(0x2008, f64::INFINITY);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, f64::INFINITY);
    assert_eq!(val2, f64::INFINITY);
}

// ============================================================================
// FUCOMP - Unordered Compare and Pop Once
// ============================================================================

#[test]
fn test_fucomp_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE9,                                  // FUCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let val = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(val, 5.0);
}

#[test]
fn test_fucomp_greater() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (3.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (8.0)
        0xDD, 0xE9,                                  // FUCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0);
    emu.maps.write_f64(0x2008, 8.0);

    emu.run(None).unwrap();

    let val = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(val, 3.0);
}

#[test]
fn test_fucomp_less() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (9.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0xE9,                                  // FUCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 9.0);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();

    let val = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(val, 9.0);
}

#[test]
fn test_fucomp_with_nan() {
    let mut emu = emu64();    // FUCOMP with NaN should not generate exception
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (NaN)
        0xDD, 0xE9,                                  // FUCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, f64::NAN);

    emu.run(None).unwrap();

    let val = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(val, 5.0);
}

#[test]
fn test_fucomp_with_constant() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0xE9,                                  // FUCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let val = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(val, 1.0);
}

#[test]
fn test_fucomp_st2() {
    let mut emu = emu64();    // FUCOMP with ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDD, 0xEA,                                  // FUCOMP ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 2.0);
    assert_eq!(val2, 1.0);
}

// ============================================================================
// FUCOMPP - Unordered Compare and Pop Twice
// ============================================================================

#[test]
fn test_fucompp_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDA, 0xE9,                                  // FUCOMPP
        0xD9, 0xE8,                                  // FLD1 (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 4.0);
    emu.maps.write_f64(0x2008, 4.0);

    emu.run(None).unwrap();

    let marker = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(marker, 1.0);
}

#[test]
fn test_fucompp_greater() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (2.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (6.0)
        0xDA, 0xE9,                                  // FUCOMPP
        0xD9, 0xE8,                                  // FLD1 (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0);
    emu.maps.write_f64(0x2008, 6.0);

    emu.run(None).unwrap();

    let marker = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(marker, 1.0);
}

#[test]
fn test_fucompp_less() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (8.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (3.0)
        0xDA, 0xE9,                                  // FUCOMPP
        0xD9, 0xEE,                                  // FLDZ (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 8.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let marker = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(marker, 0.0);
}

#[test]
fn test_fucompp_with_nan() {
    let mut emu = emu64();    // FUCOMPP with NaN should not generate exception
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (NaN)
        0xDA, 0xE9,                                  // FUCOMPP
        0xD9, 0xE8,                                  // FLD1 (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, f64::NAN);

    emu.run(None).unwrap();

    let marker = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(marker, 1.0);
}

#[test]
fn test_fucompp_with_constants() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEE,                                  // FLDZ
        0xD9, 0xE8,                                  // FLD1
        0xDA, 0xE9,                                  // FUCOMPP
        0xD9, 0xEB,                                  // FLDPI (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let marker = emu.maps.read_f64(0x3000).unwrap();
    assert!((marker - std::f64::consts::PI).abs() < 1e-15);
}

// ============================================================================
// FUCOM - Various Numeric Comparisons
// ============================================================================

#[test]
fn test_fucom_very_close_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 1.0 + 1e-15);

    emu.run(None).unwrap();
}

#[test]
fn test_fucom_negative_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (-10.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (-5.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -10.0);
    emu.maps.write_f64(0x2008, -5.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, -5.0);
    assert_eq!(val2, -10.0);
}

#[test]
fn test_fucom_mixed_signs() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (-3.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (3.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -3.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 3.0);
    assert_eq!(val2, -3.0);
}

#[test]
fn test_fucom_tiny_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e-100);
    emu.maps.write_f64(0x2008, 2e-100);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 2e-100);
    assert_eq!(val2, 1e-100);
}

#[test]
fn test_fucom_huge_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e100);
    emu.maps.write_f64(0x2008, 2e100);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 2e100);
    assert_eq!(val2, 1e100);
}

#[test]
fn test_fucom_denormal_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let denormal1 = f64::MIN_POSITIVE / 2.0;
    let denormal2 = f64::MIN_POSITIVE / 4.0;
    emu.maps.write_f64(0x2000, denormal2);
    emu.maps.write_f64(0x2008, denormal1);

    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Operations
// ============================================================================

#[test]
fn test_fucom_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    let val3 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(val1, 3.0);
    assert_eq!(val2, 2.0);
    assert_eq!(val3, 1.0);
}

#[test]
fn test_mixed_fucom_operations() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1 (1.0)
        0xD9, 0xE8,                                  // FLD1 (1.0)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xD9, 0xE8,                                  // FLD1 (1.0)
        0xDD, 0xE9,                                  // FUCOMP ST(1)
        0xDA, 0xE9,                                  // FUCOMPP
        0xD9, 0xEE,                                  // FLDZ (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let marker = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(marker, 0.0);
}

#[test]
fn test_fucom_with_arithmetic() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP (1 + 1 = 2)
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP (1 + 1 = 2)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, 2.0);
    assert_eq!(val2, 2.0);
}

#[test]
fn test_fucompp_negative_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDA, 0xE9,                                  // FUCOMPP
        0xD9, 0xEB,                                  // FLDPI (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NEG_INFINITY);
    emu.maps.write_f64(0x2008, f64::NEG_INFINITY);

    emu.run(None).unwrap();

    let marker = emu.maps.read_f64(0x3000).unwrap();
    assert!((marker - std::f64::consts::PI).abs() < 1e-15);
}

#[test]
fn test_fucom_epsilon() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 1.0 + f64::EPSILON);

    emu.run(None).unwrap();
}

#[test]
fn test_fucomp_sequential() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0xE9,                                  // FUCOMP ST(1)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDD, 0xE9,                                  // FUCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();
}

#[test]
fn test_fucom_max_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::MAX);
    emu.maps.write_f64(0x2008, f64::MAX);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, f64::MAX);
    assert_eq!(val2, f64::MAX);
}

#[test]
fn test_fucompp_after_arithmetic() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP (2)
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC9,                                  // FMULP (1)
        0xDA, 0xE9,                                  // FUCOMPP
        0xD9, 0xEE,                                  // FLDZ (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let marker = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(marker, 0.0);
}

#[test]
fn test_fucom_inf_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (NaN)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);
    emu.maps.write_f64(0x2008, f64::INFINITY);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val1, f64::INFINITY);
    assert!(val2.is_nan());
}

#[test]
fn test_fucom_constants_comparison() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xD9, 0xEA,                                  // FLDL2E
        0xDD, 0xE1,                                  // FUCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let val1 = emu.maps.read_f64(0x3000).unwrap();
    let val2 = emu.maps.read_f64(0x3008).unwrap();
    assert!((val1 - std::f64::consts::LOG2_E).abs() < 1e-15);
    assert!((val2 - std::f64::consts::PI).abs() < 1e-15);
}
