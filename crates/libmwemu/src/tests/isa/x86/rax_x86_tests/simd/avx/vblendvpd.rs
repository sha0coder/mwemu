use crate::*;

// VBLENDVPD - Variable Blend Packed Double-Precision Floating-Point Values
//
// VEX.128.66.0F3A.W0 4B /r /is4       VBLENDVPD xmm1, xmm2, xmm3/m128, xmm4
// VEX.256.66.0F3A.W0 4B /r /is4       VBLENDVPD ymm1, ymm2, ymm3/m256, ymm4

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vblendvpd_xmm0_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x4b, 0xc2, 0x30, // VBLENDVPD XMM0, XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_xmm1_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x69, 0x4b, 0xcb, 0x40, // VBLENDVPD XMM1, XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_xmm2_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x61, 0x4b, 0xd4, 0x50, // VBLENDVPD XMM2, XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_xmm7_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x4b, 0xf9, 0x20, // VBLENDVPD XMM7, XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_xmm8_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x31, 0x4b, 0xc2, 0xb0, // VBLENDVPD XMM8, XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_xmm15_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x63, 0x79, 0x4b, 0xf9, 0x20, // VBLENDVPD XMM15, XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_ymm0_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x4b, 0xc2, 0x30, // VBLENDVPD YMM0, YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_ymm1_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x6d, 0x4b, 0xcb, 0x40, // VBLENDVPD YMM1, YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_ymm2_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x65, 0x4b, 0xd4, 0x50, // VBLENDVPD YMM2, YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_ymm7_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x4b, 0xf9, 0x20, // VBLENDVPD YMM7, YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vblendvpd_xmm0_xmm1_mem_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x4b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x30, // VBLENDVPD XMM0, XMM1, [0x3000], XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
