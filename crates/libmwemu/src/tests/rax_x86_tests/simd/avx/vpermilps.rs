use crate::*;

// VPERMILPS - Permute In-Lane of Pairs of Single-Precision Floating-Point Values
//
// Permutes single-precision floating-point values in the first source operand
// using 8-bit control fields in the corresponding elements of the second source
// operand or using 8-bit immediate values.
//
// Opcodes:
// VEX.128.66.0F38.W0 0C /r           VPERMILPS xmm1, xmm2, xmm3/m128        - Variable permute
// VEX.256.66.0F38.W0 0C /r           VPERMILPS ymm1, ymm2, ymm3/m256        - Variable permute
// VEX.128.66.0F3A.W0 04 /r ib        VPERMILPS xmm1, xmm2/m128, imm8        - Immediate permute
// VEX.256.66.0F3A.W0 04 /r ib        VPERMILPS ymm1, ymm2/m256, imm8        - Immediate permute

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPERMILPS Variable Tests (VEX.128)
// ============================================================================

#[test]
fn test_vpermilps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x0c, 0xc2, // VPERMILPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x69, 0x0c, 0xcb, // VPERMILPS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x61, 0x0c, 0xd4, // VPERMILPS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x59, 0x0c, 0xdd, // VPERMILPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x0c, 0xf9, // VPERMILPS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x31, 0x0c, 0xc2, // VPERMILPS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_xmm15_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x79, 0x0c, 0xf9, // VPERMILPS XMM15, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x0c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VPERMILPS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMILPS Variable Tests (VEX.256)
// ============================================================================

#[test]
fn test_vpermilps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x0c, 0xc2, // VPERMILPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x6d, 0x0c, 0xcb, // VPERMILPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x65, 0x0c, 0xd4, // VPERMILPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x0c, 0xdd, // VPERMILPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x0c, 0xf9, // VPERMILPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMILPS Immediate Tests (VEX.128)
// ============================================================================

#[test]
fn test_vpermilps_imm_xmm0_xmm1_0x00() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xc1, 0x00, // VPERMILPS XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_xmm0_xmm1_0x1b() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xc1, 0x1b, // VPERMILPS XMM0, XMM1, 0x1B (reverse)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_xmm1_xmm2_0xe4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xca, 0xe4, // VPERMILPS XMM1, XMM2, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_xmm2_xmm3_0x4e() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xd3, 0x4e, // VPERMILPS XMM2, XMM3, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_xmm3_xmm4_0xb1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xdc, 0xb1, // VPERMILPS XMM3, XMM4, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_xmm7_xmm0_0xff() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xf8, 0xff, // VPERMILPS XMM7, XMM0, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_xmm8_xmm9_0x00() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x79, 0x04, 0xc1, 0x00, // VPERMILPS XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_xmm15_xmm0_0x1b() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x63, 0x79, 0x04, 0xf8, 0x1b, // VPERMILPS XMM15, XMM0, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMILPS Immediate Tests (VEX.256)
// ============================================================================

#[test]
fn test_vpermilps_imm_ymm0_ymm1_0x00() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x04, 0xc1, 0x00, // VPERMILPS YMM0, YMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_ymm0_ymm1_0x1b() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x04, 0xc1, 0x1b, // VPERMILPS YMM0, YMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_ymm1_ymm2_0xe4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x04, 0xca, 0xe4, // VPERMILPS YMM1, YMM2, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_ymm2_ymm3_0x4e() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x04, 0xd3, 0x4e, // VPERMILPS YMM2, YMM3, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_ymm3_ymm4_0xb1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x04, 0xdc, 0xb1, // VPERMILPS YMM3, YMM4, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_ymm7_ymm0_0xff() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x04, 0xf8, 0xff, // VPERMILPS YMM7, YMM0, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_vpermilps_imm_xmm0_mem_0x1b() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x1b, // VPERMILPS XMM0, [0x3000], 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_imm_ymm0_mem_0xe4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x04, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xe4, // VPERMILPS YMM0, [0x3000], 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Different permutation patterns
// ============================================================================

#[test]
fn test_vpermilps_broadcast_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xc1, 0x00, // VPERMILPS XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_reverse_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xc1, 0x1b, // VPERMILPS XMM0, XMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermilps_swap_pairs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x79, 0x04, 0xc1, 0xb1, // VPERMILPS XMM0, XMM1, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
