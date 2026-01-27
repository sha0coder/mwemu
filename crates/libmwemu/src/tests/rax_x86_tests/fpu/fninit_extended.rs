//! Extended tests for the FNINIT instruction in various states.
//!
//! FNINIT - Initialize Floating-Point Unit (non-waiting)
//!
//! This file contains comprehensive tests for FNINIT covering:
//! - Initialization from various FPU states
//! - Stack handling
//! - Control word initialization
//! - Status word clearing
//! - Tag word reset
//!
//! FNINIT sets the FPU to its default state:
//! - Control Word: 0x037F
//! - Status Word: 0x0000
//! - Tag Word: 0xFFFF (all empty)
//! - Clears exception flags
//!
//! Opcode: DB E3
//!
//! Reference: /Users/int/dev/rax/docs/finit:fninit.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

fn write_f64(mem: u64, addr: u64, val: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

fn write_u16(mem: u64, addr: u64, val: u16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

fn read_u16(mem: u64, addr: u64) -> u16 {
    let emu = emu64();    let mut buf = [0u8; 2];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u16::from_le_bytes(buf)
}

#[test]
fn test_fninit_basic() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,  // FNINIT
        0xF4,        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_fninit_clears_stack() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0xE8,                                  // FLD1 (should work on empty stack)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 99.99);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_resets_control_word() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x0C7F);
    emu.run(None).unwrap();
    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, 0x037F, "Control word should be reset to default");
}

#[test]
fn test_fninit_with_full_stack() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,  // FLD1 x8
        0xD9, 0xE8,
        0xD9, 0xE8,
        0xD9, 0xE8,
        0xD9, 0xE8,
        0xD9, 0xE8,
        0xD9, 0xE8,
        0xD9, 0xE8,
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_multiple_times() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,  // FNINIT
        0xD9, 0xE8,  // FLD1
        0xDB, 0xE3,  // FNINIT
        0xD9, 0xE8,  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_after_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xDE, 0xC1,                                  // FADDP
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 20.0);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

#[test]
fn test_fninit_clears_status() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xD8, 0xD1,                                  // FCOM
        0xDB, 0xE3,                                  // FNINIT
        0xDF, 0xE0,                                  // FNSTSW AX
        0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV [0x3000], EAX
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.run(None).unwrap();
    let status = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(status & 0x3FFF, 0, "Status should be cleared");
}

// Tests with different pre-init states
#[test]
fn test_fninit_after_division_by_zero() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xEE,                                  // FLDZ
        0xDE, 0xF9,                                  // FDIVP (may set exception)
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_after_sqrt_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xD9, 0xFA,                                  // FSQRT (may produce NaN)
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -1.0);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_sequence_operations() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xDB, 0xE3,                                  // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 42.0);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

// Comprehensive state reset tests
macro_rules! fninit_state_test {
    ($name:ident, $setup:expr) => {
        #[test]
        fn $name() {
    let mut emu = emu64();            let mut code = Vec::from($setup);
            code.extend_from_slice(&[
                0xDB, 0xE3,                                  // FNINIT
                0xD9, 0xE8,                                  // FLD1
                0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP [0x3000]
                0xF4,                                        // HLT
            ]);
            emu.load_code_bytes(&code);
            emu.maps.write_f64(0x2000, 100.0);
            emu.maps.write_f64(0x2008, 200.0);
    emu.run(None).unwrap();
            let result = emu.maps.read_f64(0x3000).unwrap();
            assert_eq!(result, 1.0, "Stack should be cleared");
        }
    };
}

fninit_state_test!(test_fninit_after_fld, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD
]);

fninit_state_test!(test_fninit_after_fadd, &[
    0xD9, 0xE8,  // FLD1
    0xD9, 0xE8,  // FLD1
    0xDE, 0xC1,  // FADDP
]);

fninit_state_test!(test_fninit_after_fmul, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
    0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
    0xDE, 0xC9,  // FMULP
]);

fninit_state_test!(test_fninit_after_fsub, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
    0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
    0xDE, 0xE9,  // FSUBP
]);

fninit_state_test!(test_fninit_after_fdiv, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
    0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
    0xDE, 0xF9,  // FDIVP
]);

fninit_state_test!(test_fninit_after_fsqrt, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
    0xD9, 0xFA,  // FSQRT
]);

fninit_state_test!(test_fninit_after_fabs, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
    0xD9, 0xE1,  // FABS
]);

fninit_state_test!(test_fninit_after_fchs, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
    0xD9, 0xE0,  // FCHS
]);

fninit_state_test!(test_fninit_after_fcom, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
    0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
    0xD8, 0xD1,  // FCOM
]);

fninit_state_test!(test_fninit_after_fxch, &[
    0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
    0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,
    0xD9, 0xC9,  // FXCH
]);

// Tests with constants
#[test]
fn test_fninit_after_fldpi() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB,  // FLDPI
        0xDB, 0xE3,  // FNINIT
        0xD9, 0xE8,  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_after_fldz() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEE,  // FLDZ
        0xDB, 0xE3,  // FNINIT
        0xD9, 0xE8,  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_after_fld1() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,  // FLD1
        0xD9, 0xE8,  // FLD1
        0xD9, 0xE8,  // FLD1
        0xDB, 0xE3,  // FNINIT
        0xD9, 0xEE,  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

// Edge cases
#[test]
fn test_fninit_with_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0xE3,
        0xD9, 0xE8,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_with_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDB, 0xE3,
        0xD9, 0xE8,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

// Consistency tests
#[test]
fn test_fninit_idempotent() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,  // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xDB, 0xE3,  // FNINIT again
        0xD9, 0x3C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FNSTCW [0x3008]
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let cw1 = emu.maps.read_word(0x3000).unwrap();
    let cw2 = emu.maps.read_word(0x3008).unwrap();
    assert_eq!(cw1, cw2, "FNINIT should be idempotent");
}

#[test]
fn test_fninit_after_complex_operations() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xEB,                                  // FLDPI
        0xDE, 0xC1,                                  // FADDP
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC9,                                  // FMULP
        0xDB, 0xE3,                                  // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 7.5);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 7.5);
}

// Stress tests
#[test]
fn test_fninit_rapid_succession() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,
        0xDB, 0xE3,
        0xDB, 0xE3,
        0xDB, 0xE3,
        0xDB, 0xE3,
        0xD9, 0xE8,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_alternating_operations() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,  // FLD1
        0xDB, 0xE3,  // FNINIT
        0xD9, 0xEE,  // FLDZ
        0xDB, 0xE3,  // FNINIT
        0xD9, 0xEB,  // FLDPI
        0xDB, 0xE3,  // FNINIT
        0xD9, 0xE8,  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}
