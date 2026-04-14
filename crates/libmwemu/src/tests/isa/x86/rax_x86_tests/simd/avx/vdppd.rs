use crate::*;

// VDPPD - Dot Product of Packed Double-Precision Floating-Point Values
//
// VEX.128.66.0F3A.W0 41 /r ib       VDPPD xmm1, xmm2, xmm3/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vdppd_xmm0_xmm1_xmm2_0xff() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x41, 0xc2, 0xff, // VDPPD XMM0, XMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_xmm1_xmm2_xmm3_0x33() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x69, 0x41, 0xcb, 0x33, // VDPPD XMM1, XMM2, XMM3, 0x33
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_xmm2_xmm3_xmm4_0x11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x61, 0x41, 0xd4, 0x11, // VDPPD XMM2, XMM3, XMM4, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_xmm3_xmm4_xmm5_0x22() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x59, 0x41, 0xdd, 0x22, // VDPPD XMM3, XMM4, XMM5, 0x22
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_xmm7_xmm0_xmm1_0xff() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x41, 0xf9, 0xff, // VDPPD XMM7, XMM0, XMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_xmm8_xmm9_xmm10_0xf0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x31, 0x41, 0xc2, 0xf0, // VDPPD XMM8, XMM9, XMM10, 0xF0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_xmm15_xmm0_xmm1_0x0f() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x63, 0x79, 0x41, 0xf9, 0x0f, // VDPPD XMM15, XMM0, XMM1, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_xmm0_xmm1_mem_0xff() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x41, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xff, // VDPPD XMM0, XMM1, [0x3000], 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_both_elements() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x41, 0xc2, 0x33, // VDPPD XMM0, XMM1, XMM2, 0x33 (both elements)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_first_element_only() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x41, 0xc2, 0x31, // VDPPD XMM0, XMM1, XMM2, 0x31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdppd_second_element_only() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x41, 0xc2, 0x32, // VDPPD XMM0, XMM1, XMM2, 0x32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
