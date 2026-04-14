use crate::*;

// VMOVSD - Move Scalar Double-Precision Floating-Point

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovsd_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xfb, 0x10, 0xd1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf3, 0x10, 0xda, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xeb, 0x10, 0xe3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe3, 0x10, 0xec, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xdb, 0x10, 0xf5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd3, 0x10, 0xfe, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0x4b, 0x10, 0xc7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x43, 0x10, 0xc8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x3b, 0x10, 0xd1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x33, 0x10, 0xda, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x2b, 0x10, 0xe3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x23, 0x10, 0xec, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x1b, 0x10, 0xf5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x13, 0x10, 0xfe, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc5, 0x0b, 0x11, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsd_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc5, 0x83, 0x10, 0xc8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
