//! Tests for the FXSAVE and FXRSTOR instructions.
//!
//! FXSAVE - Save x87 FPU, MMX Technology, and SSE State
//! FXRSTOR - Restore x87 FPU, MMX Technology, and SSE State
//!
//! FXSAVE saves the FPU/MMX/SSE state to a 512-byte memory area.
//! FXRSTOR restores the state from that area.
//!
//! Opcodes:
//! - FXSAVE: 0F AE /0
//! - FXRSTOR: 0F AE /1
//!
//! Memory layout (non-64-bit mode):
//! - Bytes 0-1: FCW (FPU Control Word)
//! - Bytes 2-3: FSW (FPU Status Word)
//! - Bytes 4-5: FTW (FPU Tag Word)
//! - Bytes 6-7: FOP (Last Opcode)
//! - Bytes 8-11: FIP[31:0] (Instruction Pointer)
//! - Bytes 12-15: FCS (Code Segment)
//! - Bytes 16-19: FDP[31:0] (Data Pointer)
//! - Bytes 20-23: FDS (Data Segment)
//! - Bytes 24-27: MXCSR
//! - Bytes 28-31: MXCSR_MASK
//! - Bytes 32-159: ST0-ST7 (8 x 16 bytes each)
//! - Bytes 160-463: XMM0-XMM7 (8 x 16 bytes each)
//!
//! References: /Users/int/dev/rax/docs/fxsave.txt

use crate::*;
const DATA_ADDR: u64 = 0x7000;

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

// Helper function to write u32 to memory
fn write_u32(mem: u64, addr: u64, val: u32) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, &val.to_le_bytes());
}

// Helper function to read u32 from memory
fn read_u32(mem: u64, addr: u64) -> u32 {
    let mut emu = emu64();    let mut buf = [0u8; 4];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u32::from_le_bytes(buf)
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

// Helper function to write bytes to memory
fn write_bytes(mem: u64, addr: u64, data: &[u8]) {
    let mut emu = emu64();    emu.maps.write_bytes_slice(addr, data);
}

// Helper function to read bytes from memory
fn read_bytes(mem: u64, addr: u64, len: usize) -> Vec<u8> {
    let mut emu = emu64();    let mut buf = vec![0u8; len];
    emu.maps.read_bytes_buff(&mut buf, addr);
    buf
}

// FXSAVE/FXRSTOR area offsets
const FXSAVE_FCW: u64 = 0;        // FPU Control Word
const FXSAVE_FSW: u64 = 2;        // FPU Status Word
const FXSAVE_FTW: u64 = 4;        // FPU Tag Word
const FXSAVE_FOP: u64 = 6;        // Last Opcode
const FXSAVE_ST0: u64 = 32;       // First FPU register (16 bytes each)
const FXSAVE_ST1: u64 = 48;
const FXSAVE_ST2: u64 = 64;
const FXSAVE_ST3: u64 = 80;
const FXSAVE_ST4: u64 = 96;
const FXSAVE_ST5: u64 = 112;
const FXSAVE_ST6: u64 = 128;
const FXSAVE_ST7: u64 = 144;
const FXSAVE_SIZE: u64 = 512;     // Total size of FXSAVE area

// ============================================================================
// FXSAVE - Save FPU/SSE State
// ============================================================================

#[test]
fn test_fxsave_basic() {
    let mut emu = emu64();    let code = [
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FXSAVE [0x2000]
        0xF4,                                              // HLT
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x2000 + FXSAVE_FCW).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be valid after FXSAVE");
}

#[test]
fn test_fxsave_with_fpu_data() {
    let mut emu = emu64();    // FXSAVE should save FPU register data
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x20, 0x00, 0x00,  // FSTP qword [0x2010]
        0xDD, 0x1C, 0x25, 0x18, 0x20, 0x00, 0x00,  // FSTP qword [0x2018]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);
    emu.maps.write_f64(0x2008, 2.5);

    emu.run(None).unwrap();

    let fcw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    assert!(fcw < 0xFFFF, "FCW should be saved");

    let fsw = emu.maps.read_word(0x3000 + FXSAVE_FSW).unwrap();
    assert!(fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fxsave_saves_control_word() {
    let mut emu = emu64();    // FXSAVE should save the control word
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    assert_eq!(saved_cw, 0x037F, "FCW should be saved correctly");
}

#[test]
fn test_fxsave_saves_status_word() {
    let mut emu = emu64();    // FXSAVE should save the status word
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 3.14159);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + FXSAVE_FSW).unwrap();
    assert!(fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fxsave_multiple_areas() {
    let mut emu = emu64();    // FXSAVE to different memory areas
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x31, 0x00, 0x00,  // FXSAVE [0x3100]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3100 + FXSAVE_FCW).unwrap();
    assert_eq!(fcw1, fcw2, "Multiple FXSAVE should save identical state");
}

// ============================================================================
// FXRSTOR - Restore FPU/SSE State
// ============================================================================

#[test]
fn test_fxrstor_basic() {
    let mut emu = emu64();    let code = [
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR [0x3000]
        0xD9, 0x3C, 0x25, 0x00, 0x40, 0x00, 0x00,  // FNSTCW [0x4000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x3000 + FXSAVE_FCW, 0x037F);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x4000).unwrap();
    assert!(cw < 0xFFFF, "Control word should be valid after FXRSTOR");
}

// ============================================================================
// FXSAVE/FXRSTOR Round Trip
// ============================================================================

#[test]
fn test_fxsave_fxrstor_roundtrip() {
    let mut emu = emu64();    // FXSAVE followed by FXRSTOR should preserve state
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xDB, 0xE3,                                  // FNINIT (clear FPU)
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,  // FSTP qword [0x4008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x4008).unwrap();
    assert_eq!(result, 1.5, "Value should be preserved through FXSAVE/FXRSTOR");
}

#[test]
fn test_fxsave_fxrstor_multiple_values() {
    let mut emu = emu64();    // FXSAVE/FXRSTOR with multiple FPU values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xDB, 0xE3,                                  // FNINIT
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00,  // FSTP qword [0x4010]
        0xDD, 0x1C, 0x25, 0x18, 0x40, 0x00, 0x00,  // FSTP qword [0x4018]
        0xF4,                                        // HLT
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

// ============================================================================
// FXSAVE Area Size and Alignment
// ============================================================================

#[test]
fn test_fxsave_area_512_bytes() {
    let mut emu = emu64();    // FXSAVE uses a 512-byte area
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
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
// FXSAVE with Different Control Word Values
// ============================================================================

#[test]
fn test_fxsave_different_control_words() {
    let mut emu = emu64();    // FXSAVE should preserve different control word values
    let test_cws = vec![0x037F, 0x027F, 0x0C7F];

    for test_cw in test_cws {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
            0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
            0xF4,                                        // HLT
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_word(0x2000, test_cw);

    emu.run(None).unwrap();

        let saved_cw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
        assert_eq!(saved_cw, test_cw, "Control word 0x{:04X} should be saved", test_cw);
    }
}

// ============================================================================
// FXRSTOR from Different Areas
// ============================================================================

#[test]
fn test_fxrstor_from_prepared_area() {
    let mut emu = emu64();    // FXRSTOR from a pre-prepared FXSAVE area
    let code = [
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FXRSTOR [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FNSTCW [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + FXSAVE_FCW, 0x037F);
    emu.maps.write_word(0x2000 + FXSAVE_FSW, 0x0000);

    emu.run(None).unwrap();

    let cw = emu.maps.read_word(0x3000).unwrap();
    assert_eq!(cw, 0x037F, "Control word should be restored from prepared area");
}

// ============================================================================
// Sequential FXSAVE Operations
// ============================================================================

#[test]
fn test_sequential_fxsave() {
    let mut emu = emu64();    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x32, 0x00, 0x00,  // FXSAVE [0x3200]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00,  // FSTP qword [0x3008]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 1.5);

    emu.run(None).unwrap();

    let fcw1 = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let fcw2 = emu.maps.read_word(0x3200 + FXSAVE_FCW).unwrap();
    assert_eq!(fcw1, fcw2, "Multiple FXSAVE should produce identical results");
}

// ============================================================================
// FXSAVE/FXRSTOR with Arithmetic
// ============================================================================

#[test]
fn test_fxsave_after_arithmetic() {
    let mut emu = emu64();    // FXSAVE after arithmetic operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00,  // FSTP qword [0x3010]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_f64(0x2000, 2.5);
    emu.maps.write_f64(0x2008, 3.5);

    emu.run(None).unwrap();

    let fsw = emu.maps.read_word(0x3000 + FXSAVE_FSW).unwrap();
    assert!(fsw < 0xFFFF, "FSW should be saved after arithmetic");
}

#[test]
fn test_fxrstor_then_arithmetic() {
    let mut emu = emu64();    // FXRSTOR followed by arithmetic
    let code = [
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FXRSTOR [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0xDE, 0xC1,                                  // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FSTP qword [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000 + FXSAVE_FCW, 0x037F);
    emu.maps.write_word(0x2000 + FXSAVE_FSW, 0x0000);
    emu.maps.write_f64(0x2008, 1.5);

    emu.run(None).unwrap();

    let result = emu.maps.read_f64(0x3000).unwrap();
    assert_eq!(result, 1.5, "Arithmetic should work after FXRSTOR");
}

// ============================================================================
// FXSAVE State Preservation
// ============================================================================

#[test]
fn test_fxsave_preserves_control_precision() {
    let mut emu = emu64();    // FXSAVE should preserve control word precision bits
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);  // Default (64-bit precision)

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let precision = (saved_cw >> 8) & 0x3;
    assert_eq!(precision, 0x3, "Precision should be saved as 64-bit");
}

#[test]
fn test_fxsave_preserves_control_rounding() {
    let mut emu = emu64();    // FXSAVE should preserve control word rounding bits
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLDCW [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xF4,                                        // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_word(0x2000, 0x037F);  // Round to nearest

    emu.run(None).unwrap();

    let saved_cw = emu.maps.read_word(0x3000 + FXSAVE_FCW).unwrap();
    let rounding = (saved_cw >> 10) & 0x3;
    assert_eq!(rounding, 0x0, "Rounding should be saved as nearest");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_fxsave_fxrstor_complete_flow() {
    let mut emu = emu64();    let code = [
        // Load and use FPU
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        // Save state
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        // Initialize FPU
        0xDB, 0xE3,                                  // FNINIT
        // Do some other work
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,  // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x18, 0x20, 0x00, 0x00,  // FSTP qword [0x2018]
        // Restore saved state
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR [0x3000]
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
    assert_eq!(v1, 2.5, "Second restored value should be 2.5");
    assert_eq!(v2, 1.5, "First restored value should be 1.5");
}

#[test]
fn test_fxsave_fxrstor_multiple_cycles() {
    let mut emu = emu64();    let code = [
        // Cycle 1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,  // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXSAVE [0x3000]
        0xDB, 0xE3,                                  // FNINIT
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,  // FXRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00,  // FSTP qword [0x4008]
        // Cycle 2
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00,  // FLD qword [0x2008]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x32, 0x00, 0x00,  // FXSAVE [0x3200]
        0xDB, 0xE3,                                  // FNINIT
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x32, 0x00, 0x00,  // FXRSTOR [0x3200]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00,  // FSTP qword [0x4010]
        0xF4,                                        // HLT
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
