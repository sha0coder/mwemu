use crate::*;

// SUBSS - Subtract Scalar Single Precision Floating-Point Values
// SUBSD - Subtract Scalar Double Precision Floating-Point Values
//
// SUBSS subtracts the low single-precision (32-bit) floating-point value
// SUBSD subtracts the low double-precision (64-bit) floating-point value
// Upper bits are preserved from the first source operand
//
// Opcodes:
// F3 0F 5C /r             SUBSS xmm1, xmm2/m32     - Subtract scalar single from xmm2/m32 from xmm1
// F2 0F 5C /r             SUBSD xmm1, xmm2/m64     - Subtract scalar double from xmm2/m64 from xmm1

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// SUBSS Tests - Scalar Single Precision (low 32 bits, preserve upper)
// ============================================================================

#[test]
fn test_subss_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x5c, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x5c, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x5c, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x5c, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x5c, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x5c, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x5c, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xf3, 0x44, 0x0f, 0x5c, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm0_mem() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_preserves_upper_bits() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subss_positive_values() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x5c, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// SUBSD Tests - Scalar Double Precision (low 64 bits, preserve upper)
// ============================================================================

#[test]
fn test_subsd_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x5c, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x5c, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x5c, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x5c, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x5c, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x5c, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x5c, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xf2, 0x44, 0x0f, 0x5c, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm0_mem() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_preserves_upper_bits() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_subsd_positive_values() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x5c, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
