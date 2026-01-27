//! Tests for the FCOMI and FCOMIP instructions.
//!
//! FCOMI/FCOMIP - Ordered Compare and Set EFLAGS
//!
//! FCOMI performs an ordered comparison of ST(0) with ST(i) and sets
//! the ZF, PF, and CF flags in the EFLAGS register according to the result.
//! Unlike FUCOMI, it generates an exception for any NaN operand (QNaN or SNaN).
//!
//! FCOMIP performs the same comparison, sets the EFLAGS, and then pops
//! the FPU stack.
//!
//! Comparison Results (EFLAGS):
//! - ST(0) > SRC: ZF=0, PF=0, CF=0
//! - ST(0) < SRC: ZF=0, PF=0, CF=1
//! - ST(0) = SRC: ZF=1, PF=0, CF=0
//! - Unordered:   ZF=1, PF=1, CF=1 (NaN operand, with exception)
//!
//! Opcodes:
//! - FCOMI: DB F0+i
//! - FCOMIP: DF F0+i
//!
//! Flags affected: ZF, PF, CF
//! Flags cleared: OF, SF, AF
//!
//! Reference: /Users/int/dev/rax/docs/fcomi:fcomip:fcomi:fcomip.txt

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

// Helper function to read u64 from memory
fn read_u64(mem: u64, addr: u64) -> u64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u64::from_le_bytes(buf)
}

// EFLAGS bit positions
const CF_BIT: u64 = 1 << 0;
const PF_BIT: u64 = 1 << 2;
const ZF_BIT: u64 = 1 << 6;

// ============================================================================
// FCOMI - Ordered Compare and Set EFLAGS
// ============================================================================

#[test]
fn test_fcomi_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomi_greater_than() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (10.0)
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomi_less_than() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (7.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (3.0)
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomi_st2() {
    let mut emu = emu64();    // FCOMI with ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDB, 0xF2,                                  // FCOMI ST(2)
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

#[test]
fn test_fcomi_st3() {
    let mut emu = emu64();    // FCOMI with ST(3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,  // FLD qword [0x2018] (4.0)
        0xDB, 0xF3,                                  // FCOMI ST(3)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);
    emu.maps.write_f64(0x2018, 4.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (4.0 > 1.0)");
}

#[test]
fn test_fcomi_infinity_greater() {
    let mut emu = emu64();    // +infinity > finite -> ZF=0, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (100.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomi_infinities_equal() {
    let mut emu = emu64();    // +inf == +inf -> ZF=1, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (+inf)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDB, 0xF1,                                  // FCOMI ST(1)
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

#[test]
fn test_fcomi_positive_negative_zero() {
    let mut emu = emu64();    // +0.0 == -0.0 -> ZF=1, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (-0.0)
        0xD9, 0xEE,                                  // FLDZ (+0.0)
        0xDB, 0xF1,                                  // FCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -0.0);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for equal");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for equal");
}

// ============================================================================
// FCOMIP - Ordered Compare, Set EFLAGS, and Pop
// ============================================================================

#[test]
fn test_fcomip_equal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDF, 0xF1,                                  // FCOMIP ST(1)
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
fn test_fcomip_greater() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (3.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (8.0)
        0xDF, 0xF1,                                  // FCOMIP ST(1)
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
fn test_fcomip_less() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (9.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDF, 0xF1,                                  // FCOMIP ST(1)
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
fn test_fcomip_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDF, 0xF2,                                  // FCOMIP ST(2)
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
// Conditional Branching After FCOMI/FCOMIP
// ============================================================================

#[test]
fn test_fcomi_conditional_je() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomi_conditional_jb() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (10.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (5.0)
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomi_conditional_ja() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (10.0)
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomip_conditional_jne() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDF, 0xF1,                                  // FCOMIP ST(1)
        0x75, 0x07,                                  // JNE +7 (jump if not equal)
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Operations
// ============================================================================

#[test]
fn test_fcomi_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDB, 0xF1,                                  // FCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (3.0)
        0xDB, 0xF1,                                  // FCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00,  // POP qword [0x3008]
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
fn test_fcomip_chain() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (2.0)
        0xDF, 0xF1,                                  // FCOMIP ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDF, 0xF1,                                  // FCOMIP ST(1)
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
    assert_ne!(flags1 & ZF_BIT, 0, "First comparison: 2.0 == 2.0");
    assert_eq!(flags2 & CF_BIT, 0, "Second comparison: 2.0 > 1.0");
}

#[test]
fn test_fcomi_zero_comparison() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEE,                                  // FLDZ
        0xD9, 0xEE,                                  // FLDZ
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomi_inf_vs_finite() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (1000.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (+inf)
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomip_negative_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (-10.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (-5.0)
        0xDF, 0xF1,                                  // FCOMIP ST(1)
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
fn test_fcomi_denormals() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDB, 0xF1,                                  // FCOMI ST(1)
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
fn test_fcomi_huge_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDB, 0xF1,                                  // FCOMI ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e100);
    emu.maps.write_f64(0x2008, 2e100);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (2e100 > 1e100)");
}

#[test]
fn test_fcomip_constants() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xD9, 0xEA,                                  // FLDL2E
        0xDF, 0xF1,                                  // FCOMIP ST(1)
        0x9C,                                        // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // POP qword [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let flags = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_ne!(flags & CF_BIT, 0, "CF should be set (LOG2_E < PI)");
}
