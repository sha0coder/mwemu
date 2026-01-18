use crate::*;

// VCOMISS - Compare Ordered Scalar Single-Precision Floating-Point Values
//
// Opcodes: VEX.LIG.0F.WIG 2F /r

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vcomiss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x2f, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcomiss_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x2f, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcomiss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x2f, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcomiss_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x2f, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcomiss_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x2f, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcomiss_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x2f, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcomiss_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x2f, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcomiss_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x78, 0x2f, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional tests for memory operands and YMM registers would go here
