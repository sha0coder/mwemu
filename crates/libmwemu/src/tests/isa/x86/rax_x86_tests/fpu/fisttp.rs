//! Tests for the FISTTP instruction.
//!
//! FISTTP - Store Integer with Truncation and Pop
//!
//! Converts the value in ST(0) to a signed integer using truncation (round toward zero)
//! and stores the result in the destination, then pops the register stack.
//! Unlike FISTP, FISTTP always uses truncation regardless of the rounding mode in the
//! FPU control word.
//!
//! Opcodes:
//! - FISTTP m16int: DF /1
//! - FISTTP m32int: DB /1
//! - FISTTP m64int: DD /1
//!
//! Reference: /Users/int/dev/rax/docs/fisttp.txt

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

// ============================================================================
// FISTTP m16int (opcode DF /1)
// ============================================================================

#[test]
fn test_fisttp_m16int_zero() {
    let mut emu = emu64();    // FISTTP word ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 0);
}

#[test]
fn test_fisttp_m16int_positive_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 1);
}

#[test]
fn test_fisttp_m16int_negative_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -1.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, -1);
}

#[test]
fn test_fisttp_m16int_truncate_positive() {
    let mut emu = emu64();    // 2.9 should truncate to 2 (not round to 3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.9);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 2);
}

#[test]
fn test_fisttp_m16int_truncate_negative() {
    let mut emu = emu64();    // -2.9 should truncate to -2 (toward zero, not -3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -2.9);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, -2);
}

#[test]
fn test_fisttp_m16int_truncate_half() {
    let mut emu = emu64();    // 2.5 should truncate to 2 (not round)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 2.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 2);
}

#[test]
fn test_fisttp_m16int_truncate_negative_half() {
    let mut emu = emu64();    // -2.5 should truncate to -2 (toward zero)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -2.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, -2);
}

#[test]
fn test_fisttp_m16int_truncate_small_fraction() {
    let mut emu = emu64();    // 99.1 truncates to 99
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 99.1);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 99);
}

#[test]
fn test_fisttp_m16int_truncate_large_fraction() {
    let mut emu = emu64();    // 99.9 truncates to 99
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 99.9);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 99);
}

#[test]
fn test_fisttp_m16int_max() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, i16::MAX as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, i16::MAX);
}

#[test]
fn test_fisttp_m16int_min() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, i16::MIN as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, i16::MIN);
}

// ============================================================================
// FISTTP m32int (opcode DB /1)
// ============================================================================

#[test]
fn test_fisttp_m32int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 0);
}

#[test]
fn test_fisttp_m32int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 12345.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 12345);
}

#[test]
fn test_fisttp_m32int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -67890.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, -67890);
}

#[test]
fn test_fisttp_m32int_truncate_positive() {
    let mut emu = emu64();    // 1234.99 truncates to 1234
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1234.99);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 1234);
}

#[test]
fn test_fisttp_m32int_truncate_negative() {
    let mut emu = emu64();    // -1234.99 truncates to -1234
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -1234.99);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, -1234);
}

#[test]
fn test_fisttp_m32int_truncate_half() {
    let mut emu = emu64();    // 999.5 truncates to 999
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 999.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 999);
}

#[test]
fn test_fisttp_m32int_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1000000.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 1000000);
}

#[test]
fn test_fisttp_m32int_max() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, i32::MAX as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, i32::MAX);
}

#[test]
fn test_fisttp_m32int_min() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, i32::MIN as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, i32::MIN);
}

// ============================================================================
// FISTTP m64int (opcode DD /1)
// ============================================================================

#[test]
fn test_fisttp_m64int_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, 0);
}

#[test]
fn test_fisttp_m64int_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 123456789.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, 123456789);
}

#[test]
fn test_fisttp_m64int_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -987654321.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, -987654321);
}

#[test]
fn test_fisttp_m64int_truncate_positive() {
    let mut emu = emu64();    // 999999.999 truncates to 999999
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 999999.999);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, 999999);
}

#[test]
fn test_fisttp_m64int_truncate_negative() {
    let mut emu = emu64();    // -999999.999 truncates to -999999
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -999999.999);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, -999999);
}

#[test]
fn test_fisttp_m64int_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1_000_000_000_000.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, 1_000_000_000_000);
}

#[test]
fn test_fisttp_m64int_max_safe() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let max_safe = (1i64 << 53) - 1;
    emu.maps.write_f64(DATA_ADDR, max_safe as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, max_safe);
}

#[test]
fn test_fisttp_m64int_min_safe() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let min_safe = -((1i64 << 53) - 1);
    emu.maps.write_f64(DATA_ADDR, min_safe as f64);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_qword(0x3000).unwrap() as i64, min_safe);
}

// ============================================================================
// Truncation vs rounding tests
// ============================================================================

#[test]
fn test_fisttp_truncation_positive_values() {
    let mut emu = emu64();    let test_cases = vec![
        (0.1, 0),
        (0.9, 0),
        (1.1, 1),
        (1.9, 1),
        (2.5, 2),
        (3.5, 3),
        (99.99, 99),
    ];

    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xf4,
        ];
        emu.load_code_bytes(&code);
        emu.maps.write_f64(DATA_ADDR, input);

    emu.run(None).unwrap();
        assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, expected, "Failed for input {}", input);
    }
}

#[test]
fn test_fisttp_truncation_negative_values() {
    let mut emu = emu64();    let test_cases = vec![
        (-0.1, 0),
        (-0.9, 0),
        (-1.1, -1),
        (-1.9, -1),
        (-2.5, -2),
        (-3.5, -3),
        (-99.99, -99),
    ];

    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xf4,
        ];
        emu.load_code_bytes(&code);
        emu.maps.write_f64(DATA_ADDR, input);

    emu.run(None).unwrap();
        assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, expected, "Failed for input {}", input);
    }
}

// ============================================================================
// Stack behavior tests
// ============================================================================

#[test]
fn test_fisttp_pops_stack() {
    let mut emu = emu64();    // FISTTP should pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100.5
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 200.5
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP (200)
        0xDB, 0x0C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTTP (100)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 100.5);
    emu.maps.write_f64(DATA_ADDR + 8, 200.5);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 200);
    assert_eq!(emu.maps.read_dword(0x3004).unwrap() as i32, 100);
}

#[test]
fn test_fisttp_sequential() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.7
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.7
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.7
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP word (3)
        0xDB, 0x0C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTTP dword (2)
        0xDD, 0x0C, 0x25, 0x08, 0x30, 0x00, 0x00, // FISTTP qword (1)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.7);
    emu.maps.write_f64(DATA_ADDR + 8, 2.7);
    emu.maps.write_f64(DATA_ADDR + 16, 3.7);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 3);
    assert_eq!(emu.maps.read_dword(0x3004).unwrap() as i32, 2);
    assert_eq!(emu.maps.read_qword(0x3008).unwrap() as i64, 1);
}

// ============================================================================
// Arithmetic integration tests
// ============================================================================

#[test]
fn test_fisttp_after_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.7
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 20.3
        0xDE, 0xC1, // FADDP (31.0)
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.7);
    emu.maps.write_f64(DATA_ADDR + 8, 20.3);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 31);
}

#[test]
fn test_fisttp_division_truncate() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xF9, // FDIVP (3.333...)
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP (should be 3)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 10.0);
    emu.maps.write_f64(DATA_ADDR + 8, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 3);
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_fisttp_very_small_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.00001);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 0);
}

#[test]
fn test_fisttp_very_small_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -0.00001);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 0);
}

#[test]
fn test_fisttp_almost_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.99999);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 0);
}

#[test]
fn test_fisttp_almost_minus_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -0.99999);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 0);
}

#[test]
fn test_fisttp_all_sizes_same_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1234.9
        0xD9, 0xC0, // FLD ST(0) - duplicate
        0xD9, 0xC0, // FLD ST(0) - duplicate again
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP word
        0xDB, 0x0C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTTP dword
        0xDD, 0x0C, 0x25, 0x08, 0x30, 0x00, 0x00, // FISTTP qword
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1234.9);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_word(0x3000).unwrap() as i16, 1234);
    assert_eq!(emu.maps.read_dword(0x3004).unwrap() as i32, 1234);
    assert_eq!(emu.maps.read_qword(0x3008).unwrap() as i64, 1234);
}

#[test]
fn test_fisttp_pi() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB, // FLDPI
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 3);
}

#[test]
fn test_fisttp_e() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xEA, // FLDL2E
        0xD9, 0xED, // FLDLN2
        0xDE, 0xC9, // FMULP (log2(e) * ln(2) ≈ 1)
        0xDE, 0xC1, // FADDP (1 + 1 ≈ 2)
        0xD9, 0xE8, // FLD1
        0xDE, 0xC1, // FADDP (2 + 1 ≈ 3, but let's just use simple value)
        // Actually, let's just load e directly via calculation
        0xDD, 0xD8, // FSTP ST(0) - cleanup
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // Load e value
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, std::f64::consts::E);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(0x3000).unwrap() as i32, 2);
}
