use crate::*;

// PMULLW - Multiply Packed Signed Integers and Store Low Result
//
// Performs a SIMD signed multiply of the packed signed word integers from both
// source operands and stores the low 16 bits of each 32-bit result in the destination.
//
// Opcodes:
// NP 0F D5 /r         PMULLW mm, mm/m64        - Multiply packed signed words, store low
// 66 0F D5 /r         PMULLW xmm1, xmm2/m128   - Multiply packed signed words, store low

const DATA_ADDR: u64 = 0x3000;

// MMX Tests
#[test]
fn test_pmullw_mm0_mm1() {
    let mut emu = emu64();
    let code = [0x0f, 0xd5, 0xc1, 0xf4]; // PMULLW MM0, MM1; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_mm2_mm3() {
    let mut emu = emu64();
    let code = [0x0f, 0xd5, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_mm4_mm5() {
    let mut emu = emu64();
    let code = [0x0f, 0xd5, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_mm6_mm7() {
    let mut emu = emu64();
    let code = [0x0f, 0xd5, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_mm0_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0xd5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_mm7_mem() {
    let mut emu = emu64();
    let code = [0x0f, 0xd5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// SSE2 Tests
#[test]
fn test_pmullw_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xd5, 0xc1, 0xf4]; // PMULLW XMM0, XMM1; HLT
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xd5, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xd5, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xd5, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0xd5, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0xd5, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_xmm0_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xd5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_xmm7_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0xd5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_xmm15_mem() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0xd5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_multiple_mmx() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xd5, 0xc1, // PMULLW MM0, MM1
        0x0f, 0xd5, 0xd3, // PMULLW MM2, MM3
        0x0f, 0xd5, 0xe5, // PMULLW MM4, MM5
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_multiple_xmm() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd5, 0xc1, // PMULLW XMM0, XMM1
        0x66, 0x0f, 0xd5, 0xd3, // PMULLW XMM2, XMM3
        0x66, 0x0f, 0xd5, 0xe5, // PMULLW XMM4, XMM5
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_chain() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd5, 0xc0, // PMULLW XMM0, XMM0
        0x66, 0x0f, 0xd5, 0xc0, // PMULLW XMM0, XMM0
        0x66, 0x0f, 0xd5, 0xc0, // PMULLW XMM0, XMM0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_all_xmm_pairs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd5, 0xc1, // PMULLW XMM0, XMM1
        0x66, 0x0f, 0xd5, 0xda, // PMULLW XMM3, XMM2
        0x66, 0x0f, 0xd5, 0xe5, // PMULLW XMM4, XMM5
        0x66, 0x0f, 0xd5, 0xfe, // PMULLW XMM7, XMM6
        0x66, 0x45, 0x0f, 0xd5, 0xc1, // PMULLW XMM8, XMM9
        0x66, 0x45, 0x0f, 0xd5, 0xda, // PMULLW XMM11, XMM10
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_self_multiply() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd5, 0xc0, // PMULLW XMM0, XMM0
        0x66, 0x0f, 0xd5, 0xc9, // PMULLW XMM1, XMM1
        0x66, 0x0f, 0xd5, 0xd2, // PMULLW XMM2, XMM2
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xd5, 0xc8, // PMULLW XMM9, XMM8
        0x66, 0x45, 0x0f, 0xd5, 0xda, // PMULLW XMM11, XMM10
        0x66, 0x45, 0x0f, 0xd5, 0xec, // PMULLW XMM13, XMM12
        0x66, 0x45, 0x0f, 0xd5, 0xfe, // PMULLW XMM15, XMM14
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_cross_registers() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xd5, 0xc7, // PMULLW XMM0, XMM7
        0x66, 0x44, 0x0f, 0xd5, 0xc7, // PMULLW XMM8, XMM7
        0x66, 0x41, 0x0f, 0xd5, 0xc7, // PMULLW XMM0, XMM15
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmullw_mmx_all_regs() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xd5, 0xc1, // PMULLW MM0, MM1
        0x0f, 0xd5, 0xca, // PMULLW MM1, MM2
        0x0f, 0xd5, 0xd3, // PMULLW MM2, MM3
        0x0f, 0xd5, 0xdc, // PMULLW MM3, MM4
        0x0f, 0xd5, 0xe5, // PMULLW MM4, MM5
        0x0f, 0xd5, 0xee, // PMULLW MM5, MM6
        0x0f, 0xd5, 0xf7, // PMULLW MM6, MM7
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
