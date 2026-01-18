use crate::*;

// SQRTPS - Square Root of Packed Single Precision Floating-Point Values
// SQRTPD - Square Root of Packed Double Precision Floating-Point Values
//
// SQRTPS computes square roots of 4 packed single-precision (32-bit) floating-point values
// SQRTPD computes square roots of 2 packed double-precision (64-bit) floating-point values
//
// Opcodes:
// NP 0F 51 /r             SQRTPS xmm1, xmm2/m128    - Compute square roots of packed single from xmm2/m128 to xmm1
// 66 0F 51 /r             SQRTPD xmm1, xmm2/m128    - Compute square roots of packed double from xmm2/m128 to xmm1

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// SQRTPS Tests - Packed Single Precision Square Root (4x float32)
// ============================================================================

#[test]
fn test_sqrtps_xmm0_xmm1() {
    let mut emu = emu64();
    // SQRTPS XMM0, XMM1
    let code = [
        0x0f, 0x51, 0xc1, // SQRTPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm1_xmm2() {
    let mut emu = emu64();
    // SQRTPS XMM1, XMM2
    let code = [
        0x0f, 0x51, 0xca, // SQRTPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm2_xmm3() {
    let mut emu = emu64();
    // SQRTPS XMM2, XMM3
    let code = [
        0x0f, 0x51, 0xd3, // SQRTPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm3_xmm4() {
    let mut emu = emu64();
    // SQRTPS XMM3, XMM4
    let code = [
        0x0f, 0x51, 0xdc, // SQRTPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm4_xmm5() {
    let mut emu = emu64();
    // SQRTPS XMM4, XMM5
    let code = [
        0x0f, 0x51, 0xe5, // SQRTPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm5_xmm6() {
    let mut emu = emu64();
    // SQRTPS XMM5, XMM6
    let code = [
        0x0f, 0x51, 0xee, // SQRTPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm6_xmm7() {
    let mut emu = emu64();
    // SQRTPS XMM6, XMM7
    let code = [
        0x0f, 0x51, 0xf7, // SQRTPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm7_xmm0() {
    let mut emu = emu64();
    // SQRTPS XMM7, XMM0
    let code = [
        0x0f, 0x51, 0xf8, // SQRTPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm8_xmm9() {
    let mut emu = emu64();
    // SQRTPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x51, 0xc1, // SQRTPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm10_xmm11() {
    let mut emu = emu64();
    // SQRTPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x51, 0xd3, // SQRTPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm12_xmm13() {
    let mut emu = emu64();
    // SQRTPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x51, 0xe5, // SQRTPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm14_xmm15() {
    let mut emu = emu64();
    // SQRTPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x51, 0xf7, // SQRTPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm15_xmm0() {
    let mut emu = emu64();
    // SQRTPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x51, 0xf8, // SQRTPS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm0_mem() {
    let mut emu = emu64();
    // SQRTPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x51, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm7_mem() {
    let mut emu = emu64();
    // SQRTPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x51, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_xmm15_mem() {
    let mut emu = emu64();
    // SQRTPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x44, 0x0f, 0x51, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_perfect_squares() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x51, 0xc1, // SQRTPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_non_perfect_squares() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x51, 0xd3, // SQRTPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_zero() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x51, 0xe5, // SQRTPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_positive_infinity() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x51, 0xf7, // SQRTPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_negative_value() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x51, 0xc1, // SQRTPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_qnan() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x51, 0xd3, // SQRTPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_small_values() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x51, 0xe5, // SQRTPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtps_large_values() {
    let mut emu = emu64();
    let code = [
        0x45, 0x0f, 0x51, 0xf7, // SQRTPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// SQRTPD Tests - Packed Double Precision Square Root (2x float64)
// ============================================================================

#[test]
fn test_sqrtpd_xmm0_xmm1() {
    let mut emu = emu64();
    // SQRTPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x51, 0xc1, // SQRTPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm1_xmm2() {
    let mut emu = emu64();
    // SQRTPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x51, 0xca, // SQRTPD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm2_xmm3() {
    let mut emu = emu64();
    // SQRTPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x51, 0xd3, // SQRTPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm3_xmm4() {
    let mut emu = emu64();
    // SQRTPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x51, 0xdc, // SQRTPD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm4_xmm5() {
    let mut emu = emu64();
    // SQRTPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x51, 0xe5, // SQRTPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm5_xmm6() {
    let mut emu = emu64();
    // SQRTPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x51, 0xee, // SQRTPD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm6_xmm7() {
    let mut emu = emu64();
    // SQRTPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x51, 0xf7, // SQRTPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm7_xmm0() {
    let mut emu = emu64();
    // SQRTPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x51, 0xf8, // SQRTPD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm8_xmm9() {
    let mut emu = emu64();
    // SQRTPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x51, 0xc1, // SQRTPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm10_xmm11() {
    let mut emu = emu64();
    // SQRTPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x51, 0xd3, // SQRTPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm12_xmm13() {
    let mut emu = emu64();
    // SQRTPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x51, 0xe5, // SQRTPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm14_xmm15() {
    let mut emu = emu64();
    // SQRTPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x51, 0xf7, // SQRTPD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm15_xmm0() {
    let mut emu = emu64();
    // SQRTPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x51, 0xf8, // SQRTPD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm0_mem() {
    let mut emu = emu64();
    // SQRTPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x51, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm7_mem() {
    let mut emu = emu64();
    // SQRTPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x51, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_xmm15_mem() {
    let mut emu = emu64();
    // SQRTPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x51, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_perfect_squares() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x51, 0xc1, // SQRTPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_non_perfect_squares() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x51, 0xd3, // SQRTPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_zero() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x51, 0xe5, // SQRTPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_positive_infinity() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x51, 0xf7, // SQRTPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_negative_value() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x51, 0xc1, // SQRTPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_qnan() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x51, 0xd3, // SQRTPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_small_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x51, 0xe5, // SQRTPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_sqrtpd_large_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x51, 0xf7, // SQRTPD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
