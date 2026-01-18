//! Tests for FSUB, FSUBP, FISUB, FSUBR, FSUBRP, and FISUBR instructions.
//!
//! FSUB - Subtract
//! FSUBP - Subtract and pop
//! FISUB - Subtract integer
//! FSUBR - Reverse subtract
//! FSUBRP - Reverse subtract and pop
//! FISUBR - Reverse subtract integer
//!
//! References: /Users/int/dev/rax/docs/fsub:fsubp:fisub.txt
//!             /Users/int/dev/rax/docs/fsubr:fsubrp:fisubr.txt

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
// FSUB m32fp (opcode D8 /4) - ST(0) = ST(0) - m32fp
// ============================================================================

#[test]
fn test_fsub_m32fp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f32(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.0);
}

#[test]
fn test_fsub_m32fp_negative_result() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f32(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -7.0);
}

#[test]
fn test_fsub_m32fp_zero_result() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.5);
    emu.maps.write_f32(DATA_ADDR + 8, 5.5);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

// ============================================================================
// FSUB m64fp (opcode DC /4)
// ============================================================================

#[test]
fn test_fsub_m64fp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.5);
    emu.maps.write_f64(DATA_ADDR + 8, 50.25);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 50.25);
}

#[test]
fn test_fsub_m64fp_large_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0e15);
    emu.maps.write_f64(DATA_ADDR + 8, 1.0e15);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0e15);
}

// ============================================================================
// FSUB ST(0), ST(i) (opcode D8 E0+i)
// ============================================================================

#[test]
fn test_fsub_st0_st1() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD8, 0xE1, // FSUB ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 12.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.0); // 12.0 - 5.0
}

// ============================================================================
// FSUB ST(i), ST(0) (opcode DC E8+i)
// ============================================================================

#[test]
fn test_fsub_st1_st0() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0xE9, // FSUB ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 12.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 12.0); // ST(0) unchanged
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), -7.0); // ST(1) = 5.0 - 12.0
}

// ============================================================================
// FSUBP ST(i), ST(0) (opcode DE E8+i)
// ============================================================================

#[test]
fn test_fsubp_st1_st0() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDE, 0xE9, // FSUBP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f64(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -7.0); // 3.0 - 10.0
}

// ============================================================================
// FISUB m16int (opcode DE /4)
// ============================================================================

#[test]
fn test_fisub_m16int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.5);
    emu.maps.write_word(DATA_ADDR + 8, (25) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 75.5);
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
    emu.maps.write_f64(DATA_ADDR, 50.0);
    emu.maps.write_word(DATA_ADDR + 8, (-10) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 60.0);
}

// ============================================================================
// FISUB m32int (opcode DA /4)
// ============================================================================

#[test]
fn test_fisub_m32int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1000.5);
    emu.maps.write_dword(DATA_ADDR + 8, (250) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 750.5);
}

// ============================================================================
// FSUBR m32fp (opcode D8 /5) - ST(0) = m32fp - ST(0)
// ============================================================================

#[test]
fn test_fsubr_m32fp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f32(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.0); // 10.0 - 3.0
}

#[test]
fn test_fsubr_m32fp_negative_result() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD8, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f32(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -7.0); // 3.0 - 10.0
}

// ============================================================================
// FSUBR m64fp (opcode DC /5)
// ============================================================================

#[test]
fn test_fsubr_m64fp_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 25.5);
    emu.maps.write_f64(DATA_ADDR + 8, 100.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 75.0); // 100.5 - 25.5
}

// ============================================================================
// FSUBR ST(0), ST(i) (opcode D8 E8+i)
// ============================================================================

#[test]
fn test_fsubr_st0_st1() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD8, 0xE9, // FSUBR ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 12.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -7.0); // 5.0 - 12.0
}

// ============================================================================
// FSUBR ST(i), ST(0) (opcode DC E0+i)
// ============================================================================

#[test]
fn test_fsubr_st1_st0() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0xE1, // FSUBR ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 12.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 12.0); // ST(0) unchanged
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 7.0); // ST(1) = 12.0 - 5.0
}

// ============================================================================
// FSUBRP ST(i), ST(0) (opcode DE E0+i)
// ============================================================================

#[test]
fn test_fsubrp_st1_st0() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDE, 0xE1, // FSUBRP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 3.0);
    emu.maps.write_f64(DATA_ADDR + 8, 10.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 7.0); // 10.0 - 3.0
}

// ============================================================================
// FISUBR m16int (opcode DE /5)
// ============================================================================

#[test]
fn test_fisubr_m16int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 25.5);
    emu.maps.write_word(DATA_ADDR + 8, (100) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 74.5); // 100 - 25.5
}

// ============================================================================
// FISUBR m32int (opcode DA /5)
// ============================================================================

#[test]
fn test_fisubr_m32int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 250.5);
    emu.maps.write_dword(DATA_ADDR + 8, (1000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 749.5); // 1000 - 250.5
}

// ============================================================================
// Special cases and edge tests
// ============================================================================

#[test]
fn test_fsub_infinity_handling() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
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
fn test_fsub_from_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_f64(DATA_ADDR + 8, f64::INFINITY);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fsub_nan_propagation() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
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
fn test_fsub_zero_handling() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0);
}

#[test]
fn test_fsub_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -0.0);
    emu.maps.write_f64(DATA_ADDR + 8, -0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

#[test]
fn test_fsubr_vs_fsub() {
    let mut emu = emu64();    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // FSUB
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    let code2 = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDC, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FSUBR
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code1);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);
    emu.run(None).unwrap();
    let result1 = emu.maps.read_f64(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);
    emu.run(None).unwrap();
    let result2 = emu.maps.read_f64(0x3000).unwrap();

    assert_eq!(result1, result2);
}

#[test]
fn test_fsub_chain() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100.0
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // FSUB 10.0
        0xDC, 0x24, 0x25, 0x10, 0x20, 0x00, 0x00, // FSUB 20.0
        0xDC, 0x24, 0x25, 0x18, 0x20, 0x00, 0x00, // FSUB 30.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_f64(DATA_ADDR + 8, 10.0);
    emu.maps.write_f64(DATA_ADDR + 16, 20.0);
    emu.maps.write_f64(DATA_ADDR + 24, 30.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 40.0);
}

#[test]
fn test_fisub_precision() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.75);
    emu.maps.write_dword(DATA_ADDR + 8, (50) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 50.75);
}

#[test]
fn test_fisubr_precision() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 25.25);
    emu.maps.write_dword(DATA_ADDR + 8, (100) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 74.75); // 100 - 25.25
}

#[test]
fn test_fsub_small_from_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e100);
    emu.maps.write_f64(DATA_ADDR + 8, 1.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0e100);
}

#[test]
fn test_fsubp_stack_behavior() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 20.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 30.0
        0xDE, 0xE9, // FSUBP ST(1), ST(0) ; ST(1) = 20.0 - 30.0, pop
        0xDE, 0xE9, // FSUBP ST(1), ST(0) ; ST(1) = 10.0 - (-10.0), pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f64(DATA_ADDR + 8, 20.0);
    emu.maps.write_f64(DATA_ADDR + 16, 30.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 20.0); // 10.0 - (-10.0)
}

#[test]
fn test_fsubrp_stack_behavior() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 15.0
        0xDE, 0xE1, // FSUBRP ST(1), ST(0) ; ST(1) = 15.0 - 5.0, pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 5.0);
    emu.maps.write_f64(DATA_ADDR + 8, 15.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

#[test]
fn test_fsub_alternating_signs() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -10.5);
    emu.maps.write_f64(DATA_ADDR + 8, -5.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -5.0);
}

#[test]
fn test_fsubr_symmetry() {
    let mut emu = emu64();    // FSUBR should give negative of FSUB when swapping operands
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDC, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FSUBR
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 7.5);
    emu.maps.write_f64(DATA_ADDR + 8, 12.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5.0); // 12.5 - 7.5
}
