//! Extended tests for the FRNDINT instruction with various rounding modes.
//!
//! FRNDINT - Round to Integer (Extended Testing)
//!
//! This file contains additional comprehensive tests for FRNDINT with:
//! - All rounding modes (nearest, down, up, toward zero)
//! - Edge cases and special values
//! - Various numeric ranges
//!
//! Rounding Control (RC) bits in FPU Control Word (bits 10-11):
//! - 00b: Round to nearest (even)
//! - 01b: Round down (toward -infinity)
//! - 10b: Round up (toward +infinity)
//! - 11b: Round toward zero (truncate)
//!
//! Reference: /Users/int/dev/rax/docs/frndint.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

fn write_f64(mem: u64, addr: u64, val: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

fn write_u16(mem: u64, addr: u64, val: u16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// Rounding mode constants
const RC_NEAREST: u16 = 0x037F;      // bits 10-11 = 00
const RC_DOWN: u16 = 0x077F;         // bits 10-11 = 01
const RC_UP: u16 = 0x0B7F;           // bits 10-11 = 10
const RC_ZERO: u16 = 0x0F7F;         // bits 10-11 = 11

// Tests with round to nearest (default)
#[test]
fn test_frndint_nearest_half_to_even() {
    let mut emu = emu64();    let test_cases = vec![(2.5, 2.0), (3.5, 4.0), (4.5, 4.0), (5.5, 6.0)];
    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xD9, 0xFC,
            0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xF4,
        ];
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, input);
    emu.run(None).unwrap();
        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, expected, "FRNDINT({}) nearest", input);
    }
}

// Tests with round down
#[test]
fn test_frndint_round_down() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,
    ];
    let test_cases = vec![
        (2.1, 2.0), (2.9, 2.0), (-2.1, -3.0), (-2.9, -3.0),
        (0.1, 0.0), (0.9, 0.0), (-0.1, -1.0), (-0.9, -1.0),
    ];
    for (input, expected) in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, RC_DOWN);
        emu.maps.write_f64(0x2008, input);
    emu.run(None).unwrap();
        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, expected, "FRNDINT({}) down", input);
    }
}

// Tests with round up
#[test]
fn test_frndint_round_up() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    let test_cases = vec![
        (2.1, 3.0), (2.9, 3.0), (-2.1, -2.0), (-2.9, -2.0),
        (0.1, 1.0), (0.9, 1.0), (-0.1, 0.0), (-0.9, 0.0),
    ];
    for (input, expected) in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, RC_UP);
        emu.maps.write_f64(0x2008, input);
    emu.run(None).unwrap();
        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, expected, "FRNDINT({}) up", input);
    }
}

// Tests with round toward zero (truncate)
#[test]
fn test_frndint_round_zero() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    let test_cases = vec![
        (2.1, 2.0), (2.9, 2.0), (-2.1, -2.0), (-2.9, -2.0),
        (0.1, 0.0), (0.9, 0.0), (-0.1, 0.0), (-0.9, 0.0),
    ];
    for (input, expected) in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, RC_ZERO);
        emu.maps.write_f64(0x2008, input);
    emu.run(None).unwrap();
        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, expected, "FRNDINT({}) zero", input);
    }
}

// Special values tests
macro_rules! frndint_special_test {
    ($name:ident, $val:expr, $expected:expr) => {
        #[test]
        fn $name() {
    let mut emu = emu64();            let code = [
                0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
                0xD9, 0xFC,
                0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
                0xF4,
            ];
            emu.load_code_bytes(&code);
            emu.maps.write_f64(0x2000, $val);
    emu.run(None).unwrap();
            let result = emu.maps.read_f64(0x3000).unwrap();
            let expected_val: f64 = $expected;
            if expected_val.is_nan() {
                assert!(result.is_nan());
            } else {
                assert_eq!(result, $expected);
            }
        }
    };
}

frndint_special_test!(test_frndint_inf, f64::INFINITY, f64::INFINITY);
frndint_special_test!(test_frndint_neg_inf, f64::NEG_INFINITY, f64::NEG_INFINITY);
frndint_special_test!(test_frndint_zero, 0.0, 0.0);
frndint_special_test!(test_frndint_neg_zero, -0.0, -0.0);
frndint_special_test!(test_frndint_nan, f64::NAN, f64::NAN);
frndint_special_test!(test_frndint_one, 1.0, 1.0);
frndint_special_test!(test_frndint_neg_one, -1.0, -1.0);

// Edge case tests
#[test]
fn test_frndint_large_integer() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e15);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1e15);
}

#[test]
fn test_frndint_max_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::MAX);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, f64::MAX);
}

// Tests with different modes changing mid-execution
#[test]
fn test_frndint_mode_switching() {
    let mut emu = emu64();    let code = [
        // Round to nearest
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        // Round down
        0xD9, 0x2C, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, RC_NEAREST);
    emu.maps.write_f64(0x2008, 2.5);
    emu.maps.write_word(0x2010, RC_DOWN);
    emu.maps.write_f64(0x2018, 2.9);
    emu.run(None).unwrap();
    let r1 = emu.maps.read_f64(0x3000).unwrap();
    let r2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(r1, 2.0);  // 2.5 nearest -> 2.0 (even)
    assert_eq!(r2, 2.0);  // 2.9 down -> 2.0
}

// Comprehensive test with all rounding modes
#[test]
fn test_frndint_all_modes_positive() {
    let mut emu = emu64();    let input = 7.6;
    let expected = [(RC_NEAREST, 8.0), (RC_DOWN, 7.0), (RC_UP, 8.0), (RC_ZERO, 7.0)];

    for (mode, exp) in expected {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
            0xD9, 0xFC,
            0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xF4,
        ];
        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, mode);
        emu.maps.write_f64(0x2008, input);
    emu.run(None).unwrap();
        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, exp, "Mode {:04X}", mode);
    }
}

#[test]
fn test_frndint_all_modes_negative() {
    let mut emu = emu64();    let input = -7.6;
    let expected = [(RC_NEAREST, -8.0), (RC_DOWN, -8.0), (RC_UP, -7.0), (RC_ZERO, -7.0)];

    for (mode, exp) in expected {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
            0xD9, 0xFC,
            0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xF4,
        ];
        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, mode);
        emu.maps.write_f64(0x2008, input);
    emu.run(None).unwrap();
        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, exp, "Mode {:04X}", mode);
    }
}

// Stress tests with multiple values
#[test]
fn test_frndint_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.4);
    emu.maps.write_f64(0x2008, 2.5);
    emu.maps.write_f64(0x2010, 3.6);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 1.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 2.0);
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 4.0);
}

// More comprehensive tests
macro_rules! batch_test {
    ($name:ident, $mode:expr, $cases:expr) => {
        #[test]
        fn $name() {
    let mut emu = emu64();            for (input, expected) in $cases {
                let code = [
                    0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,
                    0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
                    0xD9, 0xFC,
                    0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
                    0xF4,
                ];
                emu.load_code_bytes(&code);
                emu.maps.write_word(0x2000, $mode);
                emu.maps.write_f64(0x2008, input);
    emu.run(None).unwrap();
                let result = emu.maps.read_f64(0x3000).unwrap();
                assert_eq!(result, expected, "{}", input);
            }
        }
    };
}

batch_test!(test_nearest_fractional, RC_NEAREST, vec![
    (1.1, 1.0), (1.9, 2.0), (2.5, 2.0), (3.5, 4.0),
    (-1.1, -1.0), (-1.9, -2.0), (-2.5, -2.0), (-3.5, -4.0)
]);

batch_test!(test_down_fractional, RC_DOWN, vec![
    (1.1, 1.0), (1.9, 1.0), (2.5, 2.0), (3.5, 3.0),
    (-1.1, -2.0), (-1.9, -2.0), (-2.5, -3.0), (-3.5, -4.0)
]);

batch_test!(test_up_fractional, RC_UP, vec![
    (1.1, 2.0), (1.9, 2.0), (2.5, 3.0), (3.5, 4.0),
    (-1.1, -1.0), (-1.9, -1.0), (-2.5, -2.0), (-3.5, -3.0)
]);

batch_test!(test_zero_fractional, RC_ZERO, vec![
    (1.1, 1.0), (1.9, 1.0), (2.5, 2.0), (3.5, 3.0),
    (-1.1, -1.0), (-1.9, -1.0), (-2.5, -2.0), (-3.5, -3.0)
]);

#[test]
fn test_frndint_tiny_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 0.0001);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

#[test]
fn test_frndint_preserves_integers() {
    let mut emu = emu64();    for val in &[1.0, 2.0, 100.0, -50.0, 999.0] {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xD9, 0xFC,
            0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xF4,
        ];
        emu.load_code_bytes(&code);
        emu.maps.write_f64(0x2000, *val);
    emu.run(None).unwrap();
        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, *val);
    }
}

#[test]
fn test_frndint_denormal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFC,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::MIN_POSITIVE / 2.0);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}
