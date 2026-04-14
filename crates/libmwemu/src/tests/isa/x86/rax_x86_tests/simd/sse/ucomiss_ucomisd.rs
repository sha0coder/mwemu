use crate::*;

// UCOMISS - Unordered Compare Scalar Single Precision Floating-Point Values and Set EFLAGS
// UCOMISD - Unordered Compare Scalar Double Precision Floating-Point Values and Set EFLAGS
//
// These instructions perform unordered comparisons and set EFLAGS accordingly:
// - UNORDERED (NaN):    ZF=1, PF=1, CF=1
// - GREATER_THAN:       ZF=0, PF=0, CF=0
// - LESS_THAN:          ZF=0, PF=0, CF=1
// - EQUAL:              ZF=1, PF=0, CF=0
// - OF, SF, AF are always set to 0
//
// The difference from COMISS/COMISD is that UCOMISS/UCOMISD signals an invalid
// operation exception only if a source operand is an SNaN (not for QNaN).
//
// Opcodes:
// NP 0F 2E /r             UCOMISS xmm1, xmm2/m32    - Compare low single-precision values
// 66 0F 2E /r             UCOMISD xmm1, xmm2/m64    - Compare low double-precision values

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// UCOMISS Tests - Unordered Single Precision Comparison
// ============================================================================

#[test]
fn test_ucomiss_xmm0_xmm1() {
    let mut emu = emu64();
    // UCOMISS XMM0, XMM1
    let code = [
        0x0f, 0x2e, 0xc1, // UCOMISS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm1_xmm2() {
    let mut emu = emu64();
    // UCOMISS XMM1, XMM2
    let code = [
        0x0f, 0x2e, 0xca, // UCOMISS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm2_xmm3() {
    let mut emu = emu64();
    // UCOMISS XMM2, XMM3
    let code = [
        0x0f, 0x2e, 0xd3, // UCOMISS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm3_xmm4() {
    let mut emu = emu64();
    // UCOMISS XMM3, XMM4
    let code = [
        0x0f, 0x2e, 0xdc, // UCOMISS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm4_xmm5() {
    let mut emu = emu64();
    // UCOMISS XMM4, XMM5
    let code = [
        0x0f, 0x2e, 0xe5, // UCOMISS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm5_xmm6() {
    let mut emu = emu64();
    // UCOMISS XMM5, XMM6
    let code = [
        0x0f, 0x2e, 0xee, // UCOMISS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm6_xmm7() {
    let mut emu = emu64();
    // UCOMISS XMM6, XMM7
    let code = [
        0x0f, 0x2e, 0xf7, // UCOMISS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm7_xmm0() {
    let mut emu = emu64();
    // UCOMISS XMM7, XMM0
    let code = [
        0x0f, 0x2e, 0xf8, // UCOMISS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm8_xmm9() {
    let mut emu = emu64();
    // UCOMISS XMM8, XMM9
    let code = [
        0x45, 0x0f, 0x2e, 0xc1, // UCOMISS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm10_xmm11() {
    let mut emu = emu64();
    // UCOMISS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x2e, 0xd3, // UCOMISS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm12_xmm13() {
    let mut emu = emu64();
    // UCOMISS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x2e, 0xe5, // UCOMISS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm14_xmm15() {
    let mut emu = emu64();
    // UCOMISS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x2e, 0xf7, // UCOMISS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm0_mem() {
    let mut emu = emu64();
    // UCOMISS XMM0, [0x3000]
    let code = [
        0x0f, 0x2e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // UCOMISS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm7_mem() {
    let mut emu = emu64();
    // UCOMISS XMM7, [0x3000]
    let code = [
        0x0f, 0x2e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // UCOMISS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm15_mem() {
    let mut emu = emu64();
    // UCOMISS XMM15, [0x3000]
    let code = [
        0x44, 0x0f, 0x2e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // UCOMISS XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_equal_values() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2e, 0xc0, // UCOMISS XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_greater_than() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2e, 0xc1, // UCOMISS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_less_than() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2e, 0xc2, // UCOMISS XMM0, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_positive_vs_negative() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2e, 0xc3, // UCOMISS XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2e, 0xc4, // UCOMISS XMM0, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// UCOMISD Tests - Unordered Double Precision Comparison
// ============================================================================

#[test]
fn test_ucomisd_xmm0_xmm1() {
    let mut emu = emu64();
    // UCOMISD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x2e, 0xc1, // UCOMISD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm1_xmm2() {
    let mut emu = emu64();
    // UCOMISD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x2e, 0xca, // UCOMISD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm2_xmm3() {
    let mut emu = emu64();
    // UCOMISD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x2e, 0xd3, // UCOMISD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm3_xmm4() {
    let mut emu = emu64();
    // UCOMISD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x2e, 0xdc, // UCOMISD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm4_xmm5() {
    let mut emu = emu64();
    // UCOMISD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x2e, 0xe5, // UCOMISD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm5_xmm6() {
    let mut emu = emu64();
    // UCOMISD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x2e, 0xee, // UCOMISD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm6_xmm7() {
    let mut emu = emu64();
    // UCOMISD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x2e, 0xf7, // UCOMISD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm7_xmm0() {
    let mut emu = emu64();
    // UCOMISD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x2e, 0xf8, // UCOMISD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm8_xmm9() {
    let mut emu = emu64();
    // UCOMISD XMM8, XMM9
    let code = [
        0x66, 0x45, 0x0f, 0x2e, 0xc1, // UCOMISD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm10_xmm11() {
    let mut emu = emu64();
    // UCOMISD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x2e, 0xd3, // UCOMISD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm12_xmm13() {
    let mut emu = emu64();
    // UCOMISD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x2e, 0xe5, // UCOMISD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm14_xmm15() {
    let mut emu = emu64();
    // UCOMISD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x2e, 0xf7, // UCOMISD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm0_mem() {
    let mut emu = emu64();
    // UCOMISD XMM0, [0x3000]
    let code = [
        0x66, 0x0f, 0x2e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // UCOMISD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm7_mem() {
    let mut emu = emu64();
    // UCOMISD XMM7, [0x3000]
    let code = [
        0x66, 0x0f, 0x2e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // UCOMISD XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm15_mem() {
    let mut emu = emu64();
    // UCOMISD XMM15, [0x3000]
    let code = [
        0x66, 0x44, 0x0f, 0x2e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // UCOMISD XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_equal_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2e, 0xc0, // UCOMISD XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_greater_than() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2e, 0xc1, // UCOMISD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_less_than() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2e, 0xc2, // UCOMISD XMM0, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_positive_vs_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2e, 0xc3, // UCOMISD XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2e, 0xc4, // UCOMISD XMM0, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2e, 0xc5, // UCOMISD XMM0, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_large_vs_small() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2e, 0xc6, // UCOMISS XMM0, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_large_vs_small() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2e, 0xc6, // UCOMISD XMM0, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomiss_xmm0_xmm8() {
    let mut emu = emu64();
    // UCOMISS XMM0, XMM8
    let code = [
        0x41, 0x0f, 0x2e, 0xc0, // UCOMISS XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ucomisd_xmm0_xmm8() {
    let mut emu = emu64();
    // UCOMISD XMM0, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x2e, 0xc0, // UCOMISD XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
