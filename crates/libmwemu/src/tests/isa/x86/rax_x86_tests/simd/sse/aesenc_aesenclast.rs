use crate::*;

// AESENC/AESENCLAST - AES Encrypt Round / AES Encrypt Last Round
//
// 66 0F 38 DC /r             AESENC xmm1, xmm2/m128
// 66 0F 38 DD /r             AESENCLAST xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// AESENC Tests
// ============================================================================

#[test]
fn test_aesenc_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdc, 0xc1, // AESENC XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenc_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdc, 0xca, // AESENC XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenc_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdc, 0xd3, // AESENC XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenc_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdc, 0xf8, // AESENC XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenc_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdc, 0xc1, // AESENC XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenc_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdc, 0xf8, // AESENC XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenc_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdc, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // AESENC XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// AESENCLAST Tests
// ============================================================================

#[test]
fn test_aesenclast_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xc1, // AESENCLAST XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenclast_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xca, // AESENCLAST XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenclast_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xd3, // AESENCLAST XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenclast_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xf8, // AESENCLAST XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenclast_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xc1, // AESENCLAST XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenclast_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdd, 0xf8, // AESENCLAST XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesenclast_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // AESENCLAST XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Tests - Encrypt sequence
// ============================================================================

#[test]
fn test_aes_encrypt_rounds() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdc, 0xc1, // AESENC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xdc, 0xc2, // AESENC XMM0, XMM2
        0x66, 0x0f, 0x38, 0xdd, 0xc3, // AESENCLAST XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
