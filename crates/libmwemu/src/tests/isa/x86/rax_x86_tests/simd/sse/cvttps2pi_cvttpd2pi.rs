use crate::*;

// CVTTPS2PI - Convert with Truncation Packed Single FP to Packed Dword Integers
// CVTTPD2PI - Convert with Truncation Packed Double FP to Packed Dword Integers
//
// CVTTPS2PI converts two packed single precision floating-point values from XMM register
// or memory to two packed signed doubleword integers in MMX register using truncation.
//
// CVTTPD2PI converts two packed double precision floating-point values from XMM register
// or memory to two packed signed doubleword integers in MMX register using truncation.
//
// Truncation means rounding toward zero, regardless of the rounding control in MXCSR.
// This is equivalent to C-style type casting (int)float.
//
// Opcodes:
// NP 0F 2C /r    CVTTPS2PI mm, xmm/m64     - Convert two single FP to two dwords (truncate)
// 66 0F 2C /r    CVTTPD2PI mm, xmm/m128    - Convert two double FP to two dwords (truncate)

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// CVTTPS2PI - XMM to MMX Truncation Conversion Tests
// ============================================================================

#[test]
fn test_cvttps2pi_xmm0_to_mm0() {
    let mut emu = emu64();
    // CVTTPS2PI MM0, XMM0
    let code = [
        0x0f, 0x2c, 0xc0, // CVTTPS2PI MM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm1_to_mm1() {
    let mut emu = emu64();
    // CVTTPS2PI MM1, XMM1
    let code = [
        0x0f, 0x2c, 0xc9, // CVTTPS2PI MM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm2_to_mm2() {
    let mut emu = emu64();
    // CVTTPS2PI MM2, XMM2
    let code = [
        0x0f, 0x2c, 0xd2, // CVTTPS2PI MM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm3_to_mm3() {
    let mut emu = emu64();
    // CVTTPS2PI MM3, XMM3
    let code = [
        0x0f, 0x2c, 0xdb, // CVTTPS2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm4_to_mm4() {
    let mut emu = emu64();
    // CVTTPS2PI MM4, XMM4
    let code = [
        0x0f, 0x2c, 0xe4, // CVTTPS2PI MM4, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm5_to_mm5() {
    let mut emu = emu64();
    // CVTTPS2PI MM5, XMM5
    let code = [
        0x0f, 0x2c, 0xed, // CVTTPS2PI MM5, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm6_to_mm6() {
    let mut emu = emu64();
    // CVTTPS2PI MM6, XMM6
    let code = [
        0x0f, 0x2c, 0xf6, // CVTTPS2PI MM6, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm7_to_mm7() {
    let mut emu = emu64();
    // CVTTPS2PI MM7, XMM7
    let code = [
        0x0f, 0x2c, 0xff, // CVTTPS2PI MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm8_to_mm0() {
    let mut emu = emu64();
    // CVTTPS2PI MM0, XMM8
    let code = [
        0x41, 0x0f, 0x2c, 0xc0, // CVTTPS2PI MM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_xmm15_to_mm7() {
    let mut emu = emu64();
    // CVTTPS2PI MM7, XMM15
    let code = [
        0x41, 0x0f, 0x2c, 0xff, // CVTTPS2PI MM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTTPS2PI - Memory to MMX Truncation Conversion Tests
// ============================================================================

#[test]
fn test_cvttps2pi_mem_to_mm0() {
    let mut emu = emu64();
    // CVTTPS2PI MM0, [0x3000]
    let code = [
        0x0f, 0x2c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPS2PI MM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_mem_to_mm1() {
    let mut emu = emu64();
    // CVTTPS2PI MM1, [0x3000]
    let code = [
        0x0f, 0x2c, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPS2PI MM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_mem_to_mm3() {
    let mut emu = emu64();
    // CVTTPS2PI MM3, [0x3000]
    let code = [
        0x0f, 0x2c, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPS2PI MM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_mem_to_mm7() {
    let mut emu = emu64();
    // CVTTPS2PI MM7, [0x3000]
    let code = [
        0x0f, 0x2c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPS2PI MM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTTPD2PI - XMM to MMX Truncation Conversion Tests
// ============================================================================

#[test]
fn test_cvttpd2pi_xmm0_to_mm0() {
    let mut emu = emu64();
    // CVTTPD2PI MM0, XMM0
    let code = [
        0x66, 0x0f, 0x2c, 0xc0, // CVTTPD2PI MM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm1_to_mm1() {
    let mut emu = emu64();
    // CVTTPD2PI MM1, XMM1
    let code = [
        0x66, 0x0f, 0x2c, 0xc9, // CVTTPD2PI MM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm2_to_mm2() {
    let mut emu = emu64();
    // CVTTPD2PI MM2, XMM2
    let code = [
        0x66, 0x0f, 0x2c, 0xd2, // CVTTPD2PI MM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm3_to_mm3() {
    let mut emu = emu64();
    // CVTTPD2PI MM3, XMM3
    let code = [
        0x66, 0x0f, 0x2c, 0xdb, // CVTTPD2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm4_to_mm4() {
    let mut emu = emu64();
    // CVTTPD2PI MM4, XMM4
    let code = [
        0x66, 0x0f, 0x2c, 0xe4, // CVTTPD2PI MM4, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm5_to_mm5() {
    let mut emu = emu64();
    // CVTTPD2PI MM5, XMM5
    let code = [
        0x66, 0x0f, 0x2c, 0xed, // CVTTPD2PI MM5, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm6_to_mm6() {
    let mut emu = emu64();
    // CVTTPD2PI MM6, XMM6
    let code = [
        0x66, 0x0f, 0x2c, 0xf6, // CVTTPD2PI MM6, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm7_to_mm7() {
    let mut emu = emu64();
    // CVTTPD2PI MM7, XMM7
    let code = [
        0x66, 0x0f, 0x2c, 0xff, // CVTTPD2PI MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm8_to_mm0() {
    let mut emu = emu64();
    // CVTTPD2PI MM0, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x2c, 0xc0, // CVTTPD2PI MM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_xmm15_to_mm7() {
    let mut emu = emu64();
    // CVTTPD2PI MM7, XMM15
    let code = [
        0x66, 0x41, 0x0f, 0x2c, 0xff, // CVTTPD2PI MM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTTPD2PI - Memory to MMX Truncation Conversion Tests
// ============================================================================

#[test]
fn test_cvttpd2pi_mem_to_mm0() {
    let mut emu = emu64();
    // CVTTPD2PI MM0, [0x3000]
    let code = [
        0x66, 0x0f, 0x2c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPD2PI MM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_mem_to_mm1() {
    let mut emu = emu64();
    // CVTTPD2PI MM1, [0x3000]
    let code = [
        0x66, 0x0f, 0x2c, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPD2PI MM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_mem_to_mm3() {
    let mut emu = emu64();
    // CVTTPD2PI MM3, [0x3000]
    let code = [
        0x66, 0x0f, 0x2c, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPD2PI MM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_mem_to_mm7() {
    let mut emu = emu64();
    // CVTTPD2PI MM7, [0x3000]
    let code = [
        0x66, 0x0f, 0x2c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPD2PI MM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Multiple Truncation Conversions
// ============================================================================

#[test]
fn test_multiple_cvttps2pi() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2c, 0xc0, // CVTTPS2PI MM0, XMM0
        0x0f, 0x2c, 0xc9, // CVTTPS2PI MM1, XMM1
        0x0f, 0x2c, 0xd2, // CVTTPS2PI MM2, XMM2
        0x0f, 0x2c, 0xdb, // CVTTPS2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_cvttpd2pi() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2c, 0xc0, // CVTTPD2PI MM0, XMM0
        0x66, 0x0f, 0x2c, 0xc9, // CVTTPD2PI MM1, XMM1
        0x66, 0x0f, 0x2c, 0xd2, // CVTTPD2PI MM2, XMM2
        0x66, 0x0f, 0x2c, 0xdb, // CVTTPD2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_all_mmx_registers() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2c, 0xc0, // CVTTPS2PI MM0, XMM0
        0x0f, 0x2c, 0xc9, // CVTTPS2PI MM1, XMM1
        0x0f, 0x2c, 0xd2, // CVTTPS2PI MM2, XMM2
        0x0f, 0x2c, 0xdb, // CVTTPS2PI MM3, XMM3
        0x0f, 0x2c, 0xe4, // CVTTPS2PI MM4, XMM4
        0x0f, 0x2c, 0xed, // CVTTPS2PI MM5, XMM5
        0x0f, 0x2c, 0xf6, // CVTTPS2PI MM6, XMM6
        0x0f, 0x2c, 0xff, // CVTTPS2PI MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_all_mmx_registers() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2c, 0xc0, // CVTTPD2PI MM0, XMM0
        0x66, 0x0f, 0x2c, 0xc9, // CVTTPD2PI MM1, XMM1
        0x66, 0x0f, 0x2c, 0xd2, // CVTTPD2PI MM2, XMM2
        0x66, 0x0f, 0x2c, 0xdb, // CVTTPD2PI MM3, XMM3
        0x66, 0x0f, 0x2c, 0xe4, // CVTTPD2PI MM4, XMM4
        0x66, 0x0f, 0x2c, 0xed, // CVTTPD2PI MM5, XMM5
        0x66, 0x0f, 0x2c, 0xf6, // CVTTPD2PI MM6, XMM6
        0x66, 0x0f, 0x2c, 0xff, // CVTTPD2PI MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mixed_truncation_conversions() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2c, 0xc0, // CVTTPS2PI MM0, XMM0
        0x66, 0x0f, 0x2c, 0xc9, // CVTTPD2PI MM1, XMM1
        0x0f, 0x2c, 0xd2, // CVTTPS2PI MM2, XMM2
        0x66, 0x0f, 0x2c, 0xdb, // CVTTPD2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttps2pi_mem_from_various_addresses() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPS2PI MM0, [0x3000]
        0x0f, 0x2c, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // CVTTPS2PI MM1, [0x3008]
        0x0f, 0x2c, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // CVTTPS2PI MM2, [0x3010]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvttpd2pi_mem_from_various_addresses() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTTPD2PI MM0, [0x3000]
        0x66, 0x0f, 0x2c, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // CVTTPD2PI MM1, [0x3010]
        0x66, 0x0f, 0x2c, 0x14, 0x25, 0x20, 0x30, 0x00, 0x00, // CVTTPD2PI MM2, [0x3020]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
