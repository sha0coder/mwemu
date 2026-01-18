use crate::*;

// VTESTPS - Packed Bit Test for Single-Precision Floating-Point Values
// VTESTPD - Packed Bit Test for Double-Precision Floating-Point Values
//
// VTESTPS/VTESTPD perform a bitwise AND and ANDN operation between two operands,
// set ZF if the result of AND is all zeros, and set CF if the result of ANDN is all zeros.
//
// The instructions compute:
// - TEMP1 = SRC1 AND SRC2
// - TEMP2 = (NOT SRC1) AND SRC2
// - ZF = (TEMP1 == 0)  // All bits are zero after AND
// - CF = (TEMP2 == 0)  // All bits are zero after ANDN
//
// This is commonly used for:
// - Testing if any bits are set (ZF=0 means at least one bit matched)
// - Testing if all bits are set in masked region (CF=1 means all masked bits are set)
//
// Opcodes:
// VEX.128.66.0F38.W0 0E /r   VTESTPS xmm1, xmm2/m128   - Test 128-bit packed singles
// VEX.256.66.0F38.W0 0E /r   VTESTPS ymm1, ymm2/m256   - Test 256-bit packed singles
// VEX.128.66.0F38.W0 0F /r   VTESTPD xmm1, xmm2/m128   - Test 128-bit packed doubles
// VEX.256.66.0F38.W0 0F /r   VTESTPD ymm1, ymm2/m256   - Test 256-bit packed doubles

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VTESTPS Tests - 128-bit XMM registers
// ============================================================================

#[test]
fn test_vtestps_xmm0_xmm1() {
    let mut emu = emu64();
    // VTESTPS XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xc1, // VTESTPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm1_xmm2() {
    let mut emu = emu64();
    // VTESTPS XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xca, // VTESTPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm2_xmm3() {
    let mut emu = emu64();
    // VTESTPS XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xd3, // VTESTPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm3_xmm4() {
    let mut emu = emu64();
    // VTESTPS XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xdc, // VTESTPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm4_xmm5() {
    let mut emu = emu64();
    // VTESTPS XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xe5, // VTESTPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm5_xmm6() {
    let mut emu = emu64();
    // VTESTPS XMM5, XMM6
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xee, // VTESTPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm6_xmm7() {
    let mut emu = emu64();
    // VTESTPS XMM6, XMM7
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xf7, // VTESTPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm7_xmm0() {
    let mut emu = emu64();
    // VTESTPS XMM7, XMM0
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xf8, // VTESTPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vtestps_xmm8_xmm9() {
    let mut emu = emu64();
    // VTESTPS XMM8, XMM9
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xc1, // VTESTPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm9_xmm10() {
    let mut emu = emu64();
    // VTESTPS XMM9, XMM10
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xca, // VTESTPS XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm10_xmm11() {
    let mut emu = emu64();
    // VTESTPS XMM10, XMM11
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xd3, // VTESTPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm11_xmm12() {
    let mut emu = emu64();
    // VTESTPS XMM11, XMM12
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xdc, // VTESTPS XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm12_xmm13() {
    let mut emu = emu64();
    // VTESTPS XMM12, XMM13
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xe5, // VTESTPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm13_xmm14() {
    let mut emu = emu64();
    // VTESTPS XMM13, XMM14
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xee, // VTESTPS XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm14_xmm15() {
    let mut emu = emu64();
    // VTESTPS XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xf7, // VTESTPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm15_xmm8() {
    let mut emu = emu64();
    // VTESTPS XMM15, XMM8
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xf8, // VTESTPS XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPS Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vtestps_xmm0_xmm8() {
    let mut emu = emu64();
    // VTESTPS XMM0, XMM8
    let code = [
        0xc4, 0xc2, 0x79, 0x0e, 0xc0, // VTESTPS XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm8_xmm0() {
    let mut emu = emu64();
    // VTESTPS XMM8, XMM0
    let code = [
        0xc4, 0x42, 0x79, 0x0e, 0xc0, // VTESTPS XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm7_xmm15() {
    let mut emu = emu64();
    // VTESTPS XMM7, XMM15
    let code = [
        0xc4, 0xc2, 0x79, 0x0e, 0xff, // VTESTPS XMM7, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vtestps_xmm0_mem() {
    let mut emu = emu64();
    // VTESTPS XMM0, [mem]
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0x05, 0x00, 0x40, 0x00, 0x00, // VTESTPS XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm1_mem() {
    let mut emu = emu64();
    // VTESTPS XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0x0d, 0x00, 0x40, 0x00, 0x00, // VTESTPS XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_xmm8_mem() {
    let mut emu = emu64();
    // VTESTPS XMM8, [mem]
    let code = [
        0xc4, 0x62, 0x79, 0x0e, 0x05, 0x00, 0x40, 0x00, 0x00, // VTESTPS XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPS Tests - 256-bit YMM registers
// ============================================================================

#[test]
fn test_vtestps_ymm0_ymm1() {
    let mut emu = emu64();
    // VTESTPS YMM0, YMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0xc1, // VTESTPS YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm1_ymm2() {
    let mut emu = emu64();
    // VTESTPS YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0xca, // VTESTPS YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm2_ymm3() {
    let mut emu = emu64();
    // VTESTPS YMM2, YMM3
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0xd3, // VTESTPS YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm3_ymm4() {
    let mut emu = emu64();
    // VTESTPS YMM3, YMM4
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0xdc, // VTESTPS YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm4_ymm5() {
    let mut emu = emu64();
    // VTESTPS YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0xe5, // VTESTPS YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm5_ymm6() {
    let mut emu = emu64();
    // VTESTPS YMM5, YMM6
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0xee, // VTESTPS YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm6_ymm7() {
    let mut emu = emu64();
    // VTESTPS YMM6, YMM7
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0xf7, // VTESTPS YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm7_ymm0() {
    let mut emu = emu64();
    // VTESTPS YMM7, YMM0
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0xf8, // VTESTPS YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm8_ymm9() {
    let mut emu = emu64();
    // VTESTPS YMM8, YMM9
    let code = [
        0xc4, 0x42, 0x7d, 0x0e, 0xc1, // VTESTPS YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm15_ymm14() {
    let mut emu = emu64();
    // VTESTPS YMM15, YMM14
    let code = [
        0xc4, 0x42, 0x7d, 0x0e, 0xfe, // VTESTPS YMM15, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vtestps_ymm0_mem() {
    let mut emu = emu64();
    // VTESTPS YMM0, [mem]
    let code = [
        0xc4, 0xe2, 0x7d, 0x0e, 0x05, 0x00, 0x40, 0x00, 0x00, // VTESTPS YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_ymm8_mem() {
    let mut emu = emu64();
    // VTESTPS YMM8, [mem]
    let code = [
        0xc4, 0x62, 0x7d, 0x0e, 0x05, 0x00, 0x40, 0x00, 0x00, // VTESTPS YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPD Tests - 128-bit XMM registers
// ============================================================================

#[test]
fn test_vtestpd_xmm0_xmm1() {
    let mut emu = emu64();
    // VTESTPD XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xc1, // VTESTPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm1_xmm2() {
    let mut emu = emu64();
    // VTESTPD XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xca, // VTESTPD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm2_xmm3() {
    let mut emu = emu64();
    // VTESTPD XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xd3, // VTESTPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm3_xmm4() {
    let mut emu = emu64();
    // VTESTPD XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xdc, // VTESTPD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm4_xmm5() {
    let mut emu = emu64();
    // VTESTPD XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xe5, // VTESTPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm5_xmm6() {
    let mut emu = emu64();
    // VTESTPD XMM5, XMM6
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xee, // VTESTPD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm6_xmm7() {
    let mut emu = emu64();
    // VTESTPD XMM6, XMM7
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xf7, // VTESTPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm7_xmm0() {
    let mut emu = emu64();
    // VTESTPD XMM7, XMM0
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xf8, // VTESTPD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPD Tests - Extended XMM registers
// ============================================================================

#[test]
fn test_vtestpd_xmm8_xmm9() {
    let mut emu = emu64();
    // VTESTPD XMM8, XMM9
    let code = [
        0xc4, 0x42, 0x79, 0x0f, 0xc1, // VTESTPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm9_xmm10() {
    let mut emu = emu64();
    // VTESTPD XMM9, XMM10
    let code = [
        0xc4, 0x42, 0x79, 0x0f, 0xca, // VTESTPD XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm10_xmm11() {
    let mut emu = emu64();
    // VTESTPD XMM10, XMM11
    let code = [
        0xc4, 0x42, 0x79, 0x0f, 0xd3, // VTESTPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm15_xmm14() {
    let mut emu = emu64();
    // VTESTPD XMM15, XMM14
    let code = [
        0xc4, 0x42, 0x79, 0x0f, 0xfe, // VTESTPD XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPD Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vtestpd_xmm0_mem() {
    let mut emu = emu64();
    // VTESTPD XMM0, [mem]
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0x05, 0x00, 0x40, 0x00, 0x00, // VTESTPD XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_xmm8_mem() {
    let mut emu = emu64();
    // VTESTPD XMM8, [mem]
    let code = [
        0xc4, 0x62, 0x79, 0x0f, 0x05, 0x00, 0x40, 0x00, 0x00, // VTESTPD XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPD Tests - 256-bit YMM registers
// ============================================================================

#[test]
fn test_vtestpd_ymm0_ymm1() {
    let mut emu = emu64();
    // VTESTPD YMM0, YMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x0f, 0xc1, // VTESTPD YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_ymm1_ymm2() {
    let mut emu = emu64();
    // VTESTPD YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0x7d, 0x0f, 0xca, // VTESTPD YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_ymm2_ymm3() {
    let mut emu = emu64();
    // VTESTPD YMM2, YMM3
    let code = [
        0xc4, 0xe2, 0x7d, 0x0f, 0xd3, // VTESTPD YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_ymm3_ymm4() {
    let mut emu = emu64();
    // VTESTPD YMM3, YMM4
    let code = [
        0xc4, 0xe2, 0x7d, 0x0f, 0xdc, // VTESTPD YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_ymm4_ymm5() {
    let mut emu = emu64();
    // VTESTPD YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x7d, 0x0f, 0xe5, // VTESTPD YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_ymm8_ymm9() {
    let mut emu = emu64();
    // VTESTPD YMM8, YMM9
    let code = [
        0xc4, 0x42, 0x7d, 0x0f, 0xc1, // VTESTPD YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_ymm15_ymm8() {
    let mut emu = emu64();
    // VTESTPD YMM15, YMM8
    let code = [
        0xc4, 0x42, 0x7d, 0x0f, 0xf8, // VTESTPD YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VTESTPD Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vtestpd_ymm0_mem() {
    let mut emu = emu64();
    // VTESTPD YMM0, [mem]
    let code = [
        0xc4, 0xe2, 0x7d, 0x0f, 0x05, 0x00, 0x40, 0x00, 0x00, // VTESTPD YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_ymm8_mem() {
    let mut emu = emu64();
    // VTESTPD YMM8, [mem]
    let code = [
        0xc4, 0x62, 0x7d, 0x0f, 0x05, 0x00, 0x40, 0x00, 0x00, // VTESTPD YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// Combined tests with comparison operations
// ============================================================================

#[test]
fn test_vtestps_after_vcmpps() {
    let mut emu = emu64();
    // VCMPPS followed by VTESTPS
    let code = [
        0xc5, 0xf0, 0xc2, 0xc2, 0x00, // VCMPPS XMM0, XMM1, XMM2, 0 (EQ)
        0xc4, 0xe2, 0x79, 0x0e, 0xc0, // VTESTPS XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_after_vcmppd() {
    let mut emu = emu64();
    // VCMPPD followed by VTESTPD
    let code = [
        0xc5, 0xf1, 0xc2, 0xc2, 0x00, // VCMPPD XMM0, XMM1, XMM2, 0 (EQ)
        0xc4, 0xe2, 0x79, 0x0f, 0xc0, // VTESTPD XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestps_multiple_tests() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x0e, 0xc1, // VTESTPS XMM0, XMM1
        0xc4, 0xe2, 0x79, 0x0e, 0xd3, // VTESTPS XMM2, XMM3
        0xc4, 0xe2, 0x79, 0x0e, 0xe5, // VTESTPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vtestpd_multiple_tests() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x0f, 0xc1, // VTESTPD XMM0, XMM1
        0xc4, 0xe2, 0x79, 0x0f, 0xd3, // VTESTPD XMM2, XMM3
        0xc4, 0xe2, 0x79, 0x0f, 0xe5, // VTESTPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
