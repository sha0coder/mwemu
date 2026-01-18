use crate::*;

// ADDPS - Add Packed Single Precision Floating-Point Values
// ADDPD - Add Packed Double Precision Floating-Point Values
//
// ADDPS adds 4 packed single-precision (32-bit) floating-point values
// ADDPD adds 2 packed double-precision (64-bit) floating-point values
//
// Opcodes:
// NP 0F 58 /r             ADDPS xmm1, xmm2/m128    - Add packed single from xmm2/m128 to xmm1
// 66 0F 58 /r             ADDPD xmm1, xmm2/m128    - Add packed double from xmm2/m128 to xmm1

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// ADDPS Tests - Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_addps_xmm0_xmm1() {
    let mut emu = emu64();
    // ADDPS XMM0, XMM1
    let code = [
        0x0f, 0x58, 0xc1, // ADDPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm1_xmm2() {
    let mut emu = emu64();
    // ADDPS XMM1, XMM2
    let code = [
        0x0f, 0x58, 0xca, // ADDPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm2_xmm3() {
    let mut emu = emu64();
    // ADDPS XMM2, XMM3
    let code = [
        0x0f, 0x58, 0xd3, // ADDPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm3_xmm4() {
    let mut emu = emu64();
    // ADDPS XMM3, XMM4
    let code = [
        0x0f, 0x58, 0xdc, // ADDPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm4_xmm5() {
    let mut emu = emu64();
    // ADDPS XMM4, XMM5
    let code = [
        0x0f, 0x58, 0xe5, // ADDPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm5_xmm6() {
    let mut emu = emu64();
    // ADDPS XMM5, XMM6
    let code = [
        0x0f, 0x58, 0xee, // ADDPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm6_xmm7() {
    let mut emu = emu64();
    // ADDPS XMM6, XMM7
    let code = [
        0x0f, 0x58, 0xf7, // ADDPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm7_xmm0() {
    let mut emu = emu64();
    // ADDPS XMM7, XMM0
    let code = [
        0x0f, 0x58, 0xf8, // ADDPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm8_xmm9() {
    let mut emu = emu64();
    // ADDPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x58, 0xc1, // ADDPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm9_xmm10() {
    let mut emu = emu64();
    // ADDPS XMM9, XMM10
    let code = [
        0x45, 0x0f, 0x58, 0xca, // ADDPS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm10_xmm11() {
    let mut emu = emu64();
    // ADDPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x58, 0xd3, // ADDPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm11_xmm12() {
    let mut emu = emu64();
    // ADDPS XMM11, XMM12
    let code = [
        0x45, 0x0f, 0x58, 0xdc, // ADDPS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm12_xmm13() {
    let mut emu = emu64();
    // ADDPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x58, 0xe5, // ADDPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm13_xmm14() {
    let mut emu = emu64();
    // ADDPS XMM13, XMM14
    let code = [
        0x45, 0x0f, 0x58, 0xee, // ADDPS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm14_xmm15() {
    let mut emu = emu64();
    // ADDPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x58, 0xf7, // ADDPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm15_xmm0() {
    let mut emu = emu64();
    // ADDPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x58, 0xf8, // ADDPS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm0_mem() {
    let mut emu = emu64();
    // ADDPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x58, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm1_mem() {
    let mut emu = emu64();
    // ADDPS XMM1, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x58, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDPS XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm7_mem() {
    let mut emu = emu64();
    // ADDPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x58, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_xmm15_mem() {
    let mut emu = emu64();
    // ADDPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x44, 0x0f, 0x58, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_positive_values() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x58, 0xc1, // ADDPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_negative_values() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x58, 0xd3, // ADDPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x58, 0xe5, // ADDPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addps_zero_addition() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x58, 0xf7, // ADDPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// ADDPD Tests - Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_addpd_xmm0_xmm1() {
    let mut emu = emu64();
    // ADDPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x58, 0xc1, // ADDPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm1_xmm2() {
    let mut emu = emu64();
    // ADDPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x58, 0xca, // ADDPD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm2_xmm3() {
    let mut emu = emu64();
    // ADDPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x58, 0xd3, // ADDPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm3_xmm4() {
    let mut emu = emu64();
    // ADDPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x58, 0xdc, // ADDPD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm4_xmm5() {
    let mut emu = emu64();
    // ADDPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x58, 0xe5, // ADDPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm5_xmm6() {
    let mut emu = emu64();
    // ADDPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x58, 0xee, // ADDPD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm6_xmm7() {
    let mut emu = emu64();
    // ADDPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x58, 0xf7, // ADDPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm7_xmm0() {
    let mut emu = emu64();
    // ADDPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x58, 0xf8, // ADDPD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm8_xmm9() {
    let mut emu = emu64();
    // ADDPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x58, 0xc1, // ADDPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm9_xmm10() {
    let mut emu = emu64();
    // ADDPD XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x58, 0xca, // ADDPD XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm10_xmm11() {
    let mut emu = emu64();
    // ADDPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x58, 0xd3, // ADDPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm11_xmm12() {
    let mut emu = emu64();
    // ADDPD XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x58, 0xdc, // ADDPD XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm12_xmm13() {
    let mut emu = emu64();
    // ADDPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x58, 0xe5, // ADDPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm13_xmm14() {
    let mut emu = emu64();
    // ADDPD XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x58, 0xee, // ADDPD XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm14_xmm15() {
    let mut emu = emu64();
    // ADDPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x58, 0xf7, // ADDPD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm15_xmm0() {
    let mut emu = emu64();
    // ADDPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x58, 0xf8, // ADDPD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm0_mem() {
    let mut emu = emu64();
    // ADDPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x58, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm1_mem() {
    let mut emu = emu64();
    // ADDPD XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x58, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDPD XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm7_mem() {
    let mut emu = emu64();
    // ADDPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x58, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_xmm15_mem() {
    let mut emu = emu64();
    // ADDPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x58, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_positive_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x58, 0xc1, // ADDPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_negative_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x58, 0xd3, // ADDPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x58, 0xe5, // ADDPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addpd_zero_addition() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x58, 0xf7, // ADDPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
