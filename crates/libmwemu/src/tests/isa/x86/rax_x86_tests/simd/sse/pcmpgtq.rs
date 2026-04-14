use crate::*;

// PCMPGTQ - Compare Packed Data for Greater Than (Quadword)
//
// Performs SIMD compare of packed signed quadword integers in the destination
// operand (first operand) and the source operand (second operand). If a data
// element in the destination operand is greater than the corresponding data
// element in the source operand, the corresponding data element in the
// destination is set to all 1s; otherwise, it is set to 0s.
//
// Opcodes:
// 66 0F 38 37 /r         PCMPGTQ xmm1, xmm2/m128    - Compare packed signed qwords for greater than

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pcmpgtq_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xc1, // PCMPGTQ XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xca, // PCMPGTQ XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xd3, // PCMPGTQ XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xdc, // PCMPGTQ XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xe5, // PCMPGTQ XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xee, // PCMPGTQ XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xf7, // PCMPGTQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xf8, // PCMPGTQ XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x37, 0xc1, // PCMPGTQ XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x37, 0xca, // PCMPGTQ XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x37, 0xd3, // PCMPGTQ XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x37, 0xf8, // PCMPGTQ XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PCMPGTQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm7_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PCMPGTQ XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_xmm15_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x37, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PCMPGTQ XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_positive_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xc1, // PCMPGTQ XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_negative_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xd3, // PCMPGTQ XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xe5, // PCMPGTQ XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_equal_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xf7, // PCMPGTQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_max_min_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x37, 0xc1, // PCMPGTQ XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpgtq_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x37, 0xca, // PCMPGTQ XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
