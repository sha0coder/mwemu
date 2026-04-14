use crate::*;

// VSUBSS - Subtract Scalar Single-Precision Floating-Point Values
// VSUBSD - Subtract Scalar Double-Precision Floating-Point Values
//
// VSUBSS subtracts the low single-precision floating-point value of the third
// source operand from the low single-precision floating-point value of the second
// source operand and stores the result in the low doubleword of the destination operand.
//
// VSUBSD subtracts the low double-precision floating-point value of the third
// source operand from the low double-precision floating-point value of the second
// source operand and stores the result in the low quadword of the destination operand.
//
// Opcodes:
// VEX.LIG.F3.0F.WIG 5C /r    VSUBSS xmm1, xmm2, xmm3/m32   - Subtract scalar single from xmm3/mem to xmm2
// VEX.LIG.F2.0F.WIG 5C /r    VSUBSD xmm1, xmm2, xmm3/m64   - Subtract scalar double from xmm3/mem to xmm2

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VSUBSS Tests - Scalar Single-Precision
// ============================================================================

#[test]
fn test_vsubss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf2, 0x5c, 0xc2, // VSUBSS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xea, 0x5c, 0xcb, // VSUBSS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xe2, 0x5c, 0xd4, // VSUBSS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xda, 0x5c, 0xdd, // VSUBSS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xd2, 0x5c, 0xe6, // VSUBSS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xca, 0x5c, 0xef, // VSUBSS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x42, 0x5c, 0xf0, // VSUBSS XMM6, XMM7, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x3a, 0x5c, 0xf9, // VSUBSS XMM7, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x32, 0x5c, 0xc2, // VSUBSS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2a, 0x5c, 0xcb, // VSUBSS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x22, 0x5c, 0xd4, // VSUBSS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1a, 0x5c, 0xdd, // VSUBSS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x12, 0x5c, 0xe6, // VSUBSS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x0a, 0x5c, 0xef, // VSUBSS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm14_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x61, 0x02, 0x5c, 0xf0, // VSUBSS XMM14, XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm15_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x61, 0x7a, 0x5c, 0xf9, // VSUBSS XMM15, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory operand tests
#[test]
fn test_vsubss_xmm0_xmm1_mem32() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf2, 0x5c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VSUBSS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x80, 0x3f]; // 1.0f
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubss_xmm2_xmm3_mem32() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xe2, 0x5c, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // VSUBSS XMM2, XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x00, 0x40]; // 2.0f
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBSD Tests - Scalar Double-Precision
// ============================================================================

#[test]
fn test_vsubsd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf3, 0x5c, 0xc2, // VSUBSD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xeb, 0x5c, 0xcb, // VSUBSD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xe3, 0x5c, 0xd4, // VSUBSD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdb, 0x5c, 0xdd, // VSUBSD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xd3, 0x5c, 0xe6, // VSUBSD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xcb, 0x5c, 0xef, // VSUBSD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x43, 0x5c, 0xf0, // VSUBSD XMM6, XMM7, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x3b, 0x5c, 0xf9, // VSUBSD XMM7, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x33, 0x5c, 0xc2, // VSUBSD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2b, 0x5c, 0xcb, // VSUBSD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x23, 0x5c, 0xd4, // VSUBSD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1b, 0x5c, 0xdd, // VSUBSD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x13, 0x5c, 0xe6, // VSUBSD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x0b, 0x5c, 0xef, // VSUBSD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm14_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x61, 0x03, 0x5c, 0xf0, // VSUBSD XMM14, XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm15_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x61, 0x7b, 0x5c, 0xf9, // VSUBSD XMM15, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory operand tests
#[test]
fn test_vsubsd_xmm0_xmm1_mem64() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf3, 0x5c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VSUBSD XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f]; // 1.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm2_xmm3_mem64() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xe3, 0x5c, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // VSUBSD XMM2, XMM3, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40]; // 2.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubsd_xmm4_xmm5_mem64() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xd3, 0x5c, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // VSUBSD XMM4, XMM5, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40]; // 3.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}
