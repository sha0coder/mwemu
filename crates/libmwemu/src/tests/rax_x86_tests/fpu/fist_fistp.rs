//! Tests for the FIST and FISTP instructions.
//!
//! FIST - Store Integer
//! FISTP - Store Integer and Pop
//!
//! Converts the value in ST(0) to a signed integer and stores the result in the destination.
//! FISTP also pops the register stack. The value is rounded to an integer according to the
//! rounding mode specified in the FPU control word.
//!
//! Opcodes:
//! - FIST m16int: DF /2
//! - FIST m32int: DB /2
//! - FISTP m16int: DF /3
//! - FISTP m32int: DB /3
//! - FISTP m64int: DF /7
//!
//! Reference: /Users/int/dev/rax/docs/fist:fistp.txt

use crate::*;

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: u64, addr: u64, value: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

fn read_i16(mem: u64, addr: u64) -> i16 {
    let mut emu = emu64();    let mut buf = [0u8; 2];
    emu.maps.read_bytes_buff(&mut buf, addr);
    i16::from_le_bytes(buf)
}

fn read_i32(mem: u64, addr: u64) -> i32 {
    let mut emu = emu64();    let mut buf = [0u8; 4];
    emu.maps.read_bytes_buff(&mut buf, addr);
    i32::from_le_bytes(buf)
}

fn read_i64(mem: u64, addr: u64) -> i64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    i64::from_le_bytes(buf)
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FIST m16int (opcode DF /2)
// ============================================================================

#[test]
fn test_fist_m16int_zero() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]
    // FIST word ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8, // FSTP ST(0) to clean up
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 0);
}

#[test]
fn test_fist_m16int_positive_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 1);
}

#[test]
fn test_fist_m16int_negative_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -1.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, -1);
}

#[test]
fn test_fist_m16int_100() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 100);
}

#[test]
fn test_fist_m16int_negative_100() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -100.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, -100);
}

#[test]
fn test_fist_m16int_rounding_down() {
    let mut emu = emu64();    // 2.3 should round to 2 (default rounding mode is round-to-nearest-even)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.3);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 2);
}

#[test]
fn test_fist_m16int_rounding_up() {
    let mut emu = emu64();    // 2.7 should round to 3
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.7);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 3);
}

#[test]
fn test_fist_m16int_half_round_even() {
    let mut emu = emu64();    // 2.5 should round to 2 (even)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 2);
}

#[test]
fn test_fist_m16int_preserves_st0() {
    let mut emu = emu64();    // FIST should not pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP to verify value still there
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 42.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 42);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 42.0);
}

// ============================================================================
// FISTP m16int (opcode DF /3)
// ============================================================================

#[test]
fn test_fistp_m16int_zero() {
    let mut emu = emu64();    // FISTP word ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 0);
}

#[test]
fn test_fistp_m16int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 123.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 123);
}

#[test]
fn test_fistp_m16int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -456.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, -456);
}

#[test]
fn test_fistp_m16int_rounding() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 99.6);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 100);
}

#[test]
fn test_fistp_m16int_max_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, i16::MAX as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, i16::MAX);
}

#[test]
fn test_fistp_m16int_min_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, i16::MIN as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, i16::MIN);
}

// ============================================================================
// FIST m32int (opcode DB /2)
// ============================================================================

#[test]
fn test_fist_m32int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 0);
}

#[test]
fn test_fist_m32int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 12345.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 12345);
}

#[test]
fn test_fist_m32int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -67890.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, -67890);
}

#[test]
fn test_fist_m32int_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1000000.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 1000000);
}

#[test]
fn test_fist_m32int_rounding() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0xD8,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1234.8);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 1235);
}

#[test]
fn test_fist_m32int_preserves_st0() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 9999.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 9999);
    assert_eq!(emu.maps.read_f64(0x3004).unwrap(), 9999.0);
}

// ============================================================================
// FISTP m32int (opcode DB /3)
// ============================================================================

#[test]
fn test_fistp_m32int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 0);
}

#[test]
fn test_fistp_m32int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 987654.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 987654);
}

#[test]
fn test_fistp_m32int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -123456.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, -123456);
}

#[test]
fn test_fistp_m32int_max_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, i32::MAX as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, i32::MAX);
}

#[test]
fn test_fistp_m32int_min_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, i32::MIN as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, i32::MIN);
}

// ============================================================================
// FISTP m64int (opcode DF /7)
// ============================================================================

#[test]
fn test_fistp_m64int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, 0);
}

#[test]
fn test_fistp_m64int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 123456789.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, 123456789);
}

#[test]
fn test_fistp_m64int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -987654321.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, -987654321);
}

#[test]
fn test_fistp_m64int_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1_000_000_000_000.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, 1_000_000_000_000);
}

#[test]
fn test_fistp_m64int_rounding() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 999999.9);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, 1000000);
}

#[test]
fn test_fistp_m64int_max_safe_integer() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let max_safe = (1i64 << 53) - 1;
    emu.maps.write_f64(DATA_ADDR, max_safe as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, max_safe);
}

#[test]
fn test_fistp_m64int_min_safe_integer() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let min_safe = -((1i64 << 53) - 1);
    emu.maps.write_f64(DATA_ADDR, min_safe as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, min_safe);
}

// ============================================================================
// Stack behavior tests
// ============================================================================

#[test]
fn test_fist_does_not_pop() {
    let mut emu = emu64();    // FIST should preserve stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FIST
        0xDB, 0x14, 0x25, 0x04, 0x30, 0x00, 0x00, // FIST again
        0xDD, 0xD8, // Clean up
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 777.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 777);
    assert_eq!(emu.maps.read_dword(0x3004).unwrap() as i32, 777);
}

#[test]
fn test_fistp_pops_stack() {
    let mut emu = emu64();    // FISTP should pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 200
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP (should store 200)
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP (should store 100)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.0);
    emu.maps.write_f64(DATA_ADDR + 8, 200.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 200);
    assert_eq!(emu.maps.read_dword(0x3004).unwrap() as i32, 100);
}

// ============================================================================
// Multiple size tests
// ============================================================================

#[test]
fn test_fistp_all_sizes() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD
        0xD9, 0xC0, // FLD ST(0) - duplicate
        0xD9, 0xC0, // FLD ST(0) - duplicate again
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP word
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP dword
        0xDF, 0x3C, 0x25, 0x08, 0x30, 0x00, 0x00, // FISTP qword
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1234.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 1234);
    assert_eq!(emu.maps.read_dword(0x3004).unwrap() as i32, 1234);
    assert_eq!(emu.maps.read_qword(0x3008).unwrap() as i64, 1234);
}

#[test]
fn test_fist_fistp_mixed() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 500
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FIST (preserve)
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP (pop)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 500.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 500);
    assert_eq!(emu.maps.read_dword(0x3004).unwrap() as i32, 500);
}

// ============================================================================
// Rounding mode tests
// ============================================================================

#[test]
fn test_fistp_round_to_nearest_even_positive() {
    let mut emu = emu64();    let test_cases = vec![
        (0.5, 0),   // 0.5 rounds to 0 (even)
        (1.5, 2),   // 1.5 rounds to 2 (even)
        (2.5, 2),   // 2.5 rounds to 2 (even)
        (3.5, 4),   // 3.5 rounds to 4 (even)
    ];

    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xf4,
        ];
        emu.load_code_bytes(&code);
        emu.maps.write_f64(DATA_ADDR, input);

    emu.run(None).unwrap();
        assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, expected, "Failed for input {}", input);
    }
}

#[test]
fn test_fistp_round_to_nearest_even_negative() {
    let mut emu = emu64();    let test_cases = vec![
        (-0.5, 0),   // -0.5 rounds to 0 (even)
        (-1.5, -2),  // -1.5 rounds to -2 (even)
        (-2.5, -2),  // -2.5 rounds to -2 (even)
        (-3.5, -4),  // -3.5 rounds to -4 (even)
    ];

    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xf4,
        ];
        emu.load_code_bytes(&code);
        emu.maps.write_f64(DATA_ADDR, input);

    emu.run(None).unwrap();
        assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, expected, "Failed for input {}", input);
    }
}

// ============================================================================
// Arithmetic integration tests
// ============================================================================

#[test]
fn test_fistp_after_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.5
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 20.3
        0xDE, 0xC1, // FADDP (30.8)
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP (should be 31)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.5);
    emu.maps.write_f64(DATA_ADDR + 8, 20.3);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 31);
}

#[test]
fn test_fistp_sequential() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP word (3)
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP dword (2)
        0xDF, 0x3C, 0x25, 0x08, 0x30, 0x00, 0x00, // FISTP qword (1)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 3);
    assert_eq!(emu.maps.read_dword(0x3004).unwrap() as i32, 2);
    assert_eq!(emu.maps.read_qword(0x3008).unwrap() as i64, 1);
}
