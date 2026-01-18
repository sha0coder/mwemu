use crate::*;

// VROUNDPS - Round Packed Single-Precision Floating-Point Values
//
// Opcodes: VEX.128.66.0F3A.WIG 08 /r ib

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vroundps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x79, 0x08, 0xc1, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x79, 0x08, 0xca, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x79, 0x08, 0xd3, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x79, 0x08, 0xdc, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundps_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x79, 0x08, 0xe5, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x79, 0x08, 0xee, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundps_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x79, 0x08, 0xf7, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vroundps_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0xc3, 0x79, 0x08, 0xf8, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional tests for memory operands and YMM registers would go here
