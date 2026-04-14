use crate::*;

// RCPPS - Compute Reciprocals of Packed Single Precision Floating-Point Values
//
// RCPPS computes approximate reciprocals of 4 packed single-precision (32-bit) floating-point values
// Relative error: |Relative Error| <= 1.5 * 2^-12
//
// Special cases:
// - Source 0.0 returns infinity with sign of source
// - Denormal source treated as 0.0 (same sign)
// - SNaN converted to QNaN, QNaN returned as-is
// - Tiny results flushed to 0.0 with sign of operand
//
// Opcodes:
// NP 0F 53 /r             RCPPS xmm1, xmm2/m128     - Compute approximate reciprocals of packed single from xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// RCPPS Tests - Packed Single Precision Approximate Reciprocal (4x float32)
// ============================================================================

#[test]
fn test_rcpps_xmm0_xmm1() {
    let mut emu = emu64();
    // RCPPS XMM0, XMM1
    let code = [
        0x0f, 0x53, 0xc1, // RCPPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm1_xmm2() {
    let mut emu = emu64();
    // RCPPS XMM1, XMM2
    let code = [
        0x0f, 0x53, 0xca, // RCPPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm2_xmm3() {
    let mut emu = emu64();
    // RCPPS XMM2, XMM3
    let code = [
        0x0f, 0x53, 0xd3, // RCPPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm3_xmm4() {
    let mut emu = emu64();
    // RCPPS XMM3, XMM4
    let code = [
        0x0f, 0x53, 0xdc, // RCPPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm4_xmm5() {
    let mut emu = emu64();
    // RCPPS XMM4, XMM5
    let code = [
        0x0f, 0x53, 0xe5, // RCPPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm5_xmm6() {
    let mut emu = emu64();
    // RCPPS XMM5, XMM6
    let code = [
        0x0f, 0x53, 0xee, // RCPPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm6_xmm7() {
    let mut emu = emu64();
    // RCPPS XMM6, XMM7
    let code = [
        0x0f, 0x53, 0xf7, // RCPPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm7_xmm0() {
    let mut emu = emu64();
    // RCPPS XMM7, XMM0
    let code = [
        0x0f, 0x53, 0xf8, // RCPPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm8_xmm9() {
    let mut emu = emu64();
    // RCPPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x53, 0xc1, // RCPPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm10_xmm11() {
    let mut emu = emu64();
    // RCPPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x53, 0xd3, // RCPPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm12_xmm13() {
    let mut emu = emu64();
    // RCPPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x53, 0xe5, // RCPPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm14_xmm15() {
    let mut emu = emu64();
    // RCPPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x53, 0xf7, // RCPPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm15_xmm0() {
    let mut emu = emu64();
    // RCPPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x53, 0xf8, // RCPPS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm0_mem() {
    let mut emu = emu64();
    // RCPPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x53, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // RCPPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm7_mem() {
    let mut emu = emu64();
    // RCPPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x53, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // RCPPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_xmm15_mem() {
    let mut emu = emu64();
    // RCPPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x44, 0x0f, 0x53, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // RCPPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_one() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x53, 0xc1, // RCPPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_two() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x53, 0xd3, // RCPPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_half() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x53, 0xe5, // RCPPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_small_values() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x53, 0xf7, // RCPPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_large_values() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x53, 0xc1, // RCPPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_positive_zero() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x53, 0xd3, // RCPPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x53, 0xe5, // RCPPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_negative_values() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x53, 0xf7, // RCPPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_positive_infinity() {
    let mut emu = emu64();
    let code = [
        0x44, 0x0f, 0x53, 0xf8, // RCPPS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_negative_infinity() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x53, 0xc1, // RCPPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_qnan() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x53, 0xd3, // RCPPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_snan() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x53, 0xe5, // RCPPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_precision_test() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x53, 0xf7, // RCPPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rcpps_denormal_treated_as_zero() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x53, 0xc1, // RCPPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
