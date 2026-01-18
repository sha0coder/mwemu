use crate::*;

// VRSQRTPS - Compute Reciprocals of Square Roots of Packed Single-Precision Floating-Point Values
//
// VRSQRTPS computes the approximate reciprocal of the square root of packed single-precision
// floating-point values in the source operand and stores the results in the destination operand.
//
// The reciprocal square root approximation has a maximum relative error of less than 1.5 * 2^-12.
// For more accurate results, software should use VRSQRTPS as a starting point and perform
// additional Newton-Raphson iterations.
//
// Formula: dst[i] = APPROXIMATE(1.0 / SQRT(src[i]))
//
// Special cases:
// - RSQRT(+0.0) = +∞
// - RSQRT(-0.0) = -∞
// - RSQRT(+∞) = +0.0
// - RSQRT(x < 0) = NaN
// - RSQRT(NaN) = NaN
//
// Opcodes:
// VEX.128.0F.WIG 52 /r    VRSQRTPS xmm1, xmm2/m128   - Reciprocal sqrt of 4x float32
// VEX.256.0F.WIG 52 /r    VRSQRTPS ymm1, ymm2/m256   - Reciprocal sqrt of 8x float32

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VRSQRTPS Tests - 128-bit XMM registers (4x float32)
// ============================================================================

#[test]
fn test_vrsqrtps_xmm0_xmm1() {
    let mut emu = emu64();
    // VRSQRTPS XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x52, 0xc1, // VRSQRTPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm1_xmm2() {
    let mut emu = emu64();
    // VRSQRTPS XMM1, XMM2
    let code = [
        0xc5, 0xf8, 0x52, 0xca, // VRSQRTPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm2_xmm3() {
    let mut emu = emu64();
    // VRSQRTPS XMM2, XMM3
    let code = [
        0xc5, 0xf8, 0x52, 0xd3, // VRSQRTPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm3_xmm4() {
    let mut emu = emu64();
    // VRSQRTPS XMM3, XMM4
    let code = [
        0xc5, 0xf8, 0x52, 0xdc, // VRSQRTPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm4_xmm5() {
    let mut emu = emu64();
    // VRSQRTPS XMM4, XMM5
    let code = [
        0xc5, 0xf8, 0x52, 0xe5, // VRSQRTPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm5_xmm6() {
    let mut emu = emu64();
    // VRSQRTPS XMM5, XMM6
    let code = [
        0xc5, 0xf8, 0x52, 0xee, // VRSQRTPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm6_xmm7() {
    let mut emu = emu64();
    // VRSQRTPS XMM6, XMM7
    let code = [
        0xc5, 0xf8, 0x52, 0xf7, // VRSQRTPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm7_xmm0() {
    let mut emu = emu64();
    // VRSQRTPS XMM7, XMM0
    let code = [
        0xc5, 0xf8, 0x52, 0xf8, // VRSQRTPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRSQRTPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vrsqrtps_xmm8_xmm9() {
    let mut emu = emu64();
    // VRSQRTPS XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xc1, // VRSQRTPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm9_xmm10() {
    let mut emu = emu64();
    // VRSQRTPS XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xca, // VRSQRTPS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm10_xmm11() {
    let mut emu = emu64();
    // VRSQRTPS XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xd3, // VRSQRTPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm11_xmm12() {
    let mut emu = emu64();
    // VRSQRTPS XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xdc, // VRSQRTPS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm12_xmm13() {
    let mut emu = emu64();
    // VRSQRTPS XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xe5, // VRSQRTPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm13_xmm14() {
    let mut emu = emu64();
    // VRSQRTPS XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xee, // VRSQRTPS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm14_xmm15() {
    let mut emu = emu64();
    // VRSQRTPS XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xf7, // VRSQRTPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm15_xmm8() {
    let mut emu = emu64();
    // VRSQRTPS XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xf8, // VRSQRTPS XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRSQRTPS Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vrsqrtps_xmm0_xmm8() {
    let mut emu = emu64();
    // VRSQRTPS XMM0, XMM8
    let code = [
        0xc4, 0xc1, 0x78, 0x52, 0xc0, // VRSQRTPS XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm8_xmm0() {
    let mut emu = emu64();
    // VRSQRTPS XMM8, XMM0
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xc0, // VRSQRTPS XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm7_xmm15() {
    let mut emu = emu64();
    // VRSQRTPS XMM7, XMM15
    let code = [
        0xc4, 0xc1, 0x78, 0x52, 0xff, // VRSQRTPS XMM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm15_xmm7() {
    let mut emu = emu64();
    // VRSQRTPS XMM15, XMM7
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0xff, // VRSQRTPS XMM15, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRSQRTPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vrsqrtps_xmm0_mem() {
    let mut emu = emu64();
    // VRSQRTPS XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x52, 0x05, 0x00, 0x40, 0x00, 0x00, // VRSQRTPS XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0 -> rsqrt ~1.0
        0x00, 0x00, 0x80, 0x40, // 4.0 -> rsqrt ~0.5
        0x00, 0x00, 0x10, 0x41, // 9.0 -> rsqrt ~0.333
        0x00, 0x00, 0x80, 0x41, // 16.0 -> rsqrt ~0.25
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm1_mem() {
    let mut emu = emu64();
    // VRSQRTPS XMM1, [mem]
    let code = [
        0xc5, 0xf8, 0x52, 0x0d, 0x00, 0x40, 0x00, 0x00, // VRSQRTPS XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // rsqrt gives: 2.0, 4.0, 8.0, 16.0
    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3e, // 0.25 -> rsqrt ~2.0
        0x00, 0x00, 0x80, 0x3d, // 0.0625 -> rsqrt ~4.0
        0x00, 0x00, 0x80, 0x3c, // 0.015625 -> rsqrt ~8.0
        0x00, 0x00, 0x80, 0x3b, // 0.00390625 -> rsqrt ~16.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm2_mem() {
    let mut emu = emu64();
    // VRSQRTPS XMM2, [mem]
    let code = [
        0xc5, 0xf8, 0x52, 0x15, 0x00, 0x40, 0x00, 0x00, // VRSQRTPS XMM2, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xc8, 0x42, // 100.0 -> rsqrt ~0.1
        0x00, 0x00, 0xc8, 0x41, // 25.0 -> rsqrt ~0.2
        0x00, 0x00, 0x00, 0x40, // 2.0 -> rsqrt ~0.707
        0x00, 0x00, 0x00, 0x3f, // 0.5 -> rsqrt ~1.414
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_xmm8_mem() {
    let mut emu = emu64();
    // VRSQRTPS XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x78, 0x52, 0x05, 0x00, 0x40, 0x00, 0x00, // VRSQRTPS XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x80, 0x3f, // 1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VRSQRTPS Tests - 256-bit YMM registers (8x float32)
// ============================================================================

#[test]
fn test_vrsqrtps_ymm0_ymm1() {
    let mut emu = emu64();
    // VRSQRTPS YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x52, 0xc1, // VRSQRTPS YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm1_ymm2() {
    let mut emu = emu64();
    // VRSQRTPS YMM1, YMM2
    let code = [
        0xc5, 0xfc, 0x52, 0xca, // VRSQRTPS YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm2_ymm3() {
    let mut emu = emu64();
    // VRSQRTPS YMM2, YMM3
    let code = [
        0xc5, 0xfc, 0x52, 0xd3, // VRSQRTPS YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm3_ymm4() {
    let mut emu = emu64();
    // VRSQRTPS YMM3, YMM4
    let code = [
        0xc5, 0xfc, 0x52, 0xdc, // VRSQRTPS YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm4_ymm5() {
    let mut emu = emu64();
    // VRSQRTPS YMM4, YMM5
    let code = [
        0xc5, 0xfc, 0x52, 0xe5, // VRSQRTPS YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm5_ymm6() {
    let mut emu = emu64();
    // VRSQRTPS YMM5, YMM6
    let code = [
        0xc5, 0xfc, 0x52, 0xee, // VRSQRTPS YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm6_ymm7() {
    let mut emu = emu64();
    // VRSQRTPS YMM6, YMM7
    let code = [
        0xc5, 0xfc, 0x52, 0xf7, // VRSQRTPS YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm7_ymm0() {
    let mut emu = emu64();
    // VRSQRTPS YMM7, YMM0
    let code = [
        0xc5, 0xfc, 0x52, 0xf8, // VRSQRTPS YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRSQRTPS Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vrsqrtps_ymm8_ymm9() {
    let mut emu = emu64();
    // VRSQRTPS YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xc1, // VRSQRTPS YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm9_ymm10() {
    let mut emu = emu64();
    // VRSQRTPS YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xca, // VRSQRTPS YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm10_ymm11() {
    let mut emu = emu64();
    // VRSQRTPS YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xd3, // VRSQRTPS YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm11_ymm12() {
    let mut emu = emu64();
    // VRSQRTPS YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xdc, // VRSQRTPS YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm12_ymm13() {
    let mut emu = emu64();
    // VRSQRTPS YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xe5, // VRSQRTPS YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm13_ymm14() {
    let mut emu = emu64();
    // VRSQRTPS YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xee, // VRSQRTPS YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm14_ymm15() {
    let mut emu = emu64();
    // VRSQRTPS YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xf7, // VRSQRTPS YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm15_ymm8() {
    let mut emu = emu64();
    // VRSQRTPS YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xf8, // VRSQRTPS YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRSQRTPS Tests - Cross-domain YMM registers
// ============================================================================

#[test]
fn test_vrsqrtps_ymm0_ymm8() {
    let mut emu = emu64();
    // VRSQRTPS YMM0, YMM8
    let code = [
        0xc4, 0xc1, 0x7c, 0x52, 0xc0, // VRSQRTPS YMM0, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm8_ymm0() {
    let mut emu = emu64();
    // VRSQRTPS YMM8, YMM0
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0xc0, // VRSQRTPS YMM8, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm7_ymm15() {
    let mut emu = emu64();
    // VRSQRTPS YMM7, YMM15
    let code = [
        0xc4, 0xc1, 0x7c, 0x52, 0xff, // VRSQRTPS YMM7, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRSQRTPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vrsqrtps_ymm0_mem() {
    let mut emu = emu64();
    // VRSQRTPS YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x52, 0x05, 0x00, 0x40, 0x00, 0x00, // VRSQRTPS YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0 -> rsqrt ~1.0
        0x00, 0x00, 0x80, 0x40, // 4.0 -> rsqrt ~0.5
        0x00, 0x00, 0x10, 0x41, // 9.0 -> rsqrt ~0.333
        0x00, 0x00, 0x80, 0x41, // 16.0 -> rsqrt ~0.25
        0x00, 0x00, 0xc8, 0x41, // 25.0 -> rsqrt ~0.2
        0x00, 0x00, 0x10, 0x42, // 36.0 -> rsqrt ~0.167
        0x00, 0x00, 0x44, 0x42, // 49.0 -> rsqrt ~0.143
        0x00, 0x00, 0x80, 0x42, // 64.0 -> rsqrt ~0.125
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm1_mem() {
    let mut emu = emu64();
    // VRSQRTPS YMM1, [mem]
    let code = [
        0xc5, 0xfc, 0x52, 0x0d, 0x00, 0x40, 0x00, 0x00, // VRSQRTPS YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3e, // 0.25
        0x00, 0x00, 0x80, 0x3d, // 0.0625
        0x00, 0x00, 0x80, 0x3c, // 0.015625
        0x00, 0x00, 0x80, 0x3b, // 0.00390625
        0x00, 0x00, 0x80, 0x3a, // 0.0009765625
        0x00, 0x00, 0x80, 0x39, // 0.000244140625
        0x00, 0x00, 0x80, 0x38, // 6.103515625e-05
        0x00, 0x00, 0x80, 0x37, // 1.52587890625e-05
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_ymm8_mem() {
    let mut emu = emu64();
    // VRSQRTPS YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7c, 0x52, 0x05, 0x00, 0x40, 0x00, 0x00, // VRSQRTPS YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f]; // All 1.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VRSQRTPS Tests - Special cases and edge conditions
// ============================================================================

#[test]
fn test_vrsqrtps_self() {
    let mut emu = emu64();
    // VRSQRTPS XMM0, XMM0 (result overwrites source)
    let code = [
        0xc5, 0xf8, 0x52, 0xc0, // VRSQRTPS XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_multiple_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0x52, 0xc1, // VRSQRTPS XMM0, XMM1
        0xc5, 0xf8, 0x52, 0xca, // VRSQRTPS XMM1, XMM2
        0xc5, 0xf8, 0x52, 0xd3, // VRSQRTPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_with_multiply() {
    let mut emu = emu64();
    // VRSQRTPS followed by multiply (normalization pattern)
    let code = [
        0xc5, 0xf8, 0x52, 0xc1, // VRSQRTPS XMM0, XMM1
        0xc5, 0xf0, 0x59, 0xc1, // VMULPS XMM0, XMM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_newton_raphson_setup() {
    let mut emu = emu64();
    // x1 = 0.5 * x0 * (3 - d * x0 * x0)
    let code = [
        0xc5, 0xf8, 0x52, 0xc1, // VRSQRTPS XMM0, XMM1 (initial approximation)
        0xc5, 0xf8, 0x59, 0xc0, // VMULPS XMM0, XMM0, XMM0 (square the approximation)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrsqrtps_normalization_pattern() {
    let mut emu = emu64();
    // rsqrt(dot_product) gives 1/length
    let code = [
        0xc5, 0xf8, 0x52, 0xc1, // VRSQRTPS XMM0, XMM1
        0xc5, 0xf8, 0x59, 0xc9, // VMULPS XMM1, XMM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
