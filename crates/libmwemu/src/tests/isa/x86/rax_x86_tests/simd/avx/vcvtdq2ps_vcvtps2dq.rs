use crate::*;

// VCVTDQ2PS - Convert Packed Doubleword Integers to Packed Single-Precision Floating-Point Values
// VCVTPS2DQ - Convert Packed Single-Precision Floating-Point Values to Packed Doubleword Integers
//
// VCVTDQ2PS converts packed signed doubleword integers to packed single-precision floating-point values.
// VCVTPS2DQ converts packed single-precision floating-point values to packed signed doubleword integers.
// Rounding is controlled by MXCSR.RC (default: round to nearest even).
//
// Opcodes:
// VEX.128.0F.WIG 5B /r    VCVTDQ2PS xmm1, xmm2/m128   - Convert packed int32 to packed float32
// VEX.256.0F.WIG 5B /r    VCVTDQ2PS ymm1, ymm2/m256   - Convert packed int32 to packed float32
// VEX.128.66.0F.WIG 5B /r VCVTPS2DQ xmm1, xmm2/m128   - Convert packed float32 to packed int32
// VEX.256.66.0F.WIG 5B /r VCVTPS2DQ ymm1, ymm2/m256   - Convert packed float32 to packed int32

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VCVTDQ2PS Tests - 128-bit XMM registers (4x int32 to 4x float32)
// ============================================================================

#[test]
fn test_vcvtdq2ps_xmm0_xmm1() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x5b, 0xc1, // VCVTDQ2PS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm1_xmm2() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM1, XMM2
    let code = [
        0xc5, 0xf8, 0x5b, 0xca, // VCVTDQ2PS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm2_xmm3() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM2, XMM3
    let code = [
        0xc5, 0xf8, 0x5b, 0xd3, // VCVTDQ2PS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm3_xmm4() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM3, XMM4
    let code = [
        0xc5, 0xf8, 0x5b, 0xdc, // VCVTDQ2PS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm4_xmm5() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM4, XMM5
    let code = [
        0xc5, 0xf8, 0x5b, 0xe5, // VCVTDQ2PS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm5_xmm6() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM5, XMM6
    let code = [
        0xc5, 0xf8, 0x5b, 0xee, // VCVTDQ2PS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm6_xmm7() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM6, XMM7
    let code = [
        0xc5, 0xf8, 0x5b, 0xf7, // VCVTDQ2PS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm7_xmm0() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM7, XMM0
    let code = [
        0xc5, 0xf8, 0x5b, 0xf8, // VCVTDQ2PS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtdq2ps_xmm8_xmm9() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0xc1, // VCVTDQ2PS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm9_xmm10() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0xca, // VCVTDQ2PS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm10_xmm11() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0xd3, // VCVTDQ2PS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm11_xmm12() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0xdc, // VCVTDQ2PS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm12_xmm13() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0xe5, // VCVTDQ2PS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm13_xmm14() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0xee, // VCVTDQ2PS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm14_xmm15() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0xf7, // VCVTDQ2PS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm15_xmm8() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0xf8, // VCVTDQ2PS XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PS Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vcvtdq2ps_xmm0_xmm8() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM0, XMM8
    let code = [
        0xc4, 0xc1, 0x78, 0x5b, 0xc0, // VCVTDQ2PS XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm8_xmm0() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x78, 0x5b, 0xc0, // VCVTDQ2PS XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_xmm7_xmm15() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM7, XMM15
    let code = [
        0xc4, 0xc1, 0x78, 0x5b, 0xff, // VCVTDQ2PS XMM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PS Tests - 256-bit YMM registers (8x int32 to 8x float32)
// ============================================================================

#[test]
fn test_vcvtdq2ps_ymm0_ymm1() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x5b, 0xc1, // VCVTDQ2PS YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm1_ymm2() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM1, YMM2
    let code = [
        0xc5, 0xfc, 0x5b, 0xca, // VCVTDQ2PS YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm2_ymm3() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM2, YMM3
    let code = [
        0xc5, 0xfc, 0x5b, 0xd3, // VCVTDQ2PS YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm3_ymm4() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM3, YMM4
    let code = [
        0xc5, 0xfc, 0x5b, 0xdc, // VCVTDQ2PS YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm4_ymm5() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM4, YMM5
    let code = [
        0xc5, 0xfc, 0x5b, 0xe5, // VCVTDQ2PS YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm5_ymm6() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM5, YMM6
    let code = [
        0xc5, 0xfc, 0x5b, 0xee, // VCVTDQ2PS YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm6_ymm7() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM6, YMM7
    let code = [
        0xc5, 0xfc, 0x5b, 0xf7, // VCVTDQ2PS YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm7_ymm0() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM7, YMM0
    let code = [
        0xc5, 0xfc, 0x5b, 0xf8, // VCVTDQ2PS YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PS Tests - Extended YMM registers
// ============================================================================

#[test]
fn test_vcvtdq2ps_ymm8_ymm9() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0xc1, // VCVTDQ2PS YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm9_ymm10() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0xca, // VCVTDQ2PS YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm10_ymm11() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0xd3, // VCVTDQ2PS YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm11_ymm12() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0xdc, // VCVTDQ2PS YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm12_ymm13() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0xe5, // VCVTDQ2PS YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm13_ymm14() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0xee, // VCVTDQ2PS YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm14_ymm15() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0xf7, // VCVTDQ2PS YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm15_ymm8() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0xf8, // VCVTDQ2PS YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vcvtdq2ps_xmm0_mem() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTDQ2PS XMM0, [rip + 0x4000]
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
fn test_vcvtdq2ps_xmm8_mem() {
    let mut emu = emu64();
    // VCVTDQ2PS XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x78, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTDQ2PS XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, // -1
        0xfe, 0xff, 0xff, 0xff, // -2
        0x64, 0x00, 0x00, 0x00, // 100
        0x00, 0x04, 0x00, 0x00, // 1024
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTDQ2PS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vcvtdq2ps_ymm0_mem() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTDQ2PS YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x01, 0x00, 0x00, 0x00, // 1
        0x02, 0x00, 0x00, 0x00, // 2
        0x03, 0x00, 0x00, 0x00, // 3
        0x04, 0x00, 0x00, 0x00, // 4
        0x05, 0x00, 0x00, 0x00, // 5
        0x06, 0x00, 0x00, 0x00, // 6
        0x07, 0x00, 0x00, 0x00, // 7
        0x08, 0x00, 0x00, 0x00, // 8
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtdq2ps_ymm8_mem() {
    let mut emu = emu64();
    // VCVTDQ2PS YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7c, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTDQ2PS YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x80, // -2147483648 (INT32_MIN)
        0xff, 0xff, 0xff, 0x7f, // 2147483647 (INT32_MAX)
        0x00, 0x00, 0x00, 0x00, // 0
        0xff, 0xff, 0xff, 0xff, // -1
        0x64, 0x00, 0x00, 0x00, // 100
        0x9c, 0xff, 0xff, 0xff, // -100
        0xe8, 0x03, 0x00, 0x00, // 1000
        0x18, 0xfc, 0xff, 0xff, // -1000
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2DQ Tests - 128-bit XMM registers (4x float32 to 4x int32)
// ============================================================================

#[test]
fn test_vcvtps2dq_xmm0_xmm1() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x5b, 0xc1, // VCVTPS2DQ XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm1_xmm2() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM1, XMM2
    let code = [
        0xc5, 0xf9, 0x5b, 0xca, // VCVTPS2DQ XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm2_xmm3() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM2, XMM3
    let code = [
        0xc5, 0xf9, 0x5b, 0xd3, // VCVTPS2DQ XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm3_xmm4() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM3, XMM4
    let code = [
        0xc5, 0xf9, 0x5b, 0xdc, // VCVTPS2DQ XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm4_xmm5() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM4, XMM5
    let code = [
        0xc5, 0xf9, 0x5b, 0xe5, // VCVTPS2DQ XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm5_xmm6() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM5, XMM6
    let code = [
        0xc5, 0xf9, 0x5b, 0xee, // VCVTPS2DQ XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm6_xmm7() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM6, XMM7
    let code = [
        0xc5, 0xf9, 0x5b, 0xf7, // VCVTPS2DQ XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm7_xmm0() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM7, XMM0
    let code = [
        0xc5, 0xf9, 0x5b, 0xf8, // VCVTPS2DQ XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2DQ Tests - Extended XMM registers
// ============================================================================

#[test]
fn test_vcvtps2dq_xmm8_xmm9() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0xc1, // VCVTPS2DQ XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm9_xmm10() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0xca, // VCVTPS2DQ XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm10_xmm11() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0xd3, // VCVTPS2DQ XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm11_xmm12() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0xdc, // VCVTPS2DQ XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm12_xmm13() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0xe5, // VCVTPS2DQ XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm13_xmm14() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0xee, // VCVTPS2DQ XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm14_xmm15() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0xf7, // VCVTPS2DQ XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm15_xmm8() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0xf8, // VCVTPS2DQ XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2DQ Tests - 256-bit YMM registers
// ============================================================================

#[test]
fn test_vcvtps2dq_ymm0_ymm1() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x5b, 0xc1, // VCVTPS2DQ YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm1_ymm2() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM1, YMM2
    let code = [
        0xc5, 0xfd, 0x5b, 0xca, // VCVTPS2DQ YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm2_ymm3() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM2, YMM3
    let code = [
        0xc5, 0xfd, 0x5b, 0xd3, // VCVTPS2DQ YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm3_ymm4() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM3, YMM4
    let code = [
        0xc5, 0xfd, 0x5b, 0xdc, // VCVTPS2DQ YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm4_ymm5() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM4, YMM5
    let code = [
        0xc5, 0xfd, 0x5b, 0xe5, // VCVTPS2DQ YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm5_ymm6() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM5, YMM6
    let code = [
        0xc5, 0xfd, 0x5b, 0xee, // VCVTPS2DQ YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm6_ymm7() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM6, YMM7
    let code = [
        0xc5, 0xfd, 0x5b, 0xf7, // VCVTPS2DQ YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm7_ymm0() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM7, YMM0
    let code = [
        0xc5, 0xfd, 0x5b, 0xf8, // VCVTPS2DQ YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2DQ Tests - Extended YMM registers
// ============================================================================

#[test]
fn test_vcvtps2dq_ymm8_ymm9() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0xc1, // VCVTPS2DQ YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm9_ymm10() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0xca, // VCVTPS2DQ YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm10_ymm11() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0xd3, // VCVTPS2DQ YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm11_ymm12() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0xdc, // VCVTPS2DQ YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm12_ymm13() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0xe5, // VCVTPS2DQ YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm13_ymm14() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0xee, // VCVTPS2DQ YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm14_ymm15() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0xf7, // VCVTPS2DQ YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm15_ymm8() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0xf8, // VCVTPS2DQ YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2DQ Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vcvtps2dq_xmm0_mem() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPS2DQ XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_xmm8_mem() {
    let mut emu = emu64();
    // VCVTPS2DQ XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x79, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPS2DQ XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0xbf, // -1.0
        0x00, 0x00, 0x00, 0xc0, // -2.0
        0x00, 0x00, 0xc8, 0x42, // 100.0
        0x00, 0x00, 0xc8, 0xc2, // -100.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTPS2DQ Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vcvtps2dq_ymm0_mem() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPS2DQ YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
        0x00, 0x00, 0xa0, 0x40, // 5.0
        0x00, 0x00, 0xc0, 0x40, // 6.0
        0x00, 0x00, 0xe0, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x41, // 8.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtps2dq_ymm8_mem() {
    let mut emu = emu64();
    // VCVTPS2DQ YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7d, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTPS2DQ YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x80, 0xbf, // -1.0
        0x00, 0x00, 0xc8, 0x42, // 100.0
        0x00, 0x00, 0xc8, 0xc2, // -100.0
        0x00, 0x00, 0x7a, 0x44, // 1000.0
        0x00, 0x00, 0x7a, 0xc4, // -1000.0
        0x9a, 0x99, 0x19, 0x3f, // 0.6
        0x33, 0x33, 0xb3, 0x3f, // 1.4
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
