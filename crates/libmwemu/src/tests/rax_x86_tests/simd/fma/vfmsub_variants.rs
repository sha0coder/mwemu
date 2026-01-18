use crate::*;

// VFMSUB - Fused Multiply-Subtract Floating-Point Values
//
// These instructions perform fused multiply-subtract operations: (a * b) - c
// Available in 132, 213, and 231 variants for PS/PD/SS/SD
//
// Opcodes:
// VEX.128.66.0F38.W0 9A /r    VFMSUB132PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 9A /r    VFMSUB132PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 9A /r    VFMSUB132PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 9A /r    VFMSUB132PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 9B /r    VFMSUB132SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 9B /r    VFMSUB132SD xmm1, xmm2, xmm3/m64
//
// VEX.128.66.0F38.W0 AA /r    VFMSUB213PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 AA /r    VFMSUB213PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 AA /r    VFMSUB213PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 AA /r    VFMSUB213PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 AB /r    VFMSUB213SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 AB /r    VFMSUB213SD xmm1, xmm2, xmm3/m64
//
// VEX.128.66.0F38.W0 BA /r    VFMSUB231PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 BA /r    VFMSUB231PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 BA /r    VFMSUB231PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 BA /r    VFMSUB231PD ymm1, ymm2, ymm3/m256
// VEX.LIG.66.0F38.W0 BB /r    VFMSUB231SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W1 BB /r    VFMSUB231SD xmm1, xmm2, xmm3/m64

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VFMSUB132PS Tests - 128-bit
// ============================================================================

#[test]
fn test_vfmsub132ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9a, 0xc2, // VFMSUB132PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x61, 0x9a, 0xd4, // VFMSUB132PS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x9a, 0xc2, // VFMSUB132PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x09, 0x9a, 0xef, // VFMSUB132PS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB132PS Tests - 256-bit
// ============================================================================

#[test]
fn test_vfmsub132ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x9a, 0xc2, // VFMSUB132PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x65, 0x9a, 0xd4, // VFMSUB132PS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x9a, 0xc2, // VFMSUB132PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x0d, 0x9a, 0xef, // VFMSUB132PS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB132PD Tests - 128-bit
// ============================================================================

#[test]
fn test_vfmsub132pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x9a, 0xc2, // VFMSUB132PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132pd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe1, 0x9a, 0xd4, // VFMSUB132PD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x9a, 0xc2, // VFMSUB132PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132pd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0x9a, 0xef, // VFMSUB132PD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB132PD Tests - 256-bit
// ============================================================================

#[test]
fn test_vfmsub132pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x9a, 0xc2, // VFMSUB132PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132pd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe5, 0x9a, 0xd4, // VFMSUB132PD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb5, 0x9a, 0xc2, // VFMSUB132PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132pd_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x8d, 0x9a, 0xef, // VFMSUB132PD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB132SS Tests
// ============================================================================

#[test]
fn test_vfmsub132ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9b, 0xc2, // VFMSUB132SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x61, 0x9b, 0xd4, // VFMSUB132SS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x9b, 0xc2, // VFMSUB132SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132ss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x09, 0x9b, 0xef, // VFMSUB132SS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB132SD Tests
// ============================================================================

#[test]
fn test_vfmsub132sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x9b, 0xc2, // VFMSUB132SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132sd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe1, 0x9b, 0xd4, // VFMSUB132SD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x9b, 0xc2, // VFMSUB132SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132sd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0x9b, 0xef, // VFMSUB132SD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB213PS Tests - 128-bit
// ============================================================================

#[test]
fn test_vfmsub213ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xaa, 0xc2, // VFMSUB213PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub213ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xaa, 0xc2, // VFMSUB213PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB213PS Tests - 256-bit
// ============================================================================

#[test]
fn test_vfmsub213ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xaa, 0xc2, // VFMSUB213PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub213ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0xaa, 0xc2, // VFMSUB213PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB213PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsub213pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xaa, 0xc2, // VFMSUB213PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub213pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xaa, 0xc2, // VFMSUB213PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub213pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xaa, 0xc2, // VFMSUB213PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB213SS & VFMSUB213SD Tests
// ============================================================================

#[test]
fn test_vfmsub213ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xab, 0xc2, // VFMSUB213SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub213sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xab, 0xc2, // VFMSUB213SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub213ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xab, 0xc2, // VFMSUB213SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub213sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xab, 0xc2, // VFMSUB213SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB231PS Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsub231ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xba, 0xc2, // VFMSUB231PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub231ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0xba, 0xc2, // VFMSUB231PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub231ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xba, 0xc2, // VFMSUB231PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB231PD Tests - 128-bit & 256-bit
// ============================================================================

#[test]
fn test_vfmsub231pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xba, 0xc2, // VFMSUB231PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub231pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xba, 0xc2, // VFMSUB231PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub231pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xba, 0xc2, // VFMSUB231PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMSUB231SS & VFMSUB231SD Tests
// ============================================================================

#[test]
fn test_vfmsub231ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xbb, 0xc2, // VFMSUB231SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub231sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xbb, 0xc2, // VFMSUB231SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub231ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0xbb, 0xc2, // VFMSUB231SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub231sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xbb, 0xc2, // VFMSUB231SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_vfmsub132ps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x9a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMSUB132PS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub213pd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0xaa, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMSUB213PD YMM0, YMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub231ss_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0xbb, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMSUB231SS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmsub132sd_xmm15_xmm8_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0xb9, 0x9b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMSUB132SD XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
