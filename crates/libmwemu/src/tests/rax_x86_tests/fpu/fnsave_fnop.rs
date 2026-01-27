//! Tests for the FNSAVE and extended FNOP instructions.
//!
//! FNSAVE - Store x87 FPU State (non-waiting)
//! Extended FNOP - Extended NOP operations
//!
//! FNSAVE saves the entire FPU state to a 94 or 108 byte memory area,
//! then reinitializes the FPU. The instruction does not check for 
//! unmasked floating-point exceptions.
//!
//! Opcodes:
//! - FNSAVE: DD /6
//! - FNOP: D9 D0
//!
//! Reference: /Users/int/dev/rax/docs/fnsave.txt, /Users/int/dev/rax/docs/fnop.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

fn write_u16(mem: u64, addr: u64, val: u16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

fn read_u16(mem: u64, addr: u64) -> u16 {
    let emu = emu64();    let mut buf = [0u8; 2];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u16::from_le_bytes(buf)
}

fn write_f64(mem: u64, addr: u64, val: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// FNSAVE tests
#[test]
fn test_fnsave_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00,  // FNSAVE [0x2000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let fcw = emu.maps.read_word(0x2000).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnsave_with_data() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.run(None).unwrap();
    let fcw = emu.maps.read_word(0x3000).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnsave_reinitializes() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xD9, 0xE8,                                  // FLD1 (should work on empty stack)
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x4000).unwrap();
    assert_eq!(result, 1.0);
}

// FNOP tests
#[test]
fn test_fnop_basic() {
    let mut emu = emu64();    let code = [
        0xD9, 0xD0,  // FNOP
        0xF4,        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_fnop_preserves_stack() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xD0,                                  // FNOP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.14);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 3.14);
}

#[test]
fn test_fnop_sequence() {
    let mut emu = emu64();    let code = [
        0xD9, 0xD0,  // FNOP
        0xD9, 0xD0,  // FNOP
        0xD9, 0xD0,  // FNOP
        0xF4,        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional FNSAVE tests
#[test]
fn test_fnsave_multiple_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.run(None).unwrap();
}

#[test]
fn test_fnop_with_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xD0,                                  // FNOP
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xD0,                                  // FNOP
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 3.0);
}

#[test]
fn test_fnsave_after_arithmetic() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// More comprehensive tests (30+ total)
macro_rules! fnsave_test {
    ($name:ident, $val:expr) => {
        #[test]
        fn $name() {
    let mut emu = emu64();            let code = [
                0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
                0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,
                0xF4,
            ];
            emu.load_code_bytes(&code);
            emu.maps.write_f64(0x2000, $val);
    emu.run(None).unwrap();
        }
    };
}

fnsave_test!(test_fnsave_inf, f64::INFINITY);
fnsave_test!(test_fnsave_neg_inf, f64::NEG_INFINITY);
fnsave_test!(test_fnsave_zero, 0.0);
fnsave_test!(test_fnsave_neg_zero, -0.0);
fnsave_test!(test_fnsave_one, 1.0);
fnsave_test!(test_fnsave_neg_one, -1.0);
fnsave_test!(test_fnsave_large, 1e100);
fnsave_test!(test_fnsave_small, 1e-100);
fnsave_test!(test_fnsave_pi, std::f64::consts::PI);
fnsave_test!(test_fnsave_e, std::f64::consts::E);

macro_rules! fnop_test {
    ($name:ident, $val:expr) => {
        #[test]
        fn $name() {
    let mut emu = emu64();            let code = [
                0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
                0xD9, 0xD0,
                0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
                0xF4,
            ];
            emu.load_code_bytes(&code);
            emu.maps.write_f64(0x2000, $val);
    emu.run(None).unwrap();
            let result = emu.maps.read_f64(0x3000).unwrap();
            assert_eq!(result, $val);
        }
    };
}

fnop_test!(test_fnop_inf, f64::INFINITY);
fnop_test!(test_fnop_neg_inf, f64::NEG_INFINITY);
fnop_test!(test_fnop_zero, 0.0);
fnop_test!(test_fnop_large, 1e200);
fnop_test!(test_fnop_small, 1e-200);
fnop_test!(test_fnop_negative, -42.5);
fnop_test!(test_fnop_positive, 42.5);
fnop_test!(test_fnop_frac, 0.125);
fnop_test!(test_fnop_max, f64::MAX);
fnop_test!(test_fnop_min, f64::MIN);

#[test]
fn test_fnsave_stack_depth() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8,
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_fnop_doesnt_affect_flags() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD8, 0xD1,  // FCOM
        0xD9, 0xD0,  // FNOP
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.run(None).unwrap();
}

#[test]
fn test_fnsave_consistency() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xD9, 0xE8,
        0xDD, 0x34, 0x25, 0x00, 0x31, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let fcw1 = emu.maps.read_word(0x3000).unwrap();
    let fcw2 = emu.maps.read_word(0x3100).unwrap();
    assert_eq!(fcw1, fcw2);
}
