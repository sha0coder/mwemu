use crate::*;

// VCVTDQ2PD - Convert Packed Doubleword Integers to Packed Double-Precision Floating-Point Values
// VCVTPD2DQ - Convert Packed Double-Precision Floating-Point Values to Packed Doubleword Integers
//
// VCVTDQ2PD converts packed signed doubleword integers to packed double-precision floating-point values.
// VCVTPD2DQ converts packed double-precision floating-point values to packed signed doubleword integers.
// Rounding is controlled by MXCSR.RC (default: round to nearest even).
//
// Opcodes:
// VEX.128.F3.0F.WIG E6 /r VCVTDQ2PD xmm1, xmm2/m64    - Convert 2 packed int32 to 2 packed float64
// VEX.256.F3.0F.WIG E6 /r VCVTDQ2PD ymm1, xmm2/m128   - Convert 4 packed int32 to 4 packed float64
// VEX.128.F2.0F.WIG E6 /r VCVTPD2DQ xmm1, xmm2/m128   - Convert 2 packed float64 to 2 packed int32
// VEX.256.F2.0F.WIG E6 /r VCVTPD2DQ xmm1, ymm2/m256   - Convert 4 packed float64 to 4 packed int32

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VCVTDQ2PD Tests - 128-bit (convert 2x int32 to 2x float64)
// ============================================================================

#[test]
fn test_vcvtdq2pd_xmm0_xmm1() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM0, XMM1
    let code = [
        0xc5, 0xfa, 0xe6, 0xc1, // VCVTDQ2PD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm1_xmm2() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM1, XMM2
    let code = [
        0xc5, 0xfa, 0xe6, 0xca, // VCVTDQ2PD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm2_xmm3() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM2, XMM3
    let code = [
        0xc5, 0xfa, 0xe6, 0xd3, // VCVTDQ2PD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm3_xmm4() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM3, XMM4
    let code = [
        0xc5, 0xfa, 0xe6, 0xdc, // VCVTDQ2PD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm4_xmm5() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM4, XMM5
    let code = [
        0xc5, 0xfa, 0xe6, 0xe5, // VCVTDQ2PD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm5_xmm6() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM5, XMM6
    let code = [
        0xc5, 0xfa, 0xe6, 0xee, // VCVTDQ2PD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm6_xmm7() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM6, XMM7
    let code = [
        0xc5, 0xfa, 0xe6, 0xf7, // VCVTDQ2PD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm7_xmm0() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM7, XMM0
    let code = [
        0xc5, 0xfa, 0xe6, 0xf8, // VCVTDQ2PD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtdq2pd_xmm8_xmm9() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0xc1, // VCVTDQ2PD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm9_xmm10() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0xca, // VCVTDQ2PD XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm10_xmm11() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0xd3, // VCVTDQ2PD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm11_xmm12() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0xdc, // VCVTDQ2PD XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm12_xmm13() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0xe5, // VCVTDQ2PD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm13_xmm14() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0xee, // VCVTDQ2PD XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm14_xmm15() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0xf7, // VCVTDQ2PD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm15_xmm8() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0xf8, // VCVTDQ2PD XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PD Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vcvtdq2pd_xmm0_xmm8() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM0, XMM8
    let code = [
        0xc4, 0xc1, 0x7a, 0xe6, 0xc0, // VCVTDQ2PD XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm8_xmm0() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x7a, 0xe6, 0xc0, // VCVTDQ2PD XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm7_xmm15() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM7, XMM15
    let code = [
        0xc4, 0xc1, 0x7a, 0xe6, 0xff, // VCVTDQ2PD XMM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PD Tests - 256-bit (convert 4x int32 to 4x float64)
// ============================================================================

#[test]
fn test_vcvtdq2pd_ymm0_xmm1() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM0, XMM1
    let code = [
        0xc5, 0xfe, 0xe6, 0xc1, // VCVTDQ2PD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm1_xmm2() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM1, XMM2
    let code = [
        0xc5, 0xfe, 0xe6, 0xca, // VCVTDQ2PD YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm2_xmm3() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM2, XMM3
    let code = [
        0xc5, 0xfe, 0xe6, 0xd3, // VCVTDQ2PD YMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm3_xmm4() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM3, XMM4
    let code = [
        0xc5, 0xfe, 0xe6, 0xdc, // VCVTDQ2PD YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm4_xmm5() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM4, XMM5
    let code = [
        0xc5, 0xfe, 0xe6, 0xe5, // VCVTDQ2PD YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm5_xmm6() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM5, XMM6
    let code = [
        0xc5, 0xfe, 0xe6, 0xee, // VCVTDQ2PD YMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm6_xmm7() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM6, XMM7
    let code = [
        0xc5, 0xfe, 0xe6, 0xf7, // VCVTDQ2PD YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm7_xmm0() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM7, XMM0
    let code = [
        0xc5, 0xfe, 0xe6, 0xf8, // VCVTDQ2PD YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PD Tests - Extended YMM registers
// ============================================================================

#[test]
fn test_vcvtdq2pd_ymm8_xmm9() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM8, XMM9
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0xc1, // VCVTDQ2PD YMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm9_xmm10() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM9, XMM10
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0xca, // VCVTDQ2PD YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm10_xmm11() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM10, XMM11
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0xd3, // VCVTDQ2PD YMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm11_xmm12() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM11, XMM12
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0xdc, // VCVTDQ2PD YMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm12_xmm13() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM12, XMM13
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0xe5, // VCVTDQ2PD YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm13_xmm14() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM13, XMM14
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0xee, // VCVTDQ2PD YMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm14_xmm15() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM14, XMM15
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0xf7, // VCVTDQ2PD YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm15_xmm8() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM15, XMM8
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0xf8, // VCVTDQ2PD YMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PD Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vcvtdq2pd_xmm0_mem() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM0, [mem] (reads 64 bits / 2 int32)
    let code = [
        0xc5, 0xfa, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTDQ2PD XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x01, 0x00, 0x00, 0x00, // 1
        0x02, 0x00, 0x00, 0x00, // 2
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_xmm8_mem() {
    let mut emu = emu64();
    // VCVTDQ2PD XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7a, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTDQ2PD XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0xff, 0xff, 0xff, 0xff, // -1
        0x64, 0x00, 0x00, 0x00, // 100
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PD Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vcvtdq2pd_ymm0_mem() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM0, [mem] (reads 128 bits / 4 int32)
    let code = [
        0xc5, 0xfe, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTDQ2PD YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, // 1
        0x02, 0x00, 0x00, 0x00, // 2
        0x03, 0x00, 0x00, 0x00, // 3
        0x04, 0x00, 0x00, 0x00, // 4
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2pd_ymm8_mem() {
    let mut emu = emu64();
    // VCVTDQ2PD YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7e, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTDQ2PD YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x80, // -2147483648 (INT32_MIN)
        0xff, 0xff, 0xff, 0x7f, // 2147483647 (INT32_MAX)
        0x00, 0x00, 0x00, 0x00, // 0
        0xff, 0xff, 0xff, 0xff, // -1
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2DQ Tests - 128-bit (convert 2x float64 to 2x int32)
// ============================================================================

#[test]
fn test_vcvtpd2dq_xmm0_xmm1() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM0, XMM1
    let code = [
        0xc5, 0xfb, 0xe6, 0xc1, // VCVTPD2DQ XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm1_xmm2() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM1, XMM2
    let code = [
        0xc5, 0xfb, 0xe6, 0xca, // VCVTPD2DQ XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm2_xmm3() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM2, XMM3
    let code = [
        0xc5, 0xfb, 0xe6, 0xd3, // VCVTPD2DQ XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm3_xmm4() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM3, XMM4
    let code = [
        0xc5, 0xfb, 0xe6, 0xdc, // VCVTPD2DQ XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm4_xmm5() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM4, XMM5
    let code = [
        0xc5, 0xfb, 0xe6, 0xe5, // VCVTPD2DQ XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm5_xmm6() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM5, XMM6
    let code = [
        0xc5, 0xfb, 0xe6, 0xee, // VCVTPD2DQ XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm6_xmm7() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM6, XMM7
    let code = [
        0xc5, 0xfb, 0xe6, 0xf7, // VCVTPD2DQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm7_xmm0() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM7, XMM0
    let code = [
        0xc5, 0xfb, 0xe6, 0xf8, // VCVTPD2DQ XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2DQ Tests - Extended XMM registers
// ============================================================================

#[test]
fn test_vcvtpd2dq_xmm8_xmm9() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0xc1, // VCVTPD2DQ XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm9_xmm10() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0xca, // VCVTPD2DQ XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm10_xmm11() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0xd3, // VCVTPD2DQ XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm11_xmm12() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0xdc, // VCVTPD2DQ XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm12_xmm13() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0xe5, // VCVTPD2DQ XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm13_xmm14() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0xee, // VCVTPD2DQ XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm14_xmm15() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0xf7, // VCVTPD2DQ XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm15_xmm8() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0xf8, // VCVTPD2DQ XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2DQ Tests - 256-bit (convert 4x float64 to 4x int32)
// ============================================================================

#[test]
fn test_vcvtpd2dq_xmm0_ymm1() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM0, YMM1
    let code = [
        0xc5, 0xff, 0xe6, 0xc1, // VCVTPD2DQ XMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm1_ymm2() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM1, YMM2
    let code = [
        0xc5, 0xff, 0xe6, 0xca, // VCVTPD2DQ XMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm2_ymm3() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM2, YMM3
    let code = [
        0xc5, 0xff, 0xe6, 0xd3, // VCVTPD2DQ XMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm3_ymm4() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM3, YMM4
    let code = [
        0xc5, 0xff, 0xe6, 0xdc, // VCVTPD2DQ XMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm4_ymm5() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM4, YMM5
    let code = [
        0xc5, 0xff, 0xe6, 0xe5, // VCVTPD2DQ XMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm5_ymm6() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM5, YMM6
    let code = [
        0xc5, 0xff, 0xe6, 0xee, // VCVTPD2DQ XMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm6_ymm7() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM6, YMM7
    let code = [
        0xc5, 0xff, 0xe6, 0xf7, // VCVTPD2DQ XMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm7_ymm0() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM7, YMM0
    let code = [
        0xc5, 0xff, 0xe6, 0xf8, // VCVTPD2DQ XMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2DQ Tests - Extended YMM registers
// ============================================================================

#[test]
fn test_vcvtpd2dq_xmm8_ymm9() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0xc1, // VCVTPD2DQ XMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm9_ymm10() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0xca, // VCVTPD2DQ XMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm10_ymm11() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0xd3, // VCVTPD2DQ XMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm11_ymm12() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0xdc, // VCVTPD2DQ XMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm12_ymm13() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0xe5, // VCVTPD2DQ XMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm13_ymm14() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0xee, // VCVTPD2DQ XMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm14_ymm15() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0xf7, // VCVTPD2DQ XMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtpd2dq_xmm15_ymm8() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0xf8, // VCVTPD2DQ XMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPD2DQ Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vcvtpd2dq_xmm0_mem() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM0, [mem] (reads 128 bits / 2 doubles)
    let code = [
        0xc5, 0xfb, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPD2DQ XMM0, [rip + 0x4000]
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
fn test_vcvtpd2dq_xmm8_mem() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7b, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPD2DQ XMM8, [rip + 0x4000]
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
// VCVTPD2DQ Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vcvtpd2dq_xmm0_mem256() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM0, [mem] (reads 256 bits / 4 doubles)
    let code = [
        0xc5, 0xff, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPD2DQ XMM0, [rip + 0x4000]
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
fn test_vcvtpd2dq_xmm8_mem256() {
    let mut emu = emu64();
    // VCVTPD2DQ XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7f, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPD2DQ XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xbf, // -1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x59, 0x40, // 100.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x59, 0xc0, // -100.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x8f, 0x40, // 1000.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
