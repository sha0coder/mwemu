//! Tests for the FCMOVcc conditional move instructions.
//!
//! FCMOVcc - Floating-Point Conditional Move
//!
//! Performs a conditional move of the source operand (ST(i)) to ST(0) based on
//! the state of the EFLAGS condition code flags. The instruction tests the specified
//! condition and if it is true, the source operand is loaded into ST(0).
//!
//! Instructions:
//! - FCMOVB: Move if below (CF=1)
//! - FCMOVE: Move if equal (ZF=1)
//! - FCMOVBE: Move if below or equal (CF=1 or ZF=1)
//! - FCMOVU: Move if unordered (PF=1)
//! - FCMOVNB: Move if not below (CF=0)
//! - FCMOVNE: Move if not equal (ZF=0)
//! - FCMOVNBE: Move if not below or equal (CF=0 and ZF=0)
//! - FCMOVNU: Move if not unordered (PF=0)
//!
//! Opcodes:
//! - FCMOVB: DA C0+i
//! - FCMOVE: DA C8+i
//! - FCMOVBE: DA D0+i
//! - FCMOVU: DA D8+i
//! - FCMOVNB: DB C0+i
//! - FCMOVNE: DB C8+i
//! - FCMOVNBE: DB D0+i
//! - FCMOVNU: DB D8+i
//!
//! Reference: /Users/int/dev/rax/docs/fcmovb:fcmove:fcmovbe:fcmovu.txt
//! Reference: /Users/int/dev/rax/docs/fcmovnb:fcmovne:fcmovnbe:fcmovnu.txt

use crate::*;

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: u64, addr: u64, value: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FCMOVB - Move if below (CF=1)
// ============================================================================

#[test]
fn test_fcmovb_when_cf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0 -> ST(0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0 -> ST(0), ST(1)=1.0
        0xF9, // STC (set carry flag)
        0xDA, 0xC1, // FCMOVB ST(0), ST(1) - should move ST(1) to ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should have moved ST(1) to ST(0)
}

#[test]
fn test_fcmovb_when_cf_clear() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0 -> ST(0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0 -> ST(0), ST(1)=1.0
        0xF8, // CLC (clear carry flag)
        0xDA, 0xC1, // FCMOVB ST(0), ST(1) - should NOT move
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should remain ST(0)
}

#[test]
fn test_fcmovb_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.0
        0xF9, // STC
        0xDA, 0xC2, // FCMOVB ST(0), ST(2) - move ST(2)=1.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0);
}

#[test]
fn test_fcmovb_after_cmp() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 10.0
        0xB8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xBB, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0x39, 0xD8, // CMP EAX, EBX (1 < 2, sets CF)
        0xDA, 0xC1, // FCMOVB ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0); // Should move because CF=1
}

// ============================================================================
// FCMOVE - Move if equal (ZF=1)
// ============================================================================

#[test]
fn test_fcmove_when_zf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xBB, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xD8, // CMP EAX, EBX (equal, sets ZF)
        0xDA, 0xC9, // FCMOVE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move
}

#[test]
fn test_fcmove_when_zf_clear() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xBB, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0x39, 0xD8, // CMP EAX, EBX (not equal, clears ZF)
        0xDA, 0xC9, // FCMOVE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should NOT move
}

#[test]
fn test_fcmove_st3() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD 4.0
        0xB8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7
        0xBB, 0x07, 0x00, 0x00, 0x00, // MOV EBX, 7
        0x39, 0xD8, // CMP (sets ZF)
        0xDA, 0xCB, // FCMOVE ST(0), ST(3)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);
    emu.maps.write_f64(DATA_ADDR + 24, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move ST(3)
}

// ============================================================================
// FCMOVBE - Move if below or equal (CF=1 or ZF=1)
// ============================================================================

#[test]
fn test_fcmovbe_when_cf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xF9, // STC (CF=1)
        0xDA, 0xD1, // FCMOVBE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move
}

#[test]
fn test_fcmovbe_when_zf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0x3D, 0x03, 0x00, 0x00, 0x00, // CMP EAX, 3 (sets ZF)
        0xDA, 0xD1, // FCMOVBE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move
}

#[test]
fn test_fcmovbe_when_neither_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0x3D, 0x02, 0x00, 0x00, 0x00, // CMP EAX, 2 (5 > 2, CF=0, ZF=0)
        0xDA, 0xD1, // FCMOVBE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should NOT move
}

// ============================================================================
// FCMOVU - Move if unordered (PF=1)
// ============================================================================

#[test]
fn test_fcmovu_when_pf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB0, 0x03, // MOV AL, 3 (binary 11, even parity)
        0xA8, 0xFF, // TEST AL, 0xFF (sets PF=1)
        0xDA, 0xD9, // FCMOVU ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move
}

#[test]
fn test_fcmovu_when_pf_clear() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB0, 0x07, // MOV AL, 7 (binary 111, odd parity)
        0xA8, 0xFF, // TEST AL, 0xFF (sets PF=0)
        0xDA, 0xD9, // FCMOVU ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should NOT move
}

// ============================================================================
// FCMOVNB - Move if not below (CF=0)
// ============================================================================

#[test]
fn test_fcmovnb_when_cf_clear() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xF8, // CLC (CF=0)
        0xDB, 0xC1, // FCMOVNB ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move
}

#[test]
fn test_fcmovnb_when_cf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xF9, // STC (CF=1)
        0xDB, 0xC1, // FCMOVNB ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should NOT move
}

#[test]
fn test_fcmovnb_after_cmp_greater() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 10.0
        0xB8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xBB, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0x39, 0xD8, // CMP EAX, EBX (5 >= 2, CF=0)
        0xDB, 0xC1, // FCMOVNB ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0); // Should move
}

// ============================================================================
// FCMOVNE - Move if not equal (ZF=0)
// ============================================================================

#[test]
fn test_fcmovne_when_zf_clear() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xBB, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0x39, 0xD8, // CMP EAX, EBX (not equal, ZF=0)
        0xDB, 0xC9, // FCMOVNE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move
}

#[test]
fn test_fcmovne_when_zf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xBB, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xD8, // CMP EAX, EBX (equal, ZF=1)
        0xDB, 0xC9, // FCMOVNE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should NOT move
}

// ============================================================================
// FCMOVNBE - Move if not below or equal (CF=0 and ZF=0)
// ============================================================================

#[test]
fn test_fcmovnbe_when_neither_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0x3D, 0x02, 0x00, 0x00, 0x00, // CMP EAX, 2 (5 > 2, CF=0, ZF=0)
        0xDB, 0xD1, // FCMOVNBE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move
}

#[test]
fn test_fcmovnbe_when_cf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xF9, // STC (CF=1)
        0xDB, 0xD1, // FCMOVNBE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should NOT move
}

#[test]
fn test_fcmovnbe_when_zf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0x3D, 0x03, 0x00, 0x00, 0x00, // CMP EAX, 3 (ZF=1)
        0xDB, 0xD1, // FCMOVNBE ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should NOT move
}

// ============================================================================
// FCMOVNU - Move if not unordered (PF=0)
// ============================================================================

#[test]
fn test_fcmovnu_when_pf_clear() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB0, 0x07, // MOV AL, 7 (odd parity, PF=0)
        0xA8, 0xFF, // TEST AL, 0xFF
        0xDB, 0xD9, // FCMOVNU ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // Should move
}

#[test]
fn test_fcmovnu_when_pf_set() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xB0, 0x03, // MOV AL, 3 (even parity, PF=1)
        0xA8, 0xFF, // TEST AL, 0xFF
        0xDB, 0xD9, // FCMOVNU ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // Should NOT move
}

// ============================================================================
// Complex scenarios
// ============================================================================

#[test]
fn test_fcmov_chain() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.0
        0xF8, // CLC (CF=0)
        0xDA, 0xC1, // FCMOVB ST(0), ST(1) - should not move
        0xF9, // STC (CF=1)
        0xDB, 0xC2, // FCMOVNB ST(0), ST(2) - should not move (CF=1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3.0); // Should remain ST(0)
}

#[test]
fn test_fcmov_preserves_other_registers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.0
        0xF9, // STC
        0xDA, 0xC1, // FCMOVB ST(0), ST(1) - move 2.0 to ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP ST(0) (2.0)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP ST(0) (was ST(1), still 2.0)
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP ST(0) (was ST(2), still 1.0)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0); // ST(0) after move
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 2.0); // ST(1) unchanged
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 1.0); // ST(2) unchanged
}

#[test]
fn test_fcmov_with_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 20.0
        0xB8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xBB, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0x39, 0xD8, // CMP EAX, EBX (1 < 2, CF=1)
        0xDA, 0xC1, // FCMOVB ST(0), ST(1) - move 10.0 to ST(0)
        0xDE, 0xC1, // FADDP - add ST(0) + ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f64(DATA_ADDR + 8, 20.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 20.0); // 10.0 + 10.0 after move
}

#[test]
fn test_fcmov_all_conditions() {
    let mut emu = emu64();    let code = [
        // Test FCMOVB
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xF9, // STC
        0xDA, 0xC1, // FCMOVB
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // Store 1.0
        0xDD, 0xD8, // Pop

        // Test FCMOVE
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.0
        0xB8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0x3D, 0x05, 0x00, 0x00, 0x00, // CMP EAX, 5 (ZF=1)
        0xDA, 0xC9, // FCMOVE
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // Store 1.0
        0xDD, 0xD8, // Pop

        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0); // FCMOVB result
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 1.0); // FCMOVE result
}
