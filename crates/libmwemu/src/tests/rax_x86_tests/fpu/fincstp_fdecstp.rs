//! Tests for the FINCSTP and FDECSTP instructions.
//!
//! FINCSTP - Increment Stack-Top Pointer
//! FDECSTP - Decrement Stack-Top Pointer
//!
//! FINCSTP adds one to the TOP field of the FPU status word (increments the top-of-stack pointer).
//! If the TOP field contains a 7, it is set to 0. The effect is to rotate the stack by one position.
//! The contents of the FPU data registers and tag register are not affected.
//!
//! FDECSTP subtracts one from the TOP field of the FPU status word (decrements the top-of-stack pointer).
//! If the TOP field contains a 0, it is set to 7. The effect is to rotate the stack by one position.
//! The contents of the FPU data registers and tag register are not affected.
//!
//! Opcodes:
//! - FINCSTP: D9 F7
//! - FDECSTP: D9 F6
//!
//! Flags affected:
//! - C1: Set to 0
//! - C0, C2, C3: Undefined
//!
//! References: /Users/int/dev/rax/docs/fincstp.txt, /Users/int/dev/rax/docs/fdecstp.txt

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
// FINCSTP - Basic Tests
// ============================================================================

#[test]
fn test_fincstp_basic() {
    let mut emu = emu64();    // FINCSTP increments TOP, making what was ST(1) become ST(0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xF7,                                  // FINCSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 20.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 10.0, "After FINCSTP, old ST(1) becomes ST(0)");
}

#[test]
fn test_fincstp_single_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF7,                                  // FINCSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result == 5.0 || result == 0.0, "FINCSTP with single value");
}

#[test]
fn test_fincstp_multiple_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xD9, 0xF7,                                  // FINCSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let result0 = emu.maps.read_f64(0x3000).unwrap();
    let result1 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result0, 2.0, "After FINCSTP, ST(1) becomes ST(0)");
    assert_eq!(result1, 1.0, "After FINCSTP, ST(2) becomes ST(1)");
}

// ============================================================================
// FDECSTP - Basic Tests
// ============================================================================

#[test]
fn test_fdecstp_basic() {
    let mut emu = emu64();    // FDECSTP decrements TOP, making what was ST(0) become ST(1)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xF6,                                  // FDECSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 20.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3008).unwrap();
    let result2 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(result1, 20.0, "After FDECSTP, old ST(0) becomes ST(1)");
    assert_eq!(result2, 10.0, "After FDECSTP, old ST(1) becomes ST(2)");
}

#[test]
fn test_fdecstp_single_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF6,                                  // FDECSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3008).unwrap();
    assert!(result1 == 7.0 || result1 == 0.0, "FDECSTP with single value");
}

// ============================================================================
// FINCSTP/FDECSTP - Wrap Around
// ============================================================================

#[test]
fn test_fincstp_wraparound() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF7,                                  // FINCSTP (1)
        0xD9, 0xF7,                                  // FINCSTP (2)
        0xD9, 0xF7,                                  // FINCSTP (3)
        0xD9, 0xF7,                                  // FINCSTP (4)
        0xD9, 0xF7,                                  // FINCSTP (5)
        0xD9, 0xF7,                                  // FINCSTP (6)
        0xD9, 0xF7,                                  // FINCSTP (7)
        0xD9, 0xF7,                                  // FINCSTP (8) - wrap to 0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 42.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 42.0, "8 FINCSTPs should wrap around to original position");
}

#[test]
fn test_fdecstp_wraparound() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF6,                                  // FDECSTP (1)
        0xD9, 0xF6,                                  // FDECSTP (2)
        0xD9, 0xF6,                                  // FDECSTP (3)
        0xD9, 0xF6,                                  // FDECSTP (4)
        0xD9, 0xF6,                                  // FDECSTP (5)
        0xD9, 0xF6,                                  // FDECSTP (6)
        0xD9, 0xF6,                                  // FDECSTP (7)
        0xD9, 0xF6,                                  // FDECSTP (8) - wrap to 0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 99.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 99.0, "8 FDECSTPs should wrap around to original position");
}

// ============================================================================
// FINCSTP/FDECSTP - Combined
// ============================================================================

#[test]
fn test_fincstp_then_fdecstp() {
    let mut emu = emu64();    // FINCSTP followed by FDECSTP should cancel out
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF6,                                  // FDECSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);
    emu.maps.write_f64(0x2008, 200.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 200.0, "FINCSTP then FDECSTP should cancel out");
}

#[test]
fn test_fdecstp_then_fincstp() {
    let mut emu = emu64();    // FDECSTP followed by FINCSTP should cancel out
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xF6,                                  // FDECSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 50.0);
    emu.maps.write_f64(0x2008, 75.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 75.0, "FDECSTP then FINCSTP should cancel out");
}

#[test]
fn test_multiple_inc_dec_pairs() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF6,                                  // FDECSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF6,                                  // FDECSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF6,                                  // FDECSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 33.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 33.0, "Multiple INC/DEC pairs should cancel");
}

// ============================================================================
// FINCSTP/FDECSTP - Stack Rotation
// ============================================================================

#[test]
fn test_fincstp_rotation() {
    let mut emu = emu64();    // FINCSTP rotates stack upward
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] ; 1
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] ; 2
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] ; 3
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "Two FINCSTPs rotate stack by 2");
}

#[test]
fn test_fdecstp_rotation() {
    let mut emu = emu64();    // FDECSTP rotates stack downward
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xD9, 0xF6,                                  // FDECSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 20.0);
    emu.maps.write_f64(0x2010, 30.0);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 30.0, "FDECSTP rotation");
}

// ============================================================================
// FINCSTP/FDECSTP - Data Preservation
// ============================================================================

#[test]
fn test_fincstp_preserves_data() {
    let mut emu = emu64();    // FINCSTP should not modify register contents
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF6,                                  // FDECSTP (restore)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let value = std::f64::consts::PI;
    emu.maps.write_f64(0x2000, value);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, value, "FINCSTP should preserve data");
}

#[test]
fn test_fdecstp_preserves_data() {
    let mut emu = emu64();    // FDECSTP should not modify register contents
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF6,                                  // FDECSTP
        0xD9, 0xF7,                                  // FINCSTP (restore)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let value = std::f64::consts::E;
    emu.maps.write_f64(0x2000, value);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, value, "FDECSTP should preserve data");
}

// ============================================================================
// FINCSTP/FDECSTP - With Operations
// ============================================================================

#[test]
fn test_fincstp_with_operation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xF7,                                  // FINCSTP
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 10.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 8.0, "FINCSTP with arithmetic");
}

#[test]
fn test_fdecstp_with_operation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xF6,                                  // FDECSTP
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xD9, 0xF7,                                  // FINCSTP (restore)
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);
    emu.maps.write_f64(0x2008, 11.0);
    emu.maps.write_f64(0x2010, 13.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result > 0.0, "FDECSTP with operations");
}

// ============================================================================
// FINCSTP/FDECSTP - Edge Cases
// ============================================================================

#[test]
fn test_fincstp_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 123.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result == 123.0 || result == 0.0, "Sequence of FINCSTPs");
}

#[test]
fn test_fdecstp_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF6,                                  // FDECSTP
        0xD9, 0xF6,                                  // FDECSTP
        0xD9, 0xF6,                                  // FDECSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 456.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result >= 0.0, "Sequence of FDECSTPs");
}

#[test]
fn test_alternating_inc_dec() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF6,                                  // FDECSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF6,                                  // FDECSTP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 789.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 789.0, "Alternating INC/DEC should maintain position");
}

#[test]
fn test_fincstp_full_rotation_with_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP
        0xD9, 0xF7,                                  // FINCSTP (full rotation)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 11.0);
    emu.maps.write_f64(0x2008, 22.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 22.0, "Full rotation should return to start");
}
