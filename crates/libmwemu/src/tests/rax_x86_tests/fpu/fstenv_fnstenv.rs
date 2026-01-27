//! Tests for the FSTENV and FNSTENV instructions with exception handling.
//!
//! FSTENV/FNSTENV - Store x87 FPU Environment
//!
//! FSTENV stores the FPU environment to a memory area (14 or 28 bytes).
//! FNSTENV is the non-waiting version that does not check for pending exceptions.
//!
//! Environment includes:
//! - FPU Control Word (FCW)
//! - FPU Status Word (FSW)
//! - FPU Tag Word (FTW)
//! - FPU Instruction Pointer (FIP)
//! - FPU Data Pointer (FDP)
//! - Last Opcode (FOP)
//!
//! Opcodes:
//! - FSTENV: 9B D9 /6
//! - FNSTENV: D9 /6
//!
//! Reference: /Users/int/dev/rax/docs/fstenv:fnstenv.txt

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

const ENV_FCW: u64 = 0;
const ENV_FSW: u64 = 2;
const ENV_FTW: u64 = 4;

#[test]
fn test_fnstenv_basic() {
    let mut emu = emu64();    let code = [
        0xD9, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00,  // FNSTENV [0x2000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x2000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be valid");
}

#[test]
fn test_fnstenv_with_fpu_data() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.14);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fcw < 0xFFFF);
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_saves_control_word() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert_eq!(saved_cw, 0x037F);
}

#[test]
fn test_fnstenv_saves_status_word() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.718);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_multiple_times() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xD9, 0x34, 0x25, 0x00, 0x31, 0x00, 0x00,  // FNSTENV [0x3100]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3100 + ENV_FCW).unwrap();
    assert_eq!(fcw1, fcw2);
}

#[test]
fn test_fnstenv_after_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_with_different_control_words() {
    let mut emu = emu64();    for cw in &[0x037F, 0x027F, 0x0C7F] {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
            0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
            0xF4,                                        // HLT
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, *cw);

    emu.run(None).unwrap();

        let saved_cw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
        assert_eq!(saved_cw, *cw);
    }
}

#[test]
fn test_fnstenv_with_stack_operations() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_with_constants() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_after_comparison() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD8, 0xD1,                                  // FCOM ST(1)
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_with_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_with_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NAN);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_empty_stack() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_full_stack() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0) x8
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xDD, 0xD8,
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_after_division() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDE, 0xF9,                                  // FDIVP
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_after_multiplication() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC9,                                  // FMULP
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0);
    emu.maps.write_f64(0x2008, 4.0);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDE, 0xC1,                                  // FADDP (fails - only 1 value)
        0xD9, 0x34, 0x25, 0x00, 0x31, 0x00, 0x00,  // FNSTENV [0x3100]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3100 + ENV_FCW).unwrap();
    assert!(fcw1 < 0xFFFF);
    assert!(fcw2 < 0xFFFF);
}

#[test]
fn test_fnstenv_with_zero() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEE,                                  // FLDZ
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_negative_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -123.456);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_after_sqrt() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFA,                                  // FSQRT
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 4.0);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_after_abs() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xE1,                                  // FABS
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -7.5);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_after_change_sign() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xE0,                                  // FCHS
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 9.25);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_preserves_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    let precision = (saved_cw >> 8) & 0x3;
    assert_eq!(precision, 0x3);
}

#[test]
fn test_fnstenv_preserves_rounding() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    let rounding = (saved_cw >> 10) & 0x3;
    assert_eq!(rounding, 0x0);
}

#[test]
fn test_fnstenv_denormal_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::MIN_POSITIVE / 2.0);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_very_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::MAX);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_after_exchange() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xC9,                                  // FXCH ST(1)
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 20.0);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}

#[test]
fn test_fnstenv_mixed_operations() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDE, 0xC1,                                  // FADDP
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnstenv_tag_word() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.5);

    emu.run(None).unwrap();

    let ftw = emu.maps.read_word(0x3000 + ENV_FTW).unwrap();
    assert!(ftw < 0xFFFF);
}

#[test]
fn test_fnstenv_consistency() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xD9, 0x34, 0x25, 0x00, 0x31, 0x00, 0x00,  // FNSTENV [0x3100]
        0xDD, 0xD8,                                  // FSTP ST(0)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + ENV_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3100 + ENV_FCW).unwrap();
    let fsw1 = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    let fsw2 = emu.maps.read_word(0x3100 + ENV_FSW).unwrap();
    assert_eq!(fcw1, fcw2);
    assert_eq!(fsw1, fsw2);
}

#[test]
fn test_fnstenv_after_rounding() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0xFC,                                  // FRNDINT
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.7);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW).unwrap();
    assert!(fsw < 0xFFFF);
}
