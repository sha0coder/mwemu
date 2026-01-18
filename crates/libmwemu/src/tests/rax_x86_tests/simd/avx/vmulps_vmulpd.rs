use crate::*;

// VMULPS - Multiply Packed Single-Precision Floating-Point Values
// VMULPD - Multiply Packed Double-Precision Floating-Point Values
//
// VMULPS performs element-wise multiplication of packed single-precision floating-point values.
// VMULPD performs element-wise multiplication of packed double-precision floating-point values.
//
// Opcodes:
// VEX.128.0F.WIG 59 /r    VMULPS xmm1, xmm2, xmm3/m128   - Multiply packed single from xmm3/mem with xmm2
// VEX.256.0F.WIG 59 /r    VMULPS ymm1, ymm2, ymm3/m256   - Multiply packed single from ymm3/mem with ymm2
// VEX.128.66.0F.WIG 59 /r VMULPD xmm1, xmm2, xmm3/m128   - Multiply packed double from xmm3/mem with xmm2
// VEX.256.66.0F.WIG 59 /r VMULPD ymm1, ymm2, ymm3/m256   - Multiply packed double from ymm3/mem with ymm2

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VMULPS Tests - 128-bit XMM registers (4x float32)
// ============================================================================

#[test]
fn test_vmulps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VMULPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x59, 0xc2, // VMULPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VMULPS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe8, 0x59, 0xcb, // VMULPS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VMULPS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe0, 0x59, 0xd4, // VMULPS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VMULPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x59, 0xdd, // VMULPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VMULPS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd0, 0x59, 0xe6, // VMULPS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VMULPS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc8, 0x59, 0xef, // VMULPS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VMULPS XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc0, 0x59, 0xf0, // VMULPS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VMULPS XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x59, 0xf9, // VMULPS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vmulps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VMULPS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x30, 0x59, 0xc2, // VMULPS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VMULPS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x28, 0x59, 0xcb, // VMULPS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VMULPS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x20, 0x59, 0xd4, // VMULPS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VMULPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x18, 0x59, 0xdd, // VMULPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VMULPS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x10, 0x59, 0xe6, // VMULPS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VMULPS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x08, 0x59, 0xef, // VMULPS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VMULPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x00, 0x59, 0xf0, // VMULPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VMULPS XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x38, 0x59, 0xf9, // VMULPS XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPS Tests - Cross-domain (mixing low and high XMM registers)
// ============================================================================

#[test]
fn test_vmulps_xmm0_xmm8_xmm15() {
    let mut emu = emu64();
    // VMULPS XMM0, XMM8, XMM15
    let code = [
        0xc4, 0xc1, 0x38, 0x59, 0xc7, // VMULPS XMM0, XMM8, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm8_xmm0_xmm7() {
    let mut emu = emu64();
    // VMULPS XMM8, XMM0, XMM7
    let code = [
        0xc4, 0xc1, 0x78, 0x59, 0xc7, // VMULPS XMM8, XMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_xmm7_xmm8_xmm0() {
    let mut emu = emu64();
    // VMULPS XMM7, XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x38, 0x59, 0xf8, // VMULPS XMM7, XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPS Tests - 256-bit YMM registers (8x float32)
// ============================================================================

#[test]
fn test_vmulps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VMULPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x59, 0xc2, // VMULPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VMULPS YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xec, 0x59, 0xcb, // VMULPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VMULPS YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe4, 0x59, 0xd4, // VMULPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VMULPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x59, 0xdd, // VMULPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VMULPS YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd4, 0x59, 0xe6, // VMULPS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VMULPS YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcc, 0x59, 0xef, // VMULPS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VMULPS YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc4, 0x59, 0xf0, // VMULPS YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VMULPS YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x59, 0xf9, // VMULPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPS Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vmulps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VMULPS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x34, 0x59, 0xc2, // VMULPS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    // VMULPS YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2c, 0x59, 0xcb, // VMULPS YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm10_ymm11_ymm12() {
    let mut emu = emu64();
    // VMULPS YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x24, 0x59, 0xd4, // VMULPS YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VMULPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1c, 0x59, 0xdd, // VMULPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    // VMULPS YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x14, 0x59, 0xe6, // VMULPS YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VMULPS YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0c, 0x59, 0xef, // VMULPS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm14_ymm15_ymm8() {
    let mut emu = emu64();
    // VMULPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x04, 0x59, 0xf0, // VMULPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    // VMULPS YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3c, 0x59, 0xf9, // VMULPS YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPS Tests - Cross-domain YMM registers
// ============================================================================

#[test]
fn test_vmulps_ymm0_ymm8_ymm15() {
    let mut emu = emu64();
    // VMULPS YMM0, YMM8, YMM15
    let code = [
        0xc4, 0xc1, 0x3c, 0x59, 0xc7, // VMULPS YMM0, YMM8, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm8_ymm0_ymm7() {
    let mut emu = emu64();
    // VMULPS YMM8, YMM0, YMM7
    let code = [
        0xc4, 0xc1, 0x7c, 0x59, 0xc7, // VMULPS YMM8, YMM0, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulps_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    // VMULPS YMM15, YMM0, YMM1
    let code = [
        0xc4, 0xc1, 0x7c, 0x59, 0xf9, // VMULPS YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vmulps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VMULPS XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf0, 0x59, 0x05, 0x00, 0x40, 0x00, 0x00, // VMULPS XMM0, XMM1, [rip + 0x4000]
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
fn test_vmulps_xmm8_xmm9_mem() {
    let mut emu = emu64();
    // VMULPS XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x30, 0x59, 0x05, 0x00, 0x40, 0x00, 0x00, // VMULPS XMM8, XMM9, [rip + 0x4000]
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
// VMULPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vmulps_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VMULPS YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf4, 0x59, 0x05, 0x00, 0x40, 0x00, 0x00, // VMULPS YMM0, YMM1, [rip + 0x4000]
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
fn test_vmulps_ymm8_ymm9_mem() {
    let mut emu = emu64();
    // VMULPS YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x41, 0x34, 0x59, 0x05, 0x00, 0x40, 0x00, 0x00, // VMULPS YMM8, YMM9, [rip + 0x4000]
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
// VMULPD Tests - 128-bit XMM registers (2x float64)
// ============================================================================

#[test]
fn test_vmulpd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VMULPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x59, 0xc2, // VMULPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VMULPD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe9, 0x59, 0xcb, // VMULPD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VMULPD XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe1, 0x59, 0xd4, // VMULPD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VMULPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x59, 0xdd, // VMULPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VMULPD XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd1, 0x59, 0xe6, // VMULPD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VMULPD XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc9, 0x59, 0xef, // VMULPD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VMULPD XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc1, 0x59, 0xf0, // VMULPD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VMULPD XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x59, 0xf9, // VMULPD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vmulpd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VMULPD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x31, 0x59, 0xc2, // VMULPD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VMULPD XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x29, 0x59, 0xcb, // VMULPD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VMULPD XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x21, 0x59, 0xd4, // VMULPD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VMULPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x19, 0x59, 0xdd, // VMULPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VMULPD XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x11, 0x59, 0xe6, // VMULPD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VMULPD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x09, 0x59, 0xef, // VMULPD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VMULPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x01, 0x59, 0xf0, // VMULPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VMULPD XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x39, 0x59, 0xf9, // VMULPD XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPD Tests - 256-bit YMM registers (4x float64)
// ============================================================================

#[test]
fn test_vmulpd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VMULPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x59, 0xc2, // VMULPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VMULPD YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xed, 0x59, 0xcb, // VMULPD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VMULPD YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe5, 0x59, 0xd4, // VMULPD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VMULPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x59, 0xdd, // VMULPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VMULPD YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd5, 0x59, 0xe6, // VMULPD YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VMULPD YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcd, 0x59, 0xef, // VMULPD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VMULPD YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc5, 0x59, 0xf0, // VMULPD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VMULPD YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x59, 0xf9, // VMULPD YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPD Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vmulpd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VMULPD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x35, 0x59, 0xc2, // VMULPD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    // VMULPD YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2d, 0x59, 0xcb, // VMULPD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm10_ymm11_ymm12() {
    let mut emu = emu64();
    // VMULPD YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x25, 0x59, 0xd4, // VMULPD YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VMULPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1d, 0x59, 0xdd, // VMULPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    // VMULPD YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x15, 0x59, 0xe6, // VMULPD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VMULPD YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0d, 0x59, 0xef, // VMULPD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm14_ymm15_ymm8() {
    let mut emu = emu64();
    // VMULPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x05, 0x59, 0xf0, // VMULPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulpd_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    // VMULPD YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3d, 0x59, 0xf9, // VMULPD YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPD Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vmulpd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VMULPD XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf1, 0x59, 0x05, 0x00, 0x40, 0x00, 0x00, // VMULPD XMM0, XMM1, [rip + 0x4000]
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
fn test_vmulpd_xmm8_xmm9_mem() {
    let mut emu = emu64();
    // VMULPD XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x31, 0x59, 0x05, 0x00, 0x40, 0x00, 0x00, // VMULPD XMM8, XMM9, [rip + 0x4000]
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
// VMULPD Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vmulpd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VMULPD YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf5, 0x59, 0x05, 0x00, 0x40, 0x00, 0x00, // VMULPD YMM0, YMM1, [rip + 0x4000]
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
fn test_vmulpd_ymm8_ymm9_mem() {
    let mut emu = emu64();
    // VMULPD YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x41, 0x35, 0x59, 0x05, 0x00, 0x40, 0x00, 0x00, // VMULPD YMM8, YMM9, [rip + 0x4000]
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
