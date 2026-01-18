//! Tests for the FISUB instruction.
//!
//! FISUB - Subtract integer from floating-point (m16int and m32int)
//!
//! Reference: /Users/int/dev/rax/docs/fsub:fsubp:fisub.txt
//!
//! Opcode: DE /4 - FISUB m16int  ; ST(0) = ST(0) - m16int
//! Opcode: DA /4 - FISUB m32int  ; ST(0) = ST(0) - m32int

use crate::*;

const DATA_ADDR: u64 = 0x2000;

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
// FISUB m16int (opcode DE /4) - ST(0) = ST(0) - m16int
// ============================================================================

#[test]
fn test_fisub_m16int_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // FISUB word [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 12.5);
    emu.maps.write_word(DATA_ADDR + 8, (10) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.5);
}

#[test]
fn test_fisub_m16int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_word(DATA_ADDR + 8, (-5) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 15.0); // 10 - (-5) = 15
}

#[test]
fn test_fisub_m16int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
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
fn test_fisub_m16int_max() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 40000.0);
    emu.maps.write_word(DATA_ADDR + 8, (i16::MAX) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 40000.0 - i16::MAX as f64);
}

#[test]
fn test_fisub_m16int_min() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.5);
    emu.maps.write_word(DATA_ADDR + 8, (i16::MIN) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 0.5 - i16::MIN as f64);
}

#[test]
fn test_fisub_m16int_result_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_word(DATA_ADDR + 8, (100) as i16 as u16);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fisub_m16int_result_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_word(DATA_ADDR + 8, (10) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -5.0);
}

// ============================================================================
// FISUB m32int (opcode DA /4) - ST(0) = ST(0) - m32int
// ============================================================================

#[test]
fn test_fisub_m32int_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // FISUB dword [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1001.5);
    emu.maps.write_dword(DATA_ADDR + 8, (1000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.5);
}

#[test]
fn test_fisub_m32int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_dword(DATA_ADDR + 8, (-50) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 150.0);
}

#[test]
fn test_fisub_m32int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 999.999);
    emu.maps.write_dword(DATA_ADDR + 8, (0) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 999.999);
}

#[test]
fn test_fisub_m32int_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2000000.25);
    emu.maps.write_dword(DATA_ADDR + 8, (1000000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1000000.25);
}

#[test]
fn test_fisub_m32int_max() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5e9);
    emu.maps.write_dword(DATA_ADDR + 8, (i32::MAX) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5e9 - i32::MAX as f64);
}

#[test]
fn test_fisub_m32int_min() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.75);
    emu.maps.write_dword(DATA_ADDR + 8, (i32::MIN) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 0.75 - i32::MIN as f64);
}

#[test]
fn test_fisub_m32int_result_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 50000.0);
    emu.maps.write_dword(DATA_ADDR + 8, (50000) as i32 as u32);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

// ============================================================================
// Multiple operations and chain tests
// ============================================================================

#[test]
fn test_fisub_chain_m32() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100.5
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // FISUB 10
        0xDA, 0x24, 0x25, 0x0C, 0x20, 0x00, 0x00, // FISUB 20
        0xDA, 0x24, 0x25, 0x10, 0x20, 0x00, 0x00, // FISUB 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.5);
    emu.maps.write_dword(DATA_ADDR + 8, (10) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 12, (20) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 16, (30) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 40.5);
}

#[test]
fn test_fisub_alternating_signs() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100.0
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // FISUB 50
        0xDA, 0x24, 0x25, 0x0C, 0x20, 0x00, 0x00, // FISUB -25
        0xDA, 0x24, 0x25, 0x10, 0x20, 0x00, 0x00, // FISUB 10
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_dword(DATA_ADDR + 8, (50) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 12, (-25) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 16, (10) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 65.0); // 100 - 50 - (-25) - 10
}

// ============================================================================
// Special cases with infinity
// ============================================================================

#[test]
fn test_fisub_from_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::INFINITY);
    emu.maps.write_dword(DATA_ADDR + 8, (1000) as i32 as u32);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fisub_from_neg_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NEG_INFINITY);
    emu.maps.write_dword(DATA_ADDR + 8, (1000) as i32 as u32);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_negative());
}

// ============================================================================
// Precision tests
// ============================================================================

#[test]
fn test_fisub_precision_large_minus_small() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e100);
    emu.maps.write_dword(DATA_ADDR + 8, (1) as i32 as u32);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0e100);
}

#[test]
fn test_fisub_fractional_preservation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.123);
    emu.maps.write_dword(DATA_ADDR + 8, (5) as i32 as u32);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 5.123).abs() < 1e-12);
}
