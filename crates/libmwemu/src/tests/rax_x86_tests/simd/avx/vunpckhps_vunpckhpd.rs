use crate::*;

// VUNPCKHPS - Unpack and Interleave High Packed Single-Precision Floating-Point Values
// VUNPCKHPD - Unpack and Interleave High Packed Double-Precision Floating-Point Values
//
// VUNPCKHPS interleaves the high-order single-precision values from two source operands.
// For 128-bit: interleaves elements [3:2] and [3:2] -> [1[3], 2[3], 1[2], 2[2]]
// For 256-bit: interleaves independently in each 128-bit lane
//
// VUNPCKHPD interleaves the high-order double-precision values from two source operands.
// For 128-bit: interleaves elements [1] and [1] -> [1[1], 2[1]]
// For 256-bit: interleaves independently in each 128-bit lane
//
// Opcodes:
// VEX.128.0F.WIG 15 /r         VUNPCKHPS xmm1, xmm2, xmm3/m128
// VEX.256.0F.WIG 15 /r         VUNPCKHPS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F.WIG 15 /r      VUNPCKHPD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F.WIG 15 /r      VUNPCKHPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VUNPCKHPS 128-bit Tests - Unpack High Single-Precision (XMM)
// ============================================================================

#[test]
fn test_vunpckhps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VUNPCKHPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x15, 0xc2, // VUNPCKHPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VUNPCKHPS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe8, 0x15, 0xcb, // VUNPCKHPS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VUNPCKHPS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe0, 0x15, 0xd4, // VUNPCKHPS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VUNPCKHPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x15, 0xdd, // VUNPCKHPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VUNPCKHPS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd0, 0x15, 0xe6, // VUNPCKHPS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VUNPCKHPS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc8, 0x15, 0xef, // VUNPCKHPS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VUNPCKHPS XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc0, 0x15, 0xf0, // VUNPCKHPS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VUNPCKHPS XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x15, 0xf9, // VUNPCKHPS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm0_xmm0_xmm0() {
    let mut emu = emu64();
    // VUNPCKHPS XMM0, XMM0, XMM0 - duplicate high elements
    let code = [
        0xc5, 0xf8, 0x15, 0xc0, // VUNPCKHPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VUNPCKHPS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0xc1, 0x30, 0x15, 0xc2, // VUNPCKHPS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VUNPCKHPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x15, 0xdd, // VUNPCKHPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm14_xmm15_xmm0() {
    let mut emu = emu64();
    // VUNPCKHPS XMM14, XMM15, XMM0
    let code = [
        0xc4, 0xe1, 0x00, 0x15, 0xf0, // VUNPCKHPS XMM14, XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_xmm15_xmm14_xmm13() {
    let mut emu = emu64();
    // VUNPCKHPS XMM15, XMM14, XMM13
    let code = [
        0xc4, 0xc1, 0x08, 0x15, 0xfd, // VUNPCKHPS XMM15, XMM14, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKHPS 256-bit Tests - Unpack High Single-Precision (YMM)
// ============================================================================

#[test]
fn test_vunpckhps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VUNPCKHPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x15, 0xc2, // VUNPCKHPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VUNPCKHPS YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xec, 0x15, 0xcb, // VUNPCKHPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VUNPCKHPS YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe4, 0x15, 0xd4, // VUNPCKHPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VUNPCKHPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x15, 0xdd, // VUNPCKHPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VUNPCKHPS YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd4, 0x15, 0xe6, // VUNPCKHPS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VUNPCKHPS YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcc, 0x15, 0xef, // VUNPCKHPS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VUNPCKHPS YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc4, 0x15, 0xf0, // VUNPCKHPS YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VUNPCKHPS YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x15, 0xf9, // VUNPCKHPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm0_ymm0_ymm0() {
    let mut emu = emu64();
    // VUNPCKHPS YMM0, YMM0, YMM0 - duplicate high elements
    let code = [
        0xc5, 0xfc, 0x15, 0xc0, // VUNPCKHPS YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VUNPCKHPS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0xc1, 0x34, 0x15, 0xc2, // VUNPCKHPS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VUNPCKHPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x15, 0xdd, // VUNPCKHPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm14_ymm15_ymm0() {
    let mut emu = emu64();
    // VUNPCKHPS YMM14, YMM15, YMM0
    let code = [
        0xc4, 0xe1, 0x04, 0x15, 0xf0, // VUNPCKHPS YMM14, YMM15, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhps_ymm15_ymm14_ymm13() {
    let mut emu = emu64();
    // VUNPCKHPS YMM15, YMM14, YMM13
    let code = [
        0xc4, 0xc1, 0x0c, 0x15, 0xfd, // VUNPCKHPS YMM15, YMM14, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKHPS Memory Tests
// ============================================================================

#[test]
fn test_vunpckhps_xmm0_xmm1_mem128() {
    let mut emu = emu64();
    // VUNPCKHPS XMM0, XMM1, [mem128]
    let code = [
        0xc5, 0xf0, 0x15, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKHPS XMM0, XMM1, [rip + 0x4000]
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
fn test_vunpckhps_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    // VUNPCKHPS YMM0, YMM1, [mem256]
    let code = [
        0xc5, 0xf4, 0x15, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKHPS YMM0, YMM1, [rip + 0x4000]
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
fn test_vunpckhps_xmm8_xmm9_mem128() {
    let mut emu = emu64();
    // VUNPCKHPS XMM8, XMM9, [mem128]
    let code = [
        0xc4, 0xc1, 0x30, 0x15, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKHPS XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKHPD 128-bit Tests - Unpack High Double-Precision (XMM)
// ============================================================================

#[test]
fn test_vunpckhpd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VUNPCKHPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x15, 0xc2, // VUNPCKHPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VUNPCKHPD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe9, 0x15, 0xcb, // VUNPCKHPD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VUNPCKHPD XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe1, 0x15, 0xd4, // VUNPCKHPD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VUNPCKHPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x15, 0xdd, // VUNPCKHPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VUNPCKHPD XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd1, 0x15, 0xe6, // VUNPCKHPD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VUNPCKHPD XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc9, 0x15, 0xef, // VUNPCKHPD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VUNPCKHPD XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc1, 0x15, 0xf0, // VUNPCKHPD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VUNPCKHPD XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x15, 0xf9, // VUNPCKHPD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm0_xmm0_xmm0() {
    let mut emu = emu64();
    // VUNPCKHPD XMM0, XMM0, XMM0 - duplicate high element
    let code = [
        0xc5, 0xf9, 0x15, 0xc0, // VUNPCKHPD XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VUNPCKHPD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0xc1, 0x31, 0x15, 0xc2, // VUNPCKHPD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VUNPCKHPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x15, 0xdd, // VUNPCKHPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm14_xmm15_xmm0() {
    let mut emu = emu64();
    // VUNPCKHPD XMM14, XMM15, XMM0
    let code = [
        0xc4, 0xe1, 0x01, 0x15, 0xf0, // VUNPCKHPD XMM14, XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_xmm15_xmm14_xmm13() {
    let mut emu = emu64();
    // VUNPCKHPD XMM15, XMM14, XMM13
    let code = [
        0xc4, 0xc1, 0x09, 0x15, 0xfd, // VUNPCKHPD XMM15, XMM14, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKHPD 256-bit Tests - Unpack High Double-Precision (YMM)
// ============================================================================

#[test]
fn test_vunpckhpd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VUNPCKHPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x15, 0xc2, // VUNPCKHPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VUNPCKHPD YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xed, 0x15, 0xcb, // VUNPCKHPD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VUNPCKHPD YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe5, 0x15, 0xd4, // VUNPCKHPD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VUNPCKHPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x15, 0xdd, // VUNPCKHPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VUNPCKHPD YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd5, 0x15, 0xe6, // VUNPCKHPD YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VUNPCKHPD YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcd, 0x15, 0xef, // VUNPCKHPD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VUNPCKHPD YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc5, 0x15, 0xf0, // VUNPCKHPD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VUNPCKHPD YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x15, 0xf9, // VUNPCKHPD YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm0_ymm0_ymm0() {
    let mut emu = emu64();
    // VUNPCKHPD YMM0, YMM0, YMM0 - duplicate high elements
    let code = [
        0xc5, 0xfd, 0x15, 0xc0, // VUNPCKHPD YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VUNPCKHPD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0xc1, 0x35, 0x15, 0xc2, // VUNPCKHPD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VUNPCKHPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x15, 0xdd, // VUNPCKHPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm14_ymm15_ymm0() {
    let mut emu = emu64();
    // VUNPCKHPD YMM14, YMM15, YMM0
    let code = [
        0xc4, 0xe1, 0x05, 0x15, 0xf0, // VUNPCKHPD YMM14, YMM15, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpckhpd_ymm15_ymm14_ymm13() {
    let mut emu = emu64();
    // VUNPCKHPD YMM15, YMM14, YMM13
    let code = [
        0xc4, 0xc1, 0x0d, 0x15, 0xfd, // VUNPCKHPD YMM15, YMM14, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKHPD Memory Tests
// ============================================================================

#[test]
fn test_vunpckhpd_xmm0_xmm1_mem128() {
    let mut emu = emu64();
    // VUNPCKHPD XMM0, XMM1, [mem128]
    let code = [
        0xc5, 0xf1, 0x15, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKHPD XMM0, XMM1, [rip + 0x4000]
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
fn test_vunpckhpd_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    // VUNPCKHPD YMM0, YMM1, [mem256]
    let code = [
        0xc5, 0xf5, 0x15, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKHPD YMM0, YMM1, [rip + 0x4000]
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
fn test_vunpckhpd_xmm8_xmm9_mem128() {
    let mut emu = emu64();
    // VUNPCKHPD XMM8, XMM9, [mem128]
    let code = [
        0xc4, 0xc1, 0x31, 0x15, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKHPD XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
