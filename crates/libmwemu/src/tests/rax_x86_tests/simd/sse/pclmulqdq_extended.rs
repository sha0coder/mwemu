use crate::*;

// PCLMULQDQ - Carry-Less Multiplication Quadword (Extended Tests)
//
// PCLMULQDQ performs a carry-less multiplication of two 64-bit values selected
// from the source and destination operands. The immediate byte determines which
// 64-bit quadwords are used for the operation.
//
// Immediate byte encoding:
// Bit 0: Select quadword from source (0 = low, 1 = high)
// Bit 4: Select quadword from destination (0 = low, 1 = high)
//
// Possible values:
// 0x00 (0000b): dest[63:0] * src[63:0]
// 0x01 (0001b): dest[63:0] * src[127:64]
// 0x10 (0001 0000b): dest[127:64] * src[63:0]
// 0x11 (0001 0001b): dest[127:64] * src[127:64]
//
// This instruction is used for:
// - CRC calculations
// - Cryptographic algorithms (GCM mode)
// - Error detection codes
// - Polynomial arithmetic over GF(2)
//
// Opcode:
// 66 0F 3A 44 /r ib       PCLMULQDQ xmm1, xmm2/m128, imm8

// ============================================================================
// PCLMULQDQ Tests - All Immediate Value Combinations
// ============================================================================

// Tests for immediate value 0x00: dest[63:0] * src[63:0]

#[test]
fn test_pclmulqdq_xmm0_xmm1_imm00() {
    let mut emu = emu64();
    // PCLMULQDQ XMM0, XMM1, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_xmm3_imm00() {
    let mut emu = emu64();
    // PCLMULQDQ XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x00, // PCLMULQDQ XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm4_xmm5_imm00() {
    let mut emu = emu64();
    // PCLMULQDQ XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x00, // PCLMULQDQ XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm6_xmm7_imm00() {
    let mut emu = emu64();
    // PCLMULQDQ XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x00, // PCLMULQDQ XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Tests for immediate value 0x01: dest[63:0] * src[127:64]

#[test]
fn test_pclmulqdq_xmm0_xmm1_imm01() {
    let mut emu = emu64();
    // PCLMULQDQ XMM0, XMM1, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x01, // PCLMULQDQ XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_xmm3_imm01() {
    let mut emu = emu64();
    // PCLMULQDQ XMM2, XMM3, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x01, // PCLMULQDQ XMM2, XMM3, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm4_xmm5_imm01() {
    let mut emu = emu64();
    // PCLMULQDQ XMM4, XMM5, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x01, // PCLMULQDQ XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm6_xmm7_imm01() {
    let mut emu = emu64();
    // PCLMULQDQ XMM6, XMM7, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x01, // PCLMULQDQ XMM6, XMM7, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Tests for immediate value 0x10: dest[127:64] * src[63:0]

#[test]
fn test_pclmulqdq_xmm0_xmm1_imm10() {
    let mut emu = emu64();
    // PCLMULQDQ XMM0, XMM1, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x10, // PCLMULQDQ XMM0, XMM1, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_xmm3_imm10() {
    let mut emu = emu64();
    // PCLMULQDQ XMM2, XMM3, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x10, // PCLMULQDQ XMM2, XMM3, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm4_xmm5_imm10() {
    let mut emu = emu64();
    // PCLMULQDQ XMM4, XMM5, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x10, // PCLMULQDQ XMM4, XMM5, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm6_xmm7_imm10() {
    let mut emu = emu64();
    // PCLMULQDQ XMM6, XMM7, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x10, // PCLMULQDQ XMM6, XMM7, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Tests for immediate value 0x11: dest[127:64] * src[127:64]

#[test]
fn test_pclmulqdq_xmm0_xmm1_imm11() {
    let mut emu = emu64();
    // PCLMULQDQ XMM0, XMM1, 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x11, // PCLMULQDQ XMM0, XMM1, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_xmm3_imm11() {
    let mut emu = emu64();
    // PCLMULQDQ XMM2, XMM3, 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x11, // PCLMULQDQ XMM2, XMM3, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm4_xmm5_imm11() {
    let mut emu = emu64();
    // PCLMULQDQ XMM4, XMM5, 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x11, // PCLMULQDQ XMM4, XMM5, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm6_xmm7_imm11() {
    let mut emu = emu64();
    // PCLMULQDQ XMM6, XMM7, 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x11, // PCLMULQDQ XMM6, XMM7, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Extended register tests with REX prefix

#[test]
fn test_pclmulqdq_xmm8_xmm9_imm00() {
    let mut emu = emu64();
    // PCLMULQDQ XMM8, XMM9, 0x00
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm10_xmm11_imm01() {
    let mut emu = emu64();
    // PCLMULQDQ XMM10, XMM11, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xd3, 0x01, // PCLMULQDQ XMM10, XMM11, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm12_xmm13_imm10() {
    let mut emu = emu64();
    // PCLMULQDQ XMM12, XMM13, 0x10
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xe5, 0x10, // PCLMULQDQ XMM12, XMM13, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm14_xmm15_imm11() {
    let mut emu = emu64();
    // PCLMULQDQ XMM14, XMM15, 0x11
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xf7, 0x11, // PCLMULQDQ XMM14, XMM15, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory operand tests with all immediate values

#[test]
fn test_pclmulqdq_xmm0_mem_imm00() {
    let mut emu = emu64();
    // PCLMULQDQ XMM0, [mem], 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, // PCLMULQDQ XMM0, [0x3000], 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm1_mem_imm01() {
    let mut emu = emu64();
    // PCLMULQDQ XMM1, [mem], 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, // PCLMULQDQ XMM1, [0x3000], 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_mem_imm10() {
    let mut emu = emu64();
    // PCLMULQDQ XMM2, [mem], 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0x10, // PCLMULQDQ XMM2, [0x3000], 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm3_mem_imm11() {
    let mut emu = emu64();
    // PCLMULQDQ XMM3, [mem], 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x11, // PCLMULQDQ XMM3, [0x3000], 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// GCM mode pattern tests (used in AES-GCM)

#[test]
fn test_pclmulqdq_gcm_pattern_1() {
    let mut emu = emu64();
    // GCM multiplication pattern 1
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x11, // PCLMULQDQ XMM0, XMM1, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_gcm_pattern_2() {
    let mut emu = emu64();
    // GCM multiplication pattern 2
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x01, // PCLMULQDQ XMM2, XMM3, 0x01
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x10, // PCLMULQDQ XMM2, XMM3, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_gcm_full_sequence() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x01, // PCLMULQDQ XMM2, XMM3, 0x01
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x10, // PCLMULQDQ XMM4, XMM5, 0x10
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x11, // PCLMULQDQ XMM6, XMM7, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// CRC calculation patterns

#[test]
fn test_pclmulqdq_crc32_pattern() {
    let mut emu = emu64();
    // CRC32 calculation pattern
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_crc64_pattern() {
    let mut emu = emu64();
    // CRC64 calculation pattern
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x00, // PCLMULQDQ XMM2, XMM3, 0x00
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x11, // PCLMULQDQ XMM2, XMM3, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// All combinations with same register

#[test]
fn test_pclmulqdq_xmm0_xmm0_all_imm() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc0, 0x00, // PCLMULQDQ XMM0, XMM0, 0x00
        0x66, 0x0f, 0x3a, 0x44, 0xc0, 0x01, // PCLMULQDQ XMM0, XMM0, 0x01
        0x66, 0x0f, 0x3a, 0x44, 0xc0, 0x10, // PCLMULQDQ XMM0, XMM0, 0x10
        0x66, 0x0f, 0x3a, 0x44, 0xc0, 0x11, // PCLMULQDQ XMM0, XMM0, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Polynomial multiplication tests

#[test]
fn test_pclmulqdq_polynomial_low_low() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x00, // PCLMULQDQ XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_polynomial_low_high() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x01, // PCLMULQDQ XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_polynomial_high_low() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x10, // PCLMULQDQ XMM4, XMM5, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_polynomial_high_high() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x11, // PCLMULQDQ XMM4, XMM5, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Cross quadword multiplication patterns

#[test]
fn test_pclmulqdq_cross_multiplication() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x01, // PCLMULQDQ XMM6, XMM7, 0x01
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x10, // PCLMULQDQ XMM6, XMM7, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Sequential operations with different immediates

#[test]
fn test_pclmulqdq_sequential_all_imm() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x01, // PCLMULQDQ XMM2, XMM3, 0x01
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x10, // PCLMULQDQ XMM4, XMM5, 0x10
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x11, // PCLMULQDQ XMM6, XMM7, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Galois field arithmetic

#[test]
fn test_pclmulqdq_gf2_multiplication() {
    let mut emu = emu64();
    // GF(2) polynomial multiplication
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_reduction_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xd3, 0x01, // PCLMULQDQ XMM10, XMM11, 0x01
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xd3, 0x10, // PCLMULQDQ XMM10, XMM11, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Extended register combinations

#[test]
fn test_pclmulqdq_xmm15_xmm0_imm00() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x44, 0xf8, 0x00, // PCLMULQDQ XMM15, XMM0, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_xmm0_xmm15_imm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x41, 0x0f, 0x3a, 0x44, 0xc7, 0x11, // PCLMULQDQ XMM0, XMM15, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pclmulqdq_comprehensive_test() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x01, // PCLMULQDQ XMM0, XMM1, 0x01
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x10, // PCLMULQDQ XMM2, XMM3, 0x10
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x11, // PCLMULQDQ XMM2, XMM3, 0x11
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM8, XMM9, 0x00
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xf7, 0x11, // PCLMULQDQ XMM14, XMM15, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
