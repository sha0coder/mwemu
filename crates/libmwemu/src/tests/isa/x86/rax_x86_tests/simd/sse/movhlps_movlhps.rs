use crate::*;

// MOVHLPS - Move Packed Single Precision Floating-Point Values High to Low
// MOVLHPS - Move Packed Single Precision Floating-Point Values Low to High
//
// MOVHLPS moves two packed single-precision values from high quadword to low quadword
// MOVLHPS moves two packed single-precision values from low quadword to high quadword
//
// These instructions can only be used with register operands (not memory).
//
// Opcodes:
// NP 0F 12 /r             MOVHLPS xmm1, xmm2    - Move high to low
// NP 0F 16 /r             MOVLHPS xmm1, xmm2    - Move low to high

// ============================================================================
// MOVHLPS Tests - Move High to Low
// ============================================================================

#[test]
fn test_movhlps_xmm0_xmm1() {
    let mut emu = emu64();
    // MOVHLPS XMM0, XMM1 - Move high quadword of XMM1 to low quadword of XMM0
    let code = [
        0x0f, 0x12, 0xc1, // MOVHLPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm1_xmm2() {
    let mut emu = emu64();
    // MOVHLPS XMM1, XMM2
    let code = [
        0x0f, 0x12, 0xca, // MOVHLPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm2_xmm3() {
    let mut emu = emu64();
    // MOVHLPS XMM2, XMM3
    let code = [
        0x0f, 0x12, 0xd3, // MOVHLPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm3_xmm4() {
    let mut emu = emu64();
    // MOVHLPS XMM3, XMM4
    let code = [
        0x0f, 0x12, 0xdc, // MOVHLPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm4_xmm5() {
    let mut emu = emu64();
    // MOVHLPS XMM4, XMM5
    let code = [
        0x0f, 0x12, 0xe5, // MOVHLPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm5_xmm6() {
    let mut emu = emu64();
    // MOVHLPS XMM5, XMM6
    let code = [
        0x0f, 0x12, 0xee, // MOVHLPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm6_xmm7() {
    let mut emu = emu64();
    // MOVHLPS XMM6, XMM7
    let code = [
        0x0f, 0x12, 0xf7, // MOVHLPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm7_xmm0() {
    let mut emu = emu64();
    // MOVHLPS XMM7, XMM0
    let code = [
        0x0f, 0x12, 0xf8, // MOVHLPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm8_xmm9() {
    let mut emu = emu64();
    // MOVHLPS XMM8, XMM9
    let code = [
        0x45, 0x0f, 0x12, 0xc1, // MOVHLPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm9_xmm10() {
    let mut emu = emu64();
    // MOVHLPS XMM9, XMM10
    let code = [
        0x45, 0x0f, 0x12, 0xca, // MOVHLPS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm10_xmm11() {
    let mut emu = emu64();
    // MOVHLPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x12, 0xd3, // MOVHLPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm11_xmm12() {
    let mut emu = emu64();
    // MOVHLPS XMM11, XMM12
    let code = [
        0x45, 0x0f, 0x12, 0xdc, // MOVHLPS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm12_xmm13() {
    let mut emu = emu64();
    // MOVHLPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x12, 0xe5, // MOVHLPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm13_xmm14() {
    let mut emu = emu64();
    // MOVHLPS XMM13, XMM14
    let code = [
        0x45, 0x0f, 0x12, 0xee, // MOVHLPS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm14_xmm15() {
    let mut emu = emu64();
    // MOVHLPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x12, 0xf7, // MOVHLPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_xmm15_xmm8() {
    let mut emu = emu64();
    // MOVHLPS XMM15, XMM8
    let code = [
        0x45, 0x0f, 0x12, 0xf8, // MOVHLPS XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_same_register() {
    let mut emu = emu64();
    // MOVHLPS XMM0, XMM0 - Move high to low within same register
    let code = [
        0x0f, 0x12, 0xc0, // MOVHLPS XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVLHPS Tests - Move Low to High
// ============================================================================

#[test]
fn test_movlhps_xmm0_xmm1() {
    let mut emu = emu64();
    // MOVLHPS XMM0, XMM1 - Move low quadword of XMM1 to high quadword of XMM0
    let code = [
        0x0f, 0x16, 0xc1, // MOVLHPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm1_xmm2() {
    let mut emu = emu64();
    // MOVLHPS XMM1, XMM2
    let code = [
        0x0f, 0x16, 0xca, // MOVLHPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm2_xmm3() {
    let mut emu = emu64();
    // MOVLHPS XMM2, XMM3
    let code = [
        0x0f, 0x16, 0xd3, // MOVLHPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm3_xmm4() {
    let mut emu = emu64();
    // MOVLHPS XMM3, XMM4
    let code = [
        0x0f, 0x16, 0xdc, // MOVLHPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm4_xmm5() {
    let mut emu = emu64();
    // MOVLHPS XMM4, XMM5
    let code = [
        0x0f, 0x16, 0xe5, // MOVLHPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm5_xmm6() {
    let mut emu = emu64();
    // MOVLHPS XMM5, XMM6
    let code = [
        0x0f, 0x16, 0xee, // MOVLHPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm6_xmm7() {
    let mut emu = emu64();
    // MOVLHPS XMM6, XMM7
    let code = [
        0x0f, 0x16, 0xf7, // MOVLHPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm7_xmm0() {
    let mut emu = emu64();
    // MOVLHPS XMM7, XMM0
    let code = [
        0x0f, 0x16, 0xf8, // MOVLHPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm8_xmm9() {
    let mut emu = emu64();
    // MOVLHPS XMM8, XMM9
    let code = [
        0x45, 0x0f, 0x16, 0xc1, // MOVLHPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm9_xmm10() {
    let mut emu = emu64();
    // MOVLHPS XMM9, XMM10
    let code = [
        0x45, 0x0f, 0x16, 0xca, // MOVLHPS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm10_xmm11() {
    let mut emu = emu64();
    // MOVLHPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x16, 0xd3, // MOVLHPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm11_xmm12() {
    let mut emu = emu64();
    // MOVLHPS XMM11, XMM12
    let code = [
        0x45, 0x0f, 0x16, 0xdc, // MOVLHPS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm12_xmm13() {
    let mut emu = emu64();
    // MOVLHPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x16, 0xe5, // MOVLHPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm13_xmm14() {
    let mut emu = emu64();
    // MOVLHPS XMM13, XMM14
    let code = [
        0x45, 0x0f, 0x16, 0xee, // MOVLHPS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm14_xmm15() {
    let mut emu = emu64();
    // MOVLHPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x16, 0xf7, // MOVLHPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_xmm15_xmm8() {
    let mut emu = emu64();
    // MOVLHPS XMM15, XMM8
    let code = [
        0x45, 0x0f, 0x16, 0xf8, // MOVLHPS XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_same_register() {
    let mut emu = emu64();
    // MOVLHPS XMM0, XMM0 - Move low to high within same register
    let code = [
        0x0f, 0x16, 0xc0, // MOVLHPS XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Tests - MOVHLPS and MOVLHPS
// ============================================================================

#[test]
fn test_movhlps_movlhps_sequence() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x12, 0xc1, // MOVHLPS XMM0, XMM1
        0x0f, 0x16, 0xd2, // MOVLHPS XMM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_movhlps_sequence() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x16, 0xc1, // MOVLHPS XMM0, XMM1
        0x0f, 0x12, 0xd2, // MOVHLPS XMM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_chain() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x12, 0xc1, // MOVHLPS XMM0, XMM1
        0x0f, 0x12, 0xd0, // MOVHLPS XMM2, XMM0
        0x0f, 0x12, 0xda, // MOVHLPS XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movlhps_chain() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x16, 0xc1, // MOVLHPS XMM0, XMM1
        0x0f, 0x16, 0xd0, // MOVLHPS XMM2, XMM0
        0x0f, 0x16, 0xda, // MOVLHPS XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movhlps_movlhps_swap() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x12, 0xc1, // MOVHLPS XMM0, XMM1
        0x0f, 0x16, 0xc8, // MOVLHPS XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
