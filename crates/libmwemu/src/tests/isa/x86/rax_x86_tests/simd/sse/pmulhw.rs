use crate::*;

// PMULHW - Multiply Packed Signed Integers and Store High Result
//
// Multiplies the packed signed word integers in the destination operand
// (first operand) by the packed signed word integers in the source operand
// (second operand), and stores the high 16 bits of each intermediate
// 32-bit result in the destination operand.
//
// Opcodes:
// 66 0F E5 /r             PMULHW xmm1, xmm2/m128    - Multiply packed signed words, store high 16 bits

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// PMULHW Tests - Packed Multiply High Signed Word (8x int16)
// ============================================================================

#[test]
fn test_pmulhw_xmm0_xmm1() {
    let mut emu = emu64();
    // PMULHW XMM0, XMM1
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm1_xmm2() {
    let mut emu = emu64();
    // PMULHW XMM1, XMM2
    let code = [
        0x66, 0x0f, 0xe5, 0xca, // PMULHW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm2_xmm3() {
    let mut emu = emu64();
    // PMULHW XMM2, XMM3
    let code = [
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm3_xmm4() {
    let mut emu = emu64();
    // PMULHW XMM3, XMM4
    let code = [
        0x66, 0x0f, 0xe5, 0xdc, // PMULHW XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm4_xmm5() {
    let mut emu = emu64();
    // PMULHW XMM4, XMM5
    let code = [
        0x66, 0x0f, 0xe5, 0xe5, // PMULHW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm5_xmm6() {
    let mut emu = emu64();
    // PMULHW XMM5, XMM6
    let code = [
        0x66, 0x0f, 0xe5, 0xee, // PMULHW XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm6_xmm7() {
    let mut emu = emu64();
    // PMULHW XMM6, XMM7
    let code = [
        0x66, 0x0f, 0xe5, 0xf7, // PMULHW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm7_xmm0() {
    let mut emu = emu64();
    // PMULHW XMM7, XMM0
    let code = [
        0x66, 0x0f, 0xe5, 0xf8, // PMULHW XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm8_xmm9() {
    let mut emu = emu64();
    // PMULHW XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xc1, // PMULHW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm9_xmm10() {
    let mut emu = emu64();
    // PMULHW XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xca, // PMULHW XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm10_xmm11() {
    let mut emu = emu64();
    // PMULHW XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xd3, // PMULHW XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm11_xmm12() {
    let mut emu = emu64();
    // PMULHW XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xdc, // PMULHW XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm12_xmm13() {
    let mut emu = emu64();
    // PMULHW XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xe5, // PMULHW XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm13_xmm14() {
    let mut emu = emu64();
    // PMULHW XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xee, // PMULHW XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm14_xmm15() {
    let mut emu = emu64();
    // PMULHW XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xf7, // PMULHW XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm15_xmm0() {
    let mut emu = emu64();
    // PMULHW XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0xf8, // PMULHW XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm0_mem() {
    let mut emu = emu64();
    // PMULHW XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm1_mem() {
    let mut emu = emu64();
    // PMULHW XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm7_mem() {
    let mut emu = emu64();
    // PMULHW XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm15_mem() {
    let mut emu = emu64();
    // PMULHW XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_positive_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_negative_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xe5, // PMULHW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_zero_multiplication() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xf7, // PMULHW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_max_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_min_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_all_words_different() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xe5, // PMULHW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_self_multiply() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xc0, // PMULHW XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm1_self() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xc9, // PMULHW XMM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_power_of_two() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xf7, // PMULHW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_alternating_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xc1, // PMULHW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_mem_aligned() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_small_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_large_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xe5, // PMULHW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_high_regs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xf7, // PMULHW XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_mixed_regs_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0xc1, // PMULHW XMM8, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_mixed_regs_2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0x66, 0x45, 0x0f, 0xe5, 0xd3, // PMULHW XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm2_mem() {
    let mut emu = emu64();
    // PMULHW XMM2, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM2, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm3_mem() {
    let mut emu = emu64();
    // PMULHW XMM3, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm4_mem() {
    let mut emu = emu64();
    // PMULHW XMM4, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM4, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm5_mem() {
    let mut emu = emu64();
    // PMULHW XMM5, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM5, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm6_mem() {
    let mut emu = emu64();
    // PMULHW XMM6, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM6, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm8_mem() {
    let mut emu = emu64();
    // PMULHW XMM8, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm9_mem() {
    let mut emu = emu64();
    // PMULHW XMM9, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM9, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm10_mem() {
    let mut emu = emu64();
    // PMULHW XMM10, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM10, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm11_mem() {
    let mut emu = emu64();
    // PMULHW XMM11, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM11, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm12_mem() {
    let mut emu = emu64();
    // PMULHW XMM12, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM12, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm13_mem() {
    let mut emu = emu64();
    // PMULHW XMM13, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM13, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhw_xmm14_mem() {
    let mut emu = emu64();
    // PMULHW XMM14, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM14, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
