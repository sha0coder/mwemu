use crate::*;

// LDMXCSR - Load MXCSR Register
// STMXCSR - Store MXCSR Register State
//
// LDMXCSR loads the MXCSR control/status register from a 32-bit memory location
// STMXCSR stores the MXCSR control/status register to a 32-bit memory location
//
// MXCSR Register layout:
// Bits 0-5:   Exception flags (IE, DE, ZE, OE, UE, PE)
// Bits 6:     Denormals Are Zeros (DAZ)
// Bits 7-12:  Exception masks (IM, DM, ZM, OM, UM, PM)
// Bits 13-14: Rounding control (00=nearest, 01=down, 10=up, 11=toward zero)
// Bit 15:     Flush to Zero (FTZ)
//
// Default MXCSR value at reset: 0x1F80
//
// Opcodes:
// NP 0F AE /2             LDMXCSR m32    - Load MXCSR from m32
// NP 0F AE /3             STMXCSR m32    - Store MXCSR to m32

const MXCSR_ADDR: u64 = 0x3000; // Address for MXCSR save/restore

// ============================================================================
// STMXCSR Tests - Store MXCSR Register
// ============================================================================

#[test]
fn test_stmxcsr_basic() {
    let mut emu = emu64();
    // STMXCSR [0x3000]
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_stmxcsr_default_value() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_stmxcsr_multiple_stores() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0x0f, 0xae, 0x1c, 0x25, 0x04, 0x30, 0x00, 0x00, // STMXCSR [0x3004]
        0x0f, 0xae, 0x1c, 0x25, 0x08, 0x30, 0x00, 0x00, // STMXCSR [0x3008]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_stmxcsr_different_addresses() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x40, 0x00, 0x00, // STMXCSR [0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// LDMXCSR Tests - Load MXCSR Register
// ============================================================================

#[test]
fn test_ldmxcsr_basic() {
    let mut emu = emu64();
    // LDMXCSR [0x3000]
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_default_value() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_different_addresses() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x40, 0x00, 0x00, // LDMXCSR [0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// LDMXCSR/STMXCSR Combined Tests
// ============================================================================

#[test]
fn test_stmxcsr_ldmxcsr_roundtrip() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mxcsr_save_restore() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000] (save)
        0x0f, 0xae, 0x14, 0x25, 0x04, 0x30, 0x00, 0x00, // LDMXCSR [0x3004] (load new)
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000] (restore)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Rounding Mode Tests
// ============================================================================

#[test]
fn test_ldmxcsr_rounding_nearest() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_rounding_down() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_rounding_up() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_rounding_toward_zero() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Exception Mask Tests
// ============================================================================

#[test]
fn test_ldmxcsr_all_exceptions_masked() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_no_exceptions_masked() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_invalid_operation_mask() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_divide_by_zero_mask() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_overflow_mask() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_underflow_mask() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_precision_mask() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Flush to Zero and Denormals Are Zero Tests
// ============================================================================

#[test]
fn test_ldmxcsr_flush_to_zero_enabled() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_flush_to_zero_disabled() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_denormals_are_zero_enabled() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_denormals_are_zero_disabled() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_ftz_and_daz_enabled() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Exception Flag Tests
// ============================================================================

#[test]
fn test_stmxcsr_with_exception_flags() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_clear_exception_flags() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Mode Tests
// ============================================================================

#[test]
fn test_ldmxcsr_custom_configuration() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mxcsr_state_preservation() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000] (save)
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000] (restore)
        0x0f, 0xae, 0x1c, 0x25, 0x04, 0x30, 0x00, 0x00, // STMXCSR [0x3004] (verify)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_stmxcsr_addr2() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x10, 0x30, 0x00, 0x00, // STMXCSR [0x3010]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_addr2() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // LDMXCSR [0x3010]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_denormal_mask() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_stmxcsr_sequential() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_sequential() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_rounding_mode_change() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0x0f, 0xae, 0x1c, 0x25, 0x04, 0x30, 0x00, 0x00, // STMXCSR [0x3004]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
