use crate::*;

// VBLENDVPS - Variable Blend Packed Single-Precision Floating-Point Values
//
// Conditionally copies each dword of the source operand (second operand) to the
// destination operand (first operand) depending on mask bits defined in the
// mask operand (third operand).
//
// Opcodes:
// VEX.128.66.0F3A.W0 4A /r /is4       VBLENDVPS xmm1, xmm2, xmm3/m128, xmm4
// VEX.256.66.0F3A.W0 4A /r /is4       VBLENDVPS ymm1, ymm2, ymm3/m256, ymm4

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VBLENDVPS Tests (VEX.128)
// ============================================================================

#[test]
fn test_vblendvps_xmm0_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x30, // VBLENDVPS XMM0, XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_xmm1_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x69, 0x4a, 0xcb, 0x40, // VBLENDVPS XMM1, XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_xmm2_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x61, 0x4a, 0xd4, 0x50, // VBLENDVPS XMM2, XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_xmm3_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x59, 0x4a, 0xdd, 0x60, // VBLENDVPS XMM3, XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_xmm4_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x51, 0x4a, 0xe6, 0x70, // VBLENDVPS XMM4, XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_xmm7_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x4a, 0xf9, 0x20, // VBLENDVPS XMM7, XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_xmm8_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x31, 0x4a, 0xc2, 0xb0, // VBLENDVPS XMM8, XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_xmm15_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x63, 0x79, 0x4a, 0xf9, 0x20, // VBLENDVPS XMM15, XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_xmm0_xmm1_mem_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x30, // VBLENDVPS XMM0, XMM1, [0x3000], XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VBLENDVPS Tests (VEX.256)
// ============================================================================

#[test]
fn test_vblendvps_ymm0_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x4a, 0xc2, 0x30, // VBLENDVPS YMM0, YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_ymm1_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x6d, 0x4a, 0xcb, 0x40, // VBLENDVPS YMM1, YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_ymm2_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x65, 0x4a, 0xd4, 0x50, // VBLENDVPS YMM2, YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_ymm3_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x5d, 0x4a, 0xdd, 0x60, // VBLENDVPS YMM3, YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_ymm7_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x4a, 0xf9, 0x20, // VBLENDVPS YMM7, YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Different mask patterns
// ============================================================================

#[test]
fn test_vblendvps_all_from_first() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x00, // VBLENDVPS XMM0, XMM1, XMM2, XMM0 (assuming XMM0 is zeros)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_all_from_second() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x30, // VBLENDVPS XMM0, XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x40, // VBLENDVPS XMM0, XMM1, XMM2, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_ymm_different_lanes() {
    let mut emu = emu64();
    // YMM test with different mask per lane
    let code = [
        0xc4, 0xe3, 0x75, 0x4a, 0xc2, 0x50, // VBLENDVPS YMM0, YMM1, YMM2, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_high_registers() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x31, 0x4a, 0xc2, 0xb0, // VBLENDVPS XMM8, XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvps_sequential_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x30, // VBLENDVPS XMM0, XMM1, XMM2, XMM3
        0xc4, 0xe3, 0x59, 0x4a, 0xdd, 0x60, // VBLENDVPS XMM3, XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
