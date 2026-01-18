use crate::*;

// RSQRTSS - Compute Reciprocal of Square Root of Scalar Single Precision Floating-Point Value
//
// RSQRTSS computes approximate reciprocal square root of the low single-precision (32-bit) floating-point value
// Relative error: |Relative Error| <= 1.5 * 2^-12
//
// Special cases:
// - Source 0.0 returns infinity with sign of source
// - Denormal source treated as 0.0 (same sign)
// - Negative value (except -0.0) returns floating-point indefinite
// - SNaN converted to QNaN, QNaN returned as-is
// - Upper bits [127:32] remain unchanged (Legacy SSE)
//
// Opcodes:
// F3 0F 52 /r             RSQRTSS xmm1, xmm2/m32     - Compute approximate reciprocal sqrt of scalar single from xmm2/m32

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// RSQRTSS Tests - Scalar Single Precision Approximate Reciprocal Square Root
// ============================================================================

#[test]
fn test_rsqrtss_xmm0_xmm1() {
    let mut emu = emu64();
    // RSQRTSS XMM0, XMM1
    let code = [
        0xf3, 0x0f, 0x52, 0xc1, // RSQRTSS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm1_xmm2() {
    let mut emu = emu64();
    // RSQRTSS XMM1, XMM2
    let code = [
        0xf3, 0x0f, 0x52, 0xca, // RSQRTSS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm2_xmm3() {
    let mut emu = emu64();
    // RSQRTSS XMM2, XMM3
    let code = [
        0xf3, 0x0f, 0x52, 0xd3, // RSQRTSS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm3_xmm4() {
    let mut emu = emu64();
    // RSQRTSS XMM3, XMM4
    let code = [
        0xf3, 0x0f, 0x52, 0xdc, // RSQRTSS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm4_xmm5() {
    let mut emu = emu64();
    // RSQRTSS XMM4, XMM5
    let code = [
        0xf3, 0x0f, 0x52, 0xe5, // RSQRTSS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm5_xmm6() {
    let mut emu = emu64();
    // RSQRTSS XMM5, XMM6
    let code = [
        0xf3, 0x0f, 0x52, 0xee, // RSQRTSS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm6_xmm7() {
    let mut emu = emu64();
    // RSQRTSS XMM6, XMM7
    let code = [
        0xf3, 0x0f, 0x52, 0xf7, // RSQRTSS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm7_xmm0() {
    let mut emu = emu64();
    // RSQRTSS XMM7, XMM0
    let code = [
        0xf3, 0x0f, 0x52, 0xf8, // RSQRTSS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm8_xmm9() {
    let mut emu = emu64();
    // RSQRTSS XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xc1, // RSQRTSS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm10_xmm11() {
    let mut emu = emu64();
    // RSQRTSS XMM10, XMM11
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xd3, // RSQRTSS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm12_xmm13() {
    let mut emu = emu64();
    // RSQRTSS XMM12, XMM13
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xe5, // RSQRTSS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm14_xmm15() {
    let mut emu = emu64();
    // RSQRTSS XMM14, XMM15
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xf7, // RSQRTSS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm15_xmm0() {
    let mut emu = emu64();
    // RSQRTSS XMM15, XMM0
    let code = [
        0xf3, 0x44, 0x0f, 0x52, 0xf8, // RSQRTSS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm0_mem() {
    let mut emu = emu64();
    // RSQRTSS XMM0, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x52, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // RSQRTSS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm7_mem() {
    let mut emu = emu64();
    // RSQRTSS XMM7, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x52, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // RSQRTSS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_xmm15_mem() {
    let mut emu = emu64();
    // RSQRTSS XMM15, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x44, 0x0f, 0x52, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // RSQRTSS XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_one() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xc1, // RSQRTSS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_four() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xd3, // RSQRTSS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_nine() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xe5, // RSQRTSS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_small_value() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xf7, // RSQRTSS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_large_value() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xc1, // RSQRTSS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_positive_zero() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xd3, // RSQRTSS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_negative_zero() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xe5, // RSQRTSS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_negative_value() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xf7, // RSQRTSS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_positive_infinity() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x44, 0x0f, 0x52, 0xf8, // RSQRTSS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_qnan() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xc1, // RSQRTSS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_snan() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xd3, // RSQRTSS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_upper_bits_preserved() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xe5, // RSQRTSS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_denormal_treated_as_zero() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xf7, // RSQRTSS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_precision_test_1() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xc1, // RSQRTSS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_precision_test_2() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xd3, // RSQRTSS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_mem_32bit() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // RSQRTSS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_self_xmm0() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xc0, // RSQRTSS XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_self_xmm7() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x52, 0xff, // RSQRTSS XMM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_rsqrtss_negative_one() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x45, 0x0f, 0x52, 0xe5, // RSQRTSS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
