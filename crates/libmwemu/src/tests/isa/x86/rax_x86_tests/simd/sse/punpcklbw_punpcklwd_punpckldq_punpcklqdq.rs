use crate::*;

// PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ - Unpack Low Data
//
// These instructions unpack and interleave the low-order data elements of the
// destination and source operands into the destination operand.
//
// PUNPCKLBW - Unpack low bytes
// PUNPCKLWD - Unpack low words
// PUNPCKLDQ - Unpack low doublewords
// PUNPCKLQDQ - Unpack low quadwords
//
// Opcodes:
// 66 0F 60 /r             PUNPCKLBW xmm1, xmm2/m128     - Unpack and interleave low bytes
// 66 0F 61 /r             PUNPCKLWD xmm1, xmm2/m128     - Unpack and interleave low words
// 66 0F 62 /r             PUNPCKLDQ xmm1, xmm2/m128     - Unpack and interleave low doublewords
// 66 0F 6C /r             PUNPCKLQDQ xmm1, xmm2/m128    - Unpack and interleave low quadwords

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// PUNPCKLBW Tests - Unpack Low Bytes
// ============================================================================

#[test]
fn test_punpcklbw_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x60, 0xc1, // PUNPCKLBW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklbw_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x60, 0xca, // PUNPCKLBW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklbw_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x60, 0xd3, // PUNPCKLBW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklbw_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x60, 0xdc, // PUNPCKLBW XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklbw_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x60, 0xf8, // PUNPCKLBW XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklbw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x60, 0xc1, // PUNPCKLBW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklbw_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x60, 0xf8, // PUNPCKLBW XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklbw_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x60, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLBW XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PUNPCKLWD Tests - Unpack Low Words
// ============================================================================

#[test]
fn test_punpcklwd_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x61, 0xc1, // PUNPCKLWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklwd_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x61, 0xca, // PUNPCKLWD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklwd_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x61, 0xd3, // PUNPCKLWD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklwd_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x61, 0xdc, // PUNPCKLWD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklwd_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x61, 0xf8, // PUNPCKLWD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklwd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x61, 0xc1, // PUNPCKLWD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklwd_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x61, 0xf8, // PUNPCKLWD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklwd_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x61, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLWD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PUNPCKLDQ Tests - Unpack Low Doublewords
// ============================================================================

#[test]
fn test_punpckldq_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x62, 0xc1, // PUNPCKLDQ XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckldq_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x62, 0xca, // PUNPCKLDQ XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckldq_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x62, 0xd3, // PUNPCKLDQ XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckldq_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x62, 0xdc, // PUNPCKLDQ XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckldq_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x62, 0xe5, // PUNPCKLDQ XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckldq_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x62, 0xf8, // PUNPCKLDQ XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckldq_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x62, 0xc1, // PUNPCKLDQ XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckldq_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x62, 0xf8, // PUNPCKLDQ XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckldq_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x62, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLDQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PUNPCKLQDQ Tests - Unpack Low Quadwords
// ============================================================================

#[test]
fn test_punpcklqdq_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0xc1, // PUNPCKLQDQ XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0xca, // PUNPCKLQDQ XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0xd3, // PUNPCKLQDQ XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0xdc, // PUNPCKLQDQ XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0xe5, // PUNPCKLQDQ XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0xee, // PUNPCKLQDQ XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0xf7, // PUNPCKLQDQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0xf8, // PUNPCKLQDQ XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x6c, 0xc1, // PUNPCKLQDQ XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x6c, 0xca, // PUNPCKLQDQ XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x6c, 0xd3, // PUNPCKLQDQ XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x6c, 0xf8, // PUNPCKLQDQ XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLQDQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm7_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLQDQ XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpcklqdq_xmm15_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x6c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLQDQ XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Tests - Various Unpack Operations
// ============================================================================

#[test]
fn test_unpack_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x60, 0xc1, // PUNPCKLBW XMM0, XMM1
        0x66, 0x0f, 0x61, 0xd3, // PUNPCKLWD XMM2, XMM3
        0x66, 0x0f, 0x62, 0xe5, // PUNPCKLDQ XMM4, XMM5
        0x66, 0x0f, 0x6c, 0xf7, // PUNPCKLQDQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
