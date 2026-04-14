use crate::*;

// VPERMPS - Permute Packed Single-Precision Floating-Point Elements
//
// VEX.256.66.0F38.W0 16 /r        VPERMPS ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpermps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x16, 0xc2, // VPERMPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x6d, 0x16, 0xcb, // VPERMPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x65, 0x16, 0xd4, // VPERMPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x16, 0xdd, // VPERMPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermps_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x55, 0x16, 0xe6, // VPERMPS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x4d, 0x16, 0xef, // VPERMPS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermps_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x16, 0xf9, // VPERMPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermps_ymm0_ymm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VPERMPS YMM0, YMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermps_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x16, 0xc2, // VPERMPS YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x65, 0x16, 0xd4, // VPERMPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
