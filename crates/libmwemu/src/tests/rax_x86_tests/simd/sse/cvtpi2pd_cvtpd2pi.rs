use crate::*;

// CVTPI2PD - Convert Packed Dword Integers to Packed Double Precision FP Values
// CVTPD2PI - Convert Packed Double Precision FP Values to Packed Dword Integers
//
// CVTPI2PD converts two packed signed doubleword integers from MMX register or memory
// to two packed double precision floating-point values in XMM register.
//
// CVTPD2PI converts two packed double precision floating-point values from XMM register
// or memory to two packed signed doubleword integers in MMX register. Conversion is
// performed according to current rounding mode in MXCSR.
//
// Opcodes:
// 66 0F 2A /r    CVTPI2PD xmm, mm/m64    - Convert two signed dwords to two double FP
// 66 0F 2D /r    CVTPD2PI mm, xmm/m128   - Convert two double FP to two signed dwords
//
// CVTPI2PD (xmm,mm) causes a transition from x87 FPU to MMX technology operation.

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// CVTPI2PD - MMX to XMM Conversion Tests
// ============================================================================

#[test]
fn test_cvtpi2pd_mm0_to_xmm0() {
    let mut emu = emu64();
    // CVTPI2PD XMM0, MM0
    let code = [
        0x66, 0x0f, 0x2a, 0xc0, // CVTPI2PD XMM0, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm1_to_xmm1() {
    let mut emu = emu64();
    // CVTPI2PD XMM1, MM1
    let code = [
        0x66, 0x0f, 0x2a, 0xc9, // CVTPI2PD XMM1, MM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm2_to_xmm2() {
    let mut emu = emu64();
    // CVTPI2PD XMM2, MM2
    let code = [
        0x66, 0x0f, 0x2a, 0xd2, // CVTPI2PD XMM2, MM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm3_to_xmm3() {
    let mut emu = emu64();
    // CVTPI2PD XMM3, MM3
    let code = [
        0x66, 0x0f, 0x2a, 0xdb, // CVTPI2PD XMM3, MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm4_to_xmm4() {
    let mut emu = emu64();
    // CVTPI2PD XMM4, MM4
    let code = [
        0x66, 0x0f, 0x2a, 0xe4, // CVTPI2PD XMM4, MM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm5_to_xmm5() {
    let mut emu = emu64();
    // CVTPI2PD XMM5, MM5
    let code = [
        0x66, 0x0f, 0x2a, 0xed, // CVTPI2PD XMM5, MM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm6_to_xmm6() {
    let mut emu = emu64();
    // CVTPI2PD XMM6, MM6
    let code = [
        0x66, 0x0f, 0x2a, 0xf6, // CVTPI2PD XMM6, MM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm7_to_xmm7() {
    let mut emu = emu64();
    // CVTPI2PD XMM7, MM7
    let code = [
        0x66, 0x0f, 0x2a, 0xff, // CVTPI2PD XMM7, MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm0_to_xmm8() {
    let mut emu = emu64();
    // CVTPI2PD XMM8, MM0
    let code = [
        0x66, 0x44, 0x0f, 0x2a, 0xc0, // CVTPI2PD XMM8, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mm7_to_xmm15() {
    let mut emu = emu64();
    // CVTPI2PD XMM15, MM7
    let code = [
        0x66, 0x44, 0x0f, 0x2a, 0xff, // CVTPI2PD XMM15, MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTPI2PD - Memory to XMM Conversion Tests
// ============================================================================

#[test]
fn test_cvtpi2pd_mem_to_xmm0() {
    let mut emu = emu64();
    // CVTPI2PD XMM0, [0x3000]
    let code = [
        0x66, 0x0f, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PD XMM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mem_to_xmm1() {
    let mut emu = emu64();
    // CVTPI2PD XMM1, [0x3000]
    let code = [
        0x66, 0x0f, 0x2a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PD XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mem_to_xmm3() {
    let mut emu = emu64();
    // CVTPI2PD XMM3, [0x3000]
    let code = [
        0x66, 0x0f, 0x2a, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PD XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mem_to_xmm7() {
    let mut emu = emu64();
    // CVTPI2PD XMM7, [0x3000]
    let code = [
        0x66, 0x0f, 0x2a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PD XMM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mem_to_xmm15() {
    let mut emu = emu64();
    // CVTPI2PD XMM15, [0x3000]
    let code = [
        0x66, 0x44, 0x0f, 0x2a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PD XMM15, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTPD2PI - XMM to MMX Conversion Tests
// ============================================================================

#[test]
fn test_cvtpd2pi_xmm0_to_mm0() {
    let mut emu = emu64();
    // CVTPD2PI MM0, XMM0
    let code = [
        0x66, 0x0f, 0x2d, 0xc0, // CVTPD2PI MM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm1_to_mm1() {
    let mut emu = emu64();
    // CVTPD2PI MM1, XMM1
    let code = [
        0x66, 0x0f, 0x2d, 0xc9, // CVTPD2PI MM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm2_to_mm2() {
    let mut emu = emu64();
    // CVTPD2PI MM2, XMM2
    let code = [
        0x66, 0x0f, 0x2d, 0xd2, // CVTPD2PI MM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm3_to_mm3() {
    let mut emu = emu64();
    // CVTPD2PI MM3, XMM3
    let code = [
        0x66, 0x0f, 0x2d, 0xdb, // CVTPD2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm4_to_mm4() {
    let mut emu = emu64();
    // CVTPD2PI MM4, XMM4
    let code = [
        0x66, 0x0f, 0x2d, 0xe4, // CVTPD2PI MM4, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm5_to_mm5() {
    let mut emu = emu64();
    // CVTPD2PI MM5, XMM5
    let code = [
        0x66, 0x0f, 0x2d, 0xed, // CVTPD2PI MM5, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm6_to_mm6() {
    let mut emu = emu64();
    // CVTPD2PI MM6, XMM6
    let code = [
        0x66, 0x0f, 0x2d, 0xf6, // CVTPD2PI MM6, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm7_to_mm7() {
    let mut emu = emu64();
    // CVTPD2PI MM7, XMM7
    let code = [
        0x66, 0x0f, 0x2d, 0xff, // CVTPD2PI MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm8_to_mm0() {
    let mut emu = emu64();
    // CVTPD2PI MM0, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x2d, 0xc0, // CVTPD2PI MM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_xmm15_to_mm7() {
    let mut emu = emu64();
    // CVTPD2PI MM7, XMM15
    let code = [
        0x66, 0x41, 0x0f, 0x2d, 0xff, // CVTPD2PI MM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// CVTPD2PI - Memory to MMX Conversion Tests
// ============================================================================

#[test]
fn test_cvtpd2pi_mem_to_mm0() {
    let mut emu = emu64();
    // CVTPD2PI MM0, [0x3000]
    let code = [
        0x66, 0x0f, 0x2d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPD2PI MM0, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_mem_to_mm1() {
    let mut emu = emu64();
    // CVTPD2PI MM1, [0x3000]
    let code = [
        0x66, 0x0f, 0x2d, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPD2PI MM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_mem_to_mm3() {
    let mut emu = emu64();
    // CVTPD2PI MM3, [0x3000]
    let code = [
        0x66, 0x0f, 0x2d, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPD2PI MM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_mem_to_mm7() {
    let mut emu = emu64();
    // CVTPD2PI MM7, [0x3000]
    let code = [
        0x66, 0x0f, 0x2d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPD2PI MM7, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Conversion Chains
// ============================================================================

#[test]
fn test_cvtpi2pd_cvtpd2pi_round_trip() {
    let mut emu = emu64();
    // CVTPI2PD XMM0, MM0 then CVTPD2PI MM1, XMM0
    let code = [
        0x66, 0x0f, 0x2a, 0xc0, // CVTPI2PD XMM0, MM0
        0x66, 0x0f, 0x2d, 0xc8, // CVTPD2PI MM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_cvtpi2pd_round_trip() {
    let mut emu = emu64();
    // CVTPD2PI MM0, XMM0 then CVTPI2PD XMM1, MM0
    let code = [
        0x66, 0x0f, 0x2d, 0xc0, // CVTPD2PI MM0, XMM0
        0x66, 0x0f, 0x2a, 0xc8, // CVTPI2PD XMM1, MM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_cvtpi2pd() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2a, 0xc0, // CVTPI2PD XMM0, MM0
        0x66, 0x0f, 0x2a, 0xc9, // CVTPI2PD XMM1, MM1
        0x66, 0x0f, 0x2a, 0xd2, // CVTPI2PD XMM2, MM2
        0x66, 0x0f, 0x2a, 0xdb, // CVTPI2PD XMM3, MM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_multiple_cvtpd2pi() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2d, 0xc0, // CVTPD2PI MM0, XMM0
        0x66, 0x0f, 0x2d, 0xc9, // CVTPD2PI MM1, XMM1
        0x66, 0x0f, 0x2d, 0xd2, // CVTPD2PI MM2, XMM2
        0x66, 0x0f, 0x2d, 0xdb, // CVTPD2PI MM3, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_all_mmx_registers() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2a, 0xc0, // CVTPI2PD XMM0, MM0
        0x66, 0x0f, 0x2a, 0xc9, // CVTPI2PD XMM1, MM1
        0x66, 0x0f, 0x2a, 0xd2, // CVTPI2PD XMM2, MM2
        0x66, 0x0f, 0x2a, 0xdb, // CVTPI2PD XMM3, MM3
        0x66, 0x0f, 0x2a, 0xe4, // CVTPI2PD XMM4, MM4
        0x66, 0x0f, 0x2a, 0xed, // CVTPI2PD XMM5, MM5
        0x66, 0x0f, 0x2a, 0xf6, // CVTPI2PD XMM6, MM6
        0x66, 0x0f, 0x2a, 0xff, // CVTPI2PD XMM7, MM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_all_xmm_registers() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2d, 0xc0, // CVTPD2PI MM0, XMM0
        0x66, 0x0f, 0x2d, 0xc9, // CVTPD2PI MM1, XMM1
        0x66, 0x0f, 0x2d, 0xd2, // CVTPD2PI MM2, XMM2
        0x66, 0x0f, 0x2d, 0xdb, // CVTPD2PI MM3, XMM3
        0x66, 0x0f, 0x2d, 0xe4, // CVTPD2PI MM4, XMM4
        0x66, 0x0f, 0x2d, 0xed, // CVTPD2PI MM5, XMM5
        0x66, 0x0f, 0x2d, 0xf6, // CVTPD2PI MM6, XMM6
        0x66, 0x0f, 0x2d, 0xff, // CVTPD2PI MM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpi2pd_mem_from_various_addresses() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPI2PD XMM0, [0x3000]
        0x66, 0x0f, 0x2a, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // CVTPI2PD XMM1, [0x3008]
        0x66, 0x0f, 0x2a, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // CVTPI2PD XMM2, [0x3010]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2pi_mem_from_various_addresses() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x2d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // CVTPD2PI MM0, [0x3000]
        0x66, 0x0f, 0x2d, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // CVTPD2PI MM1, [0x3010]
        0x66, 0x0f, 0x2d, 0x14, 0x25, 0x20, 0x30, 0x00, 0x00, // CVTPD2PI MM2, [0x3020]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
