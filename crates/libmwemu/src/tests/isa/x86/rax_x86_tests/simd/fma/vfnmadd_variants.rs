use crate::*;

// VFNMADD - Fused Negative Multiply-Add Floating-Point Values
//
// These instructions perform fused negative multiply-add operations: -(a * b) + c
// Available in 132, 213, and 231 variants for PS/PD/SS/SD
//
// Opcodes:
// VEX.128.66.0F38.W0 9C /r    VFNMADD132PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 9C /r    VFNMADD132PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 9C /r    VFNMADD132PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 9C /r    VFNMADD132PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 9D /r    VFNMADD132SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 9D /r    VFNMADD132SD xmm1, xmm2, xmm3/m64
//
// VEX.128.66.0F38.W0 AC /r    VFNMADD213PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 AC /r    VFNMADD213PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 AC /r    VFNMADD213PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 AC /r    VFNMADD213PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 AD /r    VFNMADD213SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 AD /r    VFNMADD213SD xmm1, xmm2, xmm3/m64
//
// VEX.128.66.0F38.W0 BC /r    VFNMADD231PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 BC /r    VFNMADD231PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 BC /r    VFNMADD231PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 BC /r    VFNMADD231PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 BD /r    VFNMADD231SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 BD /r    VFNMADD231SD xmm1, xmm2, xmm3/m64

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VFNMADD132PS Tests - 128-bit
// ============================================================================

#[test]
fn test_vfnmadd132ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9c, 0xc2, // VFNMADD132PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x61, 0x9c, 0xd4, // VFNMADD132PS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x49, 0x9c, 0xef, // VFNMADD132PS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x9c, 0xc2, // VFNMADD132PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x09, 0x9c, 0xef, // VFNMADD132PS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD132PS Tests - 256-bit
// ============================================================================

#[test]
fn test_vfnmadd132ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x9c, 0xc2, // VFNMADD132PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x65, 0x9c, 0xd4, // VFNMADD132PS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x4d, 0x9c, 0xef, // VFNMADD132PS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x9c, 0xc2, // VFNMADD132PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x0d, 0x9c, 0xef, // VFNMADD132PS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD132PD Tests - 128-bit
// ============================================================================

#[test]
fn test_vfnmadd132pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x9c, 0xc2, // VFNMADD132PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132pd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe1, 0x9c, 0xd4, // VFNMADD132PD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132pd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc9, 0x9c, 0xef, // VFNMADD132PD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x9c, 0xc2, // VFNMADD132PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132pd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0x9c, 0xef, // VFNMADD132PD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD132PD Tests - 256-bit
// ============================================================================

#[test]
fn test_vfnmadd132pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x9c, 0xc2, // VFNMADD132PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132pd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe5, 0x9c, 0xd4, // VFNMADD132PD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132pd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xcd, 0x9c, 0xef, // VFNMADD132PD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0x9c, 0xc2, // VFNMADD132PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132pd_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x8d, 0x9c, 0xef, // VFNMADD132PD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD132SS & VFNMADD132SD Tests
// ============================================================================

#[test]
fn test_vfnmadd132ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9d, 0xc2, // VFNMADD132SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x9d, 0xc2, // VFNMADD132SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x9d, 0xc2, // VFNMADD132SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x9d, 0xc2, // VFNMADD132SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132ss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x09, 0x9d, 0xef, // VFNMADD132SS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132sd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0x9d, 0xef, // VFNMADD132SD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD213PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfnmadd213ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xac, 0xc2, // VFNMADD213PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xac, 0xc2, // VFNMADD213PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xac, 0xc2, // VFNMADD213PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xac, 0xc2, // VFNMADD213PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD213PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfnmadd213pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xac, 0xc2, // VFNMADD213PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xac, 0xc2, // VFNMADD213PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xac, 0xc2, // VFNMADD213PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0xac, 0xc2, // VFNMADD213PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD213SS & VFNMADD213SD Tests
// ============================================================================

#[test]
fn test_vfnmadd213ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xad, 0xc2, // VFNMADD213SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xad, 0xc2, // VFNMADD213SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xad, 0xc2, // VFNMADD213SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xad, 0xc2, // VFNMADD213SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD231PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfnmadd231ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xbc, 0xc2, // VFNMADD231PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xbc, 0xc2, // VFNMADD231PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xbc, 0xc2, // VFNMADD231PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xbc, 0xc2, // VFNMADD231PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD231PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfnmadd231pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xbc, 0xc2, // VFNMADD231PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xbc, 0xc2, // VFNMADD231PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xbc, 0xc2, // VFNMADD231PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0xbc, 0xc2, // VFNMADD231PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFNMADD231SS & VFNMADD231SD Tests
// ============================================================================

#[test]
fn test_vfnmadd231ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xbd, 0xc2, // VFNMADD231SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xbd, 0xc2, // VFNMADD231SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xbd, 0xc2, // VFNMADD231SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xbd, 0xc2, // VFNMADD231SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_vfnmadd132ps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFNMADD132PS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd213pd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xac, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFNMADD213PD YMM0, YMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd231ss_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xbd, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFNMADD231SS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfnmadd132sd_xmm15_xmm8_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0xb9, 0x9d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFNMADD132SD XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
