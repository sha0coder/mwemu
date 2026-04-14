use crate::*;

// VPERMILPD - Permute In-Lane of Pairs of Double-Precision Floating-Point Values
//
// Permutes double-precision floating-point values in the first source operand
// using 8-bit control fields in the corresponding elements of the second source
// operand or using 8-bit immediate values.
//
// Opcodes:
// VEX.128.66.0F38.W0 0D /r           VPERMILPD xmm1, xmm2, xmm3/m128        - Variable permute
// VEX.256.66.0F38.W0 0D /r           VPERMILPD ymm1, ymm2, ymm3/m256        - Variable permute
// VEX.128.66.0F3A.W0 05 /r ib        VPERMILPD xmm1, xmm2/m128, imm8        - Immediate permute
// VEX.256.66.0F3A.W0 05 /r ib        VPERMILPD ymm1, ymm2/m256, imm8        - Immediate permute

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPERMILPD Variable Tests (VEX.128)
// ============================================================================

#[test]
fn test_vpermilpd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x0d, 0xc2, // VPERMILPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x69, 0x0d, 0xcb, // VPERMILPD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x61, 0x0d, 0xd4, // VPERMILPD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x59, 0x0d, 0xdd, // VPERMILPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x0d, 0xf9, // VPERMILPD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x0d, 0xc2, // VPERMILPD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_xmm15_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x79, 0x0d, 0xf9, // VPERMILPD XMM15, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x0d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VPERMILPD XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMILPD Variable Tests (VEX.256)
// ============================================================================

#[test]
fn test_vpermilpd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x0d, 0xc2, // VPERMILPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x6d, 0x0d, 0xcb, // VPERMILPD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x65, 0x0d, 0xd4, // VPERMILPD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x0d, 0xdd, // VPERMILPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x0d, 0xf9, // VPERMILPD YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMILPD Immediate Tests (VEX.128)
// ============================================================================

#[test]
fn test_vpermilpd_imm_xmm0_xmm1_0x0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xc1, 0x00, // VPERMILPD XMM0, XMM1, 0x0 (no swap)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_xmm0_xmm1_0x1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xc1, 0x01, // VPERMILPD XMM0, XMM1, 0x1 (swap)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_xmm1_xmm2_0x2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xca, 0x02, // VPERMILPD XMM1, XMM2, 0x2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_xmm2_xmm3_0x3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xd3, 0x03, // VPERMILPD XMM2, XMM3, 0x3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_xmm3_xmm4_0x0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xdc, 0x00, // VPERMILPD XMM3, XMM4, 0x0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_xmm7_xmm0_0x1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xf8, 0x01, // VPERMILPD XMM7, XMM0, 0x1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_xmm8_xmm9_0x0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x79, 0x05, 0xc1, 0x00, // VPERMILPD XMM8, XMM9, 0x0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_xmm15_xmm0_0x1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x63, 0x79, 0x05, 0xf8, 0x01, // VPERMILPD XMM15, XMM0, 0x1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMILPD Immediate Tests (VEX.256)
// ============================================================================

#[test]
fn test_vpermilpd_imm_ymm0_ymm1_0x0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x05, 0xc1, 0x00, // VPERMILPD YMM0, YMM1, 0x0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_ymm0_ymm1_0x5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x05, 0xc1, 0x05, // VPERMILPD YMM0, YMM1, 0x5 (swap both lanes)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_ymm1_ymm2_0xa() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x05, 0xca, 0x0a, // VPERMILPD YMM1, YMM2, 0xA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_ymm2_ymm3_0xf() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x05, 0xd3, 0x0f, // VPERMILPD YMM2, YMM3, 0xF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_ymm3_ymm4_0x3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x05, 0xdc, 0x03, // VPERMILPD YMM3, YMM4, 0x3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_ymm7_ymm0_0xc() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x05, 0xf8, 0x0c, // VPERMILPD YMM7, YMM0, 0xC
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_vpermilpd_imm_xmm0_mem_0x1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, // VPERMILPD XMM0, [0x3000], 0x1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_imm_ymm0_mem_0x5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x05, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x05, // VPERMILPD YMM0, [0x3000], 0x5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Different permutation patterns
// ============================================================================

#[test]
fn test_vpermilpd_no_swap_128() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xc1, 0x00, // VPERMILPD XMM0, XMM1, 0x0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_swap_128() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xc1, 0x01, // VPERMILPD XMM0, XMM1, 0x1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_broadcast_low_128() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xc1, 0x00, // VPERMILPD XMM0, XMM1, 0x0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_broadcast_high_128() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x05, 0xc1, 0x03, // VPERMILPD XMM0, XMM1, 0x3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilpd_swap_both_lanes_256() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x05, 0xc1, 0x05, // VPERMILPD YMM0, YMM1, 0x5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
