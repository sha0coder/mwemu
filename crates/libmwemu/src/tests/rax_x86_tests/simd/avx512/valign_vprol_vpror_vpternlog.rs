//! Tests for AVX-512 Bit Manipulation Instructions.
//!
//! This module covers AVX-512 advanced bit manipulation operations including
//! alignment, rotation, and ternary logic.
//!
//! Instructions covered:
//! - VALIGND - Align doubleword vectors
//! - VALIGNQ - Align quadword vectors
//! - VPROLD/VPROLVD - Rotate left dwords (immediate and variable)
//! - VPROLQ/VPROLVQ - Rotate left qwords (immediate and variable)
//! - VPRORD/VPRORVD - Rotate right dwords (immediate and variable)
//! - VPRORQ/VPRORVQ - Rotate right qwords (immediate and variable)
//! - VPTERNLOGD - Ternary logic operation on dwords
//! - VPTERNLOGQ - Ternary logic operation on qwords
//!
//! These instructions are part of AVX512F and AVX512VL extensions.
//!
//! References: Intel SDM Vol. 2, AVX-512 instruction documentation

use crate::*;

// ============================================================================
// VALIGND Tests - Align Doubleword Vectors
// ============================================================================

#[test]
fn test_valignd_xmm_basic() {
    let mut emu = emu64();
    // VALIGND - Align and shift doubleword vectors (XMM)
    // EVEX.128.66.0F3A.W0 03 /r ib
    let code = [
        0x62, 0xF3, 0x7D, 0x08, 0x03, 0xC1, 0x01,           // VALIGND xmm0, xmm0, xmm1, 1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignd_ymm_basic() {
    let mut emu = emu64();
    // VALIGND - Align doubleword vectors (YMM)
    // EVEX.256.66.0F3A.W0 03 /r ib
    let code = [
        0x62, 0xF3, 0x7D, 0x28, 0x03, 0xC2, 0x02,           // VALIGND ymm0, ymm0, ymm2, 2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignd_zmm_basic() {
    let mut emu = emu64();
    // VALIGND - Align doubleword vectors (ZMM)
    // EVEX.512.66.0F3A.W0 03 /r ib
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0xC3, 0x04,           // VALIGND zmm0, zmm0, zmm3, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignd_zmm_memory() {
    let mut emu = emu64();
    // VALIGND from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00,           // MOV RAX, 0x1000
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0x00, 0x03,           // VALIGND zmm0, zmm0, [rax], 3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignd_zmm_zero_shift() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0xC1, 0x00,           // VALIGND zmm0, zmm0, zmm1, 0
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignd_zmm_max_shift() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0xC1, 0x0F,           // VALIGND zmm0, zmm0, zmm1, 15
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VALIGNQ Tests - Align Quadword Vectors
// ============================================================================

#[test]
fn test_valignq_xmm_basic() {
    let mut emu = emu64();
    // VALIGNQ - Align quadword vectors (XMM)
    // EVEX.128.66.0F3A.W1 03 /r ib
    let code = [
        0x62, 0xF3, 0xFD, 0x08, 0x03, 0xC1, 0x01,           // VALIGNQ xmm0, xmm0, xmm1, 1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignq_ymm_basic() {
    let mut emu = emu64();
    // VALIGNQ - Align quadword vectors (YMM)
    // EVEX.256.66.0F3A.W1 03 /r ib
    let code = [
        0x62, 0xF3, 0xFD, 0x28, 0x03, 0xC2, 0x02,           // VALIGNQ ymm0, ymm0, ymm2, 2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignq_zmm_basic() {
    let mut emu = emu64();
    // VALIGNQ - Align quadword vectors (ZMM)
    // EVEX.512.66.0F3A.W1 03 /r ib
    let code = [
        0x62, 0xF3, 0xFD, 0x48, 0x03, 0xC3, 0x04,           // VALIGNQ zmm0, zmm0, zmm3, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignq_zmm_memory() {
    let mut emu = emu64();
    // VALIGNQ from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00,           // MOV RAX, 0x2000
        0x62, 0xF3, 0xFD, 0x48, 0x03, 0x00, 0x05,           // VALIGNQ zmm0, zmm0, [rax], 5
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_valignq_zmm_max_shift() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF3, 0xFD, 0x48, 0x03, 0xC1, 0x07,           // VALIGNQ zmm0, zmm0, zmm1, 7
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPROLD Tests - Rotate Left Dword (Immediate)
// ============================================================================

#[test]
fn test_vprold_xmm_basic() {
    let mut emu = emu64();
    // VPROLD - Rotate left dwords by immediate (XMM)
    // EVEX.128.66.0F.W0 72 /1 ib
    let code = [
        0x62, 0xF1, 0x7D, 0x08, 0x72, 0xC8, 0x08,           // VPROLD xmm1, xmm0, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprold_ymm_basic() {
    let mut emu = emu64();
    // VPROLD - Rotate left dwords (YMM)
    let code = [
        0x62, 0xF1, 0x7D, 0x28, 0x72, 0xC8, 0x10,           // VPROLD ymm1, ymm0, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprold_zmm_basic() {
    let mut emu = emu64();
    // VPROLD - Rotate left dwords (ZMM)
    let code = [
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC8, 0x04,           // VPROLD zmm1, zmm0, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprold_zmm_memory() {
    let mut emu = emu64();
    // VPROLD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00,           // MOV RAX, 0x3000
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0x08, 0x0C,           // VPROLD zmm1, [rax], 12
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprold_full_rotation() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC8, 0x20,           // VPROLD zmm1, zmm0, 32
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPROLQ Tests - Rotate Left Qword (Immediate)
// ============================================================================

#[test]
fn test_vprolq_xmm_basic() {
    let mut emu = emu64();
    // VPROLQ - Rotate left qwords by immediate (XMM)
    // EVEX.128.66.0F.W1 72 /1 ib
    let code = [
        0x62, 0xF1, 0xFD, 0x08, 0x72, 0xC8, 0x08,           // VPROLQ xmm1, xmm0, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprolq_ymm_basic() {
    let mut emu = emu64();
    // VPROLQ - Rotate left qwords (YMM)
    let code = [
        0x62, 0xF1, 0xFD, 0x28, 0x72, 0xC8, 0x10,           // VPROLQ ymm1, ymm0, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprolq_zmm_basic() {
    let mut emu = emu64();
    // VPROLQ - Rotate left qwords (ZMM)
    let code = [
        0x62, 0xF1, 0xFD, 0x48, 0x72, 0xC8, 0x20,           // VPROLQ zmm1, zmm0, 32
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPROLVD Tests - Variable Rotate Left Dword
// ============================================================================

#[test]
fn test_vprolvd_xmm_basic() {
    let mut emu = emu64();
    // VPROLVD - Variable rotate left dwords (XMM)
    // EVEX.128.66.0F38.W0 15 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x15, 0xC1,                 // VPROLVD xmm0, xmm0, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprolvd_ymm_basic() {
    let mut emu = emu64();
    // VPROLVD - Variable rotate left dwords (YMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x28, 0x15, 0xC2,                 // VPROLVD ymm0, ymm0, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprolvd_zmm_basic() {
    let mut emu = emu64();
    // VPROLVD - Variable rotate left dwords (ZMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x15, 0xC3,                 // VPROLVD zmm0, zmm0, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprolvd_zmm_memory() {
    let mut emu = emu64();
    // VPROLVD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00,           // MOV RAX, 0x4000
        0x62, 0xF2, 0x7D, 0x48, 0x15, 0x00,                 // VPROLVD zmm0, zmm0, [rax]
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPROLVQ Tests - Variable Rotate Left Qword
// ============================================================================

#[test]
fn test_vprolvq_xmm_basic() {
    let mut emu = emu64();
    // VPROLVQ - Variable rotate left qwords (XMM)
    // EVEX.128.66.0F38.W1 15 /r
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x15, 0xC1,                 // VPROLVQ xmm0, xmm0, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprolvq_ymm_basic() {
    let mut emu = emu64();
    // VPROLVQ - Variable rotate left qwords (YMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x28, 0x15, 0xC2,                 // VPROLVQ ymm0, ymm0, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprolvq_zmm_basic() {
    let mut emu = emu64();
    // VPROLVQ - Variable rotate left qwords (ZMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x15, 0xC3,                 // VPROLVQ zmm0, zmm0, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPRORD Tests - Rotate Right Dword (Immediate)
// ============================================================================

#[test]
fn test_vprord_xmm_basic() {
    let mut emu = emu64();
    // VPRORD - Rotate right dwords by immediate (XMM)
    // EVEX.128.66.0F.W0 72 /0 ib
    let code = [
        0x62, 0xF1, 0x7D, 0x08, 0x72, 0xC0, 0x08,           // VPRORD xmm0, xmm0, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprord_ymm_basic() {
    let mut emu = emu64();
    // VPRORD - Rotate right dwords (YMM)
    let code = [
        0x62, 0xF1, 0x7D, 0x28, 0x72, 0xC0, 0x10,           // VPRORD ymm0, ymm0, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprord_zmm_basic() {
    let mut emu = emu64();
    // VPRORD - Rotate right dwords (ZMM)
    let code = [
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC0, 0x04,           // VPRORD zmm0, zmm0, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPRORQ Tests - Rotate Right Qword (Immediate)
// ============================================================================

#[test]
fn test_vprorq_xmm_basic() {
    let mut emu = emu64();
    // VPRORQ - Rotate right qwords by immediate (XMM)
    // EVEX.128.66.0F.W1 72 /0 ib
    let code = [
        0x62, 0xF1, 0xFD, 0x08, 0x72, 0xC0, 0x08,           // VPRORQ xmm0, xmm0, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprorq_ymm_basic() {
    let mut emu = emu64();
    // VPRORQ - Rotate right qwords (YMM)
    let code = [
        0x62, 0xF1, 0xFD, 0x28, 0x72, 0xC0, 0x10,           // VPRORQ ymm0, ymm0, 16
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprorq_zmm_basic() {
    let mut emu = emu64();
    // VPRORQ - Rotate right qwords (ZMM)
    let code = [
        0x62, 0xF1, 0xFD, 0x48, 0x72, 0xC0, 0x20,           // VPRORQ zmm0, zmm0, 32
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPRORVD Tests - Variable Rotate Right Dword
// ============================================================================

#[test]
fn test_vprorvd_xmm_basic() {
    let mut emu = emu64();
    // VPRORVD - Variable rotate right dwords (XMM)
    // EVEX.128.66.0F38.W0 14 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x14, 0xC1,                 // VPRORVD xmm0, xmm0, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprorvd_ymm_basic() {
    let mut emu = emu64();
    // VPRORVD - Variable rotate right dwords (YMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x28, 0x14, 0xC2,                 // VPRORVD ymm0, ymm0, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprorvd_zmm_basic() {
    let mut emu = emu64();
    // VPRORVD - Variable rotate right dwords (ZMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x14, 0xC3,                 // VPRORVD zmm0, zmm0, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPRORVQ Tests - Variable Rotate Right Qword
// ============================================================================

#[test]
fn test_vprorvq_xmm_basic() {
    let mut emu = emu64();
    // VPRORVQ - Variable rotate right qwords (XMM)
    // EVEX.128.66.0F38.W1 14 /r
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x14, 0xC1,                 // VPRORVQ xmm0, xmm0, xmm1
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprorvq_ymm_basic() {
    let mut emu = emu64();
    // VPRORVQ - Variable rotate right qwords (YMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x28, 0x14, 0xC2,                 // VPRORVQ ymm0, ymm0, ymm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vprorvq_zmm_basic() {
    let mut emu = emu64();
    // VPRORVQ - Variable rotate right qwords (ZMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x14, 0xC3,                 // VPRORVQ zmm0, zmm0, zmm3
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPTERNLOGD Tests - Ternary Logic Dword
// ============================================================================

#[test]
fn test_vpternlogd_xmm_basic() {
    let mut emu = emu64();
    // VPTERNLOGD - Ternary logic operation on dwords (XMM)
    // EVEX.128.66.0F3A.W0 25 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x08, 0x25, 0xC2, 0xF0,           // VPTERNLOGD xmm0, xmm2, xmm2, 0xF0
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpternlogd_ymm_basic() {
    let mut emu = emu64();
    // VPTERNLOGD - Ternary logic (YMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x28, 0x25, 0xC3, 0xAA,           // VPTERNLOGD ymm0, ymm2, ymm3, 0xAA
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpternlogd_zmm_basic() {
    let mut emu = emu64();
    // VPTERNLOGD - Ternary logic (ZMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x25, 0xC1, 0x96,           // VPTERNLOGD zmm0, zmm2, zmm1, 0x96 (XOR)
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpternlogd_zmm_memory() {
    let mut emu = emu64();
    // VPTERNLOGD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00,           // MOV RAX, 0x5000
        0x62, 0xF3, 0x6D, 0x48, 0x25, 0x00, 0xC0,           // VPTERNLOGD zmm0, zmm2, [rax], 0xC0
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpternlogd_logic_operations() {
    let mut emu = emu64();
    let code = [
        // AND: (A & B & C) - imm8 = 0x80
        0x62, 0xF3, 0x6D, 0x48, 0x25, 0xC1, 0x80,           // VPTERNLOGD zmm0, zmm2, zmm1, 0x80

        // OR: (A | B | C) - imm8 = 0xFE
        0x62, 0xF3, 0x65, 0x48, 0x25, 0xCA, 0xFE,           // VPTERNLOGD zmm1, zmm3, zmm2, 0xFE

        // XOR: A ^ B ^ C - imm8 = 0x96
        0x62, 0xF3, 0x5D, 0x48, 0x25, 0xD3, 0x96,           // VPTERNLOGD zmm2, zmm4, zmm3, 0x96

        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPTERNLOGQ Tests - Ternary Logic Qword
// ============================================================================

#[test]
fn test_vpternlogq_xmm_basic() {
    let mut emu = emu64();
    // VPTERNLOGQ - Ternary logic operation on qwords (XMM)
    // EVEX.128.66.0F3A.W1 25 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x25, 0xC2, 0xF0,           // VPTERNLOGQ xmm0, xmm2, xmm2, 0xF0
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpternlogq_ymm_basic() {
    let mut emu = emu64();
    // VPTERNLOGQ - Ternary logic (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x25, 0xC3, 0xAA,           // VPTERNLOGQ ymm0, ymm2, ymm3, 0xAA
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpternlogq_zmm_basic() {
    let mut emu = emu64();
    // VPTERNLOGQ - Ternary logic (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x25, 0xC1, 0x96,           // VPTERNLOGQ zmm0, zmm2, zmm1, 0x96
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpternlogq_zmm_memory() {
    let mut emu = emu64();
    // VPTERNLOGQ from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00,           // MOV RAX, 0x6000
        0x62, 0xF3, 0xED, 0x48, 0x25, 0x00, 0xC0,           // VPTERNLOGQ zmm0, zmm2, [rax], 0xC0
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_rotate_left_right_roundtrip() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC8, 0x08,           // VPROLD zmm1, zmm0, 8
        0x62, 0xF1, 0x75, 0x48, 0x72, 0xC1, 0x08,           // VPRORD zmm0, zmm1, 8
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_align_and_rotate_combo() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0xC1, 0x02,           // VALIGND zmm0, zmm0, zmm1, 2
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC8, 0x04,           // VPROLD zmm1, zmm0, 4
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ternlog_complex_expression() {
    let mut emu = emu64();
    let code = [
        // (A & B) | (~A & C) - imm8 = 0xD8
        0x62, 0xF3, 0x6D, 0x48, 0x25, 0xC3, 0xD8,           // VPTERNLOGD zmm0, zmm2, zmm3, 0xD8

        // Majority function: (A & B) | (B & C) | (A & C) - imm8 = 0xE8
        0x62, 0xF3, 0x65, 0x48, 0x25, 0xCA, 0xE8,           // VPTERNLOGD zmm1, zmm3, zmm2, 0xE8

        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_variable_rotation_pattern() {
    let mut emu = emu64();
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x15, 0xC1,                 // VPROLVD zmm0, zmm0, zmm1
        0x62, 0xF2, 0x7D, 0x48, 0x14, 0xD2,                 // VPRORVD zmm2, zmm2, zmm2
        0xF4,                                                // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
