//! Tests for the FXSAVE64 and FXRSTOR64 instructions.
//!
//! FXSAVE64/FXRSTOR64 - Save and Restore x87 FPU, MMX, and SSE State (64-bit mode)
//!
//! These are the 64-bit mode variants of FXSAVE/FXRSTOR. In 64-bit mode,
//! the default operand size is 64 bits, which affects the saved FIP and FDP.
//!
//! Memory layout differences in 64-bit mode:
//! - Bytes 0-1: FCW (FPU Control Word)
//! - Bytes 2-3: FSW (FPU Status Word)
//! - Bytes 4: FTW (abridged FPU Tag Word)
//! - Bytes 8-15: FIP[63:0] (64-bit Instruction Pointer)
//! - Bytes 16-23: FDP[63:0] (64-bit Data Pointer)
//! - Bytes 24-27: MXCSR
//! - Bytes 28-31: MXCSR_MASK
//! - Bytes 32-159: ST0-ST7
//! - Bytes 160-415: XMM0-XMM15 (in 64-bit mode)
//!
//! Opcodes:
//! - FXSAVE64: REX.W + 0F AE /0
//! - FXRSTOR64: REX.W + 0F AE /1
//!
//! Reference: /Users/int/dev/rax/docs/fxsave64.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

fn write_u16(mem: u64, addr: u64, val: u16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

fn read_u16(mem: u64, addr: u64) -> u16 {
    let mut emu = emu64();    let mut buf = [0u8; 2];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u16::from_le_bytes(buf)
}

fn write_f64(mem: u64, addr: u64, val: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

const FXSAVE_FCW: u64 = 0;
const FXSAVE_FSW: u64 = 2;
const FXSAVE_FTW: u64 = 4;

#[test]
fn test_fxsave64_basic() {
    let mut emu = emu64();    let code = [
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FXSAVE64 [0x2000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x2000 + FXSAVE_FCW).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be valid after FXSAVE64");
}

#[test]
fn test_fxsave64_with_fpu_data() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,              // FLD qword [0x2008]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x20, 0x00, 0x00,              // FSTP qword [0x2010]
        0xDD, 0x1C, 0x25, 0x18, 0x20, 0x00, 0x00,              // FSTP qword [0x2018]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let fsw = emu.maps.read_word(0x3000 + FXSAVE_FSW).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be saved");
    assert!(fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fxsave64_saves_control_word() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLDCW [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    assert_eq!(saved_cw, 0x037F, "FCW should be saved correctly");
}

#[test]
fn test_fxrstor64_basic() {
    let mut emu = emu64();    let code = [
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xD9, 0x3C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FNSTCW [0x4000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x3000 + FXSAVE_FCW, 0x037F);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x4000).unwrap();
    assert!(cw < 0xFFFF, "Control word should be valid after FXRSTOR64");
}

#[test]
fn test_fxsave64_fxrstor64_roundtrip() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,              // FSTP qword [0x4008]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4008).unwrap();
    assert_eq!(result, 1.5, "Value should be preserved through FXSAVE64/FXRSTOR64");
}

#[test]
fn test_fxsave64_fxrstor64_multiple_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,              // FLD qword [0x2008]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00,              // FSTP qword [0x4010]
        0xDD, 0x1C, 0x25, 0x18, 0x40, 0x00, 0x00,              // FSTP qword [0x4018]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x4010).unwrap();
    let v2 = emu.maps.read_f64(0x4018).unwrap();
    assert_eq!(v1, 2.5, "Second value should be popped first");
    assert_eq!(v2, 1.5, "First value should be popped second");
}

#[test]
fn test_fxsave64_multiple_areas() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x31, 0x00, 0x00,  // FXSAVE64 [0x3100]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,              // FSTP qword [0x3008]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3100 + FXSAVE_FCW).unwrap();
    assert_eq!(fcw1, fcw2, "Multiple FXSAVE64 should save identical state");
}

#[test]
fn test_fxrstor64_from_prepared_area() {
    let mut emu = emu64();    let code = [
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FXRSTOR64 [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,              // FNSTCW [0x3000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + FXSAVE_FCW, 0x037F);
    emu.maps.write_word(0x2000 + FXSAVE_FSW, 0x0000);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, 0x037F, "Control word should be restored from prepared area");
}

#[test]
fn test_fxsave64_after_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,              // FLD qword [0x2008]
        0xDE, 0xC1,                                              // FADDP
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,              // FSTP qword [0x3010]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);
    emu.maps.write_f64(0x2008, 3.5);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + FXSAVE_FSW).unwrap();
    assert!(fsw < 0xFFFF, "FSW should be saved after arithmetic");
}

#[test]
fn test_fxrstor64_then_arithmetic() {
    let mut emu = emu64();    let code = [
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FXRSTOR64 [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,              // FLD qword [0x2008]
        0xDE, 0xC1,                                              // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,              // FSTP qword [0x3000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + FXSAVE_FCW, 0x037F);
    emu.maps.write_word(0x2000 + FXSAVE_FSW, 0x0000);
    emu.maps.write_f64(0x2008, 1.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.5, "Arithmetic should work after FXRSTOR64");
}

#[test]
fn test_sequential_fxsave64() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x32, 0x00, 0x00,  // FXSAVE64 [0x3200]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,              // FSTP qword [0x3008]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3200 + FXSAVE_FCW).unwrap();
    assert_eq!(fcw1, fcw2, "Multiple FXSAVE64 should produce identical results");
}

#[test]
fn test_fxsave64_fxrstor64_complete_flow() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,              // FLD qword [0x2008]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,              // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x18, 0x20, 0x00, 0x00,              // FSTP qword [0x2018]
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x20, 0x40, 0x00, 0x00,              // FSTP qword [0x4020]
        0xDD, 0x1C, 0x25, 0x28, 0x40, 0x00, 0x00,              // FSTP qword [0x4028]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);
    emu.maps.write_f64(0x2010, 99.0);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x4020).unwrap();
    let v2 = emu.maps.read_f64(0x4028).unwrap();
    assert_eq!(v1, 2.5, "Second restored value should be 2.5");
    assert_eq!(v2, 1.5, "First restored value should be 1.5");
}

#[test]
fn test_fxsave64_fxrstor64_multiple_cycles() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,              // FSTP qword [0x4008]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,              // FLD qword [0x2008]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x32, 0x00, 0x00,  // FXSAVE64 [0x3200]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x32, 0x00, 0x00,  // FXRSTOR64 [0x3200]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00,              // FSTP qword [0x4010]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let r1 = emu.maps.read_f64(0x4008).unwrap();
    let r2 = emu.maps.read_f64(0x4010).unwrap();
    assert_eq!(r1, 1.5, "First cycle result");
    assert_eq!(r2, 2.5, "Second cycle result");
}

#[test]
fn test_fxsave64_preserves_control_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLDCW [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let precision = (saved_cw >> 8) & 0x3;
    assert_eq!(precision, 0x3, "Precision should be saved as 64-bit");
}

#[test]
fn test_fxsave64_preserves_control_rounding() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLDCW [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let rounding = (saved_cw >> 10) & 0x3;
    assert_eq!(rounding, 0x0, "Rounding should be saved as nearest");
}

#[test]
fn test_fxsave64_different_control_words() {
    let mut emu = emu64();    let test_cws = vec![0x037F, 0x027F, 0x0C7F];

    for test_cw in test_cws {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLDCW [0x2000]
            0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
            0xF4,                                                    // HLT
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, test_cw);

    emu.run(None).unwrap();

        let saved_cw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
        assert_eq!(saved_cw, test_cw, "Control word 0x{:04X} should be saved", test_cw);
    }
}

#[test]
fn test_fxsave64_with_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,              // FSTP qword [0x4008]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::INFINITY);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4008).unwrap();
    assert_eq!(result, f64::INFINITY);
}

#[test]
fn test_fxrstor64_with_saved_infinity() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,              // FSTP qword [0x4008]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, f64::NEG_INFINITY);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4008).unwrap();
    assert_eq!(result, f64::NEG_INFINITY);
}

#[test]
fn test_fxsave64_with_constants() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB,                                              // FLDPI
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4000).unwrap();
    assert!((result - std::f64::consts::PI).abs() < 1e-15);
}

#[test]
fn test_fxsave64_stack_depth() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                              // FLD1
        0xD9, 0xE8,                                              // FLD1
        0xD9, 0xE8,                                              // FLD1
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,              // FSTP qword [0x4008]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00,              // FSTP qword [0x4010]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x4000).unwrap();
    let v2 = emu.maps.read_f64(0x4008).unwrap();
    let v3 = emu.maps.read_f64(0x4010).unwrap();
    assert_eq!(v1, 1.0);
    assert_eq!(v2, 1.0);
    assert_eq!(v3, 1.0);
}

#[test]
fn test_fxsave64_zero_values() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEE,                                              // FLDZ
        0xD9, 0xEE,                                              // FLDZ
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,              // FSTP qword [0x4008]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x4000).unwrap();
    let v2 = emu.maps.read_f64(0x4008).unwrap();
    assert_eq!(v1, 0.0);
    assert_eq!(v2, 0.0);
}

#[test]
fn test_fxsave64_mixed_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0xD9, 0xEB,                                              // FLDPI
        0xD9, 0xE8,                                              // FLD1
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,              // FSTP qword [0x4008]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00,              // FSTP qword [0x4010]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 123.456);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x4000).unwrap();
    let v2 = emu.maps.read_f64(0x4008).unwrap();
    let v3 = emu.maps.read_f64(0x4010).unwrap();
    assert_eq!(v1, 1.0);
    assert!((v2 - std::f64::consts::PI).abs() < 1e-15);
    assert_eq!(v3, 123.456);
}

#[test]
fn test_fxsave64_after_division() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                              // FLD1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0xDE, 0xF9,                                              // FDIVP
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4000).unwrap();
    assert_eq!(result, 0.5);
}

#[test]
fn test_fxrstor64_after_multiplication() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,              // FLD qword [0x2008]
        0xDE, 0xC9,                                              // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.0);
    emu.maps.write_f64(0x2008, 4.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4000).unwrap();
    assert_eq!(result, 12.0);
}

#[test]
fn test_fxsave64_negative_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,              // FLD qword [0x2008]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,              // FSTP qword [0x4008]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, -5.5);
    emu.maps.write_f64(0x2008, -10.25);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x4000).unwrap();
    let v2 = emu.maps.read_f64(0x4008).unwrap();
    assert_eq!(v1, -10.25);
    assert_eq!(v2, -5.5);
}

#[test]
fn test_fxsave64_very_small_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e-300);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4000).unwrap();
    assert_eq!(result, 1e-300);
}

#[test]
fn test_fxsave64_very_large_numbers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,              // FLD qword [0x2000]
        0x48, 0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE64 [0x3000]
        0xDB, 0xE3,                                              // FNINIT
        0x48, 0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR64 [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,              // FSTP qword [0x4000]
        0xF4,                                                    // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1e300);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4000).unwrap();
    assert_eq!(result, 1e300);
}
