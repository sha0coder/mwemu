//! Tests for the FXCH instruction.
//!
//! FXCH - Exchange Register Contents
//!
//! Exchanges the contents of ST(0) and ST(i).
//! If no source operand is specified, exchanges ST(0) and ST(1).
//!
//! Opcode: D9 C8+i (exchange ST(0) with ST(i))
//! Opcode: D9 C9 (exchange ST(0) with ST(1))
//!
//! Flags affected:
//! - C1: Set to 0
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fxch.txt

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
// FXCH - Basic Exchange with ST(1)
// ============================================================================

#[test]
fn test_fxch_st1_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; ST(0) = 3.14
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; ST(0) = 2.71, ST(1) = 3.14
        0xD9, 0xC9,                                  // FXCH ST(1)        ; ST(0) = 3.14, ST(1) = 2.71
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; store ST(0) = 3.14
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; store ST(0) = 2.71
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.14);
    emu.maps.write_f64(0x2008, 2.71);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 3.14, "After FXCH, ST(0) should be 3.14");
    assert_eq!(result2, 2.71, "After FXCH, ST(1) should be 2.71");
}

#[test]
fn test_fxch_default() {
    let mut emu = emu64();    // FXCH without operand exchanges ST(0) and ST(1)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; ST(0) = 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; ST(0) = 2.0, ST(1) = 1.0
        0xD9, 0xC9,                                  // FXCH              ; ST(0) = 1.0, ST(1) = 2.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 1.0, "FXCH exchanged ST(0)");
    assert_eq!(result2, 2.0, "FXCH exchanged ST(1)");
}

// ============================================================================
// FXCH - Exchange with Different Stack Positions
// ============================================================================

#[test]
fn test_fxch_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; ST(0) = 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; ST(0) = 2.0, ST(1) = 1.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; ST(0) = 3.0, ST(1) = 2.0, ST(2) = 1.0
        0xD9, 0xCA,                                  // FXCH ST(2)        ; ST(0) = 1.0, ST(1) = 2.0, ST(2) = 3.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; store 1.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; store 2.0
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010] ; store 3.0
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    let result3 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(result1, 1.0, "After FXCH ST(2), ST(0) should be 1.0");
    assert_eq!(result2, 2.0, "ST(1) should remain 2.0");
    assert_eq!(result3, 3.0, "After FXCH ST(2), old ST(0) should be at ST(2)");
}

#[test]
fn test_fxch_st3() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; ST(0) = 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; ST(0) = 20.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; ST(0) = 30.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] ; ST(0) = 40.0
        0xD9, 0xCB,                                  // FXCH ST(3)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 20.0);
    emu.maps.write_f64(0x2010, 30.0);
    emu.maps.write_f64(0x2018, 40.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 10.0, "After FXCH ST(3), ST(0) should be 10.0");
}

#[test]
fn test_fxch_st4() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00,  // FLD qword [0x2020] ; 5.0
        0xD9, 0xCC,                                  // FXCH ST(4)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);
    emu.maps.write_f64(0x2018, 4.0);
    emu.maps.write_f64(0x2020, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "After FXCH ST(4), ST(0) should be 1.0");
}

#[test]
fn test_fxch_st5() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00,  // FLD qword [0x2020] ; 5.0
        0xDD, 0x04, 0x25, 0x28, 0x20, 0x00, 0x00,  // FLD qword [0x2028] ; 6.0
        0xD9, 0xCD,                                  // FXCH ST(5)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);
    emu.maps.write_f64(0x2018, 4.0);
    emu.maps.write_f64(0x2020, 5.0);
    emu.maps.write_f64(0x2028, 6.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "After FXCH ST(5), ST(0) should be 1.0");
}

#[test]
fn test_fxch_st6() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00,  // FLD qword [0x2020] ; 5.0
        0xDD, 0x04, 0x25, 0x28, 0x20, 0x00, 0x00,  // FLD qword [0x2028] ; 6.0
        0xDD, 0x04, 0x25, 0x30, 0x20, 0x00, 0x00,  // FLD qword [0x2030] ; 7.0
        0xD9, 0xCE,                                  // FXCH ST(6)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);
    emu.maps.write_f64(0x2018, 4.0);
    emu.maps.write_f64(0x2020, 5.0);
    emu.maps.write_f64(0x2028, 6.0);
    emu.maps.write_f64(0x2030, 7.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "After FXCH ST(6), ST(0) should be 1.0");
}

#[test]
fn test_fxch_st7() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00,  // FLD qword [0x2020] ; 5.0
        0xDD, 0x04, 0x25, 0x28, 0x20, 0x00, 0x00,  // FLD qword [0x2028] ; 6.0
        0xDD, 0x04, 0x25, 0x30, 0x20, 0x00, 0x00,  // FLD qword [0x2030] ; 7.0
        0xDD, 0x04, 0x25, 0x38, 0x20, 0x00, 0x00,  // FLD qword [0x2038] ; 8.0
        0xD9, 0xCF,                                  // FXCH ST(7)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);
    emu.maps.write_f64(0x2018, 4.0);
    emu.maps.write_f64(0x2020, 5.0);
    emu.maps.write_f64(0x2028, 6.0);
    emu.maps.write_f64(0x2030, 7.0);
    emu.maps.write_f64(0x2038, 8.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "After FXCH ST(7), ST(0) should be 1.0");
}

// ============================================================================
// FXCH - Multiple Exchanges
// ============================================================================

#[test]
fn test_fxch_twice_cancels() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xD9, 0xC9,                                  // FXCH ST(1) again
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 7.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 7.0, "Two FXCH should cancel, ST(0) = 7.0");
    assert_eq!(result2, 5.0, "Two FXCH should cancel, ST(1) = 5.0");
}

#[test]
fn test_fxch_chain() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3.0
        0xD9, 0xC9,                                  // FXCH ST(1) ; swap ST(0) and ST(1)
        0xD9, 0xCA,                                  // FXCH ST(2) ; swap ST(0) and ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "After chain of FXCH");
}

// ============================================================================
// FXCH - With Arithmetic Operations
// ============================================================================

#[test]
fn test_fxch_with_fadd() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 3.0
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDE, 0xC1,                                  // FADDP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 8.0, "FXCH + FADDP should compute 5.0 + 3.0 = 8.0");
}

#[test]
fn test_fxch_sqrt_pattern() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 16.0 (to sqrt)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; value to keep
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; value to keep
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] ; value to keep
        0xD9, 0xCB,                                  // FXCH ST(3) ; bring 16.0 to top
        0xD9, 0xFA,                                  // FSQRT
        0xD9, 0xCB,                                  // FXCH ST(3) ; put result back
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; discard top
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00,  // FSTP qword [0x3018] ; sqrt result
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 16.0);
    emu.maps.write_f64(0x2008, 1.0);
    emu.maps.write_f64(0x2010, 2.0);
    emu.maps.write_f64(0x2018, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3018).unwrap();
    assert_eq!(result, 4.0, "SQRT(16) = 4.0 after FXCH pattern");
}

// ============================================================================
// FXCH - Special Values
// ============================================================================

#[test]
fn test_fxch_with_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 0.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 5.0
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 0.0, "FXCH with zero works");
    assert_eq!(result2, 5.0, "FXCH with zero works");
}

#[test]
fn test_fxch_with_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);
    emu.maps.write_f64(0x2008, 42.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite(), "FXCH with infinity works");
}

#[test]
fn test_fxch_with_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);
    emu.maps.write_f64(0x2008, 7.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_nan(), "FXCH with NaN works");
}

#[test]
fn test_fxch_negative_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -3.14);
    emu.maps.write_f64(0x2008, -2.71);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, -3.14, "FXCH with negative values");
    assert_eq!(result2, -2.71, "FXCH with negative values");
}

// ============================================================================
// FXCH - Edge Cases
// ============================================================================

#[test]
fn test_fxch_st0_is_nop() {
    let mut emu = emu64();    // FXCH ST(0) should be a no-op (exchange ST(0) with itself)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xC8,                                  // FXCH ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 9.9);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 9.9, "FXCH ST(0) should be no-op");
}

#[test]
fn test_fxch_preserves_precision() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let precise_val = std::f64::consts::PI;
    emu.maps.write_f64(0x2000, precise_val);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, precise_val, "FXCH should preserve full precision");
}

#[test]
fn test_fxch_with_very_small_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e-100);
    emu.maps.write_f64(0x2008, 1e100);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1e-100, "FXCH with very small value");
}

#[test]
fn test_fxch_with_denormals() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let denormal = f64::MIN_POSITIVE / 2.0;
    emu.maps.write_f64(0x2000, denormal);
    emu.maps.write_f64(0x2008, 1.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, denormal, "FXCH with denormal value");
}

#[test]
fn test_fxch_alternating_exchanges() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3.0
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xD9, 0xCA,                                  // FXCH ST(2)
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 3.0, "After alternating FXCH");
}

#[test]
fn test_fxch_multiple_values_deep_stack() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00,  // FLD qword [0x2020] ; 5.0
        0xD9, 0xCC,                                  // FXCH ST(4) ; bring 1.0 to top
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);
    emu.maps.write_f64(0x2018, 4.0);
    emu.maps.write_f64(0x2020, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "FXCH ST(4) should bring bottom to top");
}

#[test]
fn test_fxch_pi_and_e() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; PI
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; E
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, std::f64::consts::PI);
    emu.maps.write_f64(0x2008, std::f64::consts::E);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, std::f64::consts::PI, "FXCH with PI");
    assert_eq!(result2, std::f64::consts::E, "FXCH with E");
}

#[test]
fn test_fxch_max_and_min() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::MAX);
    emu.maps.write_f64(0x2008, f64::MIN_POSITIVE);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, f64::MAX, "FXCH with MAX value");
}

#[test]
fn test_fxch_mixed_signs() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xD9, 0xCA,                                  // FXCH ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);
    emu.maps.write_f64(0x2008, -50.0);
    emu.maps.write_f64(0x2010, 25.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 100.0, "FXCH with mixed signs");
}

#[test]
fn test_fxch_fractional_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.123456789);
    emu.maps.write_f64(0x2008, 0.987654321);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.123456789, "FXCH preserves fractional precision");
}

#[test]
fn test_fxch_power_of_two_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1024.0);
    emu.maps.write_f64(0x2008, 2048.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1024.0, "FXCH with powers of 2");
}

#[test]
fn test_fxch_after_arithmetic() {
    let mut emu = emu64();    // FXCH after arithmetic operation
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 5.0
        0xDE, 0xC1,                                  // FADDP ; ST(0) = 15.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3.0
        0xD9, 0xC9,                                  // FXCH ST(1) ; swap 3.0 and 15.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; store 15.0
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 5.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 15.0, "FXCH after arithmetic");
}

#[test]
fn test_fxch_before_comparison() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.5);
    emu.maps.write_f64(0x2008, 3.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 7.5, "FXCH for comparison setup");
}

#[test]
fn test_fxch_zero_and_nonzero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);
    emu.maps.write_f64(0x2008, 100.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 0.0, "Zero value exchanged");
    assert_eq!(result2, 100.0, "Non-zero value exchanged");
}
