use crate::*;

// VCVTPS2PD - Convert Packed Single-Precision to Packed Double-Precision Floating-Point Values
// VCVTPD2PS - Convert Packed Double-Precision to Packed Single-Precision Floating-Point Values
//
// VCVTPS2PD converts packed single-precision floating-point values to packed double-precision.
// VCVTPD2PS converts packed double-precision floating-point values to packed single-precision.
//
// Opcodes:
// VEX.128.0F.WIG 5A /r    VCVTPS2PD xmm1, xmm2/m64   - Convert 2 packed single to double
// VEX.256.0F.WIG 5A /r    VCVTPS2PD ymm1, xmm2/m128  - Convert 4 packed single to double
// VEX.128.66.0F.WIG 5A /r VCVTPD2PS xmm1, xmm2/m128  - Convert 2 packed double to single
// VEX.256.66.0F.WIG 5A /r VCVTPD2PS xmm1, ymm2/m256  - Convert 4 packed double to single

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VCVTPS2PD Tests - 128-bit (convert 2x float32 to 2x float64)
// ============================================================================

#[test]
fn test_vcvtps2pd_xmm0_xmm1() {
    let mut emu = emu64();
    // VCVTPS2PD XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x5a, 0xc1, // VCVTPS2PD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm1_xmm2() {
    let mut emu = emu64();
    // VCVTPS2PD XMM1, XMM2
    let code = [
        0xc5, 0xf8, 0x5a, 0xca, // VCVTPS2PD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm2_xmm3() {
    let mut emu = emu64();
    // VCVTPS2PD XMM2, XMM3
    let code = [
        0xc5, 0xf8, 0x5a, 0xd3, // VCVTPS2PD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm3_xmm4() {
    let mut emu = emu64();
    // VCVTPS2PD XMM3, XMM4
    let code = [
        0xc5, 0xf8, 0x5a, 0xdc, // VCVTPS2PD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm4_xmm5() {
    let mut emu = emu64();
    // VCVTPS2PD XMM4, XMM5
    let code = [
        0xc5, 0xf8, 0x5a, 0xe5, // VCVTPS2PD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm5_xmm6() {
    let mut emu = emu64();
    // VCVTPS2PD XMM5, XMM6
    let code = [
        0xc5, 0xf8, 0x5a, 0xee, // VCVTPS2PD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm6_xmm7() {
    let mut emu = emu64();
    // VCVTPS2PD XMM6, XMM7
    let code = [
        0xc5, 0xf8, 0x5a, 0xf7, // VCVTPS2PD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm7_xmm0() {
    let mut emu = emu64();
    // VCVTPS2PD XMM7, XMM0
    let code = [
        0xc5, 0xf8, 0x5a, 0xf8, // VCVTPS2PD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2PD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtps2pd_xmm8_xmm9() {
    let mut emu = emu64();
    // VCVTPS2PD XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0xc1, // VCVTPS2PD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm9_xmm10() {
    let mut emu = emu64();
    // VCVTPS2PD XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0xca, // VCVTPS2PD XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm10_xmm11() {
    let mut emu = emu64();
    // VCVTPS2PD XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0xd3, // VCVTPS2PD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm11_xmm12() {
    let mut emu = emu64();
    // VCVTPS2PD XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0xdc, // VCVTPS2PD XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm12_xmm13() {
    let mut emu = emu64();
    // VCVTPS2PD XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0xe5, // VCVTPS2PD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm13_xmm14() {
    let mut emu = emu64();
    // VCVTPS2PD XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0xee, // VCVTPS2PD XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm14_xmm15() {
    let mut emu = emu64();
    // VCVTPS2PD XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0xf7, // VCVTPS2PD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm15_xmm8() {
    let mut emu = emu64();
    // VCVTPS2PD XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0xf8, // VCVTPS2PD XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2PD Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vcvtps2pd_xmm0_xmm8() {
    let mut emu = emu64();
    // VCVTPS2PD XMM0, XMM8
    let code = [
        0xc4, 0xc1, 0x78, 0x5a, 0xc0, // VCVTPS2PD XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm8_xmm0() {
    let mut emu = emu64();
    // VCVTPS2PD XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x78, 0x5a, 0xc0, // VCVTPS2PD XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm7_xmm15() {
    let mut emu = emu64();
    // VCVTPS2PD XMM7, XMM15
    let code = [
        0xc4, 0xc1, 0x78, 0x5a, 0xff, // VCVTPS2PD XMM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2PD Tests - 256-bit (convert 4x float32 to 4x float64)
// ============================================================================

#[test]
fn test_vcvtps2pd_ymm0_xmm1() {
    let mut emu = emu64();
    // VCVTPS2PD YMM0, XMM1
    let code = [
        0xc5, 0xfc, 0x5a, 0xc1, // VCVTPS2PD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm1_xmm2() {
    let mut emu = emu64();
    // VCVTPS2PD YMM1, XMM2
    let code = [
        0xc5, 0xfc, 0x5a, 0xca, // VCVTPS2PD YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm2_xmm3() {
    let mut emu = emu64();
    // VCVTPS2PD YMM2, XMM3
    let code = [
        0xc5, 0xfc, 0x5a, 0xd3, // VCVTPS2PD YMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm3_xmm4() {
    let mut emu = emu64();
    // VCVTPS2PD YMM3, XMM4
    let code = [
        0xc5, 0xfc, 0x5a, 0xdc, // VCVTPS2PD YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm4_xmm5() {
    let mut emu = emu64();
    // VCVTPS2PD YMM4, XMM5
    let code = [
        0xc5, 0xfc, 0x5a, 0xe5, // VCVTPS2PD YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm5_xmm6() {
    let mut emu = emu64();
    // VCVTPS2PD YMM5, XMM6
    let code = [
        0xc5, 0xfc, 0x5a, 0xee, // VCVTPS2PD YMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm6_xmm7() {
    let mut emu = emu64();
    // VCVTPS2PD YMM6, XMM7
    let code = [
        0xc5, 0xfc, 0x5a, 0xf7, // VCVTPS2PD YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm7_xmm0() {
    let mut emu = emu64();
    // VCVTPS2PD YMM7, XMM0
    let code = [
        0xc5, 0xfc, 0x5a, 0xf8, // VCVTPS2PD YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2PD Tests - Extended YMM registers
// ============================================================================

#[test]
fn test_vcvtps2pd_ymm8_xmm9() {
    let mut emu = emu64();
    // VCVTPS2PD YMM8, XMM9
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0xc1, // VCVTPS2PD YMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm9_xmm10() {
    let mut emu = emu64();
    // VCVTPS2PD YMM9, XMM10
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0xca, // VCVTPS2PD YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm10_xmm11() {
    let mut emu = emu64();
    // VCVTPS2PD YMM10, XMM11
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0xd3, // VCVTPS2PD YMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm11_xmm12() {
    let mut emu = emu64();
    // VCVTPS2PD YMM11, XMM12
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0xdc, // VCVTPS2PD YMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm12_xmm13() {
    let mut emu = emu64();
    // VCVTPS2PD YMM12, XMM13
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0xe5, // VCVTPS2PD YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm13_xmm14() {
    let mut emu = emu64();
    // VCVTPS2PD YMM13, XMM14
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0xee, // VCVTPS2PD YMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm14_xmm15() {
    let mut emu = emu64();
    // VCVTPS2PD YMM14, XMM15
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0xf7, // VCVTPS2PD YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm15_xmm8() {
    let mut emu = emu64();
    // VCVTPS2PD YMM15, XMM8
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0xf8, // VCVTPS2PD YMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2PD Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vcvtps2pd_xmm0_mem() {
    let mut emu = emu64();
    // VCVTPS2PD XMM0, [mem] (reads 64 bits / 2 floats)
    let code = [
        0xc5, 0xf8, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPS2PD XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0f
        0x00, 0x00, 0x00, 0x40, // 2.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_xmm8_mem() {
    let mut emu = emu64();
    // VCVTPS2PD XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x78, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPS2PD XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x40, 0x40, // 3.0f
        0x00, 0x00, 0x80, 0x40, // 4.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2PD Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vcvtps2pd_ymm0_mem() {
    let mut emu = emu64();
    // VCVTPS2PD YMM0, [mem] (reads 128 bits / 4 floats)
    let code = [
        0xc5, 0xfc, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPS2PD YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0f
        0x00, 0x00, 0x00, 0x40, // 2.0f
        0x00, 0x00, 0x40, 0x40, // 3.0f
        0x00, 0x00, 0x80, 0x40, // 4.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2pd_ymm8_mem() {
    let mut emu = emu64();
    // VCVTPS2PD YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7c, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPS2PD YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xa0, 0x40, // 5.0f
        0x00, 0x00, 0xc0, 0x40, // 6.0f
        0x00, 0x00, 0xe0, 0x40, // 7.0f
        0x00, 0x00, 0x00, 0x41, // 8.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2PS Tests - 128-bit (convert 2x float64 to 2x float32)
// ============================================================================

#[test]
fn test_vcvtpd2ps_xmm0_xmm1() {
    let mut emu = emu64();
    // VCVTPD2PS XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x5a, 0xc1, // VCVTPD2PS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm1_xmm2() {
    let mut emu = emu64();
    // VCVTPD2PS XMM1, XMM2
    let code = [
        0xc5, 0xf9, 0x5a, 0xca, // VCVTPD2PS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm2_xmm3() {
    let mut emu = emu64();
    // VCVTPD2PS XMM2, XMM3
    let code = [
        0xc5, 0xf9, 0x5a, 0xd3, // VCVTPD2PS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm3_xmm4() {
    let mut emu = emu64();
    // VCVTPD2PS XMM3, XMM4
    let code = [
        0xc5, 0xf9, 0x5a, 0xdc, // VCVTPD2PS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm4_xmm5() {
    let mut emu = emu64();
    // VCVTPD2PS XMM4, XMM5
    let code = [
        0xc5, 0xf9, 0x5a, 0xe5, // VCVTPD2PS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm5_xmm6() {
    let mut emu = emu64();
    // VCVTPD2PS XMM5, XMM6
    let code = [
        0xc5, 0xf9, 0x5a, 0xee, // VCVTPD2PS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm6_xmm7() {
    let mut emu = emu64();
    // VCVTPD2PS XMM6, XMM7
    let code = [
        0xc5, 0xf9, 0x5a, 0xf7, // VCVTPD2PS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm7_xmm0() {
    let mut emu = emu64();
    // VCVTPD2PS XMM7, XMM0
    let code = [
        0xc5, 0xf9, 0x5a, 0xf8, // VCVTPD2PS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2PS Tests - Extended XMM registers
// ============================================================================

#[test]
fn test_vcvtpd2ps_xmm8_xmm9() {
    let mut emu = emu64();
    // VCVTPD2PS XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0xc1, // VCVTPD2PS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm9_xmm10() {
    let mut emu = emu64();
    // VCVTPD2PS XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0xca, // VCVTPD2PS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm10_xmm11() {
    let mut emu = emu64();
    // VCVTPD2PS XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0xd3, // VCVTPD2PS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm11_xmm12() {
    let mut emu = emu64();
    // VCVTPD2PS XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0xdc, // VCVTPD2PS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm12_xmm13() {
    let mut emu = emu64();
    // VCVTPD2PS XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0xe5, // VCVTPD2PS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm13_xmm14() {
    let mut emu = emu64();
    // VCVTPD2PS XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0xee, // VCVTPD2PS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm14_xmm15() {
    let mut emu = emu64();
    // VCVTPD2PS XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0xf7, // VCVTPD2PS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm15_xmm8() {
    let mut emu = emu64();
    // VCVTPD2PS XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0xf8, // VCVTPD2PS XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2PS Tests - 256-bit (convert 4x float64 to 4x float32)
// ============================================================================

#[test]
fn test_vcvtpd2ps_xmm0_ymm1() {
    let mut emu = emu64();
    // VCVTPD2PS XMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x5a, 0xc1, // VCVTPD2PS XMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm1_ymm2() {
    let mut emu = emu64();
    // VCVTPD2PS XMM1, YMM2
    let code = [
        0xc5, 0xfd, 0x5a, 0xca, // VCVTPD2PS XMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm2_ymm3() {
    let mut emu = emu64();
    // VCVTPD2PS XMM2, YMM3
    let code = [
        0xc5, 0xfd, 0x5a, 0xd3, // VCVTPD2PS XMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm3_ymm4() {
    let mut emu = emu64();
    // VCVTPD2PS XMM3, YMM4
    let code = [
        0xc5, 0xfd, 0x5a, 0xdc, // VCVTPD2PS XMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm4_ymm5() {
    let mut emu = emu64();
    // VCVTPD2PS XMM4, YMM5
    let code = [
        0xc5, 0xfd, 0x5a, 0xe5, // VCVTPD2PS XMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm5_ymm6() {
    let mut emu = emu64();
    // VCVTPD2PS XMM5, YMM6
    let code = [
        0xc5, 0xfd, 0x5a, 0xee, // VCVTPD2PS XMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm6_ymm7() {
    let mut emu = emu64();
    // VCVTPD2PS XMM6, YMM7
    let code = [
        0xc5, 0xfd, 0x5a, 0xf7, // VCVTPD2PS XMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm7_ymm0() {
    let mut emu = emu64();
    // VCVTPD2PS XMM7, YMM0
    let code = [
        0xc5, 0xfd, 0x5a, 0xf8, // VCVTPD2PS XMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2PS Tests - Extended YMM registers
// ============================================================================

#[test]
fn test_vcvtpd2ps_xmm8_ymm9() {
    let mut emu = emu64();
    // VCVTPD2PS XMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0xc1, // VCVTPD2PS XMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm9_ymm10() {
    let mut emu = emu64();
    // VCVTPD2PS XMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0xca, // VCVTPD2PS XMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm10_ymm11() {
    let mut emu = emu64();
    // VCVTPD2PS XMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0xd3, // VCVTPD2PS XMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm11_ymm12() {
    let mut emu = emu64();
    // VCVTPD2PS XMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0xdc, // VCVTPD2PS XMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm12_ymm13() {
    let mut emu = emu64();
    // VCVTPD2PS XMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0xe5, // VCVTPD2PS XMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm13_ymm14() {
    let mut emu = emu64();
    // VCVTPD2PS XMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0xee, // VCVTPD2PS XMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm14_ymm15() {
    let mut emu = emu64();
    // VCVTPD2PS XMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0xf7, // VCVTPD2PS XMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm15_ymm8() {
    let mut emu = emu64();
    // VCVTPD2PS XMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0xf8, // VCVTPD2PS XMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2PS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vcvtpd2ps_xmm0_mem() {
    let mut emu = emu64();
    // VCVTPD2PS XMM0, [mem] (reads 128 bits / 2 doubles)
    let code = [
        0xc5, 0xf9, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPD2PS XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm8_mem() {
    let mut emu = emu64();
    // VCVTPD2PS XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x79, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPD2PS XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2PS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vcvtpd2ps_xmm0_mem256() {
    let mut emu = emu64();
    // VCVTPD2PS XMM0, [mem] (reads 256 bits / 4 doubles)
    let code = [
        0xc5, 0xfd, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPD2PS XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2ps_xmm8_mem256() {
    let mut emu = emu64();
    // VCVTPD2PS XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7d, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPD2PS XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x40, // 5.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x40, // 6.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x40, // 8.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
