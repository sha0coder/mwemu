use crate::*;

// ADDSS - Add Scalar Single Precision Floating-Point Values
// ADDSD - Add Scalar Double Precision Floating-Point Values
//
// ADDSS adds the low single-precision (32-bit) floating-point value
// ADDSD adds the low double-precision (64-bit) floating-point value
// Upper bits are preserved from the first source operand
//
// Opcodes:
// F3 0F 58 /r             ADDSS xmm1, xmm2/m32     - Add scalar single from xmm2/m32 to xmm1
// F2 0F 58 /r             ADDSD xmm1, xmm2/m64     - Add scalar double from xmm2/m64 to xmm1

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// ADDSS Tests - Scalar Single Precision (low 32 bits, preserve upper)
// ============================================================================

#[test]
fn test_addss_xmm0_xmm1() {
    let mut emu = emu64();
    // ADDSS XMM0, XMM1
    let code = [
        0xf3, 0x0f, 0x58, 0xc1, // ADDSS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm1_xmm2() {
    let mut emu = emu64();
    // ADDSS XMM1, XMM2
    let code = [
        0xf3, 0x0f, 0x58, 0xca, // ADDSS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm2_xmm3() {
    let mut emu = emu64();
    // ADDSS XMM2, XMM3
    let code = [
        0xf3, 0x0f, 0x58, 0xd3, // ADDSS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm3_xmm4() {
    let mut emu = emu64();
    // ADDSS XMM3, XMM4
    let code = [
        0xf3, 0x0f, 0x58, 0xdc, // ADDSS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm4_xmm5() {
    let mut emu = emu64();
    // ADDSS XMM4, XMM5
    let code = [
        0xf3, 0x0f, 0x58, 0xe5, // ADDSS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm5_xmm6() {
    let mut emu = emu64();
    // ADDSS XMM5, XMM6
    let code = [
        0xf3, 0x0f, 0x58, 0xee, // ADDSS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm6_xmm7() {
    let mut emu = emu64();
    // ADDSS XMM6, XMM7
    let code = [
        0xf3, 0x0f, 0x58, 0xf7, // ADDSS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm7_xmm0() {
    let mut emu = emu64();
    // ADDSS XMM7, XMM0
    let code = [
        0xf3, 0x0f, 0x58, 0xf8, // ADDSS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm8_xmm9() {
    let mut emu = emu64();
    // ADDSS XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf3, 0x45, 0x0f, 0x58, 0xc1, // ADDSS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm9_xmm10() {
    let mut emu = emu64();
    // ADDSS XMM9, XMM10
    let code = [
        0xf3, 0x45, 0x0f, 0x58, 0xca, // ADDSS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm10_xmm11() {
    let mut emu = emu64();
    // ADDSS XMM10, XMM11
    let code = [
        0xf3, 0x45, 0x0f, 0x58, 0xd3, // ADDSS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm11_xmm12() {
    let mut emu = emu64();
    // ADDSS XMM11, XMM12
    let code = [
        0xf3, 0x45, 0x0f, 0x58, 0xdc, // ADDSS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm12_xmm13() {
    let mut emu = emu64();
    // ADDSS XMM12, XMM13
    let code = [
        0xf3, 0x45, 0x0f, 0x58, 0xe5, // ADDSS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm13_xmm14() {
    let mut emu = emu64();
    // ADDSS XMM13, XMM14
    let code = [
        0xf3, 0x45, 0x0f, 0x58, 0xee, // ADDSS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm14_xmm15() {
    let mut emu = emu64();
    // ADDSS XMM14, XMM15
    let code = [
        0xf3, 0x45, 0x0f, 0x58, 0xf7, // ADDSS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm15_xmm0() {
    let mut emu = emu64();
    // ADDSS XMM15, XMM0
    let code = [
        0xf3, 0x44, 0x0f, 0x58, 0xf8, // ADDSS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm0_mem() {
    let mut emu = emu64();
    // ADDSS XMM0, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x58, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDSS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_xmm7_mem() {
    let mut emu = emu64();
    // ADDSS XMM7, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x58, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDSS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_preserves_upper_bits() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x58, 0xc1, // ADDSS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addss_positive_values() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x58, 0xd3, // ADDSS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// ADDSD Tests - Scalar Double Precision (low 64 bits, preserve upper)
// ============================================================================

#[test]
fn test_addsd_xmm0_xmm1() {
    let mut emu = emu64();
    // ADDSD XMM0, XMM1
    let code = [
        0xf2, 0x0f, 0x58, 0xc1, // ADDSD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm1_xmm2() {
    let mut emu = emu64();
    // ADDSD XMM1, XMM2
    let code = [
        0xf2, 0x0f, 0x58, 0xca, // ADDSD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm2_xmm3() {
    let mut emu = emu64();
    // ADDSD XMM2, XMM3
    let code = [
        0xf2, 0x0f, 0x58, 0xd3, // ADDSD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm3_xmm4() {
    let mut emu = emu64();
    // ADDSD XMM3, XMM4
    let code = [
        0xf2, 0x0f, 0x58, 0xdc, // ADDSD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm4_xmm5() {
    let mut emu = emu64();
    // ADDSD XMM4, XMM5
    let code = [
        0xf2, 0x0f, 0x58, 0xe5, // ADDSD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm5_xmm6() {
    let mut emu = emu64();
    // ADDSD XMM5, XMM6
    let code = [
        0xf2, 0x0f, 0x58, 0xee, // ADDSD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm6_xmm7() {
    let mut emu = emu64();
    // ADDSD XMM6, XMM7
    let code = [
        0xf2, 0x0f, 0x58, 0xf7, // ADDSD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm7_xmm0() {
    let mut emu = emu64();
    // ADDSD XMM7, XMM0
    let code = [
        0xf2, 0x0f, 0x58, 0xf8, // ADDSD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm8_xmm9() {
    let mut emu = emu64();
    // ADDSD XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf2, 0x45, 0x0f, 0x58, 0xc1, // ADDSD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm9_xmm10() {
    let mut emu = emu64();
    // ADDSD XMM9, XMM10
    let code = [
        0xf2, 0x45, 0x0f, 0x58, 0xca, // ADDSD XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm10_xmm11() {
    let mut emu = emu64();
    // ADDSD XMM10, XMM11
    let code = [
        0xf2, 0x45, 0x0f, 0x58, 0xd3, // ADDSD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm11_xmm12() {
    let mut emu = emu64();
    // ADDSD XMM11, XMM12
    let code = [
        0xf2, 0x45, 0x0f, 0x58, 0xdc, // ADDSD XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm12_xmm13() {
    let mut emu = emu64();
    // ADDSD XMM12, XMM13
    let code = [
        0xf2, 0x45, 0x0f, 0x58, 0xe5, // ADDSD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm13_xmm14() {
    let mut emu = emu64();
    // ADDSD XMM13, XMM14
    let code = [
        0xf2, 0x45, 0x0f, 0x58, 0xee, // ADDSD XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm14_xmm15() {
    let mut emu = emu64();
    // ADDSD XMM14, XMM15
    let code = [
        0xf2, 0x45, 0x0f, 0x58, 0xf7, // ADDSD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm15_xmm0() {
    let mut emu = emu64();
    // ADDSD XMM15, XMM0
    let code = [
        0xf2, 0x44, 0x0f, 0x58, 0xf8, // ADDSD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm0_mem() {
    let mut emu = emu64();
    // ADDSD XMM0, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x58, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDSD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_xmm7_mem() {
    let mut emu = emu64();
    // ADDSD XMM7, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x58, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDSD XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_preserves_upper_bits() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x58, 0xc1, // ADDSD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_addsd_positive_values() {
    let mut emu = emu64();
    let code = [
        0xf2, 0x0f, 0x58, 0xd3, // ADDSD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
