//! Tests for the FILD instruction.
//!
//! FILD - Load Integer
//!
//! Converts the integer source operand to double extended-precision floating-point
//! format and pushes the value onto the FPU register stack. The source operand can be
//! a word integer (2 bytes), a doubleword integer (4 bytes), or a quadword integer (8 bytes).
//!
//! Opcodes:
//! - FILD m16int: DF /0
//! - FILD m32int: DB /0
//! - FILD m64int: DF /5
//!
//! Reference: /Users/int/dev/rax/docs/fild.txt

use crate::*;

const DATA_ADDR: u64 = 0x2000;

fn write_i16(mem: u64, addr: u64, value: i16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn write_i32(mem: u64, addr: u64, value: i32) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn write_i64(mem: u64, addr: u64, value: i64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FILD m16int (opcode DF /0)
// ============================================================================

#[test]
fn test_fild_m16int_zero() {
    let mut emu = emu64();    // FILD word ptr [0x2000]
    // FSTP qword ptr [0x3000]
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (0) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 0.0);
}

#[test]
fn test_fild_m16int_positive_one() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (1) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0);
}

#[test]
fn test_fild_m16int_negative_one() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (-1) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1.0);
}

#[test]
fn test_fild_m16int_max() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (i16::MAX) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), i16::MAX as f64);
}

#[test]
fn test_fild_m16int_min() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (i16::MIN) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), i16::MIN as f64);
}

#[test]
fn test_fild_m16int_positive_100() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (100) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 100.0);
}

#[test]
fn test_fild_m16int_negative_100() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (-100) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -100.0);
}

#[test]
fn test_fild_m16int_1000() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (1000) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1000.0);
}

#[test]
fn test_fild_m16int_multiple() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD 10
        0xDF, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, // FILD 20
        0xDF, 0x04, 0x25, 0x04, 0x20, 0x00, 0x00, // FILD 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 30
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 20
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 10
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (10) as i16 as u16);
    emu.maps.write_word(DATA_ADDR + 2, (20) as i16 as u16);
    emu.maps.write_word(DATA_ADDR + 4, (30) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 30.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 20.0);
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 10.0);
}

#[test]
fn test_fild_m16int_arithmetic() {
    let mut emu = emu64();    // FILD 5, FILD 3, FADDP -> 8
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00,
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (5) as i16 as u16);
    emu.maps.write_word(DATA_ADDR + 2, (3) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 8.0);
}

// ============================================================================
// FILD m32int (opcode DB /0)
// ============================================================================

#[test]
fn test_fild_m32int_zero() {
    let mut emu = emu64();    // FILD dword ptr [0x2000]
    // FSTP qword ptr [0x3000]
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (0) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 0.0);
}

#[test]
fn test_fild_m32int_positive_one() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (1) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0);
}

#[test]
fn test_fild_m32int_negative_one() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (-1) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1.0);
}

#[test]
fn test_fild_m32int_max() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (i32::MAX) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), i32::MAX as f64);
}

#[test]
fn test_fild_m32int_min() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (i32::MIN) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), i32::MIN as f64);
}

#[test]
fn test_fild_m32int_1000000() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (1000000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1000000.0);
}

#[test]
fn test_fild_m32int_negative_1000000() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (-1000000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1000000.0);
}

#[test]
fn test_fild_m32int_12345() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (12345) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 12345.0);
}

#[test]
fn test_fild_m32int_large_positive() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (1_000_000_000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1_000_000_000.0);
}

#[test]
fn test_fild_m32int_large_negative() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (-1_000_000_000) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1_000_000_000.0);
}

#[test]
fn test_fild_m32int_multiple() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD 100
        0xDB, 0x04, 0x25, 0x04, 0x20, 0x00, 0x00, // FILD 200
        0xDB, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FILD 300
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 300
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 200
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 100
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (100) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 4, (200) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 8, (300) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 300.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 200.0);
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 100.0);
}

#[test]
fn test_fild_m32int_arithmetic() {
    let mut emu = emu64();    // FILD 50, FILD 30, FSUBP -> 20
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x04, 0x25, 0x04, 0x20, 0x00, 0x00,
        0xDE, 0xE9, // FSUBP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (50) as i32 as u32);
    emu.maps.write_dword(DATA_ADDR + 4, (30) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 20.0);
}

// ============================================================================
// FILD m64int (opcode DF /5)
// ============================================================================

#[test]
fn test_fild_m64int_zero() {
    let mut emu = emu64();    // FILD qword ptr [0x2000]
    // FSTP qword ptr [0x3000]
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (0) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 0.0);
}

#[test]
fn test_fild_m64int_positive_one() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (1) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0);
}

#[test]
fn test_fild_m64int_negative_one() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (-1) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1.0);
}

#[test]
fn test_fild_m64int_large_positive() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (1_000_000_000_000i64) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1_000_000_000_000.0);
}

#[test]
fn test_fild_m64int_large_negative() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (-1_000_000_000_000i64) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1_000_000_000_000.0);
}

#[test]
fn test_fild_m64int_max_safe_integer() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let max_safe = (1i64 << 53) - 1;
    emu.maps.write_qword(DATA_ADDR, (max_safe) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), max_safe as f64);
}

#[test]
fn test_fild_m64int_min_safe_integer() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let min_safe = -((1i64 << 53) - 1);
    emu.maps.write_qword(DATA_ADDR, (min_safe) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), min_safe as f64);
}

#[test]
fn test_fild_m64int_123456789() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (123456789) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 123456789.0);
}

#[test]
fn test_fild_m64int_negative_123456789() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (-123456789) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -123456789.0);
}

#[test]
fn test_fild_m64int_multiple() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD 1000
        0xDF, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FILD 2000
        0xDF, 0x2C, 0x25, 0x10, 0x20, 0x00, 0x00, // FILD 3000
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 3000
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 2000
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 1000
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (1000) as i64 as u64);
    emu.maps.write_qword(DATA_ADDR + 8, (2000) as i64 as u64);
    emu.maps.write_qword(DATA_ADDR + 16, (3000) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3000.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 2000.0);
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 1000.0);
}

#[test]
fn test_fild_m64int_arithmetic() {
    let mut emu = emu64();    // FILD 100, FILD 50, FMULP -> 5000
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDE, 0xC9, // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (100) as i64 as u64);
    emu.maps.write_qword(DATA_ADDR + 8, (50) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 5000.0);
}

// ============================================================================
// Mixed size tests
// ============================================================================

#[test]
fn test_fild_mixed_sizes() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD word 10
        0xDB, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, // FILD dword 20
        0xDF, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FILD qword 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 30
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 20
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 10
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (10) as i16 as u16);
    emu.maps.write_dword(DATA_ADDR + 2, (20) as i32 as u32);
    emu.maps.write_qword(DATA_ADDR + 8, (30) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 30.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 20.0);
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 10.0);
}

#[test]
fn test_fild_all_sizes_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD word 100
        0xDB, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, // FILD dword 1000
        0xDE, 0xC1, // FADDP
        0xDF, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FILD qword 10000
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (100) as i16 as u16);
    emu.maps.write_dword(DATA_ADDR + 2, (1000) as i32 as u32);
    emu.maps.write_qword(DATA_ADDR + 8, (10000) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 11100.0);
}

// ============================================================================
// Edge cases and special values
// ============================================================================

#[test]
fn test_fild_m16int_power_of_two() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (1024) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1024.0);
}

#[test]
fn test_fild_m32int_power_of_two() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (1 << 20) as i32 as u32); // 1048576

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1048576.0);
}

#[test]
fn test_fild_m64int_power_of_two() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (1i64 << 40) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), (1i64 << 40) as f64);
}

#[test]
fn test_fild_m16int_all_bits_set() {
    let mut emu = emu64();    // -1 in two's complement
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (-1) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1.0);
}

#[test]
fn test_fild_m32int_all_bits_set() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (-1) as i32 as u32);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1.0);
}

#[test]
fn test_fild_m64int_all_bits_set() {
    let mut emu = emu64();    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, (-1) as i64 as u64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), -1.0);
}

#[test]
fn test_fild_stack_operations() {
    let mut emu = emu64();    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD 1
        0xDF, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, // FILD 2
        0xDF, 0x04, 0x25, 0x04, 0x20, 0x00, 0x00, // FILD 3
        0xDF, 0x04, 0x25, 0x06, 0x20, 0x00, 0x00, // FILD 4
        0xDE, 0xC1, // FADDP ST(1), ST(0) -> 7
        0xDE, 0xC1, // FADDP ST(1), ST(0) -> 9
        0xDE, 0xC1, // FADDP ST(1), ST(0) -> 10
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, (1) as i16 as u16);
    emu.maps.write_word(DATA_ADDR + 2, (2) as i16 as u16);
    emu.maps.write_word(DATA_ADDR + 4, (3) as i16 as u16);
    emu.maps.write_word(DATA_ADDR + 6, (4) as i16 as u16);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 10.0);
}

#[test]
fn test_fild_conversion_exact() {
    let mut emu = emu64();    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, (42) as i32 as u32);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result.to_bits(), 42.0_f64.to_bits());
}
