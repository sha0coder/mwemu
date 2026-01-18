//! Tests for FMUL, FMULP, and FIMUL instructions.
//!
//! FMUL - Multiply floating-point
//! FMULP - Multiply floating-point and pop
//! FIMUL - Multiply integer with floating-point
//!
//! Reference: /Users/int/dev/rax/docs/fmul:fmulp:fimul.txt

use crate::*;

const DATA_ADDR: u64 = 0x2000;

fn write_f32(mem: u64, addr: u64, value: f32) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn write_f64(mem: u64, addr: u64, value: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn write_i16(mem: u64, addr: u64, value: i16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn write_i32(mem: u64, addr: u64, value: i32) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FMUL m32fp (opcode D8 /1) - ST(0) = ST(0) * m32fp
// ============================================================================

#[test]
fn test_fmul_m32fp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.5);
    emu.maps.write_f32(DATA_ADDR + 8, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

#[test]
fn test_fmul_m32fp_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.5);
    emu.maps.write_f32(DATA_ADDR + 8, 0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fmul_m32fp_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f32(DATA_ADDR + 8, -2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -6.0);
}

#[test]
fn test_fmul_m32fp_both_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -3.0);
    emu.maps.write_f32(DATA_ADDR + 8, -2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 6.0);
}

// ============================================================================
// FMUL m64fp (opcode DC /1)
// ============================================================================

#[test]
fn test_fmul_m64fp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.5);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3.0);
}

#[test]
fn test_fmul_m64fp_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e10);
    emu.maps.write_f64(DATA_ADDR + 8, 1.0e5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0e15);
}

#[test]
fn test_fmul_m64fp_small() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e-10);
    emu.maps.write_f64(DATA_ADDR + 8, 1.0e-5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0e-15);
}

// ============================================================================
// FMUL ST(0), ST(i) (opcode D8 C8+i)
// ============================================================================

#[test]
fn test_fmul_st0_st1() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD8, 0xC9, // FMUL ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f64(DATA_ADDR + 8, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 12.0);
}

#[test]
fn test_fmul_st0_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xD8, 0xCA, // FMUL ST(0), ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);
    emu.maps.write_f64(DATA_ADDR + 16, 5.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0); // 5.0 * 2.0
}

// ============================================================================
// FMUL ST(i), ST(0) (opcode DC C8+i)
// ============================================================================

#[test]
fn test_fmul_st1_st0() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0xC9, // FMUL ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 7.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.0); // ST(0) unchanged
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 14.0); // ST(1) = 2.0 * 7.0
}

// ============================================================================
// FMULP ST(i), ST(0) (opcode DE C8+i)
// ============================================================================

#[test]
fn test_fmulp_st1_st0() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDE, 0xC9, // FMULP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f64(DATA_ADDR + 8, 5.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 15.0);
}

#[test]
fn test_fmulp_no_operand() {
    let mut emu = emu64();    // FMULP with no operand = FMULP ST(1), ST(0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDE, 0xC9,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 4.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

// ============================================================================
// FIMUL m16int (opcode DE /1)
// ============================================================================

#[test]
fn test_fimul_m16int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.5);
    emu.maps.write_word(DATA_ADDR + 8, (4) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

#[test]
fn test_fimul_m16int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_word(DATA_ADDR + 8, (-3) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -15.0);
}

#[test]
fn test_fimul_m16int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 123.456);
    emu.maps.write_word(DATA_ADDR + 8, (0) as i16 as u16);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

// ============================================================================
// FIMUL m32int (opcode DA /1)
// ============================================================================

#[test]
fn test_fimul_m32int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.5);
    emu.maps.write_dword(DATA_ADDR + 8, (100) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 150.0);
}

#[test]
fn test_fimul_m32int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.5);
    emu.maps.write_dword(DATA_ADDR + 8, (-40) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -100.0);
}

#[test]
fn test_fimul_m32int_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.5);
    emu.maps.write_dword(DATA_ADDR + 8, (1000000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 500000.0);
}

// ============================================================================
// Special cases
// ============================================================================

#[test]
fn test_fmul_infinity_finite() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::INFINITY);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fmul_neg_infinity_finite() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NEG_INFINITY);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fmul_infinity_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::INFINITY);
    emu.maps.write_f64(DATA_ADDR + 8, -2.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fmul_nan_propagation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NAN);
    emu.maps.write_f64(DATA_ADDR + 8, 5.0);

    emu.run(None).unwrap();
    assert!(emu.maps.read_f64(0x3000).unwrap().is_nan());
}

#[test]
fn test_fmul_sign_rules() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 6.0);
    assert!(!result.is_sign_negative());

    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -3.0);
    emu.maps.write_f64(DATA_ADDR + 8, -2.0);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 6.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fmul_zero_sign() {
    let mut emu = emu64();    // 0.0 * x should preserve sign based on XOR rule
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);
    emu.maps.write_f64(DATA_ADDR + 8, 5.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fmul_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -0.0);
    emu.maps.write_f64(DATA_ADDR + 8, 5.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result == 0.0 && result.is_sign_negative());
}

#[test]
fn test_fmul_commutative() {
    let mut emu = emu64();    // a * b should equal b * a
    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    let code2 = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code1);
    emu.maps.write_f64(DATA_ADDR, 7.5);
    emu.maps.write_f64(DATA_ADDR + 8, 3.2);
    emu.run(None).unwrap();
    let result1 = emu.maps.read_f64(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.maps.write_f64(DATA_ADDR, 7.5);
    emu.maps.write_f64(DATA_ADDR + 8, 3.2);
    emu.run(None).unwrap();
    let result2 = emu.maps.read_f64(0x3000).unwrap();

    assert_eq!(result1, result2);
}

#[test]
fn test_fmul_associative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);
    emu.maps.write_f64(DATA_ADDR + 16, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 24.0);
}

#[test]
fn test_fmul_by_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 123.456);
    emu.maps.write_f64(DATA_ADDR + 8, 1.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 123.456);
}

#[test]
fn test_fmul_chain() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xC9, // FMULP
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xC9, // FMULP
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD 5.0
        0xDE, 0xC9, // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);
    emu.maps.write_f64(DATA_ADDR + 16, 4.0);
    emu.maps.write_f64(DATA_ADDR + 24, 5.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 120.0); // 2*3*4*5
}

#[test]
fn test_fimul_preserves_fraction() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.5);
    emu.maps.write_dword(DATA_ADDR + 8, (3) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.5);
}

#[test]
fn test_fmul_small_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e-100);
    emu.maps.write_f64(DATA_ADDR + 8, 1.0e100);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0);
}

#[test]
fn test_fmulp_stack_behavior() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xC9, // FMULP ST(1), ST(0) ; ST(1) = 3.0 * 4.0, pop
        0xDE, 0xC9, // FMULP ST(1), ST(0) ; ST(1) = 2.0 * 12.0, pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);
    emu.maps.write_f64(DATA_ADDR + 16, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 24.0);
}

#[test]
fn test_fmul_fractions() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.5);
    emu.maps.write_f64(DATA_ADDR + 8, 0.25);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 0.125);
}
