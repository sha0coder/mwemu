//! Tests for AVX-512 KADD Mask Addition Instructions.
//!
//! This module covers the AVX-512 mask addition operations that add two mask
//! registers and store the result in a destination mask register.
//!
//! Instructions covered:
//! - KADDW - Add two 16-bit mask registers
//! - KADDB - Add two 8-bit mask registers
//! - KADDQ - Add two 64-bit mask registers
//! - KADDD - Add two 32-bit mask registers
//!
//! These instructions are part of AVX512DQ and AVX512BW extensions.
//!
//! References: Intel SDM Vol. 2, KADD instruction documentation

use crate::*;

// ============================================================================
// KADDW Tests - 16-bit Mask Addition
// ============================================================================

#[test]
fn test_kaddw_basic() {
    let mut emu = emu64();
    // KADDW - Add two 16-bit masks
    // VEX.L1.0F.W0 4A /r
    let code = [
        0xC5, 0xF8, 0x90, 0xC1,                             // KMOVW k0, ecx (pseudo: set k0)
        0xC5, 0xF8, 0x90, 0xCA,                             // KMOVW k1, edx (pseudo: set k1)
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDW k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddw_zero_masks() {
    let mut emu = emu64();
    let code = [
        0xC5, 0xF8, 0x91, 0xC0,                             // KMOVW eax, k0 (clear k0)
        0xC5, 0xF8, 0x91, 0xC1,                             // KMOVW ecx, k1 (clear k1)
        0xC4, 0xE1, 0xFC, 0x4A, 0xD0,                       // KADDW k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddw_all_ones() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDW k2, k0, k1
        0xC5, 0xF8, 0x93, 0xC2,                             // KMOVW eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddw_pattern_5555_aaaa() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x55, 0x55, 0x00, 0x00,                       // MOV EAX, 0x5555
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xB8, 0xAA, 0xAA, 0x00, 0x00,                       // MOV EAX, 0xAAAA
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDW k2, k0, k1
        0xC5, 0xF8, 0x93, 0xC2,                             // KMOVW eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddw_overflow() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0x7F, 0x00, 0x00,                       // MOV EAX, 0x7FFF
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xB8, 0x02, 0x00, 0x00, 0x00,                       // MOV EAX, 2
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDW k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x11, 0x11, 0x00, 0x00,                       // MOV EAX, 0x1111
        0xC5, 0xF8, 0x92, 0xC0,                             // KMOVW k0, eax
        0xC5, 0xF8, 0x92, 0xC8,                             // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDW k2, k0, k1
        0xC4, 0xE1, 0xEC, 0x4A, 0xD8,                       // KADDW k3, k2, k0
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KADDB Tests - 8-bit Mask Addition
// ============================================================================

#[test]
fn test_kaddb_basic() {
    let mut emu = emu64();
    // KADDB - Add two 8-bit masks
    // VEX.L1.66.0F.W0 4A /r
    let code = [
        0xB8, 0x0F, 0x00, 0x00, 0x00,                       // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0,                             // KADDB k0, eax
        0xB8, 0xF0, 0x00, 0x00, 0x00,                       // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC8,                             // KADDB k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDB k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddb_zero_result() {
    let mut emu = emu64();
    let code = [
        0x31, 0xC0,                                         // XOR EAX, EAX
        0xC5, 0xF9, 0x92, 0xC0,                             // KADDB k0, eax
        0xC5, 0xF9, 0x92, 0xC8,                             // KADDB k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDB k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddb_max_value() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00,                       // MOV EAX, 0xFF
        0xC5, 0xF9, 0x92, 0xC0,                             // KADDB k0, eax
        0xC5, 0xF9, 0x92, 0xC8,                             // KADDB k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDB k2, k0, k1
        0xC5, 0xF9, 0x93, 0xC2,                             // KADDB eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddb_bit_patterns() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xAA, 0x00, 0x00, 0x00,                       // MOV EAX, 0xAA
        0xC5, 0xF9, 0x92, 0xC0,                             // KADDB k0, eax
        0xB8, 0x55, 0x00, 0x00, 0x00,                       // MOV EAX, 0x55
        0xC5, 0xF9, 0x92, 0xC8,                             // KADDB k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDB k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddb_carry_behavior() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x80, 0x00, 0x00, 0x00,                       // MOV EAX, 0x80
        0xC5, 0xF9, 0x92, 0xC0,                             // KADDB k0, eax
        0xB8, 0x81, 0x00, 0x00, 0x00,                       // MOV EAX, 0x81
        0xC5, 0xF9, 0x92, 0xC8,                             // KADDB k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDB k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KADDQ Tests - 64-bit Mask Addition
// ============================================================================

#[test]
fn test_kaddq_basic() {
    let mut emu = emu64();
    // KADDQ - Add two 64-bit masks
    // VEX.L1.0F.W1 4A /r
    let code = [
        0x48, 0xB8, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,  // MOV RAX, 0xFFFFFFFF
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,  // MOV RAX, 0xFFFFFFFF00000000
        0xC4, 0xE1, 0xF8, 0x92, 0xC8,                       // KMOVQ k1, rax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDQ k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddq_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0x31, 0xC0,                                   // XOR RAX, RAX
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0xC4, 0xE1, 0xF8, 0x92, 0xC8,                       // KMOVQ k1, rax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDQ k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddq_all_ones() {
    let mut emu = emu64();
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF,           // MOV RAX, -1
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0xC4, 0xE1, 0xF8, 0x92, 0xC8,                       // KMOVQ k1, rax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDQ k2, k0, k1
        0xC4, 0xE1, 0xF8, 0x93, 0xC2,                       // KMOVQ rax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddq_large_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,  // MOV RAX, 0x8000000000000000
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0x48, 0xB8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // MOV RAX, 1
        0xC4, 0xE1, 0xF8, 0x92, 0xC8,                       // KMOVQ k1, rax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDQ k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddq_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xB8, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,  // MOV RAX, 0x5555555555555555
        0xC4, 0xE1, 0xF8, 0x92, 0xC0,                       // KMOVQ k0, rax
        0x48, 0xB8, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,  // MOV RAX, 0xAAAAAAAAAAAAAAAA
        0xC4, 0xE1, 0xF8, 0x92, 0xC8,                       // KMOVQ k1, rax
        0xC4, 0xE1, 0xF4, 0x4A, 0xD0,                       // KADDQ k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KADDD Tests - 32-bit Mask Addition
// ============================================================================

#[test]
fn test_kaddd_basic() {
    let mut emu = emu64();
    // KADDD - Add two 32-bit masks
    // VEX.L1.66.0F.W1 4A /r
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xB8, 0x00, 0x00, 0xFF, 0xFF,                       // MOV EAX, 0xFFFF0000
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDD k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddd_zero() {
    let mut emu = emu64();
    let code = [
        0x31, 0xC0,                                         // XOR EAX, EAX
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDD k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddd_max_value() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0xFF, 0xFF, 0xFF,                       // MOV EAX, 0xFFFFFFFF
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDD k2, k0, k1
        0xC5, 0xFB, 0x93, 0xC2,                             // KMOVD eax, k2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddd_alternating_bits() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x55, 0x55, 0x55, 0x55,                       // MOV EAX, 0x55555555
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xB8, 0xAA, 0xAA, 0xAA, 0xAA,                       // MOV EAX, 0xAAAAAAAA
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDD k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddd_overflow() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0xFF, 0xFF, 0x7F,                       // MOV EAX, 0x7FFFFFFF
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xB8, 0x02, 0x00, 0x00, 0x00,                       // MOV EAX, 2
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDD k2, k0, k1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kaddd_multiple_operations() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x11, 0x11, 0x11, 0x11,                       // MOV EAX, 0x11111111
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xB8, 0x22, 0x22, 0x22, 0x22,                       // MOV EAX, 0x22222222
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDD k2, k0, k1
        0xB8, 0x33, 0x33, 0x33, 0x33,                       // MOV EAX, 0x33333333
        0xC5, 0xFB, 0x92, 0xD8,                             // KMOVD k3, eax
        0xC5, 0xED, 0x4A, 0xE2,                             // KADDD k4, k2, k3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Size Tests
// ============================================================================

#[test]
fn test_kadd_mixed_operations() {
    let mut emu = emu64();
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00,                       // MOV EAX, 0xFF
        0xC5, 0xF9, 0x92, 0xC0,                             // KADDB k0, eax
        0xC5, 0xF9, 0x92, 0xC8,                             // KADDB k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDB k2, k0, k1
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xD8,                             // KADDW k3, eax
        0xC5, 0xF8, 0x92, 0xE0,                             // KADDW k4, eax
        0xC4, 0xE1, 0xDC, 0x4A, 0xEB,                       // KADDW k5, k3, k4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kadd_sequential_chain() {
    let mut emu = emu64();
    let code = [
        0xB8, 0x01, 0x00, 0x00, 0x00,                       // MOV EAX, 1
        0xC5, 0xFB, 0x92, 0xC0,                             // KMOVD k0, eax
        0xC5, 0xFB, 0x92, 0xC8,                             // KMOVD k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDD k2, k0, k1
        0xC5, 0xED, 0x4A, 0xD8,                             // KADDD k3, k2, k0
        0xC5, 0xE5, 0x4A, 0xE0,                             // KADDD k4, k3, k0
        0xC5, 0xDD, 0x4A, 0xE8,                             // KADDD k5, k4, k0
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kadd_all_sizes_comprehensive() {
    let mut emu = emu64();
    let code = [
        // KADDB
        0xB8, 0x0F, 0x00, 0x00, 0x00,                       // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0,                             // KADDB k0, eax
        0xB8, 0xF0, 0x00, 0x00, 0x00,                       // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC8,                             // KADDB k1, eax
        0xC5, 0xF5, 0x4A, 0xD0,                             // KADDB k2, k0, k1

        // KADDW
        0xB8, 0xFF, 0x0F, 0x00, 0x00,                       // MOV EAX, 0x0FFF
        0xC5, 0xF8, 0x92, 0xD8,                             // KADDW k3, eax
        0xB8, 0x00, 0xF0, 0x00, 0x00,                       // MOV EAX, 0xF000
        0xC5, 0xF8, 0x92, 0xE0,                             // KADDW k4, eax
        0xC4, 0xE1, 0xDC, 0x4A, 0xEB,                       // KADDW k5, k3, k4

        // KADDD
        0xB8, 0xFF, 0xFF, 0x00, 0x00,                       // MOV EAX, 0xFFFF
        0xC5, 0xFB, 0x92, 0xF0,                             // KMOVD k6, eax
        0xB8, 0x00, 0x00, 0xFF, 0xFF,                       // MOV EAX, 0xFFFF0000
        0xC5, 0xFB, 0x92, 0xF8,                             // KMOVD k7, eax
        0xC4, 0xC1, 0x4D, 0x4A, 0xC7,                       // KADDD k0, k6, k7

        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
