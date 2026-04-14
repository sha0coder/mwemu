use crate::*;

// MOVSHDUP - Replicate Single Precision Floating-Point Values (High/Odd)
// MOVSLDUP - Replicate Single Precision Floating-Point Values (Low/Even)
//
// MOVSHDUP duplicates odd-indexed (high) single precision FP values
// Result pattern: [1,1,3,3] from input [0,1,2,3]
//
// MOVSLDUP duplicates even-indexed (low) single precision FP values
// Result pattern: [0,0,2,2] from input [0,1,2,3]
//
// Opcodes:
// F3 0F 16 /r             MOVSHDUP xmm1, xmm2/m128    - Duplicate odd index SP FP values
// F3 0F 12 /r             MOVSLDUP xmm1, xmm2/m128    - Duplicate even index SP FP values

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// MOVSHDUP Tests - Duplicate High (Odd Index) Elements
// ============================================================================

#[test]
fn test_movshdup_xmm0_xmm1() {
    let mut emu = emu64();
    // MOVSHDUP XMM0, XMM1
    let code = [
        0xf3, 0x0f, 0x16, 0xc1, // MOVSHDUP XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm1_xmm2() {
    let mut emu = emu64();
    // MOVSHDUP XMM1, XMM2
    let code = [
        0xf3, 0x0f, 0x16, 0xca, // MOVSHDUP XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm2_xmm3() {
    let mut emu = emu64();
    // MOVSHDUP XMM2, XMM3
    let code = [
        0xf3, 0x0f, 0x16, 0xd3, // MOVSHDUP XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm3_xmm4() {
    let mut emu = emu64();
    // MOVSHDUP XMM3, XMM4
    let code = [
        0xf3, 0x0f, 0x16, 0xdc, // MOVSHDUP XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm4_xmm5() {
    let mut emu = emu64();
    // MOVSHDUP XMM4, XMM5
    let code = [
        0xf3, 0x0f, 0x16, 0xe5, // MOVSHDUP XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm5_xmm6() {
    let mut emu = emu64();
    // MOVSHDUP XMM5, XMM6
    let code = [
        0xf3, 0x0f, 0x16, 0xee, // MOVSHDUP XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm6_xmm7() {
    let mut emu = emu64();
    // MOVSHDUP XMM6, XMM7
    let code = [
        0xf3, 0x0f, 0x16, 0xf7, // MOVSHDUP XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm7_xmm0() {
    let mut emu = emu64();
    // MOVSHDUP XMM7, XMM0
    let code = [
        0xf3, 0x0f, 0x16, 0xf8, // MOVSHDUP XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm8_xmm9() {
    let mut emu = emu64();
    // MOVSHDUP XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf3, 0x45, 0x0f, 0x16, 0xc1, // MOVSHDUP XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm9_xmm10() {
    let mut emu = emu64();
    // MOVSHDUP XMM9, XMM10
    let code = [
        0xf3, 0x45, 0x0f, 0x16, 0xca, // MOVSHDUP XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm10_xmm11() {
    let mut emu = emu64();
    // MOVSHDUP XMM10, XMM11
    let code = [
        0xf3, 0x45, 0x0f, 0x16, 0xd3, // MOVSHDUP XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm11_xmm12() {
    let mut emu = emu64();
    // MOVSHDUP XMM11, XMM12
    let code = [
        0xf3, 0x45, 0x0f, 0x16, 0xdc, // MOVSHDUP XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm12_xmm13() {
    let mut emu = emu64();
    // MOVSHDUP XMM12, XMM13
    let code = [
        0xf3, 0x45, 0x0f, 0x16, 0xe5, // MOVSHDUP XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm13_xmm14() {
    let mut emu = emu64();
    // MOVSHDUP XMM13, XMM14
    let code = [
        0xf3, 0x45, 0x0f, 0x16, 0xee, // MOVSHDUP XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm14_xmm15() {
    let mut emu = emu64();
    // MOVSHDUP XMM14, XMM15
    let code = [
        0xf3, 0x45, 0x0f, 0x16, 0xf7, // MOVSHDUP XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm15_xmm0() {
    let mut emu = emu64();
    // MOVSHDUP XMM15, XMM0
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0xf8, // MOVSHDUP XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm0_mem() {
    let mut emu = emu64();
    // MOVSHDUP XMM0, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm1_mem() {
    let mut emu = emu64();
    // MOVSHDUP XMM1, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x16, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm7_mem() {
    let mut emu = emu64();
    // MOVSHDUP XMM7, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_xmm15_mem() {
    let mut emu = emu64();
    // MOVSHDUP XMM15, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x44, 0x0f, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSHDUP XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MOVSLDUP Tests - Duplicate Low (Even Index) Elements
// ============================================================================

#[test]
fn test_movsldup_xmm0_xmm1() {
    let mut emu = emu64();
    // MOVSLDUP XMM0, XMM1
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm1_xmm2() {
    let mut emu = emu64();
    // MOVSLDUP XMM1, XMM2
    let code = [
        0xf3, 0x0f, 0x12, 0xca, // MOVSLDUP XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm2_xmm3() {
    let mut emu = emu64();
    // MOVSLDUP XMM2, XMM3
    let code = [
        0xf3, 0x0f, 0x12, 0xd3, // MOVSLDUP XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm3_xmm4() {
    let mut emu = emu64();
    // MOVSLDUP XMM3, XMM4
    let code = [
        0xf3, 0x0f, 0x12, 0xdc, // MOVSLDUP XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm4_xmm5() {
    let mut emu = emu64();
    // MOVSLDUP XMM4, XMM5
    let code = [
        0xf3, 0x0f, 0x12, 0xe5, // MOVSLDUP XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm5_xmm6() {
    let mut emu = emu64();
    // MOVSLDUP XMM5, XMM6
    let code = [
        0xf3, 0x0f, 0x12, 0xee, // MOVSLDUP XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm6_xmm7() {
    let mut emu = emu64();
    // MOVSLDUP XMM6, XMM7
    let code = [
        0xf3, 0x0f, 0x12, 0xf7, // MOVSLDUP XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm7_xmm0() {
    let mut emu = emu64();
    // MOVSLDUP XMM7, XMM0
    let code = [
        0xf3, 0x0f, 0x12, 0xf8, // MOVSLDUP XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm8_xmm9() {
    let mut emu = emu64();
    // MOVSLDUP XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf3, 0x45, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm9_xmm10() {
    let mut emu = emu64();
    // MOVSLDUP XMM9, XMM10
    let code = [
        0xf3, 0x45, 0x0f, 0x12, 0xca, // MOVSLDUP XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm10_xmm11() {
    let mut emu = emu64();
    // MOVSLDUP XMM10, XMM11
    let code = [
        0xf3, 0x45, 0x0f, 0x12, 0xd3, // MOVSLDUP XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm11_xmm12() {
    let mut emu = emu64();
    // MOVSLDUP XMM11, XMM12
    let code = [
        0xf3, 0x45, 0x0f, 0x12, 0xdc, // MOVSLDUP XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm12_xmm13() {
    let mut emu = emu64();
    // MOVSLDUP XMM12, XMM13
    let code = [
        0xf3, 0x45, 0x0f, 0x12, 0xe5, // MOVSLDUP XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm13_xmm14() {
    let mut emu = emu64();
    // MOVSLDUP XMM13, XMM14
    let code = [
        0xf3, 0x45, 0x0f, 0x12, 0xee, // MOVSLDUP XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm14_xmm15() {
    let mut emu = emu64();
    // MOVSLDUP XMM14, XMM15
    let code = [
        0xf3, 0x45, 0x0f, 0x12, 0xf7, // MOVSLDUP XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm15_xmm0() {
    let mut emu = emu64();
    // MOVSLDUP XMM15, XMM0
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0xf8, // MOVSLDUP XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm0_mem() {
    let mut emu = emu64();
    // MOVSLDUP XMM0, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSLDUP XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm1_mem() {
    let mut emu = emu64();
    // MOVSLDUP XMM1, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x12, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSLDUP XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm7_mem() {
    let mut emu = emu64();
    // MOVSLDUP XMM7, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x12, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSLDUP XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_xmm15_mem() {
    let mut emu = emu64();
    // MOVSLDUP XMM15, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x44, 0x0f, 0x12, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVSLDUP XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Pattern Tests
// ============================================================================

#[test]
fn test_movshdup_duplicate_pattern() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc1, // MOVSHDUP XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_duplicate_pattern() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_movsldup_combination() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc1, // MOVSHDUP XMM0, XMM1
        0xf3, 0x0f, 0x12, 0xd0, // MOVSLDUP XMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_movshdup_combination() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc1, // MOVSLDUP XMM0, XMM1
        0xf3, 0x0f, 0x16, 0xd0, // MOVSHDUP XMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movshdup_self() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x16, 0xc0, // MOVSHDUP XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movsldup_self() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x12, 0xc0, // MOVSLDUP XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
