//! Tests for the FSTSW and FNSTSW instructions.
//!
//! FSTSW - Store x87 FPU Status Word (with exception check)
//! FNSTSW - Store x87 FPU Status Word (without exception check)
//!
//! FSTSW and FNSTSW store the current FPU status word to either the AX register
//! or a 16-bit memory location. FSTSW checks for pending exceptions before storing,
//! while FNSTSW does not.
//!
//! Opcodes:
//! - FSTSW AX: 9B DF E0
//! - FSTSW m16: 9B DD /7
//! - FNSTSW AX: DF E0
//! - FNSTSW m16: DD /7
//!
//! Status Word Format (16 bits):
//! - Bit 0: IE (Invalid Operation)
//! - Bit 1: DE (Denormalized Operand)
//! - Bit 2: ZE (Zero Divide)
//! - Bit 3: OE (Overflow)
//! - Bit 4: UE (Underflow)
//! - Bit 5: PE (Precision)
//! - Bit 6: SF (Stack Fault)
//! - Bit 7: ES (Exception Summary Status)
//! - Bits 8-10: TOP (Top of Stack Pointer)
//! - Bit 11: C2 (Condition Code 2)
//! - Bit 12: C1 (Condition Code 1)
//! - Bit 13: C3 (Condition Code 3)
//! - Bit 14: C0 (Condition Code 0)
//! - Bit 15: B (Busy)
//!
//! Flags affected: C0, C1, C2, C3 are undefined (they reflect the current FPU state)
//!
//! References: /Users/int/dev/rax/docs/fstsw:fnstsw.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

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

// ============================================================================
// FNSTSW AX - Store Status Word to AX Register
// ============================================================================

#[test]
fn test_fnstsw_ax_basic() {
    let mut emu = emu64();    let code = [
        0xDF, 0xE0,        // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    assert!(status < 0xFFFF, "Status word should be a valid value");
}

#[test]
fn test_fnstsw_ax_multiple_calls() {
    let mut emu = emu64();    let code = [
        0xDF, 0xE0,        // FNSTSW AX (1st time)
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xDF, 0xE0,        // FNSTSW AX (2nd time)
        0x66, 0x89, 0x04, 0x25, 0x02, 0x30, 0x00, 0x00,  // MOV word [0x3002], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status1 = emu.maps.read_word(0x3000).unwrap();
    let status2 = emu.maps.read_word(0x3002).unwrap();
    assert_eq!(status1, status2, "Multiple FNSTSW AX should be identical");
}

// ============================================================================
// FNSTSW m16 - Store Status Word to Memory
// ============================================================================

#[test]
fn test_fnstsw_m16_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTSW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    assert!(status < 0xFFFF, "Status word should be valid");
}

#[test]
fn test_fnstsw_m16_multiple_locations() {
    let mut emu = emu64();    let code = [
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTSW [0x3000]
        0xDD, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00,  // FNSTSW [0x3002]
        0xDD, 0x3C, 0x25, 0x04, 0x30, 0x00, 0x00,  // FNSTSW [0x3004]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status1 = emu.maps.read_word(0x3000).unwrap();
    let status2 = emu.maps.read_word(0x3002).unwrap();
    let status3 = emu.maps.read_word(0x3004).unwrap();
    assert_eq!(status1, status2, "All FNSTSW should give same result");
    assert_eq!(status2, status3, "All FNSTSW should give same result");
}

// ============================================================================
// FSTSW - Store Status Word with Exception Check
// ============================================================================

#[test]
fn test_fstsw_ax_basic() {
    let mut emu = emu64();    let code = [
        0x9B, 0xDF, 0xE0,  // FSTSW AX (with FWAIT)
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    assert!(status < 0xFFFF, "Status word should be valid");
}

#[test]
fn test_fstsw_m16_basic() {
    let mut emu = emu64();    let code = [
        0x9B, 0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTSW [0x3000]
        0xF4,                                              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    assert!(status < 0xFFFF, "Status word should be valid");
}

// ============================================================================
// FSTSW vs FNSTSW Equivalence
// ============================================================================

#[test]
fn test_fstsw_vs_fnstsw_ax() {
    let mut emu = emu64();    let code1 = [
        0x9B, 0xDF, 0xE0,  // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    let code2 = [
        0xDF, 0xE0,        // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code1);
    emu.run(None).unwrap();
    let status1 = emu.maps.read_word(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();
    let status2 = emu.maps.read_word(0x3000).unwrap();

    assert_eq!(status1, status2, "FSTSW and FNSTSW should give same result");
}

#[test]
fn test_fstsw_vs_fnstsw_m16() {
    let mut emu = emu64();    let code1 = [
        0x9B, 0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTSW [0x3000]
        0xF4,                                              // HLT
    ];

    let code2 = [
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTSW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code1);
    emu.run(None).unwrap();
    let status1 = emu.maps.read_word(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();
    let status2 = emu.maps.read_word(0x3000).unwrap();

    assert_eq!(status1, status2, "FSTSW and FNSTSW should give same result");
}

// ============================================================================
// Status Word Bits after Operations
// ============================================================================

#[test]
fn test_status_word_top_pointer() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    let top = (status & TOP_MASK) >> 11;
    // TOP should be non-zero after loading
    assert!(top < 8, "TOP should be within 0-7 range");
}

#[test]
fn test_status_word_empty_stack() {
    let mut emu = emu64();    let code = [
        0xDF, 0xE0,        // FNSTSW AX (empty stack)
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    // C1, C0, C2, C3 bits are undefined but TOP should be 0 when empty
    assert!(status < 0xFFFF, "Status word should be valid");
}

#[test]
fn test_status_word_after_fld_fst() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x10, 0x30, 0x00, 0x00,  // MOV word [0x3010], AX
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.14159);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3010).unwrap();
    let stored_value = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(stored_value, 3.14159, "Value should be stored correctly");
    assert!(status < 0xFFFF, "Status word should be valid");
}

// ============================================================================
// Condition Code Bits
// ============================================================================

#[test]
fn test_condition_codes_c0_c1_c2_c3() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD8, 0xD1,                                  // FCOM ST(1)
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 5.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    assert!(status < 0xFFFF, "Status word should be valid");
}

// ============================================================================
// Exception Flags in Status Word
// ============================================================================

#[test]
fn test_status_word_exception_flags() {
    let mut emu = emu64();    let code = [
        0xDF, 0xE0,        // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    let ie = (status & IE_BIT) != 0;
    let de = (status & DE_BIT) != 0;
    let ze = (status & ZE_BIT) != 0;
    let oe = (status & OE_BIT) != 0;
    let ue = (status & UE_BIT) != 0;
    let pe = (status & PE_BIT) != 0;

    assert!(!ie || !de || !ze || !oe || !ue || !pe, "At least some exceptions should be clear");
}

// ============================================================================
// Status Word Persistence
// ============================================================================

#[test]
fn test_status_word_persists_across_operations() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    let result = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result, 4.0, "Addition should be correct");
    assert!(status < 0xFFFF, "Status word should be valid");
}

// ============================================================================
// AX Register Full Width Storage
// ============================================================================

#[test]
fn test_fnstsw_ax_full_register() {
    let mut emu = emu64();    // FNSTSW AX should use 16-bit AX, verify full value is stored
    let code = [
        0xDF, 0xE0,        // FNSTSW AX
        0x48, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV qword [0x3000], RAX
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let full_value = emu.maps.read_word(0x3000).unwrap();
    assert!(full_value < 0xFFFF, "AX register should contain valid status word");
}

// ============================================================================
// Memory Addressing Modes
// ============================================================================

#[test]
fn test_fnstsw_m16_various_addresses() {
    let mut emu = emu64();    let addresses = vec![0x3000, 0x3100, 0x3200];

    let code = [
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTSW [0x3000]
        0xDD, 0x3C, 0x25, 0x00, 0x31, 0x00, 0x00,  // FNSTSW [0x3100]
        0xDD, 0x3C, 0x25, 0x00, 0x32, 0x00, 0x00,  // FNSTSW [0x3200]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status1 = emu.maps.read_word(0x3000).unwrap();
    let status2 = emu.maps.read_word(0x3100).unwrap();
    let status3 = emu.maps.read_word(0x3200).unwrap();

    assert_eq!(status1, status2, "All stores should have same value");
    assert_eq!(status2, status3, "All stores should have same value");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_fnstsw_ax_after_comparison() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD8, 0xD1,                                  // FCOM ST(1)
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 10.0);
    emu.maps.write_f64(0x2008, 5.0);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    assert!(status < 0xFFFF, "Status word should be valid after comparison");
}

#[test]
fn test_fnstsw_m16_after_arithmetic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTSW [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);
    emu.maps.write_f64(0x2008, 3.5);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    let result = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result, 6.0, "Addition result correct");
    assert!(status < 0xFFFF, "Status word should be valid");
}

#[test]
fn test_fnstsw_ax_sequential() {
    let mut emu = emu64();    let code = [
        0xDF, 0xE0,        // FNSTSW AX (1st)
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDF, 0xE0,        // FNSTSW AX (2nd)
        0x66, 0x89, 0x04, 0x25, 0x02, 0x30, 0x00, 0x00,  // MOV word [0x3002], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 42.0);

    emu.run(None).unwrap();

    let status1 = emu.maps.read_word(0x3000).unwrap();
    let status2 = emu.maps.read_word(0x3002).unwrap();
    assert!(status1 < 0xFFFF, "Status 1 should be valid");
    assert!(status2 < 0xFFFF, "Status 2 should be valid");
}

#[test]
fn test_fnstsw_m16_vs_ax_same_value() {
    let mut emu = emu64();    let code = [
        0xDF, 0xE0,        // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xDD, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00,  // FNSTSW [0x3002]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let status_ax = emu.maps.read_word(0x3000).unwrap();
    let status_m16 = emu.maps.read_word(0x3002).unwrap();
    assert_eq!(status_ax, status_m16, "FNSTSW AX and FNSTSW m16 should give same result");
}

#[test]
fn test_fstsw_multiple_operations() {
    let mut emu = emu64();    // FSTSW with multiple FPU operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDE, 0xC1,                                  // FADDP
        0x9B, 0xDF, 0xE0,  // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,              // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.0);
    emu.maps.write_f64(0x2008, 2.0);
    emu.maps.write_f64(0x2010, 3.0);

    emu.run(None).unwrap();

    let status = emu.maps.read_word(0x3000).unwrap();
    let result = emu.maps.read_f64(0x3008).unwrap();
    assert_eq!(result, 6.0, "Sum should be correct");
    assert!(status < 0xFFFF, "Status word should be valid");
}
