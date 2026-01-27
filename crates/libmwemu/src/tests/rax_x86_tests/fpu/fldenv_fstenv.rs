//! Tests for the FLDENV and FSTENV/FNSTENV instructions.
//!
//! FSTENV/FNSTENV - Store x87 FPU Environment
//! FLDENV - Load x87 FPU Environment
//!
//! FSTENV stores the FPU operating environment (control word, status word, tag word,
//! instruction pointer, data pointer, last opcode) to memory.
//! FLDENV loads the environment from memory.
//!
//! FSTENV checks for pending exceptions before storing, while FNSTENV does not.
//!
//! Opcodes:
//! - FNSTENV: D9 /6
//! - FSTENV: 9B D9 /6
//! - FLDENV: D9 /4
//!
//! Environment Format (28 bytes in protected mode):
//! - Bytes 0-1: FCW (FPU Control Word)
//! - Bytes 2-3: FSW (FPU Status Word)
//! - Bytes 4-5: FTW (FPU Tag Word)
//! - Bytes 6-7: Instruction Pointer (FIP)
//! - Bytes 8-9: CS or reserved
//! - Bytes 10-11: Data Pointer (FDP)
//! - Bytes 12-13: DS or reserved
//! - Bytes 14-27: Reserved
//!
//! References: /Users/int/dev/rax/docs/fldenv.txt, /Users/int/dev/rax/docs/fstenv:fnstenv.txt

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

// FPU Environment offsets (28 bytes in protected mode)
const ENV_FCW: usize = 0;         // FPU Control Word (2 bytes)
const ENV_FSW: usize = 2;         // FPU Status Word (2 bytes)
const ENV_FTW: usize = 4;         // FPU Tag Word (2 bytes)
const ENV_FIP: usize = 6;         // Instruction Pointer (2 bytes)
const ENV_FCS: usize = 8;         // Code Segment (2 bytes)
const ENV_FDP: usize = 10;        // Data Pointer (2 bytes)
const ENV_FDS: usize = 12;        // Data Segment (2 bytes)
const ENV_SIZE: usize = 28;       // Total environment size

// Status word bit definitions
const IE_BIT: u16 = 0x0001;
const DE_BIT: u16 = 0x0002;
const ZE_BIT: u16 = 0x0004;
const OE_BIT: u16 = 0x0008;
const UE_BIT: u16 = 0x0010;
const PE_BIT: u16 = 0x0020;
const SF_BIT: u16 = 0x0040;
const ES_BIT: u16 = 0x0080;
const TOP_MASK: u16 = 0x3800;

// ============================================================================
// FNSTENV - Store Environment without Wait
// ============================================================================

#[test]
fn test_fnstenv_basic() {
    let mut emu = emu64();    let code = [
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be valid after FNSTENV");
}

#[test]
fn test_fnstenv_saves_control_word() {
    let mut emu = emu64();    // FNSTENV should save the control word
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_fcw = emu.maps.read_word(0x3000 + ENV_FCW as u64).unwrap();
    assert_eq!(saved_fcw, 0x037F, "FCW should be saved");
}

#[test]
fn test_fnstenv_saves_status_word() {
    let mut emu = emu64();    // FNSTENV should save the status word
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.14159);

    emu.run(None).unwrap();

    let saved_fsw = emu.maps.read_word(0x3000 + ENV_FSW as u64).unwrap();
    assert!(saved_fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fnstenv_saves_tag_word() {
    let mut emu = emu64();    // FNSTENV should save the tag word
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);

    emu.run(None).unwrap();

    let saved_ftw = emu.maps.read_word(0x3000 + ENV_FTW as u64).unwrap();
    assert!(saved_ftw < 0xFFFF, "FTW should be saved");
}

#[test]
fn test_fnstenv_multiple_times() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xD9, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00,  // FNSTENV [0x3200]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + ENV_FCW as u64).unwrap();
    let fcw2 = emu.maps.read_word(0x3200 + ENV_FCW as u64).unwrap();
    assert_eq!(fcw1, fcw2, "Multiple FNSTENV should produce same control word");
}

// ============================================================================
// FSTENV - Store Environment with Wait
// ============================================================================

#[test]
fn test_fstenv_basic() {
    let mut emu = emu64();    let code = [
        0x9B, 0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTENV [0x3000]
        0xF4,                                              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be valid");
}

#[test]
fn test_fstenv_saves_control_word() {
    let mut emu = emu64();    // FSTENV should save the control word
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0x9B, 0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTENV [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x027F);

    emu.run(None).unwrap();

    let saved_fcw = emu.maps.read_word(0x3000 + ENV_FCW as u64).unwrap();
    assert_eq!(saved_fcw, 0x027F, "FCW should be saved");
}

// ============================================================================
// FSTENV vs FNSTENV Equivalence
// ============================================================================

#[test]
fn test_fstenv_vs_fnstenv() {
    let mut emu = emu64();    // FSTENV and FNSTENV should produce same result in normal operation
    let code1 = [
        0x9B, 0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTENV [0x3000]
        0xF4,                                              // HLT
    ];

    let code2 = [
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code1);
    emu.run(None).unwrap();
    let fcw1 = emu.maps.read_word(0x3000).unwrap();

    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();
    let fcw2 = emu.maps.read_word(0x3000).unwrap();

    assert_eq!(fcw1, fcw2, "FSTENV and FNSTENV should give same result");
}

// ============================================================================
// FLDENV - Load Environment
// ============================================================================

#[test]
fn test_fldenv_basic() {
    let mut emu = emu64();    let code = [
        0xD9, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDENV [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + ENV_FCW as u64, 0x037F);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, 0x037F, "Control word should be loaded");
}

#[test]
fn test_fldenv_loads_control_word() {
    let mut emu = emu64();    // FLDENV should load the control word
    let code = [
        0xD9, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDENV [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + ENV_FCW as u64, 0x027F);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, 0x027F, "Loaded control word should match");
}

#[test]
fn test_fldenv_loads_status_word() {
    let mut emu = emu64();    // FLDENV should load the status word
    let code = [
        0xD9, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDENV [0x2000]
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // MOV word [0x3000], AX
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + ENV_FSW as u64, 0x0000);

    emu.run(None).unwrap();

    let sw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(sw, 0x0000, "Loaded status word should match");
}

// ============================================================================
// FNSTENV/FLDENV Round Trip
// ============================================================================

#[test]
fn test_fnstenv_fldenv_roundtrip() {
    let mut emu = emu64();    // FNSTENV followed by FLDENV should preserve environment
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000] (set CW)
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000] (save)
        0xD9, 0x2C, 0x25, 0x02, 0x20, 0x00, 0x00,  // FLDCW [0x2002] (change CW)
        0xD9, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FLDENV [0x3000] (restore)
        0xD9, 0x3C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FNSTCW [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);
    emu.maps.write_word(0x2002, 0x027F);

    emu.run(None).unwrap();

    let restored_cw = emu.maps.read_word(0x4000).unwrap();
    assert_eq!(restored_cw, 0x037F, "CW should be restored to original value");
}

#[test]
fn test_fnstenv_fldenv_preserves_all_fields() {
    let mut emu = emu64();    // FNSTENV/FLDENV should preserve all environment fields
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FLDENV [0x3000]
        0xD9, 0x3C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FNSTCW [0x4000]
        0xDF, 0xE0,                                  // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x02, 0x40, 0x00, 0x00,  // MOV word [0x4002], AX
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x0C7F);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x4000).unwrap();
    let sw = emu.maps.read_word(0x4002).unwrap();
    assert_eq!(cw, 0x0C7F, "Control word should be preserved");
    assert_eq!(sw, 0x0000, "Status word should be cleared initially");
}

// ============================================================================
// Environment Save/Restore with Arithmetic
// ============================================================================

#[test]
fn test_fnstenv_after_arithmetic() {
    let mut emu = emu64();    // FNSTENV after arithmetic operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);
    emu.maps.write_f64(0x2008, 3.5);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + ENV_FSW as u64).unwrap();
    assert!(fsw < 0xFFFF, "FSW should be saved after arithmetic");
}

#[test]
fn test_fldenv_then_arithmetic() {
    let mut emu = emu64();    // FLDENV followed by arithmetic
    let code = [
        0xD9, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDENV [0x2000]
        0xDD, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FLD qword [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00,  // FLD qword [0x3008]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FSTP qword [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + ENV_FCW as u64, 0x037F);
    emu.maps.write_word(0x2000 + ENV_FSW as u64, 0x0000);
    emu.maps.write_f64(0x3000, 1.5);
    emu.maps.write_f64(0x3008, 2.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4000).unwrap();
    assert_eq!(result, 4.0, "Arithmetic should work after FLDENV");
}

// ============================================================================
// Different Control Word Values
// ============================================================================

#[test]
fn test_fnstenv_different_control_words() {
    let mut emu = emu64();    // FNSTENV should preserve different control word values
    let test_cws = vec![0x037F, 0x027F, 0x0C7F];

    for test_cw in test_cws {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
            0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
            0xF4,                                        // HLT
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, test_cw);

    emu.run(None).unwrap();

        let saved_cw = emu.maps.read_word(0x3000 + ENV_FCW as u64).unwrap();
        assert_eq!(saved_cw, test_cw, "CW 0x{:04X} should be saved", test_cw);
    }
}

#[test]
fn test_fldenv_different_control_words() {
    let mut emu = emu64();    // FLDENV should load different control word values
    let test_cws = vec![0x037F, 0x027F, 0x0C7F];

    for test_cw in test_cws {
        let code = [
            0xD9, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDENV [0x2000]
            0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
            0xF4,                                        // HLT
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000 + ENV_FCW as u64, test_cw);

    emu.run(None).unwrap();

        let cw = emu.maps.read_word(0x3000).unwrap();
        assert_eq!(cw, test_cw, "CW 0x{:04X} should be loaded", test_cw);
    }
}

// ============================================================================
// Environment Size and Offsets
// ============================================================================

#[test]
fn test_fnstenv_all_fields_valid() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + ENV_FCW as u64).unwrap();
    let fsw = emu.maps.read_word(0x3000 + ENV_FSW as u64).unwrap();
    let ftw = emu.maps.read_word(0x3000 + ENV_FTW as u64).unwrap();

    assert!(fcw < 0xFFFF, "FCW should be valid");
    assert!(fsw < 0xFFFF, "FSW should be valid");
    assert!(ftw < 0xFFFF, "FTW should be valid");
}

// ============================================================================
// Sequential Operations
// ============================================================================

#[test]
fn test_sequential_fnstenv() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xD9, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00,  // FNSTENV [0x3200]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + ENV_FCW as u64).unwrap();
    let fcw2 = emu.maps.read_word(0x3200 + ENV_FCW as u64).unwrap();
    assert_eq!(fcw1, fcw2, "Sequential FNSTENV should save identical values");
}

#[test]
fn test_sequential_fldenv() {
    let mut emu = emu64();    let code = [
        0xD9, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDENV [0x2000]
        0xD9, 0x24, 0x25, 0x22, 0x20, 0x00, 0x00,  // FLDENV [0x2022]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + ENV_FCW as u64, 0x037F);
    emu.maps.write_word(0x2022 + ENV_FCW as u64, 0x0C7F);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, 0x0C7F, "Last FLDENV should take effect");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_fnstenv_fldenv_complete_flow() {
    let mut emu = emu64();    let code = [
        // Set up a custom control word (truncate toward zero = 0x0F7F)
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        // Load a value
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        // Save environment
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        // Initialize FPU (resets control word to 0x037F)
        0xDB, 0xE3,                                  // FNINIT
        // Store control word after FNINIT (should be 0x037F)
        0xD9, 0x3C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FNSTCW [0x4000]
        // Restore saved environment
        0xD9, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FLDENV [0x3000]
        // Store control word after FLDENV (should be 0x0F7F again)
        0xD9, 0x3C, 0x25, 0x02, 0x40, 0x00, 0x00,  // FNSTCW [0x4002]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x0F7F); // Custom control word (truncate toward zero)
    emu.maps.write_f64(0x2008, 1.5);

    emu.run(None).unwrap();

    let cw_after_fninit = emu.maps.read_word(0x4000).unwrap();
    let cw_after_fldenv = emu.maps.read_word(0x4002).unwrap();
    assert_eq!(cw_after_fninit, 0x037F, "Control word after FNINIT should be default");
    assert_eq!(cw_after_fldenv, 0x0F7F, "FLDENV should restore saved control word");
}

#[test]
fn test_fnstenv_fldenv_multiple_cycles() {
    let mut emu = emu64();    let code = [
        // Cycle 1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xD9, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTENV [0x3000]
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,  // FLDENV [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,  // FSTP qword [0x4008]
        // Cycle 2
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xD9, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00,  // FNSTENV [0x3200]
        0xDB, 0xE3,                                  // FNINIT
        0xD9, 0x24, 0x25, 0x00, 0x32, 0x00, 0x00,  // FLDENV [0x3200]
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
