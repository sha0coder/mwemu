use crate::*;

// AESDEC/AESDECLAST - AES Decrypt Round / AES Decrypt Last Round
//
// 66 0F 38 DE /r             AESDEC xmm1, xmm2/m128
// 66 0F 38 DF /r             AESDECLAST xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_aesdec_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesdec_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xca, // AESDEC XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesdec_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesdec_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesdeclast_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xc1, // AESDECLAST XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesdeclast_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xca, // AESDECLAST XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesdeclast_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xc1, // AESDECLAST XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesdeclast_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDECLAST XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aes_decrypt_rounds() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xde, 0xc2, // AESDEC XMM0, XMM2
        0x66, 0x0f, 0x38, 0xdf, 0xc3, // AESDECLAST XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
