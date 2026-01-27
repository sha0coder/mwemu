//! Tests for the FADD, FADDP, and FIADD instructions.
//!
//! FADD - Add floating-point
//! FADDP - Add floating-point and pop
//! FIADD - Add integer to floating-point
//!
//! Reference: /Users/int/dev/rax/docs/fadd:faddp:fiadd.txt

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
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FADD m32fp (opcode D8 /0)
// ============================================================================

#[test]
fn test_fadd_m32fp_basic() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 2.0
    // FADD dword ptr [0x2008] ; Add 3.0
    // FSTP qword ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f32(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0);
}

#[test]
fn test_fadd_m32fp_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.5);
    emu.maps.write_f32(DATA_ADDR + 8, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.5);
}

#[test]
fn test_fadd_m32fp_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f32(DATA_ADDR + 8, -3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.0);
}

#[test]
fn test_fadd_m32fp_result_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f32(DATA_ADDR + 8, -5.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

// ============================================================================
// FADD m64fp (opcode DC /0)
// ============================================================================

#[test]
fn test_fadd_m64fp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.5);
    emu.maps.write_f64(DATA_ADDR + 8, 2.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 4.0);
}

#[test]
fn test_fadd_m64fp_large_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e15);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0e15);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3.0e15);
}

#[test]
fn test_fadd_m64fp_small_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e-15);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0e-15);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 3.0e-15).abs() < 1e-30);
}

#[test]
fn test_fadd_m64fp_pi_e() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, std::f64::consts::PI);
    emu.maps.write_f64(DATA_ADDR + 8, std::f64::consts::E);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), std::f64::consts::PI + std::f64::consts::E);
}

// ============================================================================
// FADD ST(0), ST(i) (opcode D8 C0+i)
// ============================================================================

#[test]
fn test_fadd_st0_st1() {
    let mut emu = emu64();    // FLD 2.0, FLD 3.0, FADD ST(0), ST(1) -> ST(0) = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD8, 0xC1, // FADD ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0);
}

#[test]
fn test_fadd_st0_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xD8, 0xC2, // FADD ST(0), ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 4.0); // 3.0 + 1.0
}

// ============================================================================
// FADD ST(i), ST(0) (opcode DC C0+i)
// ============================================================================

#[test]
fn test_fadd_st1_st0() {
    let mut emu = emu64();    // FLD 2.0, FLD 3.0, FADD ST(1), ST(0) -> ST(1) = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0xC1, // FADD ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // Pop ST(0)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // Pop ST(0)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3.0); // ST(0) unchanged
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 5.0); // ST(1) = 2.0 + 3.0
}

// ============================================================================
// FADDP ST(i), ST(0) (opcode DE C0+i)
// ============================================================================

#[test]
fn test_faddp_st1_st0() {
    let mut emu = emu64();    // FADDP adds and pops
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDE, 0xC1, // FADDP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 4.0);
    emu.maps.write_f64(DATA_ADDR + 8, 6.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

#[test]
fn test_faddp_no_operand() {
    let mut emu = emu64();    // FADDP with no operand = FADDP ST(1), ST(0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 7.5);
    emu.maps.write_f64(DATA_ADDR + 8, 2.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

// ============================================================================
// FIADD m16int (opcode DE /0)
// ============================================================================

#[test]
fn test_fiadd_m16int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.5);
    emu.maps.write_word(DATA_ADDR + 8, (10) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 12.5);
}

#[test]
fn test_fiadd_m16int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_word(DATA_ADDR + 8, (-5) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0);
}

#[test]
fn test_fiadd_m16int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.14159);
    emu.maps.write_word(DATA_ADDR + 8, (0) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3.14159);
}

#[test]
fn test_fiadd_m16int_max() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.5);
    emu.maps.write_word(DATA_ADDR + 8, (i16::MAX) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), i16::MAX as f64 + 0.5);
}

#[test]
fn test_fiadd_m16int_min() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.5);
    emu.maps.write_word(DATA_ADDR + 8, (i16::MIN) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), i16::MIN as f64 + 0.5);
}

// ============================================================================
// FIADD m32int (opcode DA /0)
// ============================================================================

#[test]
fn test_fiadd_m32int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.5);
    emu.maps.write_dword(DATA_ADDR + 8, (1000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1001.5);
}

#[test]
fn test_fiadd_m32int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_dword(DATA_ADDR + 8, (-50) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 50.0);
}

#[test]
fn test_fiadd_m32int_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.25);
    emu.maps.write_dword(DATA_ADDR + 8, (1000000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1000000.25);
}

// ============================================================================
// Special cases
// ============================================================================

#[test]
fn test_fadd_infinity_finite() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::INFINITY);
    emu.maps.write_f64(DATA_ADDR + 8, 100.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fadd_neg_infinity_finite() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NEG_INFINITY);
    emu.maps.write_f64(DATA_ADDR + 8, 100.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fadd_same_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::INFINITY);
    emu.maps.write_f64(DATA_ADDR + 8, f64::INFINITY);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fadd_nan_propagation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
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
fn test_fadd_zero_plus_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);
    emu.maps.write_f64(DATA_ADDR + 8, 0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fadd_negative_zero_handling() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);
    emu.maps.write_f64(DATA_ADDR + 8, -0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

// ============================================================================
// Precision tests
// ============================================================================

#[test]
fn test_fadd_precision_accumulation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.1);
    emu.maps.write_f64(DATA_ADDR + 8, 0.2);
    emu.maps.write_f64(DATA_ADDR + 16, 0.3);
    emu.maps.write_f64(DATA_ADDR + 24, 0.4);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_fadd_mixed_magnitude() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e100);
    emu.maps.write_f64(DATA_ADDR + 8, 1.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0e100);
}

#[test]
fn test_fadd_commutative() {
    let mut emu = emu64();    // a + b should equal b + a
    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    let code2 = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code1);
    emu.maps.write_f64(DATA_ADDR, 123.456);
    emu.maps.write_f64(DATA_ADDR + 8, 789.012);
    emu.run(None).unwrap();
    let result1 = emu.maps.read_f64(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.maps.write_f64(DATA_ADDR, 123.456);
    emu.maps.write_f64(DATA_ADDR + 8, 789.012);
    emu.run(None).unwrap();
    let result2 = emu.maps.read_f64(0x3000).unwrap();

    assert_eq!(result1, result2);
}

#[test]
fn test_fadd_associative_property() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 6.0);
}

#[test]
fn test_fadd_chain_operations() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xDE, 0xC1, // FADDP
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xC1, // FADDP
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);
    emu.maps.write_f64(DATA_ADDR + 24, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

#[test]
fn test_fadd_stack_preservation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDC, 0xC1, // FADD ST(1), ST(0) ; ST(1) = 8.0, ST(0) = 3.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP [0x3000] ; 3.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP [0x3008] ; 8.0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 8.0);
}

#[test]
fn test_fiadd_multiple_integers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.5
        0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FIADD 10
        0xDA, 0x04, 0x25, 0x0C, 0x20, 0x00, 0x00, // FIADD 20
        0xDA, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FIADD 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.5);
    emu.maps.write_dword(DATA_ADDR + 8, (10) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 12, (20) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 16, (30) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 61.5);
}
