use crate::*;

// VMOVHLPS - Move High to Low Packed Single-Precision

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovhlps_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x12, 0xd1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf0, 0x12, 0xda, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xe8, 0x12, 0xe3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe0, 0x12, 0xec, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xd8, 0x12, 0xf5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd0, 0x12, 0xfe, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0x48, 0x12, 0xc7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x40, 0x12, 0xc8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x38, 0x12, 0xd1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x30, 0x12, 0xda, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x28, 0x12, 0xe3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x20, 0x12, 0xec, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x18, 0x12, 0xf5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x10, 0x12, 0xfe, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x08, 0x12, 0xc7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhlps_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc5, 0x80, 0x12, 0xc8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
