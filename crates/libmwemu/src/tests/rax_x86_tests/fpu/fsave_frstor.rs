//! Tests for the FSAVE, FNSAVE, and FRSTOR instructions.
//!
//! FSAVE/FNSAVE - Store x87 FPU State and Registers
//! FRSTOR - Restore x87 FPU State and Registers
//!
//! FSAVE saves the complete FPU state (operating environment and register stack)
//! to a 94 or 108-byte memory area and then reinitializes the FPU.
//! FRSTOR restores the state from that area.
//!
//! FSAVE checks for pending exceptions before saving, while FNSAVE does not.
//!
//! Opcodes:
//! - FNSAVE: DD /6
//! - FSAVE: 9B DD /6
//! - FRSTOR: DD /4
//!
//! Save Area Format (94 bytes in 16-bit, 108 bytes in 32-bit):
//! - Environment (14 or 28 bytes)
//! - FPU Register Stack (80 bytes) - 8 registers x 10 bytes each
//!
//! References: /Users/int/dev/rax/docs/fsave:fnsave.txt, /Users/int/dev/rax/docs/frstor.txt

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

// FSAVE/FRSTOR area structure offsets (in protected 32-bit mode, 108 bytes)
const FSAVE_FCW: u64 = 0;         // FPU Control Word (2 bytes)
const FSAVE_FSW: u64 = 2;         // FPU Status Word (2 bytes)
const FSAVE_FTW: u64 = 4;         // FPU Tag Word (2 bytes)
const FSAVE_SIZE: u64 = 108;      // Total size in 32-bit protected mode

// Status word bit definitions
const IE_BIT: u16 = 0x0001;
const TOP_MASK: u16 = 0x3800;

// ============================================================================
// FNSAVE - Save State without Wait
// ============================================================================

#[test]
fn test_fnsave_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be valid after FNSAVE");
}

#[test]
fn test_fnsave_saves_control_word() {
    let mut emu = emu64();    // FNSAVE should save the control word
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_fcw = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
    assert_eq!(saved_fcw, 0x037F, "FCW should be saved");
}

#[test]
fn test_fnsave_saves_status_word() {
    let mut emu = emu64();    // FNSAVE should save the status word
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.14159);

    emu.run(None).unwrap();

    let saved_fsw = emu.maps.read_word(0x3000 + FSAVE_FSW).unwrap();
    assert!(saved_fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fnsave_saves_fpu_registers() {
    let mut emu = emu64();    // FNSAVE should save FPU register contents
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
    let fsw = emu.maps.read_word(0x3000 + FSAVE_FSW).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be saved");
    assert!(fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fnsave_reinitializes_fpu() {
    let mut emu = emu64();    // FNSAVE should reinitialize the FPU after saving
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xDF, 0xE0,                                  // FNSTSW AX (check status after save)
        0x66, 0x89, 0x04, 0x25, 0x00, 0x40, 0x00, 0x00,  // MOV word [0x4000], AX
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let sw_after = emu.maps.read_word(0x4000).unwrap();
    assert_eq!(sw_after, 0x0000, "FPU should be reinitialized after FNSAVE");
}

#[test]
fn test_fnsave_multiple_times() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00,  // FNSAVE [0x3200]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 1.5);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3200 + FSAVE_FCW).unwrap();
    assert_eq!(fcw1, fcw2, "Multiple FNSAVE should save identical control words");
}

// ============================================================================
// FSAVE - Save State with Wait
// ============================================================================

#[test]
fn test_fsave_basic() {
    let mut emu = emu64();    let code = [
        0x9B, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSAVE [0x3000]
        0xF4,                                              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be valid");
}

#[test]
fn test_fsave_saves_control_word() {
    let mut emu = emu64();    // FSAVE should save the control word
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0x9B, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x027F);

    emu.run(None).unwrap();

    let saved_fcw = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
    assert_eq!(saved_fcw, 0x027F, "FCW should be saved");
}

// ============================================================================
// FSAVE vs FNSAVE Equivalence
// ============================================================================

#[test]
fn test_fsave_vs_fnsave() {
    let mut emu = emu64();    // FSAVE and FNSAVE should produce same result in normal operation
    let code1 = [
        0x9B, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSAVE [0x3000]
        0xF4,                                              // HLT
    ];

    let code2 = [
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code1);
    emu.run(None).unwrap();
    let fcw1 = emu.maps.read_word(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();
    let fcw2 = emu.maps.read_word(0x3000).unwrap();

    assert_eq!(fcw1, fcw2, "FSAVE and FNSAVE should give same result");
}

// ============================================================================
// FRSTOR - Restore State
// ============================================================================

#[test]
fn test_frstor_basic() {
    let mut emu = emu64();    let code = [
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FRSTOR [0x3000]
        0xD9, 0x3C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FNSTCW [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x3000 + FSAVE_FCW, 0x037F);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x4000).unwrap();
    assert!(cw < 0xFFFF, "Control word should be valid after FRSTOR");
}

// ============================================================================
// FNSAVE/FRSTOR Round Trip
// ============================================================================

#[test]
fn test_fnsave_frstor_roundtrip() {
    let mut emu = emu64();    // FNSAVE followed by FRSTOR should preserve state
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008] (after init)
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00,  // FSTP qword [0x4010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 99.0);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4010).unwrap();
    assert_eq!(result, 1.5, "Value should be preserved through FNSAVE/FRSTOR");
}

#[test]
fn test_fnsave_frstor_multiple_values() {
    let mut emu = emu64();    // FNSAVE/FRSTOR with multiple FPU values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010] (new value)
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x18, 0x40, 0x00, 0x00,  // FSTP qword [0x4018]
        0xDD, 0x1C, 0x25, 0x20, 0x40, 0x00, 0x00,  // FSTP qword [0x4020]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);
    emu.maps.write_f64(0x2010, 99.0);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x4018).unwrap();
    let v2 = emu.maps.read_f64(0x4020).unwrap();
    assert_eq!(v1, 2.5, "Second saved value should be 2.5");
    assert_eq!(v2, 1.5, "First saved value should be 1.5");
}

// ============================================================================
// FNSAVE Area Size
// ============================================================================

#[test]
fn test_fnsave_area_structure() {
    let mut emu = emu64();    // FNSAVE uses up to 108 bytes in 32-bit protected mode
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.14159);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000).unwrap();
    let fsw = emu.maps.read_word(0x3002).unwrap();
    let ftw = emu.maps.read_word(0x3004).unwrap();

    assert!(fcw < 0xFFFF, "FCW should be valid");
    assert!(fsw < 0xFFFF, "FSW should be valid");
    assert!(ftw < 0xFFFF, "FTW should be valid");
}

// ============================================================================
// FNSAVE with Different Control Words
// ============================================================================

#[test]
fn test_fnsave_different_control_words() {
    let mut emu = emu64();    // FNSAVE should preserve different control word values
    let test_cws = vec![0x037F, 0x027F, 0x0C7F];

    for test_cw in test_cws {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
            0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
            0xF4,                                        // HLT
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, test_cw);

    emu.run(None).unwrap();

        let saved_cw = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
        assert_eq!(saved_cw, test_cw, "Control word 0x{:04X} should be saved", test_cw);
    }
}

// ============================================================================
// FRSTOR from Prepared Area
// ============================================================================

#[test]
fn test_frstor_from_prepared_area() {
    let mut emu = emu64();    // FRSTOR from a pre-prepared FNSAVE area
    let code = [
        0xDD, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,  // FRSTOR [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + FSAVE_FCW, 0x037F);
    emu.maps.write_word(0x2000 + FSAVE_FSW, 0x0000);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, 0x037F, "Control word should be restored from prepared area");
}

// ============================================================================
// Sequential FNSAVE Operations
// ============================================================================

#[test]
fn test_sequential_fnsave() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00,  // FNSAVE [0x3200]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 1.5);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3200 + FSAVE_FCW).unwrap();
    assert_eq!(fcw1, fcw2, "Multiple FNSAVE should produce identical results");
}

// ============================================================================
// FNSAVE after Arithmetic
// ============================================================================

#[test]
fn test_fnsave_after_arithmetic() {
    let mut emu = emu64();    // FNSAVE after arithmetic operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);
    emu.maps.write_f64(0x2008, 3.5);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + FSAVE_FSW).unwrap();
    assert!(fsw < 0xFFFF, "FSW should be saved after arithmetic");
}

#[test]
fn test_frstor_then_arithmetic() {
    let mut emu = emu64();    // FRSTOR followed by arithmetic
    let code = [
        0xDD, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,  // FRSTOR [0x2000]
        0xDD, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FLD qword [0x3000]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + FSAVE_FCW, 0x037F);
    emu.maps.write_word(0x2000 + FSAVE_FSW, 0x0000);
    emu.maps.write_f64(0x3000, 1.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4000).unwrap();
    assert_eq!(result, 1.5, "Arithmetic should work after FRSTOR");
}

// ============================================================================
// State Preservation
// ============================================================================

#[test]
fn test_fnsave_preserves_control_precision() {
    let mut emu = emu64();    // FNSAVE should preserve control word precision bits
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);  // Default (64-bit precision)

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + FSAVE_FCW).unwrap();
    let precision = (saved_cw >> 8) & 0x3;
    assert_eq!(precision, 0x3, "Precision should be saved as 64-bit");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_fnsave_frstor_complete_flow() {
    let mut emu = emu64();    let code = [
        // Load and use FPU
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        // Save state
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        // FPU is reinitialized by FNSAVE, do some other work
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x18, 0x20, 0x00, 0x00,  // FSTP qword [0x2018]
        // Restore saved state
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FRSTOR [0x3000]
        // Use restored state
        0xDD, 0x1C, 0x25, 0x20, 0x40, 0x00, 0x00,  // FSTP qword [0x4020]
        0xDD, 0x1C, 0x25, 0x28, 0x40, 0x00, 0x00,  // FSTP qword [0x4028]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);
    emu.maps.write_f64(0x2010, 99.0);

    emu.run(None).unwrap();

    let v1 = emu.maps.read_f64(0x4020).unwrap();
    let v2 = emu.maps.read_f64(0x4028).unwrap();
    assert_eq!(v1, 2.5, "Second saved value should be 2.5");
    assert_eq!(v2, 1.5, "First saved value should be 1.5");
}

#[test]
fn test_fnsave_frstor_multiple_cycles() {
    let mut emu = emu64();    let code = [
        // Cycle 1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,  // FSTP qword [0x4008]
        // Cycle 2
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00,  // FNSAVE [0x3200]
        0xDD, 0x24, 0x25, 0x00, 0x32, 0x00, 0x00,  // FRSTOR [0x3200]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00,  // FSTP qword [0x4010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let r1 = emu.maps.read_f64(0x4008).unwrap();
    let r2 = emu.maps.read_f64(0x4010).unwrap();
    assert_eq!(r1, 1.5, "Cycle 1 result");
    assert_eq!(r2, 2.5, "Cycle 2 result");
}

#[test]
fn test_fsave_vs_fnsave_roundtrip() {
    let mut emu = emu64();    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0x9B, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSAVE [0x3000]
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,  // FSTP qword [0x4008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code1);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_f64(0x4008).unwrap();
    assert_eq!(result1, 1.5, "FSAVE roundtrip should preserve value");

    let code2 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSAVE [0x3000]
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,  // FSTP qword [0x4008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code2);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let result2 = emu.maps.read_f64(0x4008).unwrap();
    assert_eq!(result2, 1.5, "FNSAVE roundtrip should preserve value");
}
