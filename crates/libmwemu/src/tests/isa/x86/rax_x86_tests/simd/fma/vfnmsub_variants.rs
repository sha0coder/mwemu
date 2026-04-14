use crate::*;

// VFNMSUB - Fused Negative Multiply-Subtract Floating-Point Values
//
// These instructions perform fused negative multiply-subtract operations: -(a * b) - c
// Available in 132, 213, and 231 variants for PS/PD/SS/SD
//
// Opcodes:
// VEX.128.66.0F38.W0 9E /r    VFNMSUB132PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 9E /r    VFNMSUB132PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 9E /r    VFNMSUB132PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 9E /r    VFNMSUB132PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 9F /r    VFNMSUB132SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 9F /r    VFNMSUB132SD xmm1, xmm2, xmm3/m64
//
// VEX.128.66.0F38.W0 AE /r    VFNMSUB213PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 AE /r    VFNMSUB213PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 AE /r    VFNMSUB213PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 AE /r    VFNMSUB213PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 AF /r    VFNMSUB213SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 AF /r    VFNMSUB213SD xmm1, xmm2, xmm3/m64
//
// VEX.128.66.0F38.W0 BE /r    VFNMSUB231PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 BE /r    VFNMSUB231PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 BE /r    VFNMSUB231PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 BE /r    VFNMSUB231PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 BF /r    VFNMSUB231SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 BF /r    VFNMSUB231SD xmm1, xmm2, xmm3/m64

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VFNMSUB132PS Tests - 128-bit
// ============================================================================

#[test]
fn test_vfnmsub132ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9e, 0xc2, // VFNMSUB132PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x61, 0x9e, 0xd4, // VFNMSUB132PS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x49, 0x9e, 0xef, // VFNMSUB132PS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x9e, 0xc2, // VFNMSUB132PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x09, 0x9e, 0xef, // VFNMSUB132PS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB132PS Tests - 256-bit
// ============================================================================

#[test]
fn test_vfnmsub132ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x9e, 0xc2, // VFNMSUB132PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x65, 0x9e, 0xd4, // VFNMSUB132PS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x4d, 0x9e, 0xef, // VFNMSUB132PS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x9e, 0xc2, // VFNMSUB132PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x0d, 0x9e, 0xef, // VFNMSUB132PS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB132PD Tests - 128-bit
// ============================================================================

#[test]
fn test_vfnmsub132pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x9e, 0xc2, // VFNMSUB132PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132pd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe1, 0x9e, 0xd4, // VFNMSUB132PD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132pd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc9, 0x9e, 0xef, // VFNMSUB132PD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x9e, 0xc2, // VFNMSUB132PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132pd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0x9e, 0xef, // VFNMSUB132PD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB132PD Tests - 256-bit
// ============================================================================

#[test]
fn test_vfnmsub132pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x9e, 0xc2, // VFNMSUB132PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132pd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe5, 0x9e, 0xd4, // VFNMSUB132PD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132pd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xcd, 0x9e, 0xef, // VFNMSUB132PD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0x9e, 0xc2, // VFNMSUB132PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132pd_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x8d, 0x9e, 0xef, // VFNMSUB132PD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB132SS & VFNMSUB132SD Tests
// ============================================================================

#[test]
fn test_vfnmsub132ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9f, 0xc2, // VFNMSUB132SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x9f, 0xc2, // VFNMSUB132SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x9f, 0xc2, // VFNMSUB132SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x9f, 0xc2, // VFNMSUB132SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132ss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x09, 0x9f, 0xef, // VFNMSUB132SS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132sd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0x9f, 0xef, // VFNMSUB132SD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB213PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfnmsub213ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xae, 0xc2, // VFNMSUB213PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xae, 0xc2, // VFNMSUB213PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xae, 0xc2, // VFNMSUB213PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xae, 0xc2, // VFNMSUB213PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB213PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfnmsub213pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xae, 0xc2, // VFNMSUB213PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xae, 0xc2, // VFNMSUB213PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xae, 0xc2, // VFNMSUB213PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0xae, 0xc2, // VFNMSUB213PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB213SS & VFNMSUB213SD Tests
// ============================================================================

#[test]
fn test_vfnmsub213ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xaf, 0xc2, // VFNMSUB213SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xaf, 0xc2, // VFNMSUB213SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xaf, 0xc2, // VFNMSUB213SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xaf, 0xc2, // VFNMSUB213SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB231PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfnmsub231ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xbe, 0xc2, // VFNMSUB231PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xbe, 0xc2, // VFNMSUB231PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xbe, 0xc2, // VFNMSUB231PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xbe, 0xc2, // VFNMSUB231PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB231PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfnmsub231pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xbe, 0xc2, // VFNMSUB231PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xbe, 0xc2, // VFNMSUB231PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xbe, 0xc2, // VFNMSUB231PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0xbe, 0xc2, // VFNMSUB231PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMSUB231SS & VFNMSUB231SD Tests
// ============================================================================

#[test]
fn test_vfnmsub231ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xbf, 0xc2, // VFNMSUB231SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xbf, 0xc2, // VFNMSUB231SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xbf, 0xc2, // VFNMSUB231SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xbf, 0xc2, // VFNMSUB231SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_vfnmsub132ps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFNMSUB132PS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub213pd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xae, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFNMSUB213PD YMM0, YMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub231ss_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xbf, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFNMSUB231SS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmsub132sd_xmm15_xmm8_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0xb9, 0x9f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFNMSUB132SD XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
