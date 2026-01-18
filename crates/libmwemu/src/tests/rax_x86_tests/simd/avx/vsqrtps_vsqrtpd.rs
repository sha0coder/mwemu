use crate::*;

// VSQRTPS - Square Root of Packed Single-Precision Floating-Point Values
// VSQRTPD - Square Root of Packed Double-Precision Floating-Point Values
//
// VSQRTPS performs element-wise square root of packed single-precision floating-point values.
// VSQRTPD performs element-wise square root of packed double-precision floating-point values.
//
// Opcodes:
// VEX.128.0F.WIG 51 /r    VSQRTPS xmm1, xmm2/m128   - Square root of packed single from xmm2/mem
// VEX.256.0F.WIG 51 /r    VSQRTPS ymm1, ymm2/m256   - Square root of packed single from ymm2/mem
// VEX.128.66.0F.WIG 51 /r VSQRTPD xmm1, xmm2/m128   - Square root of packed double from xmm2/mem
// VEX.256.66.0F.WIG 51 /r VSQRTPD ymm1, ymm2/m256   - Square root of packed double from ymm2/mem

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VSQRTPS Tests - 128-bit XMM registers (4x float32)
// ============================================================================

#[test]
fn test_vsqrtps_xmm0_xmm1() {
    let mut emu = emu64();
    // VSQRTPS XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x51, 0xc1, // VSQRTPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm1_xmm2() {
    let mut emu = emu64();
    // VSQRTPS XMM1, XMM2
    let code = [
        0xc5, 0xf8, 0x51, 0xca, // VSQRTPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm2_xmm3() {
    let mut emu = emu64();
    // VSQRTPS XMM2, XMM3
    let code = [
        0xc5, 0xf8, 0x51, 0xd3, // VSQRTPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm3_xmm4() {
    let mut emu = emu64();
    // VSQRTPS XMM3, XMM4
    let code = [
        0xc5, 0xf8, 0x51, 0xdc, // VSQRTPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm4_xmm5() {
    let mut emu = emu64();
    // VSQRTPS XMM4, XMM5
    let code = [
        0xc5, 0xf8, 0x51, 0xe5, // VSQRTPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm5_xmm6() {
    let mut emu = emu64();
    // VSQRTPS XMM5, XMM6
    let code = [
        0xc5, 0xf8, 0x51, 0xee, // VSQRTPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm6_xmm7() {
    let mut emu = emu64();
    // VSQRTPS XMM6, XMM7
    let code = [
        0xc5, 0xf8, 0x51, 0xf7, // VSQRTPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm7_xmm0() {
    let mut emu = emu64();
    // VSQRTPS XMM7, XMM0
    let code = [
        0xc5, 0xf8, 0x51, 0xf8, // VSQRTPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vsqrtps_xmm8_xmm9() {
    let mut emu = emu64();
    // VSQRTPS XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0xc1, // VSQRTPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm9_xmm10() {
    let mut emu = emu64();
    // VSQRTPS XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0xca, // VSQRTPS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm10_xmm11() {
    let mut emu = emu64();
    // VSQRTPS XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0xd3, // VSQRTPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm11_xmm12() {
    let mut emu = emu64();
    // VSQRTPS XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0xdc, // VSQRTPS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm12_xmm13() {
    let mut emu = emu64();
    // VSQRTPS XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0xe5, // VSQRTPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm13_xmm14() {
    let mut emu = emu64();
    // VSQRTPS XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0xee, // VSQRTPS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm14_xmm15() {
    let mut emu = emu64();
    // VSQRTPS XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0xf7, // VSQRTPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm15_xmm8() {
    let mut emu = emu64();
    // VSQRTPS XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0xf8, // VSQRTPS XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPS Tests - Cross-domain (mixing low and high XMM registers)
// ============================================================================

#[test]
fn test_vsqrtps_xmm0_xmm8() {
    let mut emu = emu64();
    // VSQRTPS XMM0, XMM8
    let code = [
        0xc4, 0xc1, 0x78, 0x51, 0xc0, // VSQRTPS XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm8_xmm0() {
    let mut emu = emu64();
    // VSQRTPS XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x78, 0x51, 0xc0, // VSQRTPS XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm7_xmm15() {
    let mut emu = emu64();
    // VSQRTPS XMM7, XMM15
    let code = [
        0xc4, 0xc1, 0x78, 0x51, 0xff, // VSQRTPS XMM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPS Tests - 256-bit YMM registers (8x float32)
// ============================================================================

#[test]
fn test_vsqrtps_ymm0_ymm1() {
    let mut emu = emu64();
    // VSQRTPS YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x51, 0xc1, // VSQRTPS YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm1_ymm2() {
    let mut emu = emu64();
    // VSQRTPS YMM1, YMM2
    let code = [
        0xc5, 0xfc, 0x51, 0xca, // VSQRTPS YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm2_ymm3() {
    let mut emu = emu64();
    // VSQRTPS YMM2, YMM3
    let code = [
        0xc5, 0xfc, 0x51, 0xd3, // VSQRTPS YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm3_ymm4() {
    let mut emu = emu64();
    // VSQRTPS YMM3, YMM4
    let code = [
        0xc5, 0xfc, 0x51, 0xdc, // VSQRTPS YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm4_ymm5() {
    let mut emu = emu64();
    // VSQRTPS YMM4, YMM5
    let code = [
        0xc5, 0xfc, 0x51, 0xe5, // VSQRTPS YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm5_ymm6() {
    let mut emu = emu64();
    // VSQRTPS YMM5, YMM6
    let code = [
        0xc5, 0xfc, 0x51, 0xee, // VSQRTPS YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm6_ymm7() {
    let mut emu = emu64();
    // VSQRTPS YMM6, YMM7
    let code = [
        0xc5, 0xfc, 0x51, 0xf7, // VSQRTPS YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm7_ymm0() {
    let mut emu = emu64();
    // VSQRTPS YMM7, YMM0
    let code = [
        0xc5, 0xfc, 0x51, 0xf8, // VSQRTPS YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPS Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vsqrtps_ymm8_ymm9() {
    let mut emu = emu64();
    // VSQRTPS YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0xc1, // VSQRTPS YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm9_ymm10() {
    let mut emu = emu64();
    // VSQRTPS YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0xca, // VSQRTPS YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm10_ymm11() {
    let mut emu = emu64();
    // VSQRTPS YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0xd3, // VSQRTPS YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm11_ymm12() {
    let mut emu = emu64();
    // VSQRTPS YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0xdc, // VSQRTPS YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm12_ymm13() {
    let mut emu = emu64();
    // VSQRTPS YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0xe5, // VSQRTPS YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm13_ymm14() {
    let mut emu = emu64();
    // VSQRTPS YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0xee, // VSQRTPS YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm14_ymm15() {
    let mut emu = emu64();
    // VSQRTPS YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0xf7, // VSQRTPS YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm15_ymm8() {
    let mut emu = emu64();
    // VSQRTPS YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0xf8, // VSQRTPS YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPS Tests - Cross-domain YMM registers
// ============================================================================

#[test]
fn test_vsqrtps_ymm0_ymm15() {
    let mut emu = emu64();
    // VSQRTPS YMM0, YMM15
    let code = [
        0xc4, 0xc1, 0x7c, 0x51, 0xc7, // VSQRTPS YMM0, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm15_ymm0() {
    let mut emu = emu64();
    // VSQRTPS YMM15, YMM0
    let code = [
        0xc4, 0xc1, 0x7c, 0x51, 0xf8, // VSQRTPS YMM15, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vsqrtps_xmm0_mem() {
    let mut emu = emu64();
    // VSQRTPS XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x51, 0x05, 0x00, 0x40, 0x00, 0x00, // VSQRTPS XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x80, 0x40, // 4.0
        0x00, 0x00, 0x10, 0x41, // 9.0
        0x00, 0x00, 0x80, 0x41, // 16.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_xmm8_mem() {
    let mut emu = emu64();
    // VSQRTPS XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x78, 0x51, 0x05, 0x00, 0x40, 0x00, 0x00, // VSQRTPS XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xc8, 0x41, // 25.0
        0x00, 0x00, 0x10, 0x42, // 36.0
        0x00, 0x00, 0x3d, 0x42, // 49.0
        0x00, 0x00, 0x80, 0x42, // 64.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vsqrtps_ymm0_mem() {
    let mut emu = emu64();
    // VSQRTPS YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x51, 0x05, 0x00, 0x40, 0x00, 0x00, // VSQRTPS YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x80, 0x40, // 4.0
        0x00, 0x00, 0x10, 0x41, // 9.0
        0x00, 0x00, 0x80, 0x41, // 16.0
        0x00, 0x00, 0xc8, 0x41, // 25.0
        0x00, 0x00, 0x10, 0x42, // 36.0
        0x00, 0x00, 0x3d, 0x42, // 49.0
        0x00, 0x00, 0x80, 0x42, // 64.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtps_ymm8_mem() {
    let mut emu = emu64();
    // VSQRTPS YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7c, 0x51, 0x05, 0x00, 0x40, 0x00, 0x00, // VSQRTPS YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0xa1, 0x42, // 81.0
        0x00, 0x00, 0xc8, 0x42, // 100.0
        0x00, 0x00, 0xf1, 0x42, // 121.0
        0x00, 0x00, 0x10, 0x43, // 144.0
        0x00, 0x00, 0x29, 0x43, // 169.0
        0x00, 0x00, 0x44, 0x43, // 196.0
        0x00, 0x00, 0x5f, 0x43, // 225.0
        0x00, 0x80, 0x7f, 0x43, // 256.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPD Tests - 128-bit XMM registers (2x float64)
// ============================================================================

#[test]
fn test_vsqrtpd_xmm0_xmm1() {
    let mut emu = emu64();
    // VSQRTPD XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x51, 0xc1, // VSQRTPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm1_xmm2() {
    let mut emu = emu64();
    // VSQRTPD XMM1, XMM2
    let code = [
        0xc5, 0xf9, 0x51, 0xca, // VSQRTPD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm2_xmm3() {
    let mut emu = emu64();
    // VSQRTPD XMM2, XMM3
    let code = [
        0xc5, 0xf9, 0x51, 0xd3, // VSQRTPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm3_xmm4() {
    let mut emu = emu64();
    // VSQRTPD XMM3, XMM4
    let code = [
        0xc5, 0xf9, 0x51, 0xdc, // VSQRTPD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm4_xmm5() {
    let mut emu = emu64();
    // VSQRTPD XMM4, XMM5
    let code = [
        0xc5, 0xf9, 0x51, 0xe5, // VSQRTPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm5_xmm6() {
    let mut emu = emu64();
    // VSQRTPD XMM5, XMM6
    let code = [
        0xc5, 0xf9, 0x51, 0xee, // VSQRTPD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm6_xmm7() {
    let mut emu = emu64();
    // VSQRTPD XMM6, XMM7
    let code = [
        0xc5, 0xf9, 0x51, 0xf7, // VSQRTPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm7_xmm0() {
    let mut emu = emu64();
    // VSQRTPD XMM7, XMM0
    let code = [
        0xc5, 0xf9, 0x51, 0xf8, // VSQRTPD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vsqrtpd_xmm8_xmm9() {
    let mut emu = emu64();
    // VSQRTPD XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0xc1, // VSQRTPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm9_xmm10() {
    let mut emu = emu64();
    // VSQRTPD XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0xca, // VSQRTPD XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm10_xmm11() {
    let mut emu = emu64();
    // VSQRTPD XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0xd3, // VSQRTPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm11_xmm12() {
    let mut emu = emu64();
    // VSQRTPD XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0xdc, // VSQRTPD XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm12_xmm13() {
    let mut emu = emu64();
    // VSQRTPD XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0xe5, // VSQRTPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm13_xmm14() {
    let mut emu = emu64();
    // VSQRTPD XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0xee, // VSQRTPD XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm14_xmm15() {
    let mut emu = emu64();
    // VSQRTPD XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0xf7, // VSQRTPD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm15_xmm8() {
    let mut emu = emu64();
    // VSQRTPD XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0xf8, // VSQRTPD XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPD Tests - 256-bit YMM registers (4x float64)
// ============================================================================

#[test]
fn test_vsqrtpd_ymm0_ymm1() {
    let mut emu = emu64();
    // VSQRTPD YMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x51, 0xc1, // VSQRTPD YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm1_ymm2() {
    let mut emu = emu64();
    // VSQRTPD YMM1, YMM2
    let code = [
        0xc5, 0xfd, 0x51, 0xca, // VSQRTPD YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm2_ymm3() {
    let mut emu = emu64();
    // VSQRTPD YMM2, YMM3
    let code = [
        0xc5, 0xfd, 0x51, 0xd3, // VSQRTPD YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm3_ymm4() {
    let mut emu = emu64();
    // VSQRTPD YMM3, YMM4
    let code = [
        0xc5, 0xfd, 0x51, 0xdc, // VSQRTPD YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm4_ymm5() {
    let mut emu = emu64();
    // VSQRTPD YMM4, YMM5
    let code = [
        0xc5, 0xfd, 0x51, 0xe5, // VSQRTPD YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm5_ymm6() {
    let mut emu = emu64();
    // VSQRTPD YMM5, YMM6
    let code = [
        0xc5, 0xfd, 0x51, 0xee, // VSQRTPD YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm6_ymm7() {
    let mut emu = emu64();
    // VSQRTPD YMM6, YMM7
    let code = [
        0xc5, 0xfd, 0x51, 0xf7, // VSQRTPD YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm7_ymm0() {
    let mut emu = emu64();
    // VSQRTPD YMM7, YMM0
    let code = [
        0xc5, 0xfd, 0x51, 0xf8, // VSQRTPD YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPD Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vsqrtpd_ymm8_ymm9() {
    let mut emu = emu64();
    // VSQRTPD YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0xc1, // VSQRTPD YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm9_ymm10() {
    let mut emu = emu64();
    // VSQRTPD YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0xca, // VSQRTPD YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm10_ymm11() {
    let mut emu = emu64();
    // VSQRTPD YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0xd3, // VSQRTPD YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm11_ymm12() {
    let mut emu = emu64();
    // VSQRTPD YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0xdc, // VSQRTPD YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm12_ymm13() {
    let mut emu = emu64();
    // VSQRTPD YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0xe5, // VSQRTPD YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm13_ymm14() {
    let mut emu = emu64();
    // VSQRTPD YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0xee, // VSQRTPD YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm14_ymm15() {
    let mut emu = emu64();
    // VSQRTPD YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0xf7, // VSQRTPD YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm15_ymm8() {
    let mut emu = emu64();
    // VSQRTPD YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0xf8, // VSQRTPD YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPD Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vsqrtpd_xmm0_mem() {
    let mut emu = emu64();
    // VSQRTPD XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x51, 0x05, 0x00, 0x40, 0x00, 0x00, // VSQRTPD XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_xmm8_mem() {
    let mut emu = emu64();
    // VSQRTPD XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x79, 0x51, 0x05, 0x00, 0x40, 0x00, 0x00, // VSQRTPD XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x40, // 9.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x40, // 16.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VSQRTPD Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vsqrtpd_ymm0_mem() {
    let mut emu = emu64();
    // VSQRTPD YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x51, 0x05, 0x00, 0x40, 0x00, 0x00, // VSQRTPD YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x40, // 9.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x40, // 16.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vsqrtpd_ymm8_mem() {
    let mut emu = emu64();
    // VSQRTPD YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7d, 0x51, 0x05, 0x00, 0x40, 0x00, 0x00, // VSQRTPD YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x40, // 25.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x40, // 36.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x48, 0x40, // 49.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x40, // 64.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
