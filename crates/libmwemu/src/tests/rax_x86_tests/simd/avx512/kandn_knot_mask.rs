//! Tests for AVX-512 KANDN and KNOT Mask Operations.
//!
//! This module covers AVX-512 mask bitwise operations:
//! - AND NOT operations (KANDN*)
//! - NOT operations (KNOT*)
//!
//! Instructions covered:
//! - KANDNW - AND NOT two 16-bit mask registers
//! - KANDNB - AND NOT two 8-bit mask registers
//! - KANDNQ - AND NOT two 64-bit mask registers
//! - KANDND - AND NOT two 32-bit mask registers
//! - KNOTW - NOT a 16-bit mask register
//! - KNOTB - NOT an 8-bit mask register
//! - KNOTQ - NOT a 64-bit mask register
//! - KNOTD - NOT a 32-bit mask register
//!
//! These instructions are part of AVX512F, AVX512DQ, and AVX512BW extensions.
//!
//! References: Intel SDM Vol. 2, KANDN and KNOT instruction documentation

use crate::*;

// ============================================================================
// KANDNW Tests - 16-bit Mask AND NOT
// ============================================================================

#[test]
fn test_kandnw_basic() {
    let mut emu = emu64();
    // KANDNW - AND NOT two 16-bit masks: (~k0) & k1
    // VEX.L1.0F.W0 42 /r
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00,                       // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x42, 0xD0,                       // KANDNW k2, k0, k1
        0xC5, 0xF8, 0x93, 0xC2,                             // KMOVW eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandnw_zero_masks() {
    let mut emu = emu64();
    let code = [
        0x31, 0xC0,                                         // XOR EAX, EAX
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x42, 0xD0,                       // KANDNW k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandnw_all_ones() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x42, 0xD0,                       // KANDNW k2, k0, k1 (result: 0)
        0xC5, 0xF8, 0x93, 0xC2,                             // KMOVW eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandnw_pattern() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x55, 0x55, 0x00, 0x00,                       // MOV EAX, 0x5555
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xB8, 0xAA, 0xAA, 0x00, 0x00,                       // MOV EAX, 0xAAAA
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x42, 0xD0,                       // KANDNW k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandnw_complement_behavior() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0x0F, 0x00, 0x00,                       // MOV EAX, 0x0FFF
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x42, 0xD0,                       // KANDNW k2, k0, k1 (result: 0xF000)
        0xC5, 0xF8, 0x93, 0xC2,                             // KMOVW eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KANDNB Tests - 8-bit Mask AND NOT
// ============================================================================

#[test]
fn test_kandnb_basic() {
    let mut emu = emu64();
    // KANDNB - AND NOT two 8-bit masks
    // VEX.L1.66.0F.W0 42 /r
    let code = [
        0xB8, 0x0F, 0x00, 0x00, 0x00,                       // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0,                             // KMOVB k0, eax
        0xB8, 0xFF, 0x00, 0x00, 0x00,                       // MOV EAX, 0xFF
        0xC5, 0xF9, 0x92, 0xC8,                             // KMOVB k1, eax
        0xC5, 0xF5, 0x42, 0xD0,                             // KANDNB k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandnb_complement() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xAA, 0x00, 0x00, 0x00,                       // MOV EAX, 0xAA
        0xC5, 0xF9, 0x92, 0xC0,                             // KMOVB k0, eax
        0xB8, 0x55, 0x00, 0x00, 0x00,                       // MOV EAX, 0x55
        0xC5, 0xF9, 0x92, 0xC8,                             // KMOVB k1, eax
        0xC5, 0xF5, 0x42, 0xD0,                             // KANDNB k2, k0, k1 (result: 0x55)
        0xC5, 0xF9, 0x93, 0xC2,                             // KMOVB eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandnb_masking() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xF0, 0x00, 0x00, 0x00,                       // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC0,                             // KMOVB k0, eax
        0xB8, 0xFF, 0x00, 0x00, 0x00,                       // MOV EAX, 0xFF
        0xC5, 0xF9, 0x92, 0xC8,                             // KMOVB k1, eax
        0xC5, 0xF5, 0x42, 0xD0,                             // KANDNB k2, k0, k1 (result: 0x0F)
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KANDNQ Tests - 64-bit Mask AND NOT
// ============================================================================

#[test]
fn test_kandnq_basic() {
    let mut emu = emu64();
    // KANDNQ - AND NOT two 64-bit masks
    // VEX.L1.0F.W1 42 /r
    let code = [
        0x48, 0xB8, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,  // MOV RAX, 0xFFFFFFFF
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF,           // MOV RAX, -1
        0xC4, 0xE1, 0xF8, 0x92, 0xC8,                       // KMOVQ k1, rax
        0xC4, 0xE1, 0xF4, 0x42, 0xD0,                       // KANDNQ k2, k0, k1
        0xC4, 0xE1, 0xF8, 0x93, 0xC2,                       // KMOVQ rax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandnq_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xB8, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,  // MOV RAX, 0x5555555555555555
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0x48, 0xB8, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,  // MOV RAX, 0xAAAAAAAAAAAAAAAA
        0xC4, 0xE1, 0xF8, 0x92, 0xC8,                       // KMOVQ k1, rax
        0xC4, 0xE1, 0xF4, 0x42, 0xD0,                       // KANDNQ k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KANDND Tests - 32-bit Mask AND NOT
// ============================================================================

#[test]
fn test_kandnd_basic() {
    let mut emu = emu64();
    // KANDND - AND NOT two 32-bit masks
    // VEX.L1.66.0F.W1 42 /r
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xB8, 0xFF, 0xFF, 0xFF, 0xFF,                       // MOV EAX, 0xFFFFFFFF
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x42, 0xD0,                             // KANDND k2, k0, k1
        0xC5, 0xFB, 0x93, 0xC2,                             // KMOVD eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandnd_pattern() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x55, 0x55, 0x55, 0x55,                       // MOV EAX, 0x55555555
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xB8, 0xAA, 0xAA, 0xAA, 0xAA,                       // MOV EAX, 0xAAAAAAAA
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x42, 0xD0,                             // KANDND k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KNOTW Tests - 16-bit Mask NOT
// ============================================================================

#[test]
fn test_knotw_basic() {
    let mut emu = emu64();
    // KNOTW - NOT a 16-bit mask
    // VEX.L1.0F.W0 44 /r
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00,                       // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xC4, 0xE1, 0xFC, 0x44, 0xC8,                       // KNOTW k1, k0
        0xC5, 0xF8, 0x93, 0xC1,                             // KMOVW eax, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_knotw_all_ones() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xC4, 0xE1, 0xFC, 0x44, 0xC8,                       // KNOTW k1, k0 (result: 0)
        0xC5, 0xF8, 0x93, 0xC1,                             // KMOVW eax, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_knotw_double_not() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x55, 0xAA, 0x00, 0x00,                       // MOV EAX, 0xAA55
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xC4, 0xE1, 0xFC, 0x44, 0xC8,                       // KNOTW k1, k0
        0xC4, 0xE1, 0xF4, 0x44, 0xD1,                       // KNOTW k2, k1
        0xC5, 0xF8, 0x93, 0xC2,                             // KMOVW eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KNOTB Tests - 8-bit Mask NOT
// ============================================================================

#[test]
fn test_knotb_basic() {
    let mut emu = emu64();
    // KNOTB - NOT an 8-bit mask
    // VEX.L1.66.0F.W0 44 /r
    let code = [
        0xB8, 0x0F, 0x00, 0x00, 0x00,                       // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0,                             // KMOVB k0, eax
        0xC5, 0xFD, 0x44, 0xC8,                             // KNOTB k1, k0
        0xC5, 0xF9, 0x93, 0xC1,                             // KMOVB eax, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_knotb_pattern() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xAA, 0x00, 0x00, 0x00,                       // MOV EAX, 0xAA
        0xC5, 0xF9, 0x92, 0xC0,                             // KMOVB k0, eax
        0xC5, 0xFD, 0x44, 0xC8,                             // KNOTB k1, k0 (result: 0x55)
        0xC5, 0xF9, 0x93, 0xC1,                             // KMOVB eax, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KNOTQ Tests - 64-bit Mask NOT
// ============================================================================

#[test]
fn test_knotq_basic() {
    let mut emu = emu64();
    // KNOTQ - NOT a 64-bit mask
    // VEX.L1.0F.W1 44 /r
    let code = [
        0x48, 0xB8, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,  // MOV RAX, 0xFFFFFFFF
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0xC4, 0xE1, 0xFC, 0x44, 0xC8,                       // KNOTQ k1, k0
        0xC4, 0xE1, 0xF8, 0x93, 0xC1,                       // KMOVQ rax, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_knotq_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xB8, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,  // MOV RAX, 0x5555555555555555
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0xC4, 0xE1, 0xFC, 0x44, 0xC8,                       // KNOTQ k1, k0
        0xC4, 0xE1, 0xF8, 0x93, 0xC1,                       // KMOVQ rax, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KNOTD Tests - 32-bit Mask NOT
// ============================================================================

#[test]
fn test_knotd_basic() {
    let mut emu = emu64();
    // KNOTD - NOT a 32-bit mask
    // VEX.L1.66.0F.W1 44 /r
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xC5, 0xFD, 0x44, 0xC8,                             // KNOTD k1, k0
        0xC5, 0xFB, 0x93, 0xC1,                             // KMOVD eax, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_knotd_pattern() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x55, 0x55, 0x55, 0x55,                       // MOV EAX, 0x55555555
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xC5, 0xFD, 0x44, 0xC8,                             // KNOTD k1, k0
        0xC5, 0xFB, 0x93, 0xC1,                             // KMOVD eax, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_kandn_knot_combined() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x55, 0xAA, 0x00, 0x00,                       // MOV EAX, 0xAA55
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xC4, 0xE1, 0xFC, 0x44, 0xC8,                       // KNOTW k1, k0
        0xC4, 0xE1, 0xF4, 0x42, 0xD0,                       // KANDNW k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_demorgan_laws() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00,                       // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xB8, 0xF0, 0x0F, 0x00, 0x00,                       // MOV EAX, 0x0FF0
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax

        // NOT k0
        0xC4, 0xE1, 0xFC, 0x44, 0xD0,                       // KNOTW k2, k0
        // KANDN k2, k1
        0xC4, 0xE1, 0xED, 0x42, 0xD9,                       // KANDNW k3, k2, k1

        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mask_operations_all_sizes() {
    let mut emu = emu64();
    let code = [
        // 8-bit
        0xB8, 0xF0, 0x00, 0x00, 0x00,                       // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC0,                             // KMOVB k0, eax
        0xC5, 0xFD, 0x44, 0xC8,                             // KNOTB k1, k0

        // 16-bit
        0xB8, 0xFF, 0x0F, 0x00, 0x00,                       // MOV EAX, 0x0FFF
        0xC5, 0xF8, 0x92, 0xD0,                             // KMOVW k2, eax
        0xC4, 0xE1, 0xEC, 0x44, 0xDA,                       // KNOTW k3, k2

        // 32-bit
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xFB, 0x92, 0xE0,                             // KMOVD k4, eax
        0xC5, 0xDD, 0x44, 0xE8,                             // KNOTD k5, k4

        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
