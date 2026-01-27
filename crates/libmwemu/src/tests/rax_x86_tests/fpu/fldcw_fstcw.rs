//! Tests for the FLDCW, FSTCW, and FNSTCW instructions.
//!
//! FLDCW - Load x87 FPU Control Word
//! FSTCW - Store x87 FPU Control Word (with exception check)
//! FNSTCW - Store x87 FPU Control Word (without exception check)
//!
//! FLDCW loads the 16-bit source operand into the FPU control word.
//! The control word controls precision, rounding mode, and exception masks.
//!
//! FSTCW stores the current FPU control word to memory after checking for pending exceptions.
//! FNSTCW stores the control word without checking for exceptions.
//!
//! Opcodes:
//! - FLDCW: D9 /5
//! - FSTCW: 9B D9 /7
//! - FNSTCW: D9 /7
//!
//! Control Word Format (16 bits):
//! - Bits 0-5: Exception masks (IM, DM, ZM, OM, UM, PM)
//! - Bits 8-9: Precision control (00=single, 10=double, 11=extended)
//! - Bits 10-11: Rounding control (00=nearest, 01=down, 10=up, 11=toward zero)
//! - Bit 12: Infinity control (deprecated, should be 0)
//!
//! Flags affected:
//! - C0, C1, C2, C3: Undefined
//!
//! References: /Users/int/dev/rax/docs/fldcw.txt, /Users/int/dev/rax/docs/fstcw:fnstcw.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Helper function to write u16 to memory
fn write_u16(mem: u64, addr: u64, val: u16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// Helper function to read u16 from memory
fn read_u16(mem: u64, addr: u64) -> u16 {
    let emu = emu64();    let mut buf = [0u8; 2];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u16::from_le_bytes(buf)
}

// Helper function to write f64 to memory
fn write_f64(mem: u64, addr: u64, val: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// Helper function to read f64 from memory
fn read_f64(mem: u64, addr: u64) -> f64 {
    let emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// Control word bit definitions
const CW_MASK_INVALID: u16 = 0x0001;
const CW_MASK_DENORMAL: u16 = 0x0002;
const CW_MASK_ZERODIVIDE: u16 = 0x0004;
const CW_MASK_OVERFLOW: u16 = 0x0008;
const CW_MASK_UNDERFLOW: u16 = 0x0010;
const CW_MASK_PRECISION: u16 = 0x0020;
const CW_PRECISION_MASK: u16 = 0x0300;
const CW_ROUNDING_MASK: u16 = 0x0C00;
const CW_ROUNDING_NEAREST: u16 = 0x0000;
const CW_ROUNDING_DOWN: u16 = 0x0400;
const CW_ROUNDING_UP: u16 = 0x0800;
const CW_ROUNDING_TRUNC: u16 = 0x0C00;

// ============================================================================
// FNSTCW - Store Control Word
// ============================================================================

#[test]
fn test_fnstcw_basic() {
    let mut emu = emu64();    let code = [
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert!(cw != 0, "Control word should not be zero");
}

#[test]
fn test_fnstcw_twice() {
    let mut emu = emu64();    let code = [
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xD9, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00,  // FNSTCW [0x3002]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let cw1 = emu.maps.read_word(0x3000).unwrap();
    let cw2 = emu.maps.read_word(0x3002).unwrap();
    assert_eq!(cw1, cw2, "Control word should be consistent");
}

// ============================================================================
// FLDCW - Load Control Word
// ============================================================================

#[test]
fn test_fldcw_basic() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let test_cw: u16 = 0x037F; // Default control word
    emu.maps.write_word(0x2000, test_cw);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, test_cw, "Loaded control word should match");
}

#[test]
fn test_fldcw_different_values() {
    let mut emu = emu64();    let test_values = vec![0x027F, 0x037F, 0x047F, 0x0C7F];

    for test_cw in test_values {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
            0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
            0xF4,                                        // HLT
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, test_cw);

    emu.run(None).unwrap();

        let cw = emu.maps.read_word(0x3000).unwrap();
        assert_eq!(cw, test_cw, "Control word 0x{:04X} should match", test_cw);
    }
}

// ============================================================================
// Rounding Mode Control
// ============================================================================

#[test]
fn test_rounding_mode_nearest() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F | CW_ROUNDING_NEAREST); // Round to nearest
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "2.5 should round to nearest even (2.0)");
}

#[test]
fn test_rounding_mode_down() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F | CW_ROUNDING_DOWN); // Round down
    emu.maps.write_f64(0x2008, 2.7);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "2.7 should round down to 2.0");
}

#[test]
fn test_rounding_mode_up() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F | CW_ROUNDING_UP); // Round up
    emu.maps.write_f64(0x2008, 2.3);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 3.0, "2.3 should round up to 3.0");
}

#[test]
fn test_rounding_mode_truncate() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F | CW_ROUNDING_TRUNC); // Round toward zero
    emu.maps.write_f64(0x2008, 2.9);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.0, "2.9 should truncate to 2.0");
}

#[test]
fn test_rounding_mode_truncate_negative() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F | CW_ROUNDING_TRUNC);
    emu.maps.write_f64(0x2008, -2.9);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, -2.0, "-2.9 should truncate to -2.0");
}

// ============================================================================
// Exception Mask Tests
// ============================================================================

#[test]
fn test_exception_mask_all_set() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let cw = 0x037F; // All exception masks set
    emu.maps.write_word(0x2000, cw);

    emu.run(None).unwrap();

    let result_cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(result_cw & 0x3F, 0x3F, "All exception masks should be set");
}

#[test]
fn test_exception_mask_invalid() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let cw = 0x037E; // All except invalid operation masked
    emu.maps.write_word(0x2000, cw);

    emu.run(None).unwrap();

    let result_cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(result_cw & CW_MASK_INVALID, 0, "Invalid mask should be clear");
}

// ============================================================================
// FLDCW/FNSTCW Round Trip
// ============================================================================

#[test]
fn test_fldcw_fnstcw_roundtrip() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    let test_values = vec![
        0x027F, 0x037F, 0x047F, 0x067F,
        0x0B7F, 0x0C7F, 0x0F7F,
    ];

    for test_cw in test_values {
        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, test_cw);

    emu.run(None).unwrap();

        let result = emu.maps.read_word(0x3000).unwrap();
        assert_eq!(result, test_cw, "Round trip failed for 0x{:04X}", test_cw);
    }
}

#[test]
fn test_multiple_fldcw() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x2C, 0x25, 0x02, 0x20, 0x00, 0x00,  // FLDCW [0x2002]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x027F);
    emu.maps.write_word(0x2002, 0x0C7F);

    emu.run(None).unwrap();

    let result = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(result, 0x0C7F, "Last FLDCW should take effect");
}

// ============================================================================
// Rounding Combinations
// ============================================================================

#[test]
fn test_rounding_modes_with_different_values() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_cases = vec![
        (CW_ROUNDING_NEAREST, 1.5, 2.0), // Round to even
        (CW_ROUNDING_DOWN, 1.5, 1.0),
        (CW_ROUNDING_UP, 1.5, 2.0),
        (CW_ROUNDING_TRUNC, 1.5, 1.0),
    ];

    for (mode, input, expected) in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, 0x037F | mode);
        emu.maps.write_f64(0x2008, input);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, expected, "Rounding mode 0x{:04X} with input {}", mode, input);
    }
}

#[test]
fn test_rounding_negative_values() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    let test_cases = vec![
        (CW_ROUNDING_DOWN, -1.3, -2.0),  // Down means toward -infinity
        (CW_ROUNDING_UP, -1.3, -1.0),    // Up means toward +infinity
        (CW_ROUNDING_TRUNC, -1.9, -1.0), // Truncate toward zero
    ];

    for (mode, input, expected) in test_cases {
        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, 0x037F | mode);
        emu.maps.write_f64(0x2008, input);

    emu.run(None).unwrap();

        let result = emu.maps.read_f64(0x3000).unwrap();
        assert_eq!(result, expected, "Rounding mode 0x{:04X} with negative input {}", mode, input);
    }
}

// ============================================================================
// Control Word Persistence
// ============================================================================

#[test]
fn test_control_word_persists_across_operations() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDE, 0xC1,                                  // FADDP
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xD9, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00,  // FNSTCW [0x3002]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    let test_cw = 0x0C7F; // Truncate mode
    emu.maps.write_word(0x2000, test_cw);
    emu.maps.write_f64(0x2008, 2.3);
    emu.maps.write_f64(0x2010, 1.7);

    emu.run(None).unwrap();

    let result_cw = emu.maps.read_word(0x3002).unwrap();
    assert_eq!(result_cw, test_cw, "Control word should persist");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_control_word_all_zeros() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x0000);

    emu.run(None).unwrap();

    let result = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(result, 0x0000, "Control word of all zeros should be valid");
}

#[test]
fn test_control_word_all_ones() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0xFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_word(0x3000).unwrap();
    assert!(result != 0, "Control word loaded");
}

// ============================================================================
// FSTCW vs FNSTCW
// ============================================================================

#[test]
fn test_fstcw_fnstcw_equivalence() {
    let mut emu = emu64();    let code1 = [
        0x9B, 0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTCW [0x3000]
        0xF4,                                              // HLT
    ];

    let code2 = [
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code1);
    emu.run(None).unwrap();
    let cw1 = emu.maps.read_word(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();
    let cw2 = emu.maps.read_word(0x3000).unwrap();

    assert_eq!(cw1, cw2, "FSTCW and FNSTCW should give same result");
}

// ============================================================================
// Complex Scenarios
// ============================================================================

#[test]
fn test_changing_rounding_modes_dynamically() {
    let mut emu = emu64();    let code = [
        // Round 2.5 with nearest (should be 2.0)
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000] - nearest
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]

        // Round 2.5 with up (should be 3.0)
        0xD9, 0x2C, 0x25, 0x02, 0x20, 0x00, 0x00,  // FLDCW [0x2002] - up
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F | CW_ROUNDING_NEAREST);
    emu.maps.write_word(0x2002, 0x037F | CW_ROUNDING_UP);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x3000).unwrap();
    let result2 = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result1, 2.0, "First round should use nearest mode");
    assert_eq!(result2, 3.0, "Second round should use up mode");
}

#[test]
fn test_control_word_with_arithmetic() {
    let mut emu = emu64();    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDE, 0xC1,                                  // FADDP
        0xD9, 0xFC,                                  // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F | CW_ROUNDING_DOWN);
    emu.maps.write_f64(0x2008, 1.6);
    emu.maps.write_f64(0x2010, 1.6);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 3.0, "1.6 + 1.6 = 3.2, rounded down = 3.0");
}
