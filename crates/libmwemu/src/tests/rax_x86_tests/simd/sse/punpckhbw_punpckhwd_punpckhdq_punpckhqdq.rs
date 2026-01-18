use crate::*;

// PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ - Unpack High Data
//
// These instructions unpack and interleave the high-order data elements of the
// destination and source operands into the destination operand.
//
// PUNPCKHBW - Unpack high bytes
// PUNPCKHWD - Unpack high words
// PUNPCKHDQ - Unpack high doublewords
// PUNPCKHQDQ - Unpack high quadwords
//
// Opcodes:
// 66 0F 68 /r             PUNPCKHBW xmm1, xmm2/m128     - Unpack and interleave high bytes
// 66 0F 69 /r             PUNPCKHWD xmm1, xmm2/m128     - Unpack and interleave high words
// 66 0F 6A /r             PUNPCKHDQ xmm1, xmm2/m128     - Unpack and interleave high doublewords
// 66 0F 6D /r             PUNPCKHQDQ xmm1, xmm2/m128    - Unpack and interleave high quadwords

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// PUNPCKHBW Tests - Unpack High Bytes
// ============================================================================

#[test]
fn test_punpckhbw_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x68, 0xc1, // PUNPCKHBW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhbw_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x68, 0xca, // PUNPCKHBW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhbw_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x68, 0xd3, // PUNPCKHBW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhbw_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x68, 0xf8, // PUNPCKHBW XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhbw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x68, 0xc1, // PUNPCKHBW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhbw_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x68, 0xf8, // PUNPCKHBW XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhbw_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x68, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHBW XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhbw_xmm7_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x68, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHBW XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PUNPCKHWD Tests - Unpack High Words
// ============================================================================

#[test]
fn test_punpckhwd_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x69, 0xc1, // PUNPCKHWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x69, 0xca, // PUNPCKHWD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x69, 0xd3, // PUNPCKHWD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x69, 0xdc, // PUNPCKHWD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x69, 0xe5, // PUNPCKHWD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x69, 0xf8, // PUNPCKHWD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x69, 0xc1, // PUNPCKHWD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x69, 0xf8, // PUNPCKHWD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x69, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHWD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhwd_xmm7_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x69, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHWD XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PUNPCKHDQ Tests - Unpack High Doublewords
// ============================================================================

#[test]
fn test_punpckhdq_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0xc1, // PUNPCKHDQ XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0xca, // PUNPCKHDQ XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0xd3, // PUNPCKHDQ XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0xdc, // PUNPCKHDQ XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0xe5, // PUNPCKHDQ XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0xee, // PUNPCKHDQ XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0xf7, // PUNPCKHDQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0xf8, // PUNPCKHDQ XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x6a, 0xc1, // PUNPCKHDQ XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x6a, 0xf8, // PUNPCKHDQ XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHDQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhdq_xmm7_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHDQ XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// PUNPCKHQDQ Tests - Unpack High Quadwords
// ============================================================================

#[test]
fn test_punpckhqdq_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0xc1, // PUNPCKHQDQ XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0xca, // PUNPCKHQDQ XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0xd3, // PUNPCKHQDQ XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0xdc, // PUNPCKHQDQ XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0xe5, // PUNPCKHQDQ XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0xee, // PUNPCKHQDQ XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0xf7, // PUNPCKHQDQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0xf8, // PUNPCKHQDQ XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x6d, 0xc1, // PUNPCKHQDQ XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x6d, 0xca, // PUNPCKHQDQ XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x6d, 0xd3, // PUNPCKHQDQ XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x6d, 0xf8, // PUNPCKHQDQ XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHQDQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm7_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHQDQ XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_punpckhqdq_xmm15_mem() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x6d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHQDQ XMM15, [0x3000]
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
        0x66, 0x0f, 0x68, 0xc1, // PUNPCKHBW XMM0, XMM1
        0x66, 0x0f, 0x69, 0xd3, // PUNPCKHWD XMM2, XMM3
        0x66, 0x0f, 0x6a, 0xe5, // PUNPCKHDQ XMM4, XMM5
        0x66, 0x0f, 0x6d, 0xf7, // PUNPCKHQDQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
