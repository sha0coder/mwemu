use crate::*;

// DIVPS - Divide Packed Single Precision Floating-Point Values
// DIVPD - Divide Packed Double Precision Floating-Point Values
//
// DIVPS divides 4 packed single-precision (32-bit) floating-point values
// DIVPD divides 2 packed double-precision (64-bit) floating-point values
//
// Opcodes:
// NP 0F 5E /r             DIVPS xmm1, xmm2/m128    - Divide packed single in xmm1 by xmm2/m128
// 66 0F 5E /r             DIVPD xmm1, xmm2/m128    - Divide packed double in xmm1 by xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// DIVPS Tests - Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_divps_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5e, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5e, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5e, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5e, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5e, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5e, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5e, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0x44, 0x0f, 0x5e, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm0_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm1_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm7_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_xmm15_mem() {
    let mut emu = emu64();
    let code = [0x44, 0x0f, 0x5e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_positive_values() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_negative_values() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_mixed_signs() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divps_by_zero() {
    let mut emu = emu64();
    let code = [0x0f, 0x5e, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// DIVPD Tests - Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_divpd_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5e, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5e, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5e, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5e, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5e, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5e, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5e, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x5e, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm0_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm1_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm7_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_xmm15_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x5e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_positive_values() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_negative_values() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_mixed_signs() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_divpd_by_zero() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5e, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
