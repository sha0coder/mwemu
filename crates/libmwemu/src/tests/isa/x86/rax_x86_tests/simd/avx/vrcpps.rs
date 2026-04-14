use crate::*;

// VRCPPS - Compute Reciprocals of Packed Single-Precision Floating-Point Values
//
// VRCPPS computes the approximate reciprocal of packed single-precision floating-point
// values in the source operand and stores the results in the destination operand.
//
// The reciprocal approximation has a maximum relative error of less than 1.5 * 2^-12.
// For more accurate results, software should use VRCPPS as a starting point and
// perform additional Newton-Raphson iterations.
//
// Formula: dst[i] = APPROXIMATE(1.0 / src[i])
//
// Special cases:
// - 1.0 / ±0.0 = ±∞
// - 1.0 / ±∞ = ±0.0
// - 1.0 / NaN = NaN
//
// Opcodes:
// VEX.128.0F.WIG 53 /r    VRCPPS xmm1, xmm2/m128   - Reciprocal of 4x float32
// VEX.256.0F.WIG 53 /r    VRCPPS ymm1, ymm2/m256   - Reciprocal of 8x float32

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VRCPPS Tests - 128-bit XMM registers (4x float32)
// ============================================================================

#[test]
fn test_vrcpps_xmm0_xmm1() {
    let mut emu = emu64();
    // VRCPPS XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x53, 0xc1, // VRCPPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm1_xmm2() {
    let mut emu = emu64();
    // VRCPPS XMM1, XMM2
    let code = [
        0xc5, 0xf8, 0x53, 0xca, // VRCPPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm2_xmm3() {
    let mut emu = emu64();
    // VRCPPS XMM2, XMM3
    let code = [
        0xc5, 0xf8, 0x53, 0xd3, // VRCPPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm3_xmm4() {
    let mut emu = emu64();
    // VRCPPS XMM3, XMM4
    let code = [
        0xc5, 0xf8, 0x53, 0xdc, // VRCPPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm4_xmm5() {
    let mut emu = emu64();
    // VRCPPS XMM4, XMM5
    let code = [
        0xc5, 0xf8, 0x53, 0xe5, // VRCPPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm5_xmm6() {
    let mut emu = emu64();
    // VRCPPS XMM5, XMM6
    let code = [
        0xc5, 0xf8, 0x53, 0xee, // VRCPPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm6_xmm7() {
    let mut emu = emu64();
    // VRCPPS XMM6, XMM7
    let code = [
        0xc5, 0xf8, 0x53, 0xf7, // VRCPPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm7_xmm0() {
    let mut emu = emu64();
    // VRCPPS XMM7, XMM0
    let code = [
        0xc5, 0xf8, 0x53, 0xf8, // VRCPPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRCPPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vrcpps_xmm8_xmm9() {
    let mut emu = emu64();
    // VRCPPS XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xc1, // VRCPPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm9_xmm10() {
    let mut emu = emu64();
    // VRCPPS XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xca, // VRCPPS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm10_xmm11() {
    let mut emu = emu64();
    // VRCPPS XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xd3, // VRCPPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm11_xmm12() {
    let mut emu = emu64();
    // VRCPPS XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xdc, // VRCPPS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm12_xmm13() {
    let mut emu = emu64();
    // VRCPPS XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xe5, // VRCPPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm13_xmm14() {
    let mut emu = emu64();
    // VRCPPS XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xee, // VRCPPS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm14_xmm15() {
    let mut emu = emu64();
    // VRCPPS XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xf7, // VRCPPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm15_xmm8() {
    let mut emu = emu64();
    // VRCPPS XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xf8, // VRCPPS XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRCPPS Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vrcpps_xmm0_xmm8() {
    let mut emu = emu64();
    // VRCPPS XMM0, XMM8
    let code = [
        0xc4, 0xc1, 0x78, 0x53, 0xc0, // VRCPPS XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm8_xmm0() {
    let mut emu = emu64();
    // VRCPPS XMM8, XMM0
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xc0, // VRCPPS XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm7_xmm15() {
    let mut emu = emu64();
    // VRCPPS XMM7, XMM15
    let code = [
        0xc4, 0xc1, 0x78, 0x53, 0xff, // VRCPPS XMM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm15_xmm7() {
    let mut emu = emu64();
    // VRCPPS XMM15, XMM7
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0xff, // VRCPPS XMM15, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRCPPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vrcpps_xmm0_mem() {
    let mut emu = emu64();
    // VRCPPS XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x53, 0x05, 0x00, 0x40, 0x00, 0x00, // VRCPPS XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0 -> reciprocal ~1.0
        0x00, 0x00, 0x00, 0x40, // 2.0 -> reciprocal ~0.5
        0x00, 0x00, 0x80, 0x40, // 4.0 -> reciprocal ~0.25
        0x00, 0x00, 0x00, 0x41, // 8.0 -> reciprocal ~0.125
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm1_mem() {
    let mut emu = emu64();
    // VRCPPS XMM1, [mem]
    let code = [
        0xc5, 0xf8, 0x53, 0x0d, 0x00, 0x40, 0x00, 0x00, // VRCPPS XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x3f, // 0.5 -> reciprocal ~2.0
        0x00, 0x00, 0x80, 0x3e, // 0.25 -> reciprocal ~4.0
        0x00, 0x00, 0x00, 0x3e, // 0.125 -> reciprocal ~8.0
        0x00, 0x00, 0x80, 0x3d, // 0.0625 -> reciprocal ~16.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm2_mem() {
    let mut emu = emu64();
    // VRCPPS XMM2, [mem]
    let code = [
        0xc5, 0xf8, 0x53, 0x15, 0x00, 0x40, 0x00, 0x00, // VRCPPS XMM2, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x20, 0x41, // 10.0 -> reciprocal ~0.1
        0x00, 0x00, 0xc8, 0x42, // 100.0 -> reciprocal ~0.01
        0x00, 0x00, 0x7a, 0x44, // 1000.0 -> reciprocal ~0.001
        0xcd, 0xcc, 0xcc, 0x3d, // 0.1 -> reciprocal ~10.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_xmm8_mem() {
    let mut emu = emu64();
    // VRCPPS XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x78, 0x53, 0x05, 0x00, 0x40, 0x00, 0x00, // VRCPPS XMM8, [rip + 0x4000]
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
// VRCPPS Tests - 256-bit YMM registers (8x float32)
// ============================================================================

#[test]
fn test_vrcpps_ymm0_ymm1() {
    let mut emu = emu64();
    // VRCPPS YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x53, 0xc1, // VRCPPS YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm1_ymm2() {
    let mut emu = emu64();
    // VRCPPS YMM1, YMM2
    let code = [
        0xc5, 0xfc, 0x53, 0xca, // VRCPPS YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm2_ymm3() {
    let mut emu = emu64();
    // VRCPPS YMM2, YMM3
    let code = [
        0xc5, 0xfc, 0x53, 0xd3, // VRCPPS YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm3_ymm4() {
    let mut emu = emu64();
    // VRCPPS YMM3, YMM4
    let code = [
        0xc5, 0xfc, 0x53, 0xdc, // VRCPPS YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm4_ymm5() {
    let mut emu = emu64();
    // VRCPPS YMM4, YMM5
    let code = [
        0xc5, 0xfc, 0x53, 0xe5, // VRCPPS YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm5_ymm6() {
    let mut emu = emu64();
    // VRCPPS YMM5, YMM6
    let code = [
        0xc5, 0xfc, 0x53, 0xee, // VRCPPS YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm6_ymm7() {
    let mut emu = emu64();
    // VRCPPS YMM6, YMM7
    let code = [
        0xc5, 0xfc, 0x53, 0xf7, // VRCPPS YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm7_ymm0() {
    let mut emu = emu64();
    // VRCPPS YMM7, YMM0
    let code = [
        0xc5, 0xfc, 0x53, 0xf8, // VRCPPS YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRCPPS Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vrcpps_ymm8_ymm9() {
    let mut emu = emu64();
    // VRCPPS YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xc1, // VRCPPS YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm9_ymm10() {
    let mut emu = emu64();
    // VRCPPS YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xca, // VRCPPS YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm10_ymm11() {
    let mut emu = emu64();
    // VRCPPS YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xd3, // VRCPPS YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm11_ymm12() {
    let mut emu = emu64();
    // VRCPPS YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xdc, // VRCPPS YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm12_ymm13() {
    let mut emu = emu64();
    // VRCPPS YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xe5, // VRCPPS YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm13_ymm14() {
    let mut emu = emu64();
    // VRCPPS YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xee, // VRCPPS YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm14_ymm15() {
    let mut emu = emu64();
    // VRCPPS YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xf7, // VRCPPS YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm15_ymm8() {
    let mut emu = emu64();
    // VRCPPS YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xf8, // VRCPPS YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRCPPS Tests - Cross-domain YMM registers
// ============================================================================

#[test]
fn test_vrcpps_ymm0_ymm8() {
    let mut emu = emu64();
    // VRCPPS YMM0, YMM8
    let code = [
        0xc4, 0xc1, 0x7c, 0x53, 0xc0, // VRCPPS YMM0, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm8_ymm0() {
    let mut emu = emu64();
    // VRCPPS YMM8, YMM0
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0xc0, // VRCPPS YMM8, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm7_ymm15() {
    let mut emu = emu64();
    // VRCPPS YMM7, YMM15
    let code = [
        0xc4, 0xc1, 0x7c, 0x53, 0xff, // VRCPPS YMM7, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VRCPPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vrcpps_ymm0_mem() {
    let mut emu = emu64();
    // VRCPPS YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x53, 0x05, 0x00, 0x40, 0x00, 0x00, // VRCPPS YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x80, 0x40, // 4.0
        0x00, 0x00, 0x00, 0x41, // 8.0
        0x00, 0x00, 0x80, 0x41, // 16.0
        0x00, 0x00, 0x00, 0x42, // 32.0
        0x00, 0x00, 0x80, 0x42, // 64.0
        0x00, 0x00, 0x00, 0x43, // 128.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm1_mem() {
    let mut emu = emu64();
    // VRCPPS YMM1, [mem]
    let code = [
        0xc5, 0xfc, 0x53, 0x0d, 0x00, 0x40, 0x00, 0x00, // VRCPPS YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x3f, // 0.5
        0x00, 0x00, 0x80, 0x3e, // 0.25
        0x00, 0x00, 0x00, 0x3e, // 0.125
        0x00, 0x00, 0x80, 0x3d, // 0.0625
        0x00, 0x00, 0x00, 0x3d, // 0.03125
        0x00, 0x00, 0x80, 0x3c, // 0.015625
        0x00, 0x00, 0x00, 0x3c, // 0.0078125
        0x00, 0x00, 0x80, 0x3b, // 0.00390625
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_ymm8_mem() {
    let mut emu = emu64();
    // VRCPPS YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7c, 0x53, 0x05, 0x00, 0x40, 0x00, 0x00, // VRCPPS YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f]; // All 1.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VRCPPS Tests - Special cases and edge conditions
// ============================================================================

#[test]
fn test_vrcpps_self_reciprocal() {
    let mut emu = emu64();
    // VRCPPS XMM0, XMM0 (result overwrites source)
    let code = [
        0xc5, 0xf8, 0x53, 0xc0, // VRCPPS XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_multiple_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0x53, 0xc1, // VRCPPS XMM0, XMM1
        0xc5, 0xf8, 0x53, 0xca, // VRCPPS XMM1, XMM2
        0xc5, 0xf8, 0x53, 0xd3, // VRCPPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_double_reciprocal() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0x53, 0xc1, // VRCPPS XMM0, XMM1
        0xc5, 0xf8, 0x53, 0xc0, // VRCPPS XMM0, XMM0 (second reciprocal)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_with_multiply() {
    let mut emu = emu64();
    // VRCPPS followed by multiply (x * rcp(y) approximates x/y)
    let code = [
        0xc5, 0xf8, 0x53, 0xc2, // VRCPPS XMM0, XMM2
        0xc5, 0xf0, 0x59, 0xc0, // VMULPS XMM0, XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vrcpps_newton_raphson_iteration() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0x53, 0xc1, // VRCPPS XMM0, XMM1 (initial approximation)
        0xc5, 0xf0, 0x59, 0xc9, // VMULPS XMM1, XMM1, XMM1 (save original)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
