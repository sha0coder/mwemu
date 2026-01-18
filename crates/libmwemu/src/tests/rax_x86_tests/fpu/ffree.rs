//! Tests for the FFREE instruction.
//!
//! FFREE - Free Floating-Point Register
//!
//! Sets the tag in the FPU tag register associated with register ST(i) to empty (11B).
//! The contents of ST(i) and the FPU stack-top pointer (TOP) are not affected.
//! This instruction is used to mark a register as empty without popping the stack.
//!
//! Opcode: DD C0+i (where i = 0-7)
//!
//! Flags affected:
//! - C0, C1, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/ffree.txt

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
// FFREE - Basic Tests
// ============================================================================

#[test]
fn test_ffree_st0() {
    let mut emu = emu64();    // FFREE ST(0) - marks ST(0) as empty but doesn't change value
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 5.0, "FFREE should not change register value");
}

#[test]
fn test_ffree_st1() {
    let mut emu = emu64();    // FFREE ST(1)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xC1,                                  // FFREE ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] ; ST(0)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] ; ST(1)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 20.0);

    emu.run(None).unwrap();

    let result0 = emu.maps.read_f64(0x3000).unwrap();
    let result1 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result0, 20.0, "ST(0) should be unchanged");
    assert_eq!(result1, 10.0, "ST(1) should be unchanged despite FFREE");
}

#[test]
fn test_ffree_st2() {
    let mut emu = emu64();    // FFREE ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0xC2,                                  // FFREE ST(2)
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

    let result0 = emu.maps.read_f64(0x3000).unwrap();
    let result1 = emu.maps.read_f64(0x3008).unwrap();
    let result2 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(result0, 3.0, "ST(0) unchanged");
    assert_eq!(result1, 2.0, "ST(1) unchanged");
    assert_eq!(result2, 1.0, "ST(2) unchanged despite FFREE");
}

#[test]
fn test_ffree_st3() {
    let mut emu = emu64();    // FFREE ST(3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018]
        0xDD, 0xC3,                                  // FFREE ST(3)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00,  // FSTP qword [0x3018]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 4.0);
    emu.maps.write_f64(0x2008, 3.0);
    emu.maps.write_f64(0x2010, 2.0);
    emu.maps.write_f64(0x2018, 1.0);

    emu.run(None).unwrap();

    let result3 = emu.maps.read_f64(0x3018).unwrap();
    assert_eq!(result3, 4.0, "ST(3) value unchanged despite FFREE");
}

#[test]
fn test_ffree_st7() {
    let mut emu = emu64();    // FFREE ST(7) - highest stack register
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0xC7,                                  // FFREE ST(7)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 7.0, "FFREE ST(7) should work");
}

// ============================================================================
// FFREE - Stack Pointer Not Affected
// ============================================================================

#[test]
fn test_ffree_does_not_pop() {
    let mut emu = emu64();    // FFREE should not pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);
    emu.maps.write_f64(0x2008, 200.0);

    emu.run(None).unwrap();

    let result0 = emu.maps.read_f64(0x3000).unwrap();
    let result1 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result0, 200.0, "Stack should still have 2 items");
    assert_eq!(result1, 100.0, "Both items should be accessible");
}

#[test]
fn test_ffree_does_not_push() {
    let mut emu = emu64();    // FFREE should not push onto the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 42.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 42.0, "Stack depth unchanged");
}

// ============================================================================
// FFREE - Multiple FFREE
// ============================================================================

#[test]
fn test_ffree_multiple_registers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0xC1,                                  // FFREE ST(1)
        0xDD, 0xC2,                                  // FFREE ST(2)
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

    let result0 = emu.maps.read_f64(0x3000).unwrap();
    let result1 = emu.maps.read_f64(0x3008).unwrap();
    let result2 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(result0, 3.0, "All values should still be accessible");
    assert_eq!(result1, 2.0);
    assert_eq!(result2, 1.0);
}

#[test]
fn test_ffree_same_register_twice() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0xC0,                                  // FFREE ST(0) again
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 99.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 99.0, "Double FFREE should be safe");
}

// ============================================================================
// FFREE - With Operations
// ============================================================================

#[test]
fn test_ffree_before_operation() {
    let mut emu = emu64();    // FFREE before using the register
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 15.0, "Operations should work after FFREE");
}

#[test]
fn test_ffree_after_operation() {
    let mut emu = emu64();    // FFREE after an operation
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 16.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 4.0, "Result should be available after FFREE");
}

// ============================================================================
// FFREE - Special Values
// ============================================================================

#[test]
fn test_ffree_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "FFREE should work with zero");
}

#[test]
fn test_ffree_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite(), "FFREE should work with infinity");
}

#[test]
fn test_ffree_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_nan(), "FFREE should work with NaN");
}

// ============================================================================
// FFREE - Various Stack Positions
// ============================================================================

#[test]
fn test_ffree_all_positions() {
    let mut emu = emu64();    for i in 0..8 {
        let mut code = vec![
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
            0xDD, 0xC0 + i,                              // FFREE ST(i)
            0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
            0xF4,                                        // HLT
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, i as f64);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, i as f64, "FFREE ST({}) should work", i);
    }
}

#[test]
fn test_ffree_with_full_stack() {
    let mut emu = emu64();    // FFREE with a full stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018]
        0xDD, 0xC3,                                  // FFREE ST(3)
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
    assert_eq!(result, 40.0, "FFREE should work with full stack");
}

#[test]
fn test_ffree_pattern() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0xC2,                                  // FFREE ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);
    emu.maps.write_f64(0x2018, 4.0);

    emu.run(None).unwrap();

    let result0 = emu.maps.read_f64(0x3000).unwrap();
    let result1 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result0, 4.0, "Pattern FFREE test");
    assert_eq!(result1, 3.0);
}

#[test]
fn test_ffree_reverse_order() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0xC2,                                  // FFREE ST(2)
        0xDD, 0xC1,                                  // FFREE ST(1)
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 11.0);
    emu.maps.write_f64(0x2008, 22.0);
    emu.maps.write_f64(0x2010, 33.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 33.0, "Reverse order FFREE should work");
}

#[test]
fn test_ffree_preserves_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0xC0,                                  // FFREE ST(0)
        0xDD, 0xC1,                                  // FFREE ST(1)
        // Use the values after FFREE
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);
    emu.maps.write_f64(0x2008, 8.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 15.0, "Values should be usable after FFREE");
}
