//! Tests for the FST and FSTP instructions.
//!
//! FST - Store Floating-Point Value (without pop)
//! FSTP - Store Floating-Point Value and Pop
//!
//! The FST instruction copies the value in the ST(0) register to the destination operand,
//! which can be a memory location or another register in the FPU register stack.
//!
//! The FSTP instruction performs the same operation as the FST instruction and then pops
//! the register stack. The FSTP instruction can also store values in memory in double
//! extended-precision floating-point format.
//!
//! Reference: /Users/int/dev/rax/docs/fst:fstp.txt

use crate::*;

const DATA_ADDR: u64 = 0x2000;

// Helper to write f64 to memory
fn write_f64(mem: u64, addr: u64, value: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

// Helper to read f32 from memory
fn read_f32(mem: u64, addr: u64) -> f32 {
    let mut emu = emu64();    let mut buf = [0u8; 4];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f32::from_le_bytes(buf)
}

// Helper to read f64 from memory
fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FST m32fp (opcode D9 /2) - Store 32-bit float without pop
// ============================================================================

#[test]
fn test_fst_m32fp_positive_one() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 1.0
    // FST dword ptr [0x3000]  ; Store as f32
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fst_m32fp_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

#[test]
fn test_fst_m32fp_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert!(result.is_sign_negative() && result == 0.0);
}

#[test]
fn test_fst_m32fp_negative_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -1.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert_eq!(result, -1.0);
}

#[test]
fn test_fst_m32fp_infinity_positive() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::INFINITY);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fst_m32fp_infinity_negative() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NEG_INFINITY);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fst_m32fp_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NAN);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert!(result.is_nan());
}

#[test]
fn test_fst_m32fp_no_pop() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 1.0
    // FST dword ptr [0x3000]  ; Store (no pop)
    // FST dword ptr [0x3004]  ; Store again (should still be 1.0)
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xD9, 0x14, 0x25, 0x04, 0x30, 0x00, 0x00, // FST dword ptr [0x3004]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f32(0x3000).unwrap(), 1.0);
    assert_eq!(emu.maps.read_f32(0x3004).unwrap(), 1.0);
}

// ============================================================================
// FST m64fp (opcode DD /2) - Store 64-bit float without pop
// ============================================================================

#[test]
fn test_fst_m64fp_positive_one() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 1.0
    // FST qword ptr [0x3000]  ; Store as f64
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fst_m64fp_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

#[test]
fn test_fst_m64fp_negative_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result.is_sign_negative() && result == 0.0);
}

#[test]
fn test_fst_m64fp_pi() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, std::f64::consts::PI);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, std::f64::consts::PI);
}

#[test]
fn test_fst_m64fp_no_pop() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xDD, 0x14, 0x25, 0x08, 0x30, 0x00, 0x00, // FST qword ptr [0x3008]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, std::f64::consts::E);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), std::f64::consts::E);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), std::f64::consts::E);
}

// ============================================================================
// FSTP m32fp (opcode D9 /3) - Store 32-bit float and pop
// ============================================================================

#[test]
fn test_fstp_m32fp_positive_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fstp_m32fp_large_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 123456.78);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert!((result - 123456.78).abs() < 0.01);
}

#[test]
fn test_fstp_m32fp_with_pop() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 1.0
    // FLD qword ptr [0x2008]  ; Load 2.0 (now ST(0))
    // FSTP dword ptr [0x3000] ; Store 2.0 and pop
    // FSTP dword ptr [0x3004] ; Store 1.0 and pop
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xD9, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3004]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f32(0x3000).unwrap(), 2.0);
    assert_eq!(emu.maps.read_f32(0x3004).unwrap(), 1.0);
}

// ============================================================================
// FSTP m64fp (opcode DD /3) - Store 64-bit float and pop
// ============================================================================

#[test]
fn test_fstp_m64fp_positive_one() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0);
}

#[test]
fn test_fstp_m64fp_zero() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0);
}

#[test]
fn test_fstp_m64fp_negative_value() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, -12345.6789);

    emu.run(None).unwrap();
    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, -12345.6789);
}

#[test]
fn test_fstp_m64fp_with_pop() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 2.0);
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 1.0);
}

// ============================================================================
// FST ST(i) (opcode DD D0+i) - Store to register without pop
// ============================================================================

#[test]
fn test_fst_st1() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 1.0 into ST(0)
    // FLD qword ptr [0x2008]  ; Load 2.0 into ST(0), 1.0 -> ST(1)
    // FST ST(1)               ; Copy ST(0) to ST(1) (both should be 2.0)
    // FSTP qword ptr [0x3000] ; Pop ST(0) (2.0)
    // FSTP qword ptr [0x3008] ; Pop ST(0) (was ST(1), now 2.0)
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0xD1, // FST ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 2.0);
}

#[test]
fn test_fst_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010] ; 3.0
        0xDD, 0xD2, // FST ST(2) ; Copy 3.0 to ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000] ; 3.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008] ; 2.0
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010] ; 3.0 (was ST(2))
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 3.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 2.0);
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 3.0);
}

// ============================================================================
// FSTP ST(i) (opcode DD D8+i) - Store to register and pop
// ============================================================================

#[test]
fn test_fstp_st1() {
    let mut emu = emu64();    // FLD qword ptr [0x2000]  ; Load 1.0 into ST(0)
    // FLD qword ptr [0x2008]  ; Load 2.0 into ST(0), 1.0 -> ST(1)
    // FSTP ST(1)              ; Copy ST(0) to ST(1) and pop
    // FSTP qword ptr [0x3000] ; Pop remaining value
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0xD9, // FSTP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0);
}

#[test]
fn test_fstp_st2() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010] ; 3.0
        0xDD, 0xDA, // FSTP ST(2) ; Copy 3.0 to ST(2) and pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000] ; 2.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008] ; 3.0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);
    emu.maps.write_f64(DATA_ADDR + 16, 3.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), 2.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 3.0);
}

// ============================================================================
// Precision and rounding tests
// ============================================================================

#[test]
fn test_fst_m32fp_precision_loss() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.2345678901234567);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    // f32 should round/truncate the value
    assert!((result - 1.234568).abs() < 0.0001);
}

#[test]
fn test_fstp_m32fp_very_small() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e-40);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert!(result > 0.0 && result < 1.0e-38);
}

#[test]
fn test_fstp_m32fp_very_large() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0e38);

    emu.run(None).unwrap();
    let result = emu.maps.read_f32(0x3000).unwrap();
    assert!(result > 1.0e37);
}

// ============================================================================
// Mixed format tests
// ============================================================================

#[test]
fn test_mixed_fst_fstp() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008] ; 2.0
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]  ; Store 2.0, no pop
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008] ; Store 2.0, pop
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010] ; Store 1.0, pop
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 1.0);
    emu.maps.write_f64(DATA_ADDR + 8, 2.0);

    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f32(0x3000).unwrap(), 2.0);
    assert_eq!(emu.maps.read_f64(0x3008).unwrap(), 2.0);
    assert_eq!(emu.maps.read_f64(0x3010).unwrap(), 1.0);
}

#[test]
fn test_fst_multiple_formats() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xDD, 0x14, 0x25, 0x08, 0x30, 0x00, 0x00, // FST qword ptr [0x3008]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, std::f64::consts::PI);

    emu.run(None).unwrap();
    let result_f32 = emu.maps.read_f32(0x3000).unwrap();
    let result_f64 = emu.maps.read_f64(0x3008).unwrap();
    assert!((result_f32 as f64 - std::f64::consts::PI).abs() < 1e-6);
    assert_eq!(result_f64, std::f64::consts::PI);
}

// ============================================================================
// Special value tests
// ============================================================================

#[test]
fn test_fstp_special_values_sequence() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, 0.0);
    emu.maps.write_f64(DATA_ADDR + 8, -0.0);
    emu.maps.write_f64(DATA_ADDR + 16, f64::INFINITY);

    emu.run(None).unwrap();
    let r1 = emu.maps.read_f64(0x3000).unwrap();
    let r2 = emu.maps.read_f64(0x3008).unwrap();
    let r3 = emu.maps.read_f64(0x3010).unwrap();
    assert!(r1.is_infinite() && r1.is_sign_positive());
    assert!(r2.is_sign_negative() && r2 == 0.0);
    assert_eq!(r3, 0.0);
}

#[test]
fn test_fst_preserves_nan() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xDD, 0x14, 0x25, 0x08, 0x30, 0x00, 0x00, // FST qword ptr [0x3008]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::NAN);

    emu.run(None).unwrap();
    assert!(emu.maps.read_f64(0x3000).unwrap().is_nan());
    assert!(emu.maps.read_f64(0x3008).unwrap().is_nan());
}

#[test]
fn test_fstp_extreme_values() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::MAX);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), f64::MAX);

    emu.load_code_bytes(&code);
    emu.maps.write_f64(DATA_ADDR, f64::MIN_POSITIVE);
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_f64(0x3000).unwrap(), f64::MIN_POSITIVE);
}
