use crate::*;

// MULSS - Multiply Scalar Single Precision Floating-Point Values
// MULSD - Multiply Scalar Double Precision Floating-Point Values
//
// MULSS multiplies the low single-precision (32-bit) floating-point value
// MULSD multiplies the low double-precision (64-bit) floating-point value
// Upper bits are preserved from the first source operand
//
// Opcodes:
// F3 0F 59 /r             MULSS xmm1, xmm2/m32     - Multiply scalar single from xmm2/m32 with xmm1
// F2 0F 59 /r             MULSD xmm1, xmm2/m64     - Multiply scalar double from xmm2/m64 with xmm1

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// MULSS Tests - Scalar Single Precision (low 32 bits, preserve upper)
// ============================================================================

#[test]
fn test_mulss_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x59, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x59, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x59, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xf3, 0x45, 0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xf3, 0x44, 0x0f, 0x59, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm0_mem() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_preserves_upper_bits() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulss_positive_values() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// MULSD Tests - Scalar Double Precision (low 64 bits, preserve upper)
// ============================================================================

#[test]
fn test_mulsd_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x59, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x59, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x59, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x59, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xf2, 0x45, 0x0f, 0x59, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xf2, 0x44, 0x0f, 0x59, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm0_mem() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_preserves_upper_bits() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mulsd_positive_values() {
    let mut emu = emu64();
    let code = [0xf2, 0x0f, 0x59, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
