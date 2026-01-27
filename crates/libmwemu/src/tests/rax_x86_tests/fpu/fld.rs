//! Tests for the FLD instruction.
//!
//! FLD - Load Floating-Point Value
//!
//! Pushes the source operand onto the FPU register stack. The source operand can be in
//! single precision, double precision, or double extended-precision floating-point format.
//! If the source operand is in single precision or double precision floating-point format,
//! it is automatically converted to the double extended-precision floating-point format
//! before being pushed on the stack.
//!
//! The FLD instruction can also push the value in a selected FPU register [ST(i)] onto
//! the stack. Here, pushing register ST(0) duplicates the stack top.
//!
//! Reference: /Users/int/dev/rax/docs/fld.txt

use crate::*;

const DATA_ADDR: u64 = 0x2000;

// Helper to write f32 to memory
fn write_f32(mem: u64, addr: u64, value: f32) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

// Helper to write f64 to memory
fn write_f64(mem: u64, addr: u64, value: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

// Helper to read ST(0) as f64 from memory after FSTP
fn read_st0_as_f64(mem: u64, addr: u64) -> f64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FLD m32fp (opcode D9 /0) - Load 32-bit floating point from memory
// ============================================================================

#[test]
fn test_fld_m32fp_positive_one() {
    let mut emu = emu64();    // FLD dword ptr [0x2000]
    // FSTP qword ptr [0x3000]  ; Store to verify
    // HLT
    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fld_m32fp_zero() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, 0.0);
}

#[test]
fn test_fld_m32fp_negative_zero() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, -0.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result.is_sign_negative() && result == 0.0);
}

#[test]
fn test_fld_m32fp_negative_one() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, -1.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, -1.0);
}

#[test]
fn test_fld_m32fp_large_positive() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, 1234567.875);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!((result - 1234567.875).abs() < 1.0);
}

#[test]
fn test_fld_m32fp_large_negative() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, -9876543.5);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!((result + 9876543.5).abs() < 1.0);
}

#[test]
fn test_fld_m32fp_small_positive() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, 0.0000152587890625); // 2^-16

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!((result - 0.0000152587890625).abs() < 1e-10);
}

#[test]
fn test_fld_m32fp_small_negative() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, -0.0000152587890625);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!((result + 0.0000152587890625).abs() < 1e-10);
}

#[test]
fn test_fld_m32fp_infinity_positive() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, f32::INFINITY);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fld_m32fp_infinity_negative() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, f32::NEG_INFINITY);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fld_m32fp_nan() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, f32::NAN);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result.is_nan());
}

#[test]
fn test_fld_m32fp_pi() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, std::f32::consts::PI);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!((result - std::f64::consts::PI).abs() < 1e-6);
}

// ============================================================================
// FLD m64fp (opcode DD /0) - Load 64-bit floating point from memory
// ============================================================================

#[test]
fn test_fld_m64fp_positive_one() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]
    // FSTP qword ptr [0x3000]
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fld_m64fp_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, 0.0);
}

#[test]
fn test_fld_m64fp_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -0.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result.is_sign_negative() && result == 0.0);
}

#[test]
fn test_fld_m64fp_negative_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -1.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, -1.0);
}

#[test]
fn test_fld_m64fp_large_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.234567890123456e15);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, 1.234567890123456e15);
}

#[test]
fn test_fld_m64fp_large_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -9.876543210987654e15);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, -9.876543210987654e15);
}

#[test]
fn test_fld_m64fp_small_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.234567890123456e-15);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, 1.234567890123456e-15);
}

#[test]
fn test_fld_m64fp_small_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -9.876543210987654e-15);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, -9.876543210987654e-15);
}

#[test]
fn test_fld_m64fp_infinity_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::INFINITY);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fld_m64fp_infinity_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NEG_INFINITY);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fld_m64fp_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NAN);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result.is_nan());
}

#[test]
fn test_fld_m64fp_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, std::f64::consts::PI);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, std::f64::consts::PI);
}

#[test]
fn test_fld_m64fp_e() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, std::f64::consts::E);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, std::f64::consts::E);
}

// ============================================================================
// FLD ST(i) (opcode D9 C0+i) - Push FPU register onto stack
// ============================================================================

#[test]
fn test_fld_st0_duplicate_top() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 1.0 into ST(0)
    // FLD ST(0)               ; Duplicate ST(0)
    // FSTP qword ptr [0x3000] ; Store top (should be 1.0)
    // FSTP qword ptr [0x4000] ; Store next (should also be 1.0)
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0xC0, // FLD ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00, // FSTP qword ptr [0x4000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    let result1 = emu.fpu_mut().get_st(0);
    let result2 = emu.fpu_mut().get_st(0);
    assert_eq!(result1, 1.0);
    assert_eq!(result2, 1.0);
}

#[test]
fn test_fld_st1() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 1.0 into ST(0)
    // FLD qword ptr [0x2008]  ; Load 2.0 into ST(0), 1.0 is now ST(1)
    // FLD ST(1)               ; Push ST(1) (1.0) onto stack
    // FSTP qword ptr [0x3000] ; Store top (should be 1.0)
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xD9, 0xC1, // FLD ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fld_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010] ; 3.0
        0xD9, 0xC2, // FLD ST(2) ; Push 1.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, 1.0);
}

// ============================================================================
// Stack behavior tests
// ============================================================================

#[test]
fn test_fld_stack_push_behavior() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010] ; 3.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000] ; ST(0) = 3.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008] ; ST(0) = 2.0
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010] ; ST(0) = 1.0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.fpu_mut().get_st(0), 3.0);
    assert_eq!(emu.fpu_mut().get_st(0), 2.0);
    assert_eq!(emu.fpu_mut().get_st(0), 1.0);
}

#[test]
fn test_fld_multiple_formats() {
    let mut emu = emu64();    // FLD dword ptr [0x2000]  ; Load f32
    // FLD qword ptr [0x2008]  ; Load f64
    // FSTP qword ptr [0x3000] ; Store f64 value
    // FSTP qword ptr [0x3008] ; Store f32 value (converted to f64)
    // HLT
    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, 1.5);
    emu.maps.write_f64(DATA_ADDR + 8, 2.5);

    emu.run(None).unwrap();
    assert_eq!(emu.fpu_mut().get_st(0), 2.5);
    assert_eq!(emu.fpu_mut().get_st(0), 1.5);
}

// ============================================================================
// Special value tests
// ============================================================================

#[test]
fn test_fld_max_f32() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, f32::MAX);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!((result - f32::MAX as f64).abs() < 1e30);
}

#[test]
fn test_fld_min_positive_f32() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f32(DATA_ADDR, f32::MIN_POSITIVE);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!((result - f32::MIN_POSITIVE as f64).abs() < 1e-40);
}

#[test]
fn test_fld_max_f64() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::MAX);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, f64::MAX);
}

#[test]
fn test_fld_min_positive_f64() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::MIN_POSITIVE);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert_eq!(result, f64::MIN_POSITIVE);
}

#[test]
fn test_fld_various_fractions() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];

    let test_values = vec![0.5, 0.25, 0.125, 0.75, 0.375, 0.625];
    for (i, &val) in test_values.iter().enumerate() {
        emu.load_code_bytes(&code);
        emu.maps.write_f64(DATA_ADDR, val);

    emu.run(None).unwrap();
        let result = emu.fpu_mut().get_st(0);
        assert_eq!(result, val, "Test value {} failed", i);
    }
}

#[test]
fn test_fld_denormal_f32() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let denormal = f32::MIN_POSITIVE / 2.0_f32.powi(23);
    emu.maps.write_f32(DATA_ADDR, denormal);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result > 0.0 && result < f32::MIN_POSITIVE as f64);
}

#[test]
fn test_fld_denormal_f64() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let denormal = f64::MIN_POSITIVE / 2.0_f64.powi(52);
    emu.maps.write_f64(DATA_ADDR, denormal);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result > 0.0 && result < f64::MIN_POSITIVE);
}

#[test]
fn test_fld_m32fp_subnormal() {
    let mut emu = emu64();    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let subnormal = f32::from_bits(0x00000001); // Smallest positive subnormal
    emu.maps.write_f32(DATA_ADDR, subnormal);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result > 0.0 && result < f32::MIN_POSITIVE as f64);
}

#[test]
fn test_fld_m64fp_subnormal() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let subnormal = f64::from_bits(0x0000000000000001); // Smallest positive subnormal
    emu.maps.write_f64(DATA_ADDR, subnormal);

    emu.run(None).unwrap();
    let result = emu.fpu_mut().get_st(0);
    assert!(result > 0.0 && result < f64::MIN_POSITIVE);
}

#[test]
fn test_fld_st_all_registers() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD 4.0
        0xD9, 0xC3, // FLD ST(3) ; Push 1.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);
    emu.maps.write_f64(DATA_ADDR + 24, 4.0);

    emu.run(None).unwrap();
    assert_eq!(emu.fpu_mut().get_st(0), 1.0);
}
