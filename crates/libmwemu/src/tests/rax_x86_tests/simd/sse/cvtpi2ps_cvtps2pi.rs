use crate::*;

// CVTPI2PS - Convert Packed Dword Integers to Packed Single Precision FP Values
// CVTPS2PI - Convert Packed Single Precision FP Values to Packed Dword Integers
//
// CVTPI2PS converts two packed signed doubleword integers from MMX register or memory
// to two packed single precision floating-point values in XMM register (low quadword).
// The high quadword of the destination XMM register remains unchanged.
//
// CVTPS2PI converts two packed single precision floating-point values from XMM register
// or memory to two packed signed doubleword integers in MMX register. Conversion is
// performed according to current rounding mode in MXCSR.
//
// Opcodes:
// NP 0F 2A /r    CVTPI2PS xmm, mm/m64    - Convert two signed dwords to two single FP
// NP 0F 2D /r    CVTPS2PI mm, xmm/m64    - Convert two single FP to two signed dwords
//
// These instructions cause a transition from x87 FPU to MMX technology operation.

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// CVTPI2PS - MMX to XMM Conversion Tests
// ============================================================================

#[test]
fn test_cvtpi2ps_mm0_to_xmm0() {
    let mut emu = emu64();
    // CVTPI2PS XMM0, MM0
    let code = [
        0x0f, 0x2a, 0xc0, // CVTPI2PS XMM0, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm1_to_xmm1() {
    let mut emu = emu64();
    // CVTPI2PS XMM1, MM1
    let code = [
        0x0f, 0x2a, 0xc9, // CVTPI2PS XMM1, MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm2_to_xmm2() {
    let mut emu = emu64();
    // CVTPI2PS XMM2, MM2
    let code = [
        0x0f, 0x2a, 0xd2, // CVTPI2PS XMM2, MM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm3_to_xmm3() {
    let mut emu = emu64();
    // CVTPI2PS XMM3, MM3
    let code = [
        0x0f, 0x2a, 0xdb, // CVTPI2PS XMM3, MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm4_to_xmm4() {
    let mut emu = emu64();
    // CVTPI2PS XMM4, MM4
    let code = [
        0x0f, 0x2a, 0xe4, // CVTPI2PS XMM4, MM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm5_to_xmm5() {
    let mut emu = emu64();
    // CVTPI2PS XMM5, MM5
    let code = [
        0x0f, 0x2a, 0xed, // CVTPI2PS XMM5, MM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm6_to_xmm6() {
    let mut emu = emu64();
    // CVTPI2PS XMM6, MM6
    let code = [
        0x0f, 0x2a, 0xf6, // CVTPI2PS XMM6, MM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm7_to_xmm7() {
    let mut emu = emu64();
    // CVTPI2PS XMM7, MM7
    let code = [
        0x0f, 0x2a, 0xff, // CVTPI2PS XMM7, MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm0_to_xmm8() {
    let mut emu = emu64();
    // CVTPI2PS XMM8, MM0
    let code = [
        0x44, 0x0f, 0x2a, 0xc0, // CVTPI2PS XMM8, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mm1_to_xmm15() {
    let mut emu = emu64();
    // CVTPI2PS XMM15, MM1
    let code = [
        0x44, 0x0f, 0x2a, 0xf9, // CVTPI2PS XMM15, MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTPI2PS - Memory to XMM Conversion Tests
// ============================================================================

#[test]
fn test_cvtpi2ps_mem_to_xmm0() {
    let mut emu = emu64();
    // CVTPI2PS XMM0, [0x3000]
    let code = [
        0x0f, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PS XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mem_to_xmm1() {
    let mut emu = emu64();
    // CVTPI2PS XMM1, [0x3000]
    let code = [
        0x0f, 0x2a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PS XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mem_to_xmm2() {
    let mut emu = emu64();
    // CVTPI2PS XMM2, [0x3000]
    let code = [
        0x0f, 0x2a, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PS XMM2, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mem_to_xmm7() {
    let mut emu = emu64();
    // CVTPI2PS XMM7, [0x3000]
    let code = [
        0x0f, 0x2a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PS XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_mem_to_xmm15() {
    let mut emu = emu64();
    // CVTPI2PS XMM15, [0x3000]
    let code = [
        0x44, 0x0f, 0x2a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PS XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTPS2PI - XMM to MMX Conversion Tests
// ============================================================================

#[test]
fn test_cvtps2pi_xmm0_to_mm0() {
    let mut emu = emu64();
    // CVTPS2PI MM0, XMM0
    let code = [
        0x0f, 0x2d, 0xc0, // CVTPS2PI MM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm1_to_mm1() {
    let mut emu = emu64();
    // CVTPS2PI MM1, XMM1
    let code = [
        0x0f, 0x2d, 0xc9, // CVTPS2PI MM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm2_to_mm2() {
    let mut emu = emu64();
    // CVTPS2PI MM2, XMM2
    let code = [
        0x0f, 0x2d, 0xd2, // CVTPS2PI MM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm3_to_mm3() {
    let mut emu = emu64();
    // CVTPS2PI MM3, XMM3
    let code = [
        0x0f, 0x2d, 0xdb, // CVTPS2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm4_to_mm4() {
    let mut emu = emu64();
    // CVTPS2PI MM4, XMM4
    let code = [
        0x0f, 0x2d, 0xe4, // CVTPS2PI MM4, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm5_to_mm5() {
    let mut emu = emu64();
    // CVTPS2PI MM5, XMM5
    let code = [
        0x0f, 0x2d, 0xed, // CVTPS2PI MM5, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm6_to_mm6() {
    let mut emu = emu64();
    // CVTPS2PI MM6, XMM6
    let code = [
        0x0f, 0x2d, 0xf6, // CVTPS2PI MM6, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm7_to_mm7() {
    let mut emu = emu64();
    // CVTPS2PI MM7, XMM7
    let code = [
        0x0f, 0x2d, 0xff, // CVTPS2PI MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm8_to_mm0() {
    let mut emu = emu64();
    // CVTPS2PI MM0, XMM8
    let code = [
        0x41, 0x0f, 0x2d, 0xc0, // CVTPS2PI MM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_xmm15_to_mm7() {
    let mut emu = emu64();
    // CVTPS2PI MM7, XMM15
    let code = [
        0x41, 0x0f, 0x2d, 0xff, // CVTPS2PI MM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTPS2PI - Memory to MMX Conversion Tests
// ============================================================================

#[test]
fn test_cvtps2pi_mem_to_mm0() {
    let mut emu = emu64();
    // CVTPS2PI MM0, [0x3000]
    let code = [
        0x0f, 0x2d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPS2PI MM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_mem_to_mm1() {
    let mut emu = emu64();
    // CVTPS2PI MM1, [0x3000]
    let code = [
        0x0f, 0x2d, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPS2PI MM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_mem_to_mm3() {
    let mut emu = emu64();
    // CVTPS2PI MM3, [0x3000]
    let code = [
        0x0f, 0x2d, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPS2PI MM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_mem_to_mm7() {
    let mut emu = emu64();
    // CVTPS2PI MM7, [0x3000]
    let code = [
        0x0f, 0x2d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPS2PI MM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Conversion Chains
// ============================================================================

#[test]
fn test_cvtpi2ps_cvtps2pi_round_trip() {
    let mut emu = emu64();
    // CVTPI2PS XMM0, MM0 then CVTPS2PI MM1, XMM0
    let code = [
        0x0f, 0x2a, 0xc0, // CVTPI2PS XMM0, MM0
        0x0f, 0x2d, 0xc8, // CVTPS2PI MM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_cvtpi2ps_round_trip() {
    let mut emu = emu64();
    // CVTPS2PI MM0, XMM0 then CVTPI2PS XMM1, MM0
    let code = [
        0x0f, 0x2d, 0xc0, // CVTPS2PI MM0, XMM0
        0x0f, 0x2a, 0xc8, // CVTPI2PS XMM1, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_cvtpi2ps() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2a, 0xc0, // CVTPI2PS XMM0, MM0
        0x0f, 0x2a, 0xc9, // CVTPI2PS XMM1, MM1
        0x0f, 0x2a, 0xd2, // CVTPI2PS XMM2, MM2
        0x0f, 0x2a, 0xdb, // CVTPI2PS XMM3, MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_cvtps2pi() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2d, 0xc0, // CVTPS2PI MM0, XMM0
        0x0f, 0x2d, 0xc9, // CVTPS2PI MM1, XMM1
        0x0f, 0x2d, 0xd2, // CVTPS2PI MM2, XMM2
        0x0f, 0x2d, 0xdb, // CVTPS2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2ps_all_mmx_registers() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2a, 0xc0, // CVTPI2PS XMM0, MM0
        0x0f, 0x2a, 0xc9, // CVTPI2PS XMM1, MM1
        0x0f, 0x2a, 0xd2, // CVTPI2PS XMM2, MM2
        0x0f, 0x2a, 0xdb, // CVTPI2PS XMM3, MM3
        0x0f, 0x2a, 0xe4, // CVTPI2PS XMM4, MM4
        0x0f, 0x2a, 0xed, // CVTPI2PS XMM5, MM5
        0x0f, 0x2a, 0xf6, // CVTPI2PS XMM6, MM6
        0x0f, 0x2a, 0xff, // CVTPI2PS XMM7, MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pi_all_xmm_registers() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x2d, 0xc0, // CVTPS2PI MM0, XMM0
        0x0f, 0x2d, 0xc9, // CVTPS2PI MM1, XMM1
        0x0f, 0x2d, 0xd2, // CVTPS2PI MM2, XMM2
        0x0f, 0x2d, 0xdb, // CVTPS2PI MM3, XMM3
        0x0f, 0x2d, 0xe4, // CVTPS2PI MM4, XMM4
        0x0f, 0x2d, 0xed, // CVTPS2PI MM5, XMM5
        0x0f, 0x2d, 0xf6, // CVTPS2PI MM6, XMM6
        0x0f, 0x2d, 0xff, // CVTPS2PI MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
