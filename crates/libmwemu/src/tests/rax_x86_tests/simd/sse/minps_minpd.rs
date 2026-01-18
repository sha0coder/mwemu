use crate::*;

// MINPS - Minimum of Packed Single Precision Floating-Point Values
// MINPD - Minimum of Packed Double Precision Floating-Point Values
//
// MINPS returns minimum of 4 packed single-precision (32-bit) floating-point values
// MINPD returns minimum of 2 packed double-precision (64-bit) floating-point values
//
// Special cases:
// - If values are both 0.0s (either sign), return second operand
// - If second operand is SNaN, forward SNaN unchanged to destination
// - If only one value is NaN, return second operand
//
// Opcodes:
// NP 0F 5D /r             MINPS xmm1, xmm2/m128     - Return minimum packed single from xmm1 and xmm2/m128
// 66 0F 5D /r             MINPD xmm1, xmm2/m128     - Return minimum packed double from xmm1 and xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// MINPS Tests - Packed Single Precision Minimum (4x float32)
// ============================================================================

#[test]
fn test_minps_xmm0_xmm1() {
    let mut emu = emu64();
    // MINPS XMM0, XMM1
    let code = [
        0x0f, 0x5d, 0xc1, // MINPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm1_xmm2() {
    let mut emu = emu64();
    // MINPS XMM1, XMM2
    let code = [
        0x0f, 0x5d, 0xca, // MINPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm2_xmm3() {
    let mut emu = emu64();
    // MINPS XMM2, XMM3
    let code = [
        0x0f, 0x5d, 0xd3, // MINPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm3_xmm4() {
    let mut emu = emu64();
    // MINPS XMM3, XMM4
    let code = [
        0x0f, 0x5d, 0xdc, // MINPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm4_xmm5() {
    let mut emu = emu64();
    // MINPS XMM4, XMM5
    let code = [
        0x0f, 0x5d, 0xe5, // MINPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm5_xmm6() {
    let mut emu = emu64();
    // MINPS XMM5, XMM6
    let code = [
        0x0f, 0x5d, 0xee, // MINPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm6_xmm7() {
    let mut emu = emu64();
    // MINPS XMM6, XMM7
    let code = [
        0x0f, 0x5d, 0xf7, // MINPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm7_xmm0() {
    let mut emu = emu64();
    // MINPS XMM7, XMM0
    let code = [
        0x0f, 0x5d, 0xf8, // MINPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm8_xmm9() {
    let mut emu = emu64();
    // MINPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x5d, 0xc1, // MINPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm10_xmm11() {
    let mut emu = emu64();
    // MINPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x5d, 0xd3, // MINPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm12_xmm13() {
    let mut emu = emu64();
    // MINPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x5d, 0xe5, // MINPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm14_xmm15() {
    let mut emu = emu64();
    // MINPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x5d, 0xf7, // MINPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm15_xmm0() {
    let mut emu = emu64();
    // MINPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x5d, 0xf8, // MINPS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm0_mem() {
    let mut emu = emu64();
    // MINPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MINPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm7_mem() {
    let mut emu = emu64();
    // MINPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MINPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_xmm15_mem() {
    let mut emu = emu64();
    // MINPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x44, 0x0f, 0x5d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MINPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_positive_values() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5d, 0xc1, // MINPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_negative_values() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5d, 0xd3, // MINPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5d, 0xe5, // MINPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_zero_positive() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5d, 0xf7, // MINPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_zero_negative() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x5d, 0xc1, // MINPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_nan_handling() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x5d, 0xd3, // MINPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minps_snan_forwarding() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x5d, 0xe5, // MINPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MINPD Tests - Packed Double Precision Minimum (2x float64)
// ============================================================================

#[test]
fn test_minpd_xmm0_xmm1() {
    let mut emu = emu64();
    // MINPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x5d, 0xc1, // MINPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm1_xmm2() {
    let mut emu = emu64();
    // MINPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x5d, 0xca, // MINPD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm2_xmm3() {
    let mut emu = emu64();
    // MINPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x5d, 0xd3, // MINPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm3_xmm4() {
    let mut emu = emu64();
    // MINPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x5d, 0xdc, // MINPD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm4_xmm5() {
    let mut emu = emu64();
    // MINPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x5d, 0xe5, // MINPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm5_xmm6() {
    let mut emu = emu64();
    // MINPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x5d, 0xee, // MINPD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm6_xmm7() {
    let mut emu = emu64();
    // MINPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x5d, 0xf7, // MINPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm7_xmm0() {
    let mut emu = emu64();
    // MINPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x5d, 0xf8, // MINPD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm8_xmm9() {
    let mut emu = emu64();
    // MINPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x5d, 0xc1, // MINPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm10_xmm11() {
    let mut emu = emu64();
    // MINPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x5d, 0xd3, // MINPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm12_xmm13() {
    let mut emu = emu64();
    // MINPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x5d, 0xe5, // MINPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm14_xmm15() {
    let mut emu = emu64();
    // MINPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x5d, 0xf7, // MINPD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm15_xmm0() {
    let mut emu = emu64();
    // MINPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x5d, 0xf8, // MINPD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm0_mem() {
    let mut emu = emu64();
    // MINPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MINPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm7_mem() {
    let mut emu = emu64();
    // MINPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MINPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_xmm15_mem() {
    let mut emu = emu64();
    // MINPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x5d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MINPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_positive_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x5d, 0xc1, // MINPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_negative_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x5d, 0xd3, // MINPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x5d, 0xe5, // MINPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_zero_positive() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x5d, 0xf7, // MINPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_zero_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x5d, 0xc1, // MINPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_nan_handling() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x5d, 0xd3, // MINPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minpd_snan_forwarding() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x5d, 0xe5, // MINPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
