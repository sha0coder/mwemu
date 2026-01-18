//! Tests for AVX-512 Specialized Instructions.
//!
//! This module covers specialized AVX-512 operations including sum-of-absolute-differences,
//! leading zero count, and double-precision shift operations.
//!
//! Instructions covered:
//! - VDBPSADBW - Double Block Packed Sum-Absolute-Differences on unsigned bytes
//! - VPLZCNTD - Count leading zero bits in dwords
//! - VPLZCNTQ - Count leading zero bits in qwords
//! - VPSHLDW - Concatenate and shift packed words left
//! - VPSHLDD - Concatenate and shift packed dwords left
//! - VPSHLDQ - Concatenate and shift packed qwords left
//! - VPSHLDVW - Variable shift packed words left
//! - VPSHLDVD - Variable shift packed dwords left
//! - VPSHLDVQ - Variable shift packed qwords left
//! - VPSHRDW - Concatenate and shift packed words right
//! - VPSHRDD - Concatenate and shift packed dwords right
//! - VPSHRDQ - Concatenate and shift packed qwords right
//! - VPSHRDVW - Variable shift packed words right
//! - VPSHRDVD - Variable shift packed dwords right
//! - VPSHRDVQ - Variable shift packed qwords right
//!
//! These instructions are part of AVX512BW, AVX512CD, and AVX512VBMI2 extensions.
//!
//! References: Intel SDM Vol. 2, AVX-512 specialized instruction documentation

use crate::*;

// ============================================================================
// VDBPSADBW Tests - Double Block Packed SAD
// ============================================================================

#[test]
fn test_vdbpsadbw_xmm_basic() {
    let mut emu = emu64();
    // VDBPSADBW - Compute packed SAD on unsigned bytes (XMM)
    // EVEX.128.66.0F3A.W0 42 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x08, 0x42, 0xC1, 0x00,           // VDBPSADBW xmm0, xmm2, xmm1, 0
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdbpsadbw_ymm_basic() {
    let mut emu = emu64();
    // VDBPSADBW - Compute packed SAD (YMM)
    // EVEX.256.66.0F3A.W0 42 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x28, 0x42, 0xC2, 0xAA,           // VDBPSADBW ymm0, ymm2, ymm2, 0xAA
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdbpsadbw_zmm_basic() {
    let mut emu = emu64();
    // VDBPSADBW - Compute packed SAD (ZMM)
    // EVEX.512.66.0F3A.W0 42 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0xC3, 0x55,           // VDBPSADBW zmm0, zmm2, zmm3, 0x55
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdbpsadbw_zmm_memory() {
    let mut emu = emu64();
    // VDBPSADBW from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00,           // MOV RAX, 0x1000
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0x00, 0xFF,           // VDBPSADBW zmm0, zmm2, [rax], 0xFF
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdbpsadbw_shuffle_control() {
    let mut emu = emu64();
    let code = [
        // imm8 = 0xE4 (11 10 01 00)
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0xC1, 0xE4,           // VDBPSADBW zmm0, zmm2, zmm1, 0xE4

        // imm8 = 0x1B (00 01 10 11)
        0x62, 0xF3, 0x65, 0x48, 0x42, 0xCA, 0x1B,           // VDBPSADBW zmm1, zmm3, zmm2, 0x1B

        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdbpsadbw_identity_shuffle() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0xC1, 0xE4,           // VDBPSADBW zmm0, zmm2, zmm1, 0xE4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPLZCNTD Tests - Count Leading Zeros Dword
// ============================================================================

#[test]
fn test_vplzcntd_xmm_basic() {
    let mut emu = emu64();
    // VPLZCNTD - Count leading zero bits in dwords (XMM)
    // EVEX.128.66.0F38.W0 44 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x44, 0xC1,                 // VPLZCNTD xmm0, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vplzcntd_ymm_basic() {
    let mut emu = emu64();
    // VPLZCNTD - Count leading zeros (YMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x28, 0x44, 0xC2,                 // VPLZCNTD ymm0, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vplzcntd_zmm_basic() {
    let mut emu = emu64();
    // VPLZCNTD - Count leading zeros (ZMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x44, 0xC3,                 // VPLZCNTD zmm0, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vplzcntd_zmm_memory() {
    let mut emu = emu64();
    // VPLZCNTD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00,           // MOV RAX, 0x2000
        0x62, 0xF2, 0x7D, 0x48, 0x44, 0x00,                 // VPLZCNTD zmm0, [rax]
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vplzcntd_broadcast() {
    let mut emu = emu64();
    // VPLZCNTD with broadcast
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00,           // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x58, 0x44, 0x00,                 // VPLZCNTD zmm0, dword ptr [rax]{1to16}
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPLZCNTQ Tests - Count Leading Zeros Qword
// ============================================================================

#[test]
fn test_vplzcntq_xmm_basic() {
    let mut emu = emu64();
    // VPLZCNTQ - Count leading zero bits in qwords (XMM)
    // EVEX.128.66.0F38.W1 44 /r
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x44, 0xC1,                 // VPLZCNTQ xmm0, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vplzcntq_ymm_basic() {
    let mut emu = emu64();
    // VPLZCNTQ - Count leading zeros (YMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x28, 0x44, 0xC2,                 // VPLZCNTQ ymm0, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vplzcntq_zmm_basic() {
    let mut emu = emu64();
    // VPLZCNTQ - Count leading zeros (ZMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x44, 0xC3,                 // VPLZCNTQ zmm0, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vplzcntq_zmm_memory() {
    let mut emu = emu64();
    // VPLZCNTQ from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00,           // MOV RAX, 0x4000
        0x62, 0xF2, 0xFD, 0x48, 0x44, 0x00,                 // VPLZCNTQ zmm0, [rax]
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHLDW Tests - Shift Left Words
// ============================================================================

#[test]
fn test_vpshldw_xmm_basic() {
    let mut emu = emu64();
    // VPSHLDW - Concatenate and shift left words (XMM)
    // EVEX.128.66.0F3A.W1 70 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x70, 0xC1, 0x08,           // VPSHLDW xmm0, xmm2, xmm1, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldw_ymm_basic() {
    let mut emu = emu64();
    // VPSHLDW - Shift left words (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x70, 0xC2, 0x04,           // VPSHLDW ymm0, ymm2, ymm2, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldw_zmm_basic() {
    let mut emu = emu64();
    // VPSHLDW - Shift left words (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x70, 0xC3, 0x0C,           // VPSHLDW zmm0, zmm2, zmm3, 12
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHLDD Tests - Shift Left Dwords
// ============================================================================

#[test]
fn test_vpshldd_xmm_basic() {
    let mut emu = emu64();
    // VPSHLDD - Concatenate and shift left dwords (XMM)
    // EVEX.128.66.0F3A.W0 71 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x08, 0x71, 0xC1, 0x08,           // VPSHLDD xmm0, xmm2, xmm1, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldd_ymm_basic() {
    let mut emu = emu64();
    // VPSHLDD - Shift left dwords (YMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x28, 0x71, 0xC2, 0x10,           // VPSHLDD ymm0, ymm2, ymm2, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldd_zmm_basic() {
    let mut emu = emu64();
    // VPSHLDD - Shift left dwords (ZMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x71, 0xC3, 0x04,           // VPSHLDD zmm0, zmm2, zmm3, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldd_zmm_memory() {
    let mut emu = emu64();
    // VPSHLDD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00,           // MOV RAX, 0x5000
        0x62, 0xF3, 0x6D, 0x48, 0x71, 0x00, 0x10,           // VPSHLDD zmm0, zmm2, [rax], 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHLDQ Tests - Shift Left Qwords
// ============================================================================

#[test]
fn test_vpshldq_xmm_basic() {
    let mut emu = emu64();
    // VPSHLDQ - Concatenate and shift left qwords (XMM)
    // EVEX.128.66.0F3A.W1 71 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x71, 0xC1, 0x10,           // VPSHLDQ xmm0, xmm2, xmm1, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldq_ymm_basic() {
    let mut emu = emu64();
    // VPSHLDQ - Shift left qwords (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x71, 0xC2, 0x20,           // VPSHLDQ ymm0, ymm2, ymm2, 32
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldq_zmm_basic() {
    let mut emu = emu64();
    // VPSHLDQ - Shift left qwords (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x71, 0xC3, 0x08,           // VPSHLDQ zmm0, zmm2, zmm3, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHLDVW Tests - Variable Shift Left Words
// ============================================================================

#[test]
fn test_vpshldvw_xmm_basic() {
    let mut emu = emu64();
    // VPSHLDVW - Variable shift left words (XMM)
    // EVEX.128.66.0F38.W1 70 /r
    let code = [
        0x62, 0xF2, 0xED, 0x08, 0x70, 0xC1,                 // VPSHLDVW xmm0, xmm2, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldvw_ymm_basic() {
    let mut emu = emu64();
    // VPSHLDVW - Variable shift left words (YMM)
    let code = [
        0x62, 0xF2, 0xED, 0x28, 0x70, 0xC2,                 // VPSHLDVW ymm0, ymm2, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldvw_zmm_basic() {
    let mut emu = emu64();
    // VPSHLDVW - Variable shift left words (ZMM)
    let code = [
        0x62, 0xF2, 0xED, 0x48, 0x70, 0xC3,                 // VPSHLDVW zmm0, zmm2, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHLDVD Tests - Variable Shift Left Dwords
// ============================================================================

#[test]
fn test_vpshldvd_xmm_basic() {
    let mut emu = emu64();
    // VPSHLDVD - Variable shift left dwords (XMM)
    // EVEX.128.66.0F38.W0 71 /r
    let code = [
        0x62, 0xF2, 0x6D, 0x08, 0x71, 0xC1,                 // VPSHLDVD xmm0, xmm2, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldvd_ymm_basic() {
    let mut emu = emu64();
    // VPSHLDVD - Variable shift left dwords (YMM)
    let code = [
        0x62, 0xF2, 0x6D, 0x28, 0x71, 0xC2,                 // VPSHLDVD ymm0, ymm2, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldvd_zmm_basic() {
    let mut emu = emu64();
    // VPSHLDVD - Variable shift left dwords (ZMM)
    let code = [
        0x62, 0xF2, 0x6D, 0x48, 0x71, 0xC3,                 // VPSHLDVD zmm0, zmm2, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHLDVQ Tests - Variable Shift Left Qwords
// ============================================================================

#[test]
fn test_vpshldvq_xmm_basic() {
    let mut emu = emu64();
    // VPSHLDVQ - Variable shift left qwords (XMM)
    // EVEX.128.66.0F38.W1 71 /r
    let code = [
        0x62, 0xF2, 0xED, 0x08, 0x71, 0xC1,                 // VPSHLDVQ xmm0, xmm2, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldvq_ymm_basic() {
    let mut emu = emu64();
    // VPSHLDVQ - Variable shift left qwords (YMM)
    let code = [
        0x62, 0xF2, 0xED, 0x28, 0x71, 0xC2,                 // VPSHLDVQ ymm0, ymm2, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshldvq_zmm_basic() {
    let mut emu = emu64();
    // VPSHLDVQ - Variable shift left qwords (ZMM)
    let code = [
        0x62, 0xF2, 0xED, 0x48, 0x71, 0xC3,                 // VPSHLDVQ zmm0, zmm2, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHRDW Tests - Shift Right Words
// ============================================================================

#[test]
fn test_vpshrdw_xmm_basic() {
    let mut emu = emu64();
    // VPSHRDW - Concatenate and shift right words (XMM)
    // EVEX.128.66.0F3A.W1 72 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x72, 0xC1, 0x08,           // VPSHRDW xmm0, xmm2, xmm1, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshrdw_ymm_basic() {
    let mut emu = emu64();
    // VPSHRDW - Shift right words (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x72, 0xC2, 0x04,           // VPSHRDW ymm0, ymm2, ymm2, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshrdw_zmm_basic() {
    let mut emu = emu64();
    // VPSHRDW - Shift right words (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x72, 0xC3, 0x0C,           // VPSHRDW zmm0, zmm2, zmm3, 12
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHRDD Tests - Shift Right Dwords
// ============================================================================

#[test]
fn test_vpshrdd_xmm_basic() {
    let mut emu = emu64();
    // VPSHRDD - Concatenate and shift right dwords (XMM)
    // EVEX.128.66.0F3A.W0 73 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x08, 0x73, 0xC1, 0x08,           // VPSHRDD xmm0, xmm2, xmm1, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshrdd_ymm_basic() {
    let mut emu = emu64();
    // VPSHRDD - Shift right dwords (YMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x28, 0x73, 0xC2, 0x10,           // VPSHRDD ymm0, ymm2, ymm2, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshrdd_zmm_basic() {
    let mut emu = emu64();
    // VPSHRDD - Shift right dwords (ZMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x73, 0xC3, 0x04,           // VPSHRDD zmm0, zmm2, zmm3, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHRDQ Tests - Shift Right Qwords
// ============================================================================

#[test]
fn test_vpshrdq_xmm_basic() {
    let mut emu = emu64();
    // VPSHRDQ - Concatenate and shift right qwords (XMM)
    // EVEX.128.66.0F3A.W1 73 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x73, 0xC1, 0x10,           // VPSHRDQ xmm0, xmm2, xmm1, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshrdq_ymm_basic() {
    let mut emu = emu64();
    // VPSHRDQ - Shift right qwords (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x73, 0xC2, 0x20,           // VPSHRDQ ymm0, ymm2, ymm2, 32
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshrdq_zmm_basic() {
    let mut emu = emu64();
    // VPSHRDQ - Shift right qwords (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x73, 0xC3, 0x08,           // VPSHRDQ zmm0, zmm2, zmm3, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHRDVW Tests - Variable Shift Right Words
// ============================================================================

#[test]
fn test_vpshrdvw_zmm_basic() {
    let mut emu = emu64();
    // VPSHRDVW - Variable shift right words (ZMM)
    // EVEX.512.66.0F38.W1 72 /r
    let code = [
        0x62, 0xF2, 0xED, 0x48, 0x72, 0xC3,                 // VPSHRDVW zmm0, zmm2, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHRDVD Tests - Variable Shift Right Dwords
// ============================================================================

#[test]
fn test_vpshrdvd_zmm_basic() {
    let mut emu = emu64();
    // VPSHRDVD - Variable shift right dwords (ZMM)
    // EVEX.512.66.0F38.W0 73 /r
    let code = [
        0x62, 0xF2, 0x6D, 0x48, 0x73, 0xC3,                 // VPSHRDVD zmm0, zmm2, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSHRDVQ Tests - Variable Shift Right Qwords
// ============================================================================

#[test]
fn test_vpshrdvq_zmm_basic() {
    let mut emu = emu64();
    // VPSHRDVQ - Variable shift right qwords (ZMM)
    // EVEX.512.66.0F38.W1 73 /r
    let code = [
        0x62, 0xF2, 0xED, 0x48, 0x73, 0xC3,                 // VPSHRDVQ zmm0, zmm2, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_shift_left_right_roundtrip() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x71, 0xC1, 0x10,           // VPSHLDD zmm0, zmm2, zmm1, 16
        0x62, 0xF3, 0x75, 0x48, 0x73, 0xC8, 0x10,           // VPSHRDD zmm1, zmm0, zmm0, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_lzcnt_sad_combo() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x44, 0xC1,                 // VPLZCNTD zmm0, zmm1
        0x62, 0xF3, 0x7D, 0x48, 0x42, 0xD2, 0xE4,           // VDBPSADBW zmm2, zmm0, zmm2, 0xE4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
