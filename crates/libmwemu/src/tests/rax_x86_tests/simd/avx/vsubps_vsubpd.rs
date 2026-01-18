use crate::*;

// VSUBPS - Subtract Packed Single-Precision Floating-Point Values
// VSUBPD - Subtract Packed Double-Precision Floating-Point Values
//
// VSUBPS performs element-wise subtraction of packed single-precision floating-point values.
// VSUBPD performs element-wise subtraction of packed double-precision floating-point values.
//
// Opcodes:
// VEX.128.0F.WIG 5C /r    VSUBPS xmm1, xmm2, xmm3/m128   - Subtract packed single from xmm3/mem from xmm2
// VEX.256.0F.WIG 5C /r    VSUBPS ymm1, ymm2, ymm3/m256   - Subtract packed single from ymm3/mem from ymm2
// VEX.128.66.0F.WIG 5C /r VSUBPD xmm1, xmm2, xmm3/m128   - Subtract packed double from xmm3/mem from xmm2
// VEX.256.66.0F.WIG 5C /r VSUBPD ymm1, ymm2, ymm3/m256   - Subtract packed double from ymm3/mem from ymm2

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VSUBPS Tests - 128-bit XMM registers (4x float32)
// ============================================================================

#[test]
fn test_vsubps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VSUBPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x5c, 0xc2, // VSUBPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VSUBPS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe8, 0x5c, 0xcb, // VSUBPS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VSUBPS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe0, 0x5c, 0xd4, // VSUBPS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VSUBPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x5c, 0xdd, // VSUBPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VSUBPS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd0, 0x5c, 0xe6, // VSUBPS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VSUBPS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc8, 0x5c, 0xef, // VSUBPS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VSUBPS XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc0, 0x5c, 0xf0, // VSUBPS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VSUBPS XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x5c, 0xf9, // VSUBPS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vsubps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VSUBPS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x30, 0x5c, 0xc2, // VSUBPS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VSUBPS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x28, 0x5c, 0xcb, // VSUBPS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VSUBPS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x20, 0x5c, 0xd4, // VSUBPS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VSUBPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x18, 0x5c, 0xdd, // VSUBPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VSUBPS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x10, 0x5c, 0xe6, // VSUBPS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VSUBPS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x08, 0x5c, 0xef, // VSUBPS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VSUBPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x00, 0x5c, 0xf0, // VSUBPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VSUBPS XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x38, 0x5c, 0xf9, // VSUBPS XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPS Tests - Cross-domain (mixing low and high XMM registers)
// ============================================================================

#[test]
fn test_vsubps_xmm0_xmm8_xmm15() {
    let mut emu = emu64();
    // VSUBPS XMM0, XMM8, XMM15
    let code = [
        0xc4, 0xc1, 0x38, 0x5c, 0xc7, // VSUBPS XMM0, XMM8, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm8_xmm0_xmm7() {
    let mut emu = emu64();
    // VSUBPS XMM8, XMM0, XMM7
    let code = [
        0xc4, 0xc1, 0x78, 0x5c, 0xc7, // VSUBPS XMM8, XMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_xmm7_xmm8_xmm0() {
    let mut emu = emu64();
    // VSUBPS XMM7, XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x38, 0x5c, 0xf8, // VSUBPS XMM7, XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPS Tests - 256-bit YMM registers (8x float32)
// ============================================================================

#[test]
fn test_vsubps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VSUBPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x5c, 0xc2, // VSUBPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VSUBPS YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xec, 0x5c, 0xcb, // VSUBPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VSUBPS YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe4, 0x5c, 0xd4, // VSUBPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VSUBPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x5c, 0xdd, // VSUBPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VSUBPS YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd4, 0x5c, 0xe6, // VSUBPS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VSUBPS YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcc, 0x5c, 0xef, // VSUBPS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VSUBPS YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc4, 0x5c, 0xf0, // VSUBPS YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VSUBPS YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x5c, 0xf9, // VSUBPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPS Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vsubps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VSUBPS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x34, 0x5c, 0xc2, // VSUBPS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    // VSUBPS YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2c, 0x5c, 0xcb, // VSUBPS YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm10_ymm11_ymm12() {
    let mut emu = emu64();
    // VSUBPS YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x24, 0x5c, 0xd4, // VSUBPS YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VSUBPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1c, 0x5c, 0xdd, // VSUBPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    // VSUBPS YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x14, 0x5c, 0xe6, // VSUBPS YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VSUBPS YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0c, 0x5c, 0xef, // VSUBPS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm14_ymm15_ymm8() {
    let mut emu = emu64();
    // VSUBPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x04, 0x5c, 0xf0, // VSUBPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    // VSUBPS YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3c, 0x5c, 0xf9, // VSUBPS YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPS Tests - Cross-domain YMM registers
// ============================================================================

#[test]
fn test_vsubps_ymm0_ymm8_ymm15() {
    let mut emu = emu64();
    // VSUBPS YMM0, YMM8, YMM15
    let code = [
        0xc4, 0xc1, 0x3c, 0x5c, 0xc7, // VSUBPS YMM0, YMM8, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm8_ymm0_ymm7() {
    let mut emu = emu64();
    // VSUBPS YMM8, YMM0, YMM7
    let code = [
        0xc4, 0xc1, 0x7c, 0x5c, 0xc7, // VSUBPS YMM8, YMM0, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubps_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    // VSUBPS YMM15, YMM0, YMM1
    let code = [
        0xc4, 0xc1, 0x7c, 0x5c, 0xf9, // VSUBPS YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vsubps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VSUBPS XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf0, 0x5c, 0x05, 0x00, 0x40, 0x00, 0x00, // VSUBPS XMM0, XMM1, [rip + 0x4000]
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
fn test_vsubps_xmm8_xmm9_mem() {
    let mut emu = emu64();
    // VSUBPS XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x30, 0x5c, 0x05, 0x00, 0x40, 0x00, 0x00, // VSUBPS XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xa0, 0x40, // 5.0
        0x00, 0x00, 0xc0, 0x40, // 6.0
        0x00, 0x00, 0xe0, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x41, // 8.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vsubps_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VSUBPS YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf4, 0x5c, 0x05, 0x00, 0x40, 0x00, 0x00, // VSUBPS YMM0, YMM1, [rip + 0x4000]
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
fn test_vsubps_ymm8_ymm9_mem() {
    let mut emu = emu64();
    // VSUBPS YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x41, 0x34, 0x5c, 0x05, 0x00, 0x40, 0x00, 0x00, // VSUBPS YMM8, YMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x10, 0x41, // 9.0
        0x00, 0x00, 0x20, 0x41, // 10.0
        0x00, 0x00, 0x30, 0x41, // 11.0
        0x00, 0x00, 0x40, 0x41, // 12.0
        0x00, 0x00, 0x50, 0x41, // 13.0
        0x00, 0x00, 0x60, 0x41, // 14.0
        0x00, 0x00, 0x70, 0x41, // 15.0
        0x00, 0x00, 0x80, 0x41, // 16.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPD Tests - 128-bit XMM registers (2x float64)
// ============================================================================

#[test]
fn test_vsubpd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VSUBPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x5c, 0xc2, // VSUBPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VSUBPD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe9, 0x5c, 0xcb, // VSUBPD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VSUBPD XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe1, 0x5c, 0xd4, // VSUBPD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VSUBPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x5c, 0xdd, // VSUBPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VSUBPD XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd1, 0x5c, 0xe6, // VSUBPD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VSUBPD XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc9, 0x5c, 0xef, // VSUBPD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VSUBPD XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc1, 0x5c, 0xf0, // VSUBPD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VSUBPD XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x5c, 0xf9, // VSUBPD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vsubpd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VSUBPD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x31, 0x5c, 0xc2, // VSUBPD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VSUBPD XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x29, 0x5c, 0xcb, // VSUBPD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VSUBPD XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x21, 0x5c, 0xd4, // VSUBPD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VSUBPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x19, 0x5c, 0xdd, // VSUBPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VSUBPD XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x11, 0x5c, 0xe6, // VSUBPD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VSUBPD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x09, 0x5c, 0xef, // VSUBPD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VSUBPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x01, 0x5c, 0xf0, // VSUBPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VSUBPD XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x39, 0x5c, 0xf9, // VSUBPD XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPD Tests - 256-bit YMM registers (4x float64)
// ============================================================================

#[test]
fn test_vsubpd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VSUBPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x5c, 0xc2, // VSUBPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VSUBPD YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xed, 0x5c, 0xcb, // VSUBPD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VSUBPD YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe5, 0x5c, 0xd4, // VSUBPD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VSUBPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x5c, 0xdd, // VSUBPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VSUBPD YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd5, 0x5c, 0xe6, // VSUBPD YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VSUBPD YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcd, 0x5c, 0xef, // VSUBPD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VSUBPD YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc5, 0x5c, 0xf0, // VSUBPD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VSUBPD YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x5c, 0xf9, // VSUBPD YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPD Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vsubpd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VSUBPD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x35, 0x5c, 0xc2, // VSUBPD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    // VSUBPD YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2d, 0x5c, 0xcb, // VSUBPD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm10_ymm11_ymm12() {
    let mut emu = emu64();
    // VSUBPD YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x25, 0x5c, 0xd4, // VSUBPD YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VSUBPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1d, 0x5c, 0xdd, // VSUBPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    // VSUBPD YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x15, 0x5c, 0xe6, // VSUBPD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VSUBPD YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0d, 0x5c, 0xef, // VSUBPD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm14_ymm15_ymm8() {
    let mut emu = emu64();
    // VSUBPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x05, 0x5c, 0xf0, // VSUBPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubpd_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    // VSUBPD YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3d, 0x5c, 0xf9, // VSUBPD YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPD Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vsubpd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VSUBPD XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf1, 0x5c, 0x05, 0x00, 0x40, 0x00, 0x00, // VSUBPD XMM0, XMM1, [rip + 0x4000]
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
fn test_vsubpd_xmm8_xmm9_mem() {
    let mut emu = emu64();
    // VSUBPD XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x31, 0x5c, 0x05, 0x00, 0x40, 0x00, 0x00, // VSUBPD XMM8, XMM9, [rip + 0x4000]
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
// VSUBPD Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vsubpd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VSUBPD YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf5, 0x5c, 0x05, 0x00, 0x40, 0x00, 0x00, // VSUBPD YMM0, YMM1, [rip + 0x4000]
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
fn test_vsubpd_ymm8_ymm9_mem() {
    let mut emu = emu64();
    // VSUBPD YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x41, 0x35, 0x5c, 0x05, 0x00, 0x40, 0x00, 0x00, // VSUBPD YMM8, YMM9, [rip + 0x4000]
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
