//! Tests for the FINIT and FNINIT instructions.
//!
//! FINIT - Initialize FPU (with exception check)
//! FNINIT - Initialize FPU (without exception check)
//!
//! FINIT and FNINIT initialize the FPU to its default state:
//! - FPU control word set to 037FH (round to nearest, all exceptions masked, 64-bit precision)
//! - FPU status word cleared to 0
//! - Tag word set to FFFFH (all registers tagged as empty)
//! - Data and instruction pointers cleared
//! - All register stack left unchanged but tagged as empty
//!
//! FINIT checks for pending exceptions before initializing, while FNINIT does not.
//!
//! Opcodes:
//! - FINIT: 9B DB E3
//! - FNINIT: DB E3
//!
//! Control Word after FINIT/FNINIT: 037FH
//! Status Word after FINIT/FNINIT: 0000H
//! Tag Word after FINIT/FNINIT: FFFFH
//!
//! References: /Users/int/dev/rax/docs/finit:fninit.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

// FPU default values
const DEFAULT_CONTROL_WORD: u16 = 0x037F;    // Round to nearest, all exceptions masked
const DEFAULT_STATUS_WORD: u16 = 0x0000;     // No exceptions, TOP=0
const DEFAULT_TAG_WORD: u16 = 0xFFFF;        // All registers empty

// Status word bit definitions
const IE_BIT: u16 = 0x0001;      // Invalid Operation
const DE_BIT: u16 = 0x0002;      // Denormalized Operand
const ZE_BIT: u16 = 0x0004;      // Zero Divide
const OE_BIT: u16 = 0x0008;      // Overflow
const UE_BIT: u16 = 0x0010;      // Underflow
const PE_BIT: u16 = 0x0020;      // Precision
const SF_BIT: u16 = 0x0040;      // Stack Fault
const ES_BIT: u16 = 0x0080;      // Exception Summary Status
const TOP_MASK: u16 = 0x3800;    // TOP bits 11-13
const C2_BIT: u16 = 0x0400;      // Condition Code 2
const C1_BIT: u16 = 0x0200;      // Condition Code 1
const C3_BIT: u16 = 0x4000;      // Condition Code 3
const C0_BIT: u16 = 0x0100;      // Condition Code 0
const B_BIT: u16 = 0x8000;       // Busy

// Helper function to write u16 to memory
fn write_u16(mem: u64, addr: u64, val: u16) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// Helper function to read u16 from memory
fn read_u16(mem: u64, addr: u64) -> u16 {
    let mut emu = emu64();    let mut buf = [0u8; 2];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u16::from_le_bytes(buf)
}

// Helper function to write f64 to memory
fn write_f64(mem: u64, addr: u64, val: f64) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// Helper function to read f64 from memory
fn read_f64(mem: u64, addr: u64) -> f64 {
    let mut emu = emu64();    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    f64::from_le_bytes(buf)
}

// ============================================================================
// FNINIT - Initialize without Wait
// ============================================================================

#[test]
fn test_fninit_basic() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,        // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, DEFAULT_CONTROL_WORD, "Control word should be 037FH after FNINIT");
}

#[test]
fn test_fninit_clears_status_word() {
    let mut emu = emu64();    // FNINIT should clear the status word
    let code = [
        0xDB, 0xE3,        // FNINIT
        0xDF, 0xE0,        // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let sw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(sw, DEFAULT_STATUS_WORD, "Status word should be 0000H after FNINIT");
}

#[test]
fn test_fninit_sets_tag_word() {
    let mut emu = emu64();    // FNINIT should set tag word to FFFFH (all registers empty)
    let code = [
        0xDB, 0xE3,        // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (load after init)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.5, "FPU should work correctly after FNINIT");
}

#[test]
fn test_fninit_resets_top_pointer() {
    let mut emu = emu64();    // FNINIT should reset TOP (Top of Stack) pointer to 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000] (push)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (push)
        0xDB, 0xE3,                                  // FNINIT (reset TOP)
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let sw = emu.maps.read_word(0x3000).unwrap();
    let top = (sw & TOP_MASK) >> 11;
    assert_eq!(top, 0, "TOP should be 0 after FNINIT");
}

#[test]
fn test_fninit_multiple_times() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00,  // FNSTCW [0x3002]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let cw1 = emu.maps.read_word(0x3000).unwrap();
    let cw2 = emu.maps.read_word(0x3002).unwrap();
    assert_eq!(cw1, DEFAULT_CONTROL_WORD, "First FNINIT should set CW to 037FH");
    assert_eq!(cw2, DEFAULT_CONTROL_WORD, "Second FNINIT should set CW to 037FH");
}

// ============================================================================
// FINIT - Initialize with Wait
// ============================================================================

#[test]
fn test_finit_basic() {
    let mut emu = emu64();    let code = [
        0x9B, 0xDB, 0xE3,  // FINIT (with FWAIT)
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, DEFAULT_CONTROL_WORD, "Control word should be 037FH after FINIT");
}

#[test]
fn test_finit_clears_status_word() {
    let mut emu = emu64();    // FINIT should clear the status word
    let code = [
        0x9B, 0xDB, 0xE3,  // FINIT (with FWAIT)
        0xDF, 0xE0,        // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let sw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(sw, DEFAULT_STATUS_WORD, "Status word should be 0000H after FINIT");
}

// ============================================================================
// FINIT vs FNINIT Equivalence
// ============================================================================

#[test]
fn test_finit_vs_fninit() {
    let mut emu = emu64();    // FINIT and FNINIT should have same effect in normal operation
    let code1 = [
        0x9B, 0xDB, 0xE3,  // FINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,              // HLT
    ];

    let code2 = [
        0xDB, 0xE3,        // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code1);
    emu.run(None).unwrap();
    let cw1 = emu.maps.read_word(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();
    let cw2 = emu.maps.read_word(0x3000).unwrap();

    assert_eq!(cw1, cw2, "FINIT and FNINIT should give same result");
}

// ============================================================================
// Initialize after Operations
// ============================================================================

#[test]
fn test_fninit_after_arithmetic() {
    let mut emu = emu64();    // FNINIT after arithmetic operations should reset state
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDB, 0xE3,                                  // FNINIT
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let sw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(sw, DEFAULT_STATUS_WORD, "Status word should be cleared after FNINIT");
}

#[test]
fn test_finit_after_comparison() {
    let mut emu = emu64();    // FINIT after comparison
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD8, 0xD1,                                  // FCOM ST(1)
        0x9B, 0xDB, 0xE3,                           // FINIT
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let sw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(sw, DEFAULT_STATUS_WORD, "Status word should be cleared after FINIT");
}

// ============================================================================
// Control Word Verification after Initialize
// ============================================================================

#[test]
fn test_fninit_control_word_precision() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,        // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    let precision = (cw >> 8) & 0x3;
    assert_eq!(precision, 0x3, "Precision should be 64-bit (11 binary)");
}

#[test]
fn test_fninit_control_word_rounding() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,        // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    let rounding = (cw >> 10) & 0x3;
    assert_eq!(rounding, 0x0, "Rounding should be nearest (00 binary)");
}

#[test]
fn test_fninit_control_word_exceptions_masked() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,        // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    let exception_masks = cw & 0x3F;
    assert_eq!(exception_masks, 0x3F, "All exception masks should be set");
}

// ============================================================================
// FPU Usability after Initialize
// ============================================================================

#[test]
fn test_fninit_then_use_fpu() {
    let mut emu = emu64();    // FPU should be usable immediately after FNINIT
    let code = [
        0xDB, 0xE3,                                  // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.5);
    emu.maps.write_f64(0x2008, 4.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 8.0, "FPU should work correctly after FNINIT");
}

#[test]
fn test_fninit_stack_operations() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,                                  // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x3000).unwrap();
    let v2 = emu.maps.read_f64(0x3008).unwrap();
    let v3 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(v1, 3.0, "Last pushed should be first popped");
    assert_eq!(v2, 2.0, "Middle value");
    assert_eq!(v3, 1.0, "First pushed should be last popped");
}

// ============================================================================
// Status Word after Initialize
// ============================================================================

#[test]
fn test_fninit_clears_exception_flags() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,        // FNINIT
        0xDF, 0xE0,        // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let sw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(sw & IE_BIT, 0, "IE should be cleared");
    assert_eq!(sw & DE_BIT, 0, "DE should be cleared");
    assert_eq!(sw & ZE_BIT, 0, "ZE should be cleared");
    assert_eq!(sw & OE_BIT, 0, "OE should be cleared");
    assert_eq!(sw & UE_BIT, 0, "UE should be cleared");
    assert_eq!(sw & PE_BIT, 0, "PE should be cleared");
}

#[test]
fn test_fninit_clears_stack_fault() {
    let mut emu = emu64();    let code = [
        0xDB, 0xE3,        // FNINIT
        0xDF, 0xE0,        // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let sw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(sw & SF_BIT, 0, "SF should be cleared");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_fninit_complete_flow() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTSW [0x3000] (before init)
        0xDB, 0xE3,                                  // FNINIT
        0xDD, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00,  // FNSTSW [0x3002] (after init)
        0xD9, 0x3C, 0x25, 0x04, 0x30, 0x00, 0x00,  // FNSTCW [0x3004]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.0);
    emu.maps.write_f64(0x2008, 3.0);

    emu.run(None).unwrap();

    let sw_after = emu.maps.read_word(0x3002).unwrap();
    let cw = emu.maps.read_word(0x3004).unwrap();
    assert_eq!(sw_after, DEFAULT_STATUS_WORD, "Status word should be default after FNINIT");
    assert_eq!(cw, DEFAULT_CONTROL_WORD, "Control word should be default after FNINIT");
}

#[test]
fn test_finit_preserves_data() {
    let mut emu = emu64();    // FINIT doesn't modify register data, just tags them as empty
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDB, 0xE3,                                  // FNINIT (data preserved, tagged empty)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (will use ST(0))
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 2.5, "Second FLD should work correctly after FNINIT");
}

#[test]
fn test_multiple_finit_cycles() {
    let mut emu = emu64();    let code = [
        // First cycle
        0xDB, 0xE3,                                  // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        // Second cycle
        0xDB, 0xE3,                                  // FNINIT
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        // Third cycle
        0xDB, 0xE3,                                  // FNINIT
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let r1 = emu.maps.read_f64(0x3000).unwrap();
    let r2 = emu.maps.read_f64(0x3008).unwrap();
    let r3 = emu.maps.read_f64(0x3010).unwrap();
    assert_eq!(r1, 1.0, "First cycle result");
    assert_eq!(r2, 2.0, "Second cycle result");
    assert_eq!(r3, 3.0, "Third cycle result");
}
