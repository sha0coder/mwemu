use crate::*;

// VUNPCKLPS - Unpack and Interleave Low Packed Single-Precision Floating-Point Values
// VUNPCKLPD - Unpack and Interleave Low Packed Double-Precision Floating-Point Values
//
// VUNPCKLPS interleaves the low-order single-precision values from two source operands.
// For 128-bit: interleaves elements [1:0] and [1:0] -> [1[1], 2[1], 1[0], 2[0]]
// For 256-bit: interleaves independently in each 128-bit lane
//
// VUNPCKLPD interleaves the low-order double-precision values from two source operands.
// For 128-bit: interleaves elements [0] and [0] -> [1[0], 2[0]]
// For 256-bit: interleaves independently in each 128-bit lane
//
// Opcodes:
// VEX.128.0F.WIG 14 /r         VUNPCKLPS xmm1, xmm2, xmm3/m128
// VEX.256.0F.WIG 14 /r         VUNPCKLPS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F.WIG 14 /r      VUNPCKLPD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F.WIG 14 /r      VUNPCKLPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VUNPCKLPS 128-bit Tests - Unpack Low Single-Precision (XMM)
// ============================================================================

#[test]
fn test_vunpcklps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VUNPCKLPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x14, 0xc2, // VUNPCKLPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VUNPCKLPS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe8, 0x14, 0xcb, // VUNPCKLPS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VUNPCKLPS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe0, 0x14, 0xd4, // VUNPCKLPS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VUNPCKLPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x14, 0xdd, // VUNPCKLPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VUNPCKLPS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd0, 0x14, 0xe6, // VUNPCKLPS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VUNPCKLPS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc8, 0x14, 0xef, // VUNPCKLPS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VUNPCKLPS XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc0, 0x14, 0xf0, // VUNPCKLPS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VUNPCKLPS XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x14, 0xf9, // VUNPCKLPS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm0_xmm0_xmm0() {
    let mut emu = emu64();
    // VUNPCKLPS XMM0, XMM0, XMM0 - duplicate low elements
    let code = [
        0xc5, 0xf8, 0x14, 0xc0, // VUNPCKLPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VUNPCKLPS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0xc1, 0x30, 0x14, 0xc2, // VUNPCKLPS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VUNPCKLPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x14, 0xdd, // VUNPCKLPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm14_xmm15_xmm0() {
    let mut emu = emu64();
    // VUNPCKLPS XMM14, XMM15, XMM0
    let code = [
        0xc4, 0xe1, 0x00, 0x14, 0xf0, // VUNPCKLPS XMM14, XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_xmm15_xmm14_xmm13() {
    let mut emu = emu64();
    // VUNPCKLPS XMM15, XMM14, XMM13
    let code = [
        0xc4, 0xc1, 0x08, 0x14, 0xfd, // VUNPCKLPS XMM15, XMM14, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKLPS 256-bit Tests - Unpack Low Single-Precision (YMM)
// ============================================================================

#[test]
fn test_vunpcklps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VUNPCKLPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x14, 0xc2, // VUNPCKLPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VUNPCKLPS YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xec, 0x14, 0xcb, // VUNPCKLPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VUNPCKLPS YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe4, 0x14, 0xd4, // VUNPCKLPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VUNPCKLPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x14, 0xdd, // VUNPCKLPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VUNPCKLPS YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd4, 0x14, 0xe6, // VUNPCKLPS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VUNPCKLPS YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcc, 0x14, 0xef, // VUNPCKLPS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VUNPCKLPS YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc4, 0x14, 0xf0, // VUNPCKLPS YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VUNPCKLPS YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x14, 0xf9, // VUNPCKLPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm0_ymm0_ymm0() {
    let mut emu = emu64();
    // VUNPCKLPS YMM0, YMM0, YMM0 - duplicate low elements
    let code = [
        0xc5, 0xfc, 0x14, 0xc0, // VUNPCKLPS YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VUNPCKLPS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0xc1, 0x34, 0x14, 0xc2, // VUNPCKLPS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VUNPCKLPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x14, 0xdd, // VUNPCKLPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm14_ymm15_ymm0() {
    let mut emu = emu64();
    // VUNPCKLPS YMM14, YMM15, YMM0
    let code = [
        0xc4, 0xe1, 0x04, 0x14, 0xf0, // VUNPCKLPS YMM14, YMM15, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklps_ymm15_ymm14_ymm13() {
    let mut emu = emu64();
    // VUNPCKLPS YMM15, YMM14, YMM13
    let code = [
        0xc4, 0xc1, 0x0c, 0x14, 0xfd, // VUNPCKLPS YMM15, YMM14, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKLPS Memory Tests
// ============================================================================

#[test]
fn test_vunpcklps_xmm0_xmm1_mem128() {
    let mut emu = emu64();
    // VUNPCKLPS XMM0, XMM1, [mem128]
    let code = [
        0xc5, 0xf0, 0x14, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKLPS XMM0, XMM1, [rip + 0x4000]
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
fn test_vunpcklps_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    // VUNPCKLPS YMM0, YMM1, [mem256]
    let code = [
        0xc5, 0xf4, 0x14, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKLPS YMM0, YMM1, [rip + 0x4000]
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
fn test_vunpcklps_xmm8_xmm9_mem128() {
    let mut emu = emu64();
    // VUNPCKLPS XMM8, XMM9, [mem128]
    let code = [
        0xc4, 0xc1, 0x30, 0x14, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKLPS XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKLPD 128-bit Tests - Unpack Low Double-Precision (XMM)
// ============================================================================

#[test]
fn test_vunpcklpd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VUNPCKLPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x14, 0xc2, // VUNPCKLPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VUNPCKLPD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe9, 0x14, 0xcb, // VUNPCKLPD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VUNPCKLPD XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe1, 0x14, 0xd4, // VUNPCKLPD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VUNPCKLPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x14, 0xdd, // VUNPCKLPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VUNPCKLPD XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd1, 0x14, 0xe6, // VUNPCKLPD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VUNPCKLPD XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc9, 0x14, 0xef, // VUNPCKLPD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VUNPCKLPD XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc1, 0x14, 0xf0, // VUNPCKLPD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VUNPCKLPD XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x14, 0xf9, // VUNPCKLPD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm0_xmm0_xmm0() {
    let mut emu = emu64();
    // VUNPCKLPD XMM0, XMM0, XMM0 - duplicate low element
    let code = [
        0xc5, 0xf9, 0x14, 0xc0, // VUNPCKLPD XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VUNPCKLPD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0xc1, 0x31, 0x14, 0xc2, // VUNPCKLPD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VUNPCKLPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x14, 0xdd, // VUNPCKLPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm14_xmm15_xmm0() {
    let mut emu = emu64();
    // VUNPCKLPD XMM14, XMM15, XMM0
    let code = [
        0xc4, 0xe1, 0x01, 0x14, 0xf0, // VUNPCKLPD XMM14, XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_xmm15_xmm14_xmm13() {
    let mut emu = emu64();
    // VUNPCKLPD XMM15, XMM14, XMM13
    let code = [
        0xc4, 0xc1, 0x09, 0x14, 0xfd, // VUNPCKLPD XMM15, XMM14, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKLPD 256-bit Tests - Unpack Low Double-Precision (YMM)
// ============================================================================

#[test]
fn test_vunpcklpd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VUNPCKLPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x14, 0xc2, // VUNPCKLPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VUNPCKLPD YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xed, 0x14, 0xcb, // VUNPCKLPD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VUNPCKLPD YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe5, 0x14, 0xd4, // VUNPCKLPD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VUNPCKLPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x14, 0xdd, // VUNPCKLPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VUNPCKLPD YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd5, 0x14, 0xe6, // VUNPCKLPD YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VUNPCKLPD YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcd, 0x14, 0xef, // VUNPCKLPD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VUNPCKLPD YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc5, 0x14, 0xf0, // VUNPCKLPD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VUNPCKLPD YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x14, 0xf9, // VUNPCKLPD YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm0_ymm0_ymm0() {
    let mut emu = emu64();
    // VUNPCKLPD YMM0, YMM0, YMM0 - duplicate low elements
    let code = [
        0xc5, 0xfd, 0x14, 0xc0, // VUNPCKLPD YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VUNPCKLPD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0xc1, 0x35, 0x14, 0xc2, // VUNPCKLPD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VUNPCKLPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x14, 0xdd, // VUNPCKLPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm14_ymm15_ymm0() {
    let mut emu = emu64();
    // VUNPCKLPD YMM14, YMM15, YMM0
    let code = [
        0xc4, 0xe1, 0x05, 0x14, 0xf0, // VUNPCKLPD YMM14, YMM15, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vunpcklpd_ymm15_ymm14_ymm13() {
    let mut emu = emu64();
    // VUNPCKLPD YMM15, YMM14, YMM13
    let code = [
        0xc4, 0xc1, 0x0d, 0x14, 0xfd, // VUNPCKLPD YMM15, YMM14, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VUNPCKLPD Memory Tests
// ============================================================================

#[test]
fn test_vunpcklpd_xmm0_xmm1_mem128() {
    let mut emu = emu64();
    // VUNPCKLPD XMM0, XMM1, [mem128]
    let code = [
        0xc5, 0xf1, 0x14, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKLPD XMM0, XMM1, [rip + 0x4000]
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
fn test_vunpcklpd_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    // VUNPCKLPD YMM0, YMM1, [mem256]
    let code = [
        0xc5, 0xf5, 0x14, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKLPD YMM0, YMM1, [rip + 0x4000]
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
fn test_vunpcklpd_xmm8_xmm9_mem128() {
    let mut emu = emu64();
    // VUNPCKLPD XMM8, XMM9, [mem128]
    let code = [
        0xc4, 0xc1, 0x31, 0x14, 0x05, 0x00, 0x40, 0x00, 0x00, // VUNPCKLPD XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
