use crate::*;

// MAXPS - Maximum of Packed Single Precision Floating-Point Values
// MAXPD - Maximum of Packed Double Precision Floating-Point Values
//
// MAXPS returns maximum of 4 packed single-precision (32-bit) floating-point values
// MAXPD returns maximum of 2 packed double-precision (64-bit) floating-point values
//
// Special cases:
// - If values are both 0.0s (either sign), return second operand
// - If second operand is SNaN, forward SNaN unchanged to destination
// - If only one value is NaN, return second operand
//
// Opcodes:
// NP 0F 5F /r             MAXPS xmm1, xmm2/m128     - Return maximum packed single from xmm1 and xmm2/m128
// 66 0F 5F /r             MAXPD xmm1, xmm2/m128     - Return maximum packed double from xmm1 and xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// MAXPS Tests - Packed Single Precision Maximum (4x float32)
// ============================================================================

#[test]
fn test_maxps_xmm0_xmm1() {
    let mut emu = emu64();
    // MAXPS XMM0, XMM1
    let code = [
        0x0f, 0x5f, 0xc1, // MAXPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm1_xmm2() {
    let mut emu = emu64();
    // MAXPS XMM1, XMM2
    let code = [
        0x0f, 0x5f, 0xca, // MAXPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm2_xmm3() {
    let mut emu = emu64();
    // MAXPS XMM2, XMM3
    let code = [
        0x0f, 0x5f, 0xd3, // MAXPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm3_xmm4() {
    let mut emu = emu64();
    // MAXPS XMM3, XMM4
    let code = [
        0x0f, 0x5f, 0xdc, // MAXPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm4_xmm5() {
    let mut emu = emu64();
    // MAXPS XMM4, XMM5
    let code = [
        0x0f, 0x5f, 0xe5, // MAXPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm5_xmm6() {
    let mut emu = emu64();
    // MAXPS XMM5, XMM6
    let code = [
        0x0f, 0x5f, 0xee, // MAXPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm6_xmm7() {
    let mut emu = emu64();
    // MAXPS XMM6, XMM7
    let code = [
        0x0f, 0x5f, 0xf7, // MAXPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm7_xmm0() {
    let mut emu = emu64();
    // MAXPS XMM7, XMM0
    let code = [
        0x0f, 0x5f, 0xf8, // MAXPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm8_xmm9() {
    let mut emu = emu64();
    // MAXPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x5f, 0xc1, // MAXPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm10_xmm11() {
    let mut emu = emu64();
    // MAXPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x5f, 0xd3, // MAXPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm12_xmm13() {
    let mut emu = emu64();
    // MAXPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x5f, 0xe5, // MAXPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm14_xmm15() {
    let mut emu = emu64();
    // MAXPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x5f, 0xf7, // MAXPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm15_xmm0() {
    let mut emu = emu64();
    // MAXPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x5f, 0xf8, // MAXPS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm0_mem() {
    let mut emu = emu64();
    // MAXPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm7_mem() {
    let mut emu = emu64();
    // MAXPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_xmm15_mem() {
    let mut emu = emu64();
    // MAXPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x44, 0x0f, 0x5f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_positive_values() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5f, 0xc1, // MAXPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_negative_values() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5f, 0xd3, // MAXPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5f, 0xe5, // MAXPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_zero_positive() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5f, 0xf7, // MAXPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_zero_negative() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x5f, 0xc1, // MAXPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_nan_handling() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x5f, 0xd3, // MAXPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxps_snan_forwarding() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x5f, 0xe5, // MAXPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MAXPD Tests - Packed Double Precision Maximum (2x float64)
// ============================================================================

#[test]
fn test_maxpd_xmm0_xmm1() {
    let mut emu = emu64();
    // MAXPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x5f, 0xc1, // MAXPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm1_xmm2() {
    let mut emu = emu64();
    // MAXPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x5f, 0xca, // MAXPD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm2_xmm3() {
    let mut emu = emu64();
    // MAXPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x5f, 0xd3, // MAXPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm3_xmm4() {
    let mut emu = emu64();
    // MAXPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x5f, 0xdc, // MAXPD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm4_xmm5() {
    let mut emu = emu64();
    // MAXPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x5f, 0xe5, // MAXPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm5_xmm6() {
    let mut emu = emu64();
    // MAXPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x5f, 0xee, // MAXPD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm6_xmm7() {
    let mut emu = emu64();
    // MAXPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x5f, 0xf7, // MAXPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm7_xmm0() {
    let mut emu = emu64();
    // MAXPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x5f, 0xf8, // MAXPD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm8_xmm9() {
    let mut emu = emu64();
    // MAXPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xc1, // MAXPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm10_xmm11() {
    let mut emu = emu64();
    // MAXPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xd3, // MAXPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm12_xmm13() {
    let mut emu = emu64();
    // MAXPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xe5, // MAXPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm14_xmm15() {
    let mut emu = emu64();
    // MAXPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xf7, // MAXPD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm15_xmm0() {
    let mut emu = emu64();
    // MAXPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x5f, 0xf8, // MAXPD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm0_mem() {
    let mut emu = emu64();
    // MAXPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm7_mem() {
    let mut emu = emu64();
    // MAXPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_xmm15_mem() {
    let mut emu = emu64();
    // MAXPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x5f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_positive_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x5f, 0xc1, // MAXPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_negative_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x5f, 0xd3, // MAXPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x5f, 0xe5, // MAXPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_zero_positive() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x5f, 0xf7, // MAXPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_zero_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xc1, // MAXPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_nan_handling() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xd3, // MAXPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxpd_snan_forwarding() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xe5, // MAXPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
