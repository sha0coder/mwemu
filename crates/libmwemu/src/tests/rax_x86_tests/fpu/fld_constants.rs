//! Tests for the FLD constant loading instructions.
//!
//! FLD1/FLDL2T/FLDL2E/FLDPI/FLDLG2/FLDLN2/FLDZ - Load Constant
//!
//! These instructions push one of seven commonly used constants (in double
//! extended-precision floating-point format) onto the FPU register stack.
//!
//! The constants are:
//! - FLD1: +1.0
//! - FLDZ: +0.0
//! - FLDPI: π
//! - FLDL2E: log₂(e)
//! - FLDL2T: log₂(10)
//! - FLDLG2: log₁₀(2)
//! - FLDLN2: ln(2) = logₑ(2)
//!
//! Opcodes:
//! - FLD1: D9 E8
//! - FLDL2T: D9 E9
//! - FLDL2E: D9 EA
//! - FLDPI: D9 EB
//! - FLDLG2: D9 EC
//! - FLDLN2: D9 ED
//! - FLDZ: D9 EE
//!
//! Flags affected:
//! - C1: Set to 1 if stack overflow occurred; otherwise, set to 0
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fld1:fldl2t:fldl2e:fldpi:fldlg2:fldln2:fldz.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Helper function to read f64 from memory
fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FLD1 - Load +1.0
// ============================================================================

#[test]
fn test_fld1_basic() {
    let mut emu = emu64();    // FLD1                ; D9 E8
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "FLD1 should load 1.0");
}

#[test]
fn test_fld1_multiple() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 1.0, "First FLD1 should load 1.0");
    assert_eq!(result2, 1.0, "Second FLD1 should load 1.0");
}

#[test]
fn test_fld1_arithmetic() {
    let mut emu = emu64();    // FLD1 + FLD1 = 2.0
    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "FLD1 + FLD1 should equal 2.0");
}

#[test]
fn test_fld1_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result.to_bits(), 1.0_f64.to_bits(), "FLD1 should be exact");
}

// ============================================================================
// FLDZ - Load +0.0
// ============================================================================

#[test]
fn test_fldz_basic() {
    let mut emu = emu64();    // FLDZ                ; D9 EE
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "FLDZ should load 0.0");
    assert!(!result.is_sign_negative(), "FLDZ should load positive zero");
}

#[test]
fn test_fldz_multiple() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 0.0, "First FLDZ should load 0.0");
    assert_eq!(result2, 0.0, "Second FLDZ should load 0.0");
}

#[test]
fn test_fldz_arithmetic() {
    let mut emu = emu64();    // FLDZ + FLD1 = 1.0
    let code = [
        0xD9, 0xEE,                                  // FLDZ
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "FLDZ + FLD1 should equal 1.0");
}

#[test]
fn test_fldz_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result.to_bits(), 0.0_f64.to_bits(), "FLDZ should be exact positive zero");
}

// ============================================================================
// FLDPI - Load π
// ============================================================================

#[test]
fn test_fldpi_basic() {
    let mut emu = emu64();    // FLDPI               ; D9 EB
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::PI;
    assert!((result - expected).abs() < 1e-15, "FLDPI should load π accurately");
}

#[test]
fn test_fldpi_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    // PI should be very close to standard library value
    assert!((result - std::f64::consts::PI).abs() < 1e-15,
        "FLDPI precision check: got {}, expected {}", result, std::f64::consts::PI);
}

#[test]
fn test_fldpi_range() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result > 3.14159 && result < 3.14160, "FLDPI should be approximately 3.14159");
}

#[test]
fn test_fldpi_arithmetic() {
    let mut emu = emu64();    // 2 * π
    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP (1 + 1 = 2)
        0xDE, 0xC9,                                  // FMULP (π * 2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0 * std::f64::consts::PI;
    assert!((result - expected).abs() < 1e-14, "2π calculation");
}

// ============================================================================
// FLDL2E - Load log₂(e)
// ============================================================================

#[test]
fn test_fldl2e_basic() {
    let mut emu = emu64();    // FLDL2E              ; D9 EA
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xD9, 0xEA,                                  // FLDL2E
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::LOG2_E;
    assert!((result - expected).abs() < 1e-15, "FLDL2E should load log₂(e)");
}

#[test]
fn test_fldl2e_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEA,                                  // FLDL2E
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - std::f64::consts::LOG2_E).abs() < 1e-15,
        "FLDL2E precision: got {}, expected {}", result, std::f64::consts::LOG2_E);
}

#[test]
fn test_fldl2e_range() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEA,                                  // FLDL2E
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result > 1.442 && result < 1.443, "FLDL2E should be approximately 1.4427");
}

// ============================================================================
// FLDL2T - Load log₂(10)
// ============================================================================

#[test]
fn test_fldl2t_basic() {
    let mut emu = emu64();    // FLDL2T              ; D9 E9
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xD9, 0xE9,                                  // FLDL2T
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::LOG2_10;
    assert!((result - expected).abs() < 1e-15, "FLDL2T should load log₂(10)");
}

#[test]
fn test_fldl2t_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE9,                                  // FLDL2T
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - std::f64::consts::LOG2_10).abs() < 1e-15,
        "FLDL2T precision: got {}, expected {}", result, std::f64::consts::LOG2_10);
}

#[test]
fn test_fldl2t_range() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE9,                                  // FLDL2T
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result > 3.321 && result < 3.322, "FLDL2T should be approximately 3.3219");
}

// ============================================================================
// FLDLG2 - Load log₁₀(2)
// ============================================================================

#[test]
fn test_fldlg2_basic() {
    let mut emu = emu64();    // FLDLG2              ; D9 EC
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xD9, 0xEC,                                  // FLDLG2
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::LOG10_2;
    assert!((result - expected).abs() < 1e-15, "FLDLG2 should load log₁₀(2)");
}

#[test]
fn test_fldlg2_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEC,                                  // FLDLG2
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - std::f64::consts::LOG10_2).abs() < 1e-15,
        "FLDLG2 precision: got {}, expected {}", result, std::f64::consts::LOG10_2);
}

#[test]
fn test_fldlg2_range() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEC,                                  // FLDLG2
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result > 0.301 && result < 0.302, "FLDLG2 should be approximately 0.30103");
}

#[test]
fn test_fldlg2_fldl2t_reciprocal() {
    let mut emu = emu64();    // log₁₀(2) * log₂(10) should equal 1
    let code = [
        0xD9, 0xEC,                                  // FLDLG2
        0xD9, 0xE9,                                  // FLDL2T
        0xDE, 0xC9,                                  // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - 1.0).abs() < 1e-14, "log₁₀(2) * log₂(10) should equal 1");
}

// ============================================================================
// FLDLN2 - Load ln(2)
// ============================================================================

#[test]
fn test_fldln2_basic() {
    let mut emu = emu64();    // FLDLN2              ; D9 ED
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xD9, 0xED,                                  // FLDLN2
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::LN_2;
    assert!((result - expected).abs() < 1e-15, "FLDLN2 should load ln(2)");
}

#[test]
fn test_fldln2_precision() {
    let mut emu = emu64();    let code = [
        0xD9, 0xED,                                  // FLDLN2
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!((result - std::f64::consts::LN_2).abs() < 1e-15,
        "FLDLN2 precision: got {}, expected {}", result, std::f64::consts::LN_2);
}

#[test]
fn test_fldln2_range() {
    let mut emu = emu64();    let code = [
        0xD9, 0xED,                                  // FLDLN2
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert!(result > 0.693 && result < 0.694, "FLDLN2 should be approximately 0.69315");
}

// ============================================================================
// Combined Constant Tests
// ============================================================================

#[test]
fn test_all_constants_loaded() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xD9, 0xEE,                                  // FLDZ
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xD9, 0xEB,                                  // FLDPI
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xD9, 0xEA,                                  // FLDL2E
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00,  // FSTP qword [0x3018]
        0xD9, 0xE9,                                  // FLDL2T
        0xDD, 0x1C, 0x25, 0x20, 0x30, 0x00, 0x00,  // FSTP qword [0x3020]
        0xD9, 0xEC,                                  // FLDLG2
        0xDD, 0x1C, 0x25, 0x28, 0x30, 0x00, 0x00,  // FSTP qword [0x3028]
        0xD9, 0xED,                                  // FLDLN2
        0xDD, 0x1C, 0x25, 0x30, 0x30, 0x00, 0x00,  // FSTP qword [0x3030]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let fld1 = emu.maps.read_f64(0x3000).unwrap();
    let fldz = emu.maps.read_f64(0x3008).unwrap();
    let fldpi = emu.maps.read_f64(0x3010).unwrap();
    let fldl2e = emu.maps.read_f64(0x3018).unwrap();
    let fldl2t = emu.maps.read_f64(0x3020).unwrap();
    let fldlg2 = emu.maps.read_f64(0x3028).unwrap();
    let fldln2 = emu.maps.read_f64(0x3030).unwrap();

    assert_eq!(fld1, 1.0);
    assert_eq!(fldz, 0.0);
    assert!((fldpi - std::f64::consts::PI).abs() < 1e-15);
    assert!((fldl2e - std::f64::consts::LOG2_E).abs() < 1e-15);
    assert!((fldl2t - std::f64::consts::LOG2_10).abs() < 1e-15);
    assert!((fldlg2 - std::f64::consts::LOG10_2).abs() < 1e-15);
    assert!((fldln2 - std::f64::consts::LN_2).abs() < 1e-15);
}

#[test]
fn test_constant_stack_operations() {
    let mut emu = emu64();    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xEB,                                  // FLDPI
        0xD9, 0xEA,                                  // FLDL2E
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000] (L2E)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008] (PI)
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010] (1)
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let l2e = emu.maps.read_f64(0x3000).unwrap();
    let pi = emu.maps.read_f64(0x3008).unwrap();
    let one = emu.maps.read_f64(0x3010).unwrap();

    assert!((l2e - std::f64::consts::LOG2_E).abs() < 1e-15);
    assert!((pi - std::f64::consts::PI).abs() < 1e-15);
    assert_eq!(one, 1.0);
}

#[test]
fn test_pi_circle_circumference() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP (1 + π, but we want 2)
        0xD9, 0xEE,                                  // FLDZ
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP (1 + 1)
        0xD9, 0xE0,                                  // FCHS (negate to clear stack)
        0xDD, 0xD8,                                  // FSTP ST(0) (pop)
        // Restart: 2 * π
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP (2)
        0xD9, 0xEB,                                  // FLDPI
        0xDE, 0xC9,                                  // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = 2.0 * std::f64::consts::PI;
    assert!((result - expected).abs() < 1e-14, "2π calculation from constants");
}

#[test]
fn test_e_from_constants() {
    let mut emu = emu64();    let code = [
        0xD9, 0xED,                                  // FLDLN2 (ln(2))
        0xD9, 0xEA,                                  // FLDL2E (log₂(e))
        0xDE, 0xC9,                                  // FMULP (ln(2) * log₂(e))
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    // ln(2) * log₂(e) = ln(e) = 1
    assert!((result - 1.0).abs() < 1e-14, "ln(2) * log₂(e) should equal 1");
}

#[test]
fn test_constant_combinations() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEA,                                  // FLDL2E
        0xD9, 0xED,                                  // FLDLN2
        0xDE, 0xF9,                                  // FDIVP (log₂(e) / ln(2))
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    // log₂(e) / ln(2) = 1 / ln(2) * log₂(e) = log₂(e) / ln(2)
    let expected = std::f64::consts::LOG2_E / std::f64::consts::LN_2;
    assert!((result - expected).abs() < 1e-14, "log₂(e) / ln(2)");
}

#[test]
fn test_fld1_fldz_subtraction() {
    let mut emu = emu64();    // 1 - 0 = 1
    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xEE,                                  // FLDZ
        0xDE, 0xE9,                                  // FSUBP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "1 - 0 should equal 1");
}

#[test]
fn test_fldpi_divided_by_2() {
    let mut emu = emu64();    // π / 2
    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC1,                                  // FADDP (2)
        0xDE, 0xF9,                                  // FDIVP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    let expected = std::f64::consts::PI / 2.0;
    assert!((result - expected).abs() < 1e-14, "π / 2 calculation");
}

#[test]
fn test_constant_multiply_by_zero() {
    let mut emu = emu64();    // π * 0 = 0
    let code = [
        0xD9, 0xEB,                                  // FLDPI
        0xD9, 0xEE,                                  // FLDZ
        0xDE, 0xC9,                                  // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 0.0, "π * 0 should equal 0");
}

#[test]
fn test_fld1_squared() {
    let mut emu = emu64();    // 1 * 1 = 1
    let code = [
        0xD9, 0xE8,                                  // FLD1
        0xD9, 0xE8,                                  // FLD1
        0xDE, 0xC9,                                  // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.0, "1 * 1 should equal 1");
}

#[test]
fn test_all_logs_positive() {
    let mut emu = emu64();    let code = [
        0xD9, 0xEA,                                  // FLDL2E
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xD9, 0xE9,                                  // FLDL2T
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xD9, 0xEC,                                  // FLDLG2
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xD9, 0xED,                                  // FLDLN2
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00,  // FSTP qword [0x3018]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let l2e = emu.maps.read_f64(0x3000).unwrap();
    let l2t = emu.maps.read_f64(0x3008).unwrap();
    let lg2 = emu.maps.read_f64(0x3010).unwrap();
    let ln2 = emu.maps.read_f64(0x3018).unwrap();

    assert!(l2e > 0.0, "log₂(e) should be positive");
    assert!(l2t > 0.0, "log₂(10) should be positive");
    assert!(lg2 > 0.0, "log₁₀(2) should be positive");
    assert!(ln2 > 0.0, "ln(2) should be positive");
}
