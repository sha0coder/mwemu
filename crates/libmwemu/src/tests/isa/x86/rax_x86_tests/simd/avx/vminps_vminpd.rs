use crate::*;

// VMINPS - Return Minimum Packed Single-Precision Floating-Point Values
// VMINPD - Return Minimum Packed Double-Precision Floating-Point Values
//
// VMINPS performs element-wise minimum of packed single-precision floating-point values.
// VMINPD performs element-wise minimum of packed double-precision floating-point values.
//
// Opcodes:
// VEX.128.0F.WIG 5D /r    VMINPS xmm1, xmm2, xmm3/m128   - Return minimum of packed single from xmm3/mem and xmm2
// VEX.256.0F.WIG 5D /r    VMINPS ymm1, ymm2, ymm3/m256   - Return minimum of packed single from ymm3/mem and ymm2
// VEX.128.66.0F.WIG 5D /r VMINPD xmm1, xmm2, xmm3/m128   - Return minimum of packed double from xmm3/mem and xmm2
// VEX.256.66.0F.WIG 5D /r VMINPD ymm1, ymm2, ymm3/m256   - Return minimum of packed double from ymm3/mem and ymm2

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VMINPS Tests - 128-bit XMM registers (4x float32)
// ============================================================================

#[test]
fn test_vminps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VMINPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x5d, 0xc2, // VMINPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VMINPS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe8, 0x5d, 0xcb, // VMINPS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VMINPS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe0, 0x5d, 0xd4, // VMINPS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VMINPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x5d, 0xdd, // VMINPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VMINPS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd0, 0x5d, 0xe6, // VMINPS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VMINPS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc8, 0x5d, 0xef, // VMINPS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VMINPS XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc0, 0x5d, 0xf0, // VMINPS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VMINPS XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x5d, 0xf9, // VMINPS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMINPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vminps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VMINPS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x30, 0x5d, 0xc2, // VMINPS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VMINPS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x28, 0x5d, 0xcb, // VMINPS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VMINPS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x20, 0x5d, 0xd4, // VMINPS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VMINPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x18, 0x5d, 0xdd, // VMINPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VMINPS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x10, 0x5d, 0xe6, // VMINPS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VMINPS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x08, 0x5d, 0xef, // VMINPS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VMINPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x00, 0x5d, 0xf0, // VMINPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VMINPS XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x38, 0x5d, 0xf9, // VMINPS XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMINPS Tests - 256-bit YMM registers (8x float32)
// ============================================================================

#[test]
fn test_vminps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VMINPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x5d, 0xc2, // VMINPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VMINPS YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xec, 0x5d, 0xcb, // VMINPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VMINPS YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe4, 0x5d, 0xd4, // VMINPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VMINPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x5d, 0xdd, // VMINPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VMINPS YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd4, 0x5d, 0xe6, // VMINPS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VMINPS YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcc, 0x5d, 0xef, // VMINPS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VMINPS YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc4, 0x5d, 0xf0, // VMINPS YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VMINPS YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x5d, 0xf9, // VMINPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMINPS Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vminps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VMINPS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x34, 0x5d, 0xc2, // VMINPS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    // VMINPS YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2c, 0x5d, 0xcb, // VMINPS YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm10_ymm11_ymm12() {
    let mut emu = emu64();
    // VMINPS YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x24, 0x5d, 0xd4, // VMINPS YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VMINPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1c, 0x5d, 0xdd, // VMINPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    // VMINPS YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x14, 0x5d, 0xe6, // VMINPS YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VMINPS YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0c, 0x5d, 0xef, // VMINPS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm14_ymm15_ymm8() {
    let mut emu = emu64();
    // VMINPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x04, 0x5d, 0xf0, // VMINPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminps_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    // VMINPS YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3c, 0x5d, 0xf9, // VMINPS YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMINPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vminps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VMINPS XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf0, 0x5d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMINPS XMM0, XMM1, [rip + 0x4000]
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

// ============================================================================
// VMINPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vminps_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VMINPS YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf4, 0x5d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMINPS YMM0, YMM1, [rip + 0x4000]
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

// ============================================================================
// VMINPD Tests - 128-bit XMM registers (2x float64)
// ============================================================================

#[test]
fn test_vminpd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VMINPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x5d, 0xc2, // VMINPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminpd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VMINPD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe9, 0x5d, 0xcb, // VMINPD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminpd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VMINPD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x31, 0x5d, 0xc2, // VMINPD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminpd_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VMINPD XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x39, 0x5d, 0xf9, // VMINPD XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMINPD Tests - 256-bit YMM registers (4x float64)
// ============================================================================

#[test]
fn test_vminpd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VMINPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x5d, 0xc2, // VMINPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminpd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VMINPD YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xed, 0x5d, 0xcb, // VMINPD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminpd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VMINPD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x35, 0x5d, 0xc2, // VMINPD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vminpd_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    // VMINPD YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3d, 0x5d, 0xf9, // VMINPD YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMINPD Tests - Memory operands
// ============================================================================

#[test]
fn test_vminpd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VMINPD XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf1, 0x5d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMINPD XMM0, XMM1, [rip + 0x4000]
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
fn test_vminpd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VMINPD YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf5, 0x5d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMINPD YMM0, YMM1, [rip + 0x4000]
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
