use crate::*;

// MULPS - Multiply Packed Single Precision Floating-Point Values
// MULPD - Multiply Packed Double Precision Floating-Point Values
//
// MULPS multiplies 4 packed single-precision (32-bit) floating-point values
// MULPD multiplies 2 packed double-precision (64-bit) floating-point values
//
// Opcodes:
// NP 0F 59 /r             MULPS xmm1, xmm2/m128    - Multiply packed single from xmm2/m128 with xmm1
// 66 0F 59 /r             MULPD xmm1, xmm2/m128    - Multiply packed double from xmm2/m128 with xmm1

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// MULPS Tests - Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_mulps_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x59, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x59, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x59, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0x44, 0x0f, 0x59, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm0_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm1_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm7_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_xmm15_mem() {
    let mut emu = emu64();
    let code = [0x44, 0x0f, 0x59, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_positive_values() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_negative_values() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_mixed_signs() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulps_by_zero() {
    let mut emu = emu64();
    let code = [0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MULPD Tests - Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_mulpd_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x59, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x59, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x59, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x59, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm0_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm1_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm7_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_xmm15_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x59, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_positive_values() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_negative_values() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_mixed_signs() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulpd_by_zero() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
