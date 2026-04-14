use crate::*;

// AESIMC - AES Inverse Mix Columns
// AESKEYGENASSIST - AES Key Generation Assist
//
// AESIMC performs the InvMixColumns transformation on the source operand.
// AESKEYGENASSIST assists in expanding the AES cipher key by computing a round key.
//
// Opcodes:
// 66 0F 38 DB /r           AESIMC xmm1, xmm2/m128
// 66 0F 3A DF /r ib        AESKEYGENASSIST xmm1, xmm2/m128, imm8

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// AESIMC Tests
// ============================================================================

#[test]
fn test_aesimc_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0xdb, 0xc1, 0xf4]; // AESIMC XMM0, XMM1; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0xdb, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0xdb, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0xdb, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x38, 0xdb, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x38, 0xdb, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_xmm0_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0xdb, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_xmm7_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0xdb, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_xmm15_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x38, 0xdb, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_self() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0xdb, 0xc0, 0xf4]; // AESIMC XMM0, XMM0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_chain() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xc1, // AESIMC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xdb, 0xc8, // AESIMC XMM1, XMM0
        0x66, 0x0f, 0x38, 0xdb, 0xc1, // AESIMC XMM0, XMM1
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_all_pairs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xc1, // AESIMC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xdb, 0xda, // AESIMC XMM3, XMM2
        0x66, 0x0f, 0x38, 0xdb, 0xe5, // AESIMC XMM4, XMM5
        0x66, 0x0f, 0x38, 0xdb, 0xfe, // AESIMC XMM7, XMM6
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// AESKEYGENASSIST Tests
// ============================================================================

#[test]
fn test_aeskeygenassist_xmm0_xmm1_0x00() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x00, 0xf4]; // AESKEYGENASSIST XMM0, XMM1, 0x00
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm2_xmm3_0x01() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0xd3, 0x01, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm4_xmm5_0x0a() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0xe5, 0x0a, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm6_xmm7_0xff() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0xf7, 0xff, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm8_xmm9_0x01() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm14_xmm15_0x02() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xf7, 0x02, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm0_mem_0x01() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm7_mem_0x0a() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x0a, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_all_rounds() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM0, XMM1, 0x01
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x02, // AESKEYGENASSIST XMM0, XMM1, 0x02
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x04, // AESKEYGENASSIST XMM0, XMM1, 0x04
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x08, // AESKEYGENASSIST XMM0, XMM1, 0x08
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x10, // AESKEYGENASSIST XMM0, XMM1, 0x10
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_self() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0xc0, 0x01, 0xf4]; // AESKEYGENASSIST XMM0, XMM0, 0x01
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_extended_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM8, XMM9, 0x01
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xda, 0x02, // AESKEYGENASSIST XMM11, XMM10, 0x02
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xec, 0x04, // AESKEYGENASSIST XMM13, XMM12, 0x04
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xfe, 0x08, // AESKEYGENASSIST XMM15, XMM14, 0x08
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined AESIMC and AESKEYGENASSIST Tests
// ============================================================================

#[test]
fn test_aes_combined() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM0, XMM1, 0x01
        0x66, 0x0f, 0x38, 0xdb, 0xc0, // AESIMC XMM0, XMM0
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x02, // AESKEYGENASSIST XMM0, XMM1, 0x02
        0x66, 0x0f, 0x38, 0xdb, 0xc0, // AESIMC XMM0, XMM0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aes_key_expansion_sequence() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc8, 0x01, // AESKEYGENASSIST XMM1, XMM0, 0x01
        0x66, 0x0f, 0x38, 0xdb, 0xc8, // AESIMC XMM1, XMM0
        0x66, 0x0f, 0x3a, 0xdf, 0xd1, 0x02, // AESKEYGENASSIST XMM2, XMM1, 0x02
        0x66, 0x0f, 0x38, 0xdb, 0xd1, // AESIMC XMM2, XMM1
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aesimc_multiple() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xc1, // AESIMC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xdb, 0xd2, // AESIMC XMM2, XMM2
        0x66, 0x0f, 0x38, 0xdb, 0xe3, // AESIMC XMM4, XMM3
        0x66, 0x0f, 0x38, 0xdb, 0xf5, // AESIMC XMM6, XMM5
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_aeskeygenassist_all_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM0, XMM1, 0x01
        0x66, 0x0f, 0x3a, 0xdf, 0xda, 0x01, // AESKEYGENASSIST XMM3, XMM2, 0x01
        0x66, 0x0f, 0x3a, 0xdf, 0xe5, 0x01, // AESKEYGENASSIST XMM4, XMM5, 0x01
        0x66, 0x0f, 0x3a, 0xdf, 0xfe, 0x01, // AESKEYGENASSIST XMM7, XMM6, 0x01
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
