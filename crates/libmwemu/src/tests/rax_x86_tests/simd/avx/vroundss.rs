use crate::*;

// VROUNDSS - Round Scalar Single-Precision Floating-Point Value
//
// Opcodes: VEX.LIG.66.0F3A.WIG 0A /r ib

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vroundss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x71, 0x0a, 0xc2, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundss_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x69, 0x0a, 0xcb, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x61, 0x0a, 0xd4, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundss_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x59, 0x0a, 0xdd, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundss_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x51, 0x0a, 0xe6, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundss_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x49, 0x0a, 0xef, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundss_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0xc3, 0x41, 0x0a, 0xf0, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundss_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0xc3, 0x39, 0x0a, 0xf9, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional tests for memory operands and YMM registers would go here
