use crate::*;

// VFMADDSUB - Fused Multiply-Alternating Add/Subtract Packed Floating-Point Values
// VFMSUBADD - Fused Multiply-Alternating Subtract/Add Packed Floating-Point Values
//
// VFMADDSUB performs FMA with alternating addition/subtraction on even/odd elements
// VFMSUBADD performs FMA with alternating subtraction/addition on even/odd elements
//
// Opcodes:
// VEX.128.66.0F38.W0 96 /r    VFMADDSUB132PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 96 /r    VFMADDSUB132PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 96 /r    VFMADDSUB132PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 96 /r    VFMADDSUB132PD ymm1, ymm2, ymm3/m256
//
// VEX.128.66.0F38.W0 A6 /r    VFMADDSUB213PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 A6 /r    VFMADDSUB213PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 A6 /r    VFMADDSUB213PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 A6 /r    VFMADDSUB213PD ymm1, ymm2, ymm3/m256
//
// VEX.128.66.0F38.W0 B6 /r    VFMADDSUB231PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 B6 /r    VFMADDSUB231PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 B6 /r    VFMADDSUB231PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 B6 /r    VFMADDSUB231PD ymm1, ymm2, ymm3/m256
//
// VEX.128.66.0F38.W0 97 /r    VFMSUBADD132PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 97 /r    VFMSUBADD132PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 97 /r    VFMSUBADD132PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 97 /r    VFMSUBADD132PD ymm1, ymm2, ymm3/m256
//
// VEX.128.66.0F38.W0 A7 /r    VFMSUBADD213PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 A7 /r    VFMSUBADD213PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 A7 /r    VFMSUBADD213PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 A7 /r    VFMSUBADD213PD ymm1, ymm2, ymm3/m256
//
// VEX.128.66.0F38.W0 B7 /r    VFMSUBADD231PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 B7 /r    VFMSUBADD231PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 B7 /r    VFMSUBADD231PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 B7 /r    VFMSUBADD231PD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VFMADDSUB132PS Tests - 128-bit
// ============================================================================

#[test]
fn test_vfmaddsub132ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x96, 0xc2, // VFMADDSUB132PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x69, 0x96, 0xcb, // VFMADDSUB132PS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x61, 0x96, 0xd4, // VFMADDSUB132PS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x49, 0x96, 0xef, // VFMADDSUB132PS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x96, 0xc2, // VFMADDSUB132PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x09, 0x96, 0xef, // VFMADDSUB132PS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADDSUB132PS Tests - 256-bit
// ============================================================================

#[test]
fn test_vfmaddsub132ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x96, 0xc2, // VFMADDSUB132PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x6d, 0x96, 0xcb, // VFMADDSUB132PS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x4d, 0x96, 0xef, // VFMADDSUB132PS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x96, 0xc2, // VFMADDSUB132PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132ps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x0d, 0x96, 0xef, // VFMADDSUB132PS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADDSUB132PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmaddsub132pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x96, 0xc2, // VFMADDSUB132PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x96, 0xc2, // VFMADDSUB132PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x96, 0xc2, // VFMADDSUB132PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub132pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0x96, 0xc2, // VFMADDSUB132PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADDSUB213PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmaddsub213ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xa6, 0xc2, // VFMADDSUB213PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub213ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xa6, 0xc2, // VFMADDSUB213PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub213ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xa6, 0xc2, // VFMADDSUB213PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub213ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xa6, 0xc2, // VFMADDSUB213PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADDSUB213PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmaddsub213pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xa6, 0xc2, // VFMADDSUB213PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub213pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xa6, 0xc2, // VFMADDSUB213PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub213pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xa6, 0xc2, // VFMADDSUB213PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub213pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0xa6, 0xc2, // VFMADDSUB213PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADDSUB231PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmaddsub231ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xb6, 0xc2, // VFMADDSUB231PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub231ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xb6, 0xc2, // VFMADDSUB231PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub231ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xb6, 0xc2, // VFMADDSUB231PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub231ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xb6, 0xc2, // VFMADDSUB231PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADDSUB231PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmaddsub231pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xb6, 0xc2, // VFMADDSUB231PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub231pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xb6, 0xc2, // VFMADDSUB231PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub231pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xb6, 0xc2, // VFMADDSUB231PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub231pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0xb6, 0xc2, // VFMADDSUB231PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUBADD132PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsubadd132ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x97, 0xc2, // VFMSUBADD132PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd132ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x97, 0xc2, // VFMSUBADD132PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd132ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x97, 0xc2, // VFMSUBADD132PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd132ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x97, 0xc2, // VFMSUBADD132PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUBADD132PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsubadd132pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x97, 0xc2, // VFMSUBADD132PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd132pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x97, 0xc2, // VFMSUBADD132PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd132pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x97, 0xc2, // VFMSUBADD132PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd132pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0x97, 0xc2, // VFMSUBADD132PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUBADD213PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsubadd213ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xa7, 0xc2, // VFMSUBADD213PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd213ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xa7, 0xc2, // VFMSUBADD213PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd213ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xa7, 0xc2, // VFMSUBADD213PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd213ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xa7, 0xc2, // VFMSUBADD213PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUBADD213PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsubadd213pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xa7, 0xc2, // VFMSUBADD213PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd213pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xa7, 0xc2, // VFMSUBADD213PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd213pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xa7, 0xc2, // VFMSUBADD213PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd213pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0xa7, 0xc2, // VFMSUBADD213PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUBADD231PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsubadd231ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xb7, 0xc2, // VFMSUBADD231PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd231ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xb7, 0xc2, // VFMSUBADD231PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd231ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xb7, 0xc2, // VFMSUBADD231PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd231ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xb7, 0xc2, // VFMSUBADD231PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUBADD231PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsubadd231pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xb7, 0xc2, // VFMSUBADD231PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd231pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xb7, 0xc2, // VFMSUBADD231PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd231pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xb7, 0xc2, // VFMSUBADD231PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd231pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0xb7, 0xc2, // VFMSUBADD231PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_vfmaddsub132ps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x96, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADDSUB132PS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmaddsub213pd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xa6, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADDSUB213PD YMM0, YMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd132ps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x97, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMSUBADD132PS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsubadd231pd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xb7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMSUBADD231PD YMM0, YMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
