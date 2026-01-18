use crate::*;

// VPERMPD - Permute Packed Double-Precision Floating-Point Elements
//
// VEX.256.66.0F3A.W1 01 /r ib        VPERMPD ymm1, ymm2/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpermpd_ymm0_ymm1_0x00() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xc1, 0x00, // VPERMPD YMM0, YMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_ymm0_ymm1_0x1b() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xc1, 0x1b, // VPERMPD YMM0, YMM1, 0x1B (reverse)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_ymm1_ymm2_0xe4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xca, 0xe4, // VPERMPD YMM1, YMM2, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_ymm2_ymm3_0x4e() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xd3, 0x4e, // VPERMPD YMM2, YMM3, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_ymm3_ymm4_0xb1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xdc, 0xb1, // VPERMPD YMM3, YMM4, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_ymm4_ymm5_0x27() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xe5, 0x27, // VPERMPD YMM4, YMM5, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_ymm7_ymm0_0xff() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xf8, 0xff, // VPERMPD YMM7, YMM0, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_ymm0_mem_0x1b() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x1b, // VPERMPD YMM0, [0x3000], 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_broadcast_first() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xc1, 0x00, // VPERMPD YMM0, YMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermpd_broadcast_last() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x01, 0xc1, 0xff, // VPERMPD YMM0, YMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
