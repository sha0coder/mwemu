//! Tests for the FUCOMI and FUCOMIP instructions.
//!
//! FUCOMI/FUCOMIP - Unordered Compare and Set EFLAGS
//!
//! FUCOMI performs an unordered comparison of ST(0) with ST(i) and sets
//! the ZF, PF, and CF flags in the EFLAGS register according to the result.
//! Unlike FCOM, it does not generate an exception for QNaN operands.
//!
//! FUCOMIP performs the same comparison, sets the EFLAGS, and then pops
//! the FPU stack.
//!
//! Comparison Results (EFLAGS):
//! - ST(0) > SRC: ZF=0, PF=0, CF=0
//! - ST(0) < SRC: ZF=0, PF=0, CF=1
//! - ST(0) = SRC: ZF=1, PF=0, CF=0
//! - Unordered:   ZF=1, PF=1, CF=1 (NaN operand, no exception)
//!
//! Opcodes:
//! - FUCOMI: DB E8+i
//! - FUCOMIP: DF E8+i
//!
//! Flags affected: ZF, PF, CF
//! Flags cleared: OF, SF, AF
//!
//! Reference: /Users/int/dev/rax/docs/fucomi:fucomip:fucomi:fucomip.txt

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

// Helper function to read u64 from memory
fn read_u64(mem: u64, addr: u64) -> u64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u64::from_le_bytes(buf)
}

// Helper function to write u64 to memory
fn write_u64(mem: u64, addr: u64, val: u64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// EFLAGS bit positions
const CF_BIT: u64 = 1 << 0;
const PF_BIT: u64 = 1 << 2;
const ZF_BIT: u64 = 1 << 6;

// ============================================================================
// FUCOMI - Unordered Compare and Set EFLAGS
// ============================================================================

#[test]
fn test_fucomi_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val1 = emu.maps.read_f64(0x3008).unwrap();
    let val2 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(val1, 5.0);
    assert_eq!(val2, 5.0);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for equal");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for equal");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear for equal");
}

#[test]
fn test_fucomi_greater_than() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (10.0)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val1 = emu.maps.read_f64(0x3008).unwrap();
    let val2 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(val1, 10.0);
    assert_eq!(val2, 5.0);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear for greater");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for greater");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear for greater");
}

#[test]
fn test_fucomi_less_than() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (7.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (3.0)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val1 = emu.maps.read_f64(0x3008).unwrap();
    let val2 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(val1, 3.0);
    assert_eq!(val2, 7.0);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear for less");
    assert_ne!(flags & CF_BIT, 0, "CF should be set for less");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear for less");
}

#[test]
fn test_fucomi_st2() {
    let mut emu = emu64();    // FUCOMI with ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDB, 0xEA,                                  // FUCOMI ST(2)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00,  // FSTP qword [0x3018]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (3.0 > 1.0)");
}

// ============================================================================
// FUCOMI - NaN Comparisons (Unordered, No Exception)
// ============================================================================

#[test]
fn test_fucomi_nan_vs_number() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (NaN)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, f64::NAN);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val1 = emu.maps.read_f64(0x3008).unwrap();
    let val2 = emu.maps.read_f64(0x3010).unwrap();
    assert!(val1.is_nan());
    assert_eq!(val2, 5.0);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for unordered");
    assert_ne!(flags & PF_BIT, 0, "PF should be set for unordered");
    assert_ne!(flags & CF_BIT, 0, "CF should be set for unordered");
}

#[test]
fn test_fucomi_number_vs_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (NaN)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (10.0)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val1 = emu.maps.read_f64(0x3008).unwrap();
    let val2 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(val1, 10.0);
    assert!(val2.is_nan());
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for unordered");
    assert_ne!(flags & PF_BIT, 0, "PF should be set for unordered");
    assert_ne!(flags & CF_BIT, 0, "CF should be set for unordered");
}

#[test]
fn test_fucomi_nan_vs_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (NaN)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (NaN)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);
    emu.maps.write_f64(0x2008, f64::NAN);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for unordered");
    assert_ne!(flags & PF_BIT, 0, "PF should be set for unordered");
    assert_ne!(flags & CF_BIT, 0, "CF should be set for unordered");
}

#[test]
fn test_fucomi_infinity_greater() {
    let mut emu = emu64();    // +infinity > finite -> ZF=0, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (100.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 100.0);
    emu.maps.write_f64(0x2008, f64::INFINITY);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear");
}

#[test]
fn test_fucomi_infinities_equal() {
    let mut emu = emu64();    // +inf == +inf -> ZF=1, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (+inf)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);
    emu.maps.write_f64(0x2008, f64::INFINITY);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for equal");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for equal");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear for equal");
}

// ============================================================================
// FUCOMIP - Unordered Compare, Set EFLAGS, and Pop
// ============================================================================

#[test]
fn test_fucomip_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDF, 0xE9,                                  // FUCOMIP ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val, 5.0);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for equal");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for equal");
}

#[test]
fn test_fucomip_greater() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (3.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (8.0)
        0xDF, 0xE9,                                  // FUCOMIP ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0);
    emu.maps.write_f64(0x2008, 8.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val, 3.0);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear for greater");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for greater");
}

#[test]
fn test_fucomip_less() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (9.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDF, 0xE9,                                  // FUCOMIP ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 9.0);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val, 9.0);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear for less");
    assert_ne!(flags & CF_BIT, 0, "CF should be set for less");
}

#[test]
fn test_fucomip_with_nan() {
    let mut emu = emu64();    // FUCOMIP with NaN should not generate exception
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (NaN)
        0xDF, 0xE9,                                  // FUCOMIP ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, f64::NAN);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(val, 5.0);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for unordered");
    assert_ne!(flags & PF_BIT, 0, "PF should be set for unordered");
    assert_ne!(flags & CF_BIT, 0, "CF should be set for unordered");
}

#[test]
fn test_fucomip_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDF, 0xEA,                                  // FUCOMIP ST(2)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    let val1 = emu.maps.read_f64(0x3008).unwrap();
    let val2 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(val1, 2.0);
    assert_eq!(val2, 1.0);
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (3.0 > 1.0)");
}

// ============================================================================
// Conditional Branching After FUCOMI/FUCOMIP
// ============================================================================

#[test]
fn test_fucomi_conditional_je() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x74, 0x07,                                  // JE +7 (skip if equal)
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

}

#[test]
fn test_fucomi_conditional_jb() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (10.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (5.0)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x72, 0x07,                                  // JB +7 (jump if below)
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let val = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(val, 1.0, "Jump should not be taken, FLD1 should execute");
}

#[test]
fn test_fucomi_conditional_ja() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (10.0)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x77, 0x07,                                  // JA +7 (jump if above)
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 10.0);

    emu.run(None).unwrap();

    let val = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(val, 0.0, "Jump should not be taken, FLDZ should execute");
}

#[test]
fn test_fucomip_conditional_jp() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (NaN)
        0xDF, 0xE9,                                  // FUCOMIP ST(1)
        0x7A, 0x07,                                  // JP +7 (jump if unordered)
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, f64::NAN);

    emu.run(None).unwrap();

}

// ============================================================================
// Mixed Operations
// ============================================================================

#[test]
fn test_fucomi_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000] (first flags)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00,  // POP qword [0x3008] (second flags)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let flags1 = emu.maps.read_qword(0x3000).unwrap();
    let flags2 = emu.maps.read_qword(0x3008).unwrap();
    assert_eq!(flags1 & CF_BIT, 0, "First comparison: 2.0 > 1.0");
    assert_eq!(flags2 & CF_BIT, 0, "Second comparison: 3.0 > 2.0");
}

#[test]
fn test_fucomip_chain() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (2.0)
        0xDF, 0xE9,                                  // FUCOMIP ST(1) - compare and pop
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDF, 0xE9,                                  // FUCOMIP ST(1) - compare and pop
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00,  // POP qword [0x3008]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 2.0);

    emu.run(None).unwrap();

    let flags1 = emu.maps.read_qword(0x3000).unwrap();
    let flags2 = emu.maps.read_qword(0x3008).unwrap();
    assert_ne!(flags1 & ZF_BIT, 0, "First comparison: 2.0 == 2.0 should set ZF");
    assert_eq!(flags2 & CF_BIT, 0, "Second comparison: 2.0 > 1.0");
}

#[test]
fn test_fucomi_zero_comparison() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEE,                                  // FLDZ
        0xD9, 0xEE,                                  // FLDZ
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for 0.0 == 0.0");
}

#[test]
fn test_fucomi_inf_vs_finite() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1000.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1000.0);
    emu.maps.write_f64(0x2008, f64::INFINITY);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (+inf > 1000.0)");
}

#[test]
fn test_fucomip_negative_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (-10.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (-5.0)
        0xDF, 0xE9,                                  // FUCOMIP ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -10.0);
    emu.maps.write_f64(0x2008, -5.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (-5.0 > -10.0)");
}

#[test]
fn test_fucomi_denormals() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDB, 0xE9,                                  // FUCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let denormal1 = f64::MIN_POSITIVE / 2.0;
    let denormal2 = f64::MIN_POSITIVE / 4.0;
    emu.maps.write_f64(0x2000, denormal2);
    emu.maps.write_f64(0x2008, denormal1);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (denormal1 > denormal2)");
}

#[test]
fn test_fucomi_small_diff() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDB, 0xE9,
        0x9C,
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 1.0 + f64::EPSILON);
    emu.run(None).unwrap();
    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & ZF_BIT, 0);
    assert_eq!(flags & CF_BIT, 0);
}

#[test]
fn test_fucomip_large_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDF, 0xE9,
        0x9C,
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e100);
    emu.maps.write_f64(0x2008, 1e200);
    emu.run(None).unwrap();
    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & CF_BIT, 0);
}

#[test]
fn test_fucomi_mixed_sign_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDB, 0xE9,
        0x9C,
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1e100);
    emu.maps.write_f64(0x2008, 1e100);
    emu.run(None).unwrap();
    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & ZF_BIT, 0);
    assert_eq!(flags & CF_BIT, 0);
}

#[test]
fn test_fucomip_zero_neg_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xEE,
        0xDF, 0xE9,
        0x9C,
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);
    emu.run(None).unwrap();
    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_ne!(flags & ZF_BIT, 0);
}

#[test]
fn test_fucomi_st4() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8,
        0xDB, 0xEC,
        0x9C,
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8, 0xDD, 0xD8, 0xDD, 0xD8, 0xDD, 0xD8, 0xDD, 0xD8,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_ne!(flags & ZF_BIT, 0);
}

#[test]
fn test_fucomip_constants() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,
        0xD9, 0xE8,
        0xDF, 0xE9,
        0x9C,
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_ne!(flags & ZF_BIT, 0);
}
